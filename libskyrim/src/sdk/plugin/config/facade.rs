use core::str::FromStr;

use crate::ini::Section;
use crate::re::{FormCastable, TESForm};
use crate::sdk::core::GamePtr;
use crate::sdk::forms::lookup;

use super::{HotkeyCombo, Ini};

/// Borrowed typed facade over one loaded [`Ini`].
///
/// This is the preferred API when a plugin loads one INI and then reads
/// several related fields from it. The facade keeps all lookups anchored to
/// one borrowed source while exposing typed convenience methods for:
///
/// - scalar values such as [`Config::bool`] or [`Config::f32`]
/// - form-spec driven lookups such as [`Config::form_typed`]
/// - user-facing hotkey fields through [`Config::hotkey`]
///
/// Example:
///
/// ```rust,ignore
/// use libskyrim::sdk::plugin::config::{Config, load_ini};
///
/// fn load_settings() -> Result<(bool, f32), ()> {
///     let ini = load_ini("Data/SKSE/Plugins/Example.ini")?;
///     let cfg = Config::new(&ini);
///     Ok((
///         cfg.bool_or("General", "Enabled", true),
///         cfg.f32_or("Gameplay", "RangeMultiplier", 1.0),
///     ))
/// }
/// ```
#[derive(Clone, Copy)]
pub struct Config<'a> {
    ini: &'a Ini,
}

/// Failure to read one config field.
///
/// The error stays intentionally small and field-local so config-heavy code can
/// either bubble it up unchanged or cheaply map it into plugin-specific error
/// reporting.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConfigValueError {
    MissingSection,
    MissingField,
    InvalidValue,
}

impl<'a> Config<'a> {
    /// Builds a borrowed config facade over one loaded INI.
    #[inline(always)]
    pub const fn new(ini: &'a Ini) -> Self {
        Self { ini }
    }

    /// Returns the borrowed underlying INI.
    #[inline(always)]
    pub const fn ini(self) -> &'a Ini {
        self.ini
    }

    /// Returns whether the INI contains `section`.
    #[inline(always)]
    pub fn has_section(self, section: &str) -> bool {
        self.ini.section(section).is_ok()
    }

    /// Returns whether the INI contains `field` in `section`.
    #[inline(always)]
    pub fn has_field(self, section: &str, field: &str) -> bool {
        self.raw(section, field).is_some()
    }

    /// Returns a borrowed section view when the section exists.
    #[inline(always)]
    pub fn section(self, section: &str) -> Option<Section<'a>> {
        self.ini.section(section).ok()
    }

    /// Returns the raw string value for one field.
    #[inline(always)]
    pub fn raw(self, section: &str, field: &str) -> Option<&'a str> {
        self.ini.get(section, field)
    }

    /// Returns one required raw field value.
    ///
    /// This is the low-level borrowed entrypoint behind the typed helpers.
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

    /// Parses one required field through [`FromStr`].
    ///
    /// This is the generic fallback behind the more specific numeric helpers.
    #[inline]
    pub fn parsed<T: FromStr>(self, section: &str, field: &str) -> Result<T, ConfigValueError> {
        self.require(section, field)?
            .trim()
            .parse::<T>()
            .map_err(|_| ConfigValueError::InvalidValue)
    }

    /// Parses one field or falls back to `default`.
    ///
    /// This is useful for lightweight install paths that want defaults without
    /// modeling a richer plugin-local config type.
    #[inline]
    pub fn parsed_or<T: FromStr + Copy>(self, section: &str, field: &str, default: T) -> T {
        self.parsed(section, field).unwrap_or(default)
    }

    /// Returns one raw string value or falls back to `default`.
    ///
    /// Prefer this when later code still wants to decide how to interpret the
    /// string instead of committing to one typed parse here.
    #[inline]
    pub fn string_or(self, section: &str, field: &str, default: &'a str) -> &'a str {
        self.raw(section, field).unwrap_or(default)
    }

    /// Parses one boolean field using common INI spellings like `true/false`,
    /// `yes/no`, `on/off`, and `1/0`.
    #[inline]
    pub fn bool(self, section: &str, field: &str) -> Result<bool, ConfigValueError> {
        parse_bool_value(self.require(section, field)?).ok_or(ConfigValueError::InvalidValue)
    }

    /// Parses one boolean field or falls back to `default`.
    #[inline]
    pub fn bool_or(self, section: &str, field: &str, default: bool) -> bool {
        self.bool(section, field).unwrap_or(default)
    }

    /// Parses one signed integer field.
    #[inline(always)]
    pub fn i32(self, section: &str, field: &str) -> Result<i32, ConfigValueError> {
        self.parsed(section, field)
    }

    /// Parses one signed integer field or falls back to `default`.
    #[inline(always)]
    pub fn i32_or(self, section: &str, field: &str, default: i32) -> i32 {
        self.parsed_or(section, field, default)
    }

    /// Parses one unsigned integer field.
    #[inline(always)]
    pub fn u32(self, section: &str, field: &str) -> Result<u32, ConfigValueError> {
        self.parsed(section, field)
    }

    /// Parses one unsigned integer field or falls back to `default`.
    #[inline(always)]
    pub fn u32_or(self, section: &str, field: &str, default: u32) -> u32 {
        self.parsed_or(section, field, default)
    }

    /// Parses one floating-point field.
    #[inline(always)]
    pub fn f32(self, section: &str, field: &str) -> Result<f32, ConfigValueError> {
        self.parsed(section, field)
    }

    /// Parses one floating-point field or falls back to `default`.
    #[inline(always)]
    pub fn f32_or(self, section: &str, field: &str, default: f32) -> f32 {
        self.parsed_or(section, field, default)
    }

    /// Resolves one form-spec field into an untyped `TESForm`.
    ///
    /// This is useful when config validation only needs "some form exists" and
    /// the plugin will decide the concrete type later.
    #[inline(always)]
    pub fn form(self, section: &str, field: &str) -> Result<GamePtr<TESForm>, ConfigValueError> {
        Ok(lookup::lookup_form_spec(self.require(section, field)?))
    }

    /// Resolves one form-spec field and downcasts it to `T`.
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

    /// Resolves one raw form-spec field without editor-id conveniences.
    ///
    /// Use this when the plugin wants a strict `plugin:FormID`-style path and
    /// should not accept editor-ID lookup.
    #[inline(always)]
    pub fn form_raw(
        self,
        section: &str,
        field: &str,
    ) -> Result<GamePtr<TESForm>, ConfigValueError> {
        Ok(lookup::lookup_raw_form_spec(self.require(section, field)?))
    }

    /// Resolves one raw form-spec field and downcasts it to `T`.
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

    /// Parses one hotkey field into a validated [`HotkeyCombo`].
    #[inline]
    pub fn hotkey(self, section: &str, field: &str) -> Result<HotkeyCombo, ConfigValueError> {
        HotkeyCombo::parse(self.require(section, field)?)
            .map_err(|_| ConfigValueError::InvalidValue)
    }

    /// Parses one hotkey field or falls back to a default combo string.
    ///
    /// This is intended for user-facing defaults that should still route
    /// through the normal hotkey parser instead of hard-coding event logic.
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

/// Returns a borrowed [`Config`] facade for one loaded INI.
///
/// This is convenient for free-function style modules that still want access to
/// the borrowed typed facade without naming `Config::new(...)` explicitly.
#[inline(always)]
pub fn config(ini: &Ini) -> Config<'_> {
    Config::new(ini)
}

/// Loads one INI file from disk into an owned [`Ini`] value.
#[inline(always)]
pub fn load_ini(path: &str) -> Result<Ini, ()> {
    Ini::from_path_str(path)
}

/// Persists one [`Ini`] back to disk.
#[inline(always)]
pub fn write_ini(ini: &Ini, path: &str) -> Result<(), core::fmt::Error> {
    ini.write_file_str(path)
}

/// Returns whether the INI contains `section`.
#[inline(always)]
pub fn has_section(ini: &Ini, section: &str) -> bool {
    config(ini).has_section(section)
}

/// Returns whether the INI contains `field` in `section`.
#[inline(always)]
pub fn has_field(ini: &Ini, section: &str, field: &str) -> bool {
    config(ini).has_field(section, field)
}

/// Returns the raw string value for one field.
#[inline(always)]
pub fn raw_value<'a>(ini: &'a Ini, section: &str, field: &str) -> Option<&'a str> {
    config(ini).raw(section, field)
}

/// Parses one boolean field from the INI.
#[inline(always)]
pub fn bool_value(ini: &Ini, section: &str, field: &str) -> Result<bool, ConfigValueError> {
    config(ini).bool(section, field)
}

/// Parses one signed integer field from the INI.
#[inline(always)]
pub fn i32_value(ini: &Ini, section: &str, field: &str) -> Result<i32, ConfigValueError> {
    config(ini).i32(section, field)
}

/// Parses one unsigned integer field from the INI.
#[inline(always)]
pub fn u32_value(ini: &Ini, section: &str, field: &str) -> Result<u32, ConfigValueError> {
    config(ini).u32(section, field)
}

/// Parses one floating-point field from the INI.
#[inline(always)]
pub fn f32_value(ini: &Ini, section: &str, field: &str) -> Result<f32, ConfigValueError> {
    config(ini).f32(section, field)
}

/// Resolves one form-spec field into a generic `TESForm` pointer.
#[inline(always)]
pub fn form_value(
    ini: &Ini,
    section: &str,
    field: &str,
) -> Result<GamePtr<TESForm>, ConfigValueError> {
    config(ini).form(section, field)
}

/// Resolves one form-spec field and downcasts it to `T`.
#[inline(always)]
pub fn form_value_typed<T: FormCastable>(
    ini: &Ini,
    section: &str,
    field: &str,
) -> Result<GamePtr<T>, ConfigValueError> {
    config(ini).form_typed::<T>(section, field)
}

/// Parses one hotkey field into a validated [`HotkeyCombo`].
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
