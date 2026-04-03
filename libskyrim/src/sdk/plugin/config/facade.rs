use core::str::FromStr;

use crate::ini::Section;
use crate::re::{FormCastable, TESForm};
use crate::sdk::core::GamePtr;
use crate::sdk::forms::lookup;

use super::{HotkeyCombo, Ini};

/// Borrowed typed facade over one loaded [`Ini`].
#[derive(Clone, Copy)]
pub struct Config<'a> {
    ini: &'a Ini,
}

/// Failure to read one config field.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConfigValueError {
    MissingSection,
    MissingField,
    InvalidValue,
}

impl<'a> Config<'a> {
    #[inline(always)]
    pub const fn new(ini: &'a Ini) -> Self {
        Self { ini }
    }

    #[inline(always)]
    pub const fn ini(self) -> &'a Ini {
        self.ini
    }

    #[inline(always)]
    pub fn has_section(self, section: &str) -> bool {
        self.ini.section(section).is_ok()
    }

    #[inline(always)]
    pub fn has_field(self, section: &str, field: &str) -> bool {
        self.raw(section, field).is_some()
    }

    #[inline(always)]
    pub fn section(self, section: &str) -> Option<Section<'a>> {
        self.ini.section(section).ok()
    }

    #[inline(always)]
    pub fn raw(self, section: &str, field: &str) -> Option<&'a str> {
        self.ini.get(section, field)
    }

    #[inline]
    pub fn require(self, section: &str, field: &str) -> Result<&'a str, ConfigValueError> {
        let section = self
            .ini
            .section(section)
            .map_err(|_| ConfigValueError::MissingSection)?;
        let field = section
            .field(field)
            .map_err(|_| ConfigValueError::MissingField)?;
        field.value().ok_or(ConfigValueError::InvalidValue)
    }

    #[inline]
    pub fn parsed<T: FromStr>(self, section: &str, field: &str) -> Result<T, ConfigValueError> {
        self.require(section, field)?
            .trim()
            .parse::<T>()
            .map_err(|_| ConfigValueError::InvalidValue)
    }

    #[inline]
    pub fn parsed_or<T: FromStr + Copy>(self, section: &str, field: &str, default: T) -> T {
        self.parsed(section, field).unwrap_or(default)
    }

    #[inline]
    pub fn string_or(self, section: &str, field: &str, default: &'a str) -> &'a str {
        self.raw(section, field).unwrap_or(default)
    }

    #[inline]
    pub fn bool(self, section: &str, field: &str) -> Result<bool, ConfigValueError> {
        parse_bool_value(self.require(section, field)?).ok_or(ConfigValueError::InvalidValue)
    }

    #[inline]
    pub fn bool_or(self, section: &str, field: &str, default: bool) -> bool {
        self.bool(section, field).unwrap_or(default)
    }

    #[inline(always)]
    pub fn i32(self, section: &str, field: &str) -> Result<i32, ConfigValueError> {
        self.parsed(section, field)
    }

    #[inline(always)]
    pub fn i32_or(self, section: &str, field: &str, default: i32) -> i32 {
        self.parsed_or(section, field, default)
    }

    #[inline(always)]
    pub fn u32(self, section: &str, field: &str) -> Result<u32, ConfigValueError> {
        self.parsed(section, field)
    }

    #[inline(always)]
    pub fn u32_or(self, section: &str, field: &str, default: u32) -> u32 {
        self.parsed_or(section, field, default)
    }

    #[inline(always)]
    pub fn f32(self, section: &str, field: &str) -> Result<f32, ConfigValueError> {
        self.parsed(section, field)
    }

    #[inline(always)]
    pub fn f32_or(self, section: &str, field: &str, default: f32) -> f32 {
        self.parsed_or(section, field, default)
    }

    #[inline(always)]
    pub fn form(self, section: &str, field: &str) -> Result<GamePtr<TESForm>, ConfigValueError> {
        Ok(lookup::lookup_form_spec(self.require(section, field)?))
    }

    #[inline(always)]
    pub fn form_typed<T: FormCastable>(
        self,
        section: &str,
        field: &str,
    ) -> Result<GamePtr<T>, ConfigValueError> {
        Ok(lookup::lookup_form_spec_typed::<T>(
            self.require(section, field)?,
        ))
    }

    #[inline(always)]
    pub fn form_raw(
        self,
        section: &str,
        field: &str,
    ) -> Result<GamePtr<TESForm>, ConfigValueError> {
        Ok(lookup::lookup_raw_form_spec(self.require(section, field)?))
    }

    #[inline(always)]
    pub fn form_raw_typed<T: FormCastable>(
        self,
        section: &str,
        field: &str,
    ) -> Result<GamePtr<T>, ConfigValueError> {
        Ok(lookup::lookup_raw_form_spec_typed::<T>(
            self.require(section, field)?,
        ))
    }

    #[inline]
    pub fn hotkey(self, section: &str, field: &str) -> Result<HotkeyCombo, ConfigValueError> {
        HotkeyCombo::parse(self.require(section, field)?)
            .map_err(|_| ConfigValueError::InvalidValue)
    }

    #[inline]
    pub fn hotkey_or(self, section: &str, field: &str, default: &str) -> HotkeyCombo {
        self.hotkey(section, field)
            .unwrap_or_else(|_| HotkeyCombo::parse(default).unwrap_or_default())
    }
}

impl<'a> From<&'a Ini> for Config<'a> {
    #[inline(always)]
    fn from(value: &'a Ini) -> Self {
        Self::new(value)
    }
}

#[inline(always)]
pub fn config(ini: &Ini) -> Config<'_> {
    Config::new(ini)
}

#[inline(always)]
pub fn load_ini(path: &str) -> Result<Ini, ()> {
    Ini::from_path_str(path)
}

#[inline(always)]
pub fn write_ini(ini: &Ini, path: &str) -> Result<(), core::fmt::Error> {
    ini.write_file_str(path)
}

#[inline(always)]
pub fn has_section(ini: &Ini, section: &str) -> bool {
    config(ini).has_section(section)
}

#[inline(always)]
pub fn has_field(ini: &Ini, section: &str, field: &str) -> bool {
    config(ini).has_field(section, field)
}

#[inline(always)]
pub fn raw_value<'a>(ini: &'a Ini, section: &str, field: &str) -> Option<&'a str> {
    config(ini).raw(section, field)
}

#[inline(always)]
pub fn bool_value(ini: &Ini, section: &str, field: &str) -> Result<bool, ConfigValueError> {
    config(ini).bool(section, field)
}

#[inline(always)]
pub fn i32_value(ini: &Ini, section: &str, field: &str) -> Result<i32, ConfigValueError> {
    config(ini).i32(section, field)
}

#[inline(always)]
pub fn u32_value(ini: &Ini, section: &str, field: &str) -> Result<u32, ConfigValueError> {
    config(ini).u32(section, field)
}

#[inline(always)]
pub fn f32_value(ini: &Ini, section: &str, field: &str) -> Result<f32, ConfigValueError> {
    config(ini).f32(section, field)
}

#[inline(always)]
pub fn form_value(
    ini: &Ini,
    section: &str,
    field: &str,
) -> Result<GamePtr<TESForm>, ConfigValueError> {
    config(ini).form(section, field)
}

#[inline(always)]
pub fn form_value_typed<T: FormCastable>(
    ini: &Ini,
    section: &str,
    field: &str,
) -> Result<GamePtr<T>, ConfigValueError> {
    config(ini).form_typed::<T>(section, field)
}

#[inline(always)]
pub fn hotkey_value(
    ini: &Ini,
    section: &str,
    field: &str,
) -> Result<HotkeyCombo, ConfigValueError> {
    config(ini).hotkey(section, field)
}

#[inline]
fn parse_bool_value(value: &str) -> Option<bool> {
    let value = value.trim();
    if value.eq_ignore_ascii_case("true")
        || value.eq_ignore_ascii_case("yes")
        || value.eq_ignore_ascii_case("on")
        || value == "1"
    {
        Some(true)
    } else if value.eq_ignore_ascii_case("false")
        || value.eq_ignore_ascii_case("no")
        || value.eq_ignore_ascii_case("off")
        || value == "0"
    {
        Some(false)
    } else {
        None
    }
}
