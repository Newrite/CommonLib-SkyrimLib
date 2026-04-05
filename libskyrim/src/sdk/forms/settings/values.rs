use alloc::string::String;

use crate::re::{Setting, SettingType};

/// Borrowed, type-tagged view of a [`Setting`] value.
///
/// Use this when code already has a `Setting` borrow and wants to inspect or
/// apply a value without cloning owned data unnecessarily.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SettingValueRef<'a> {
    Bool(bool),
    Character(i8),
    UnsignedCharacter(u8),
    Integer(i32),
    UnsignedInteger(u32),
    Float(f32),
    String(&'a str),
    ColorRgb(u32),
    ColorRgba(u32),
    Unknown,
}

/// Owned snapshot of a [`Setting`] value.
///
/// This is useful for caches, diagnostics, or APIs that should not keep a
/// borrow to the original [`Setting`].
#[derive(Debug, Clone, PartialEq)]
pub enum SettingValue {
    Bool(bool),
    Character(i8),
    UnsignedCharacter(u8),
    Integer(i32),
    UnsignedInteger(u32),
    Float(f32),
    String(String),
    ColorRgb(u32),
    ColorRgba(u32),
    Unknown,
}

impl<'a> From<SettingValueRef<'a>> for SettingValue {
    fn from(value: SettingValueRef<'a>) -> Self {
        match value {
            SettingValueRef::Bool(value) => Self::Bool(value),
            SettingValueRef::Character(value) => Self::Character(value),
            SettingValueRef::UnsignedCharacter(value) => Self::UnsignedCharacter(value),
            SettingValueRef::Integer(value) => Self::Integer(value),
            SettingValueRef::UnsignedInteger(value) => Self::UnsignedInteger(value),
            SettingValueRef::Float(value) => Self::Float(value),
            SettingValueRef::String(value) => Self::String(value.into()),
            SettingValueRef::ColorRgb(value) => Self::ColorRgb(value),
            SettingValueRef::ColorRgba(value) => Self::ColorRgba(value),
            SettingValueRef::Unknown => Self::Unknown,
        }
    }
}

/// Returns the canonical setting name.
#[inline(always)]
pub fn setting_name(setting: &Setting) -> &str {
    setting.get_name_as_str()
}

/// Returns the engine-reported runtime type of the setting.
#[inline(always)]
pub fn setting_type(setting: &Setting) -> SettingType {
    setting.get_type()
}

/// Returns `true` when the setting belongs to the preference store.
#[inline(always)]
pub fn is_preference_setting(setting: &Setting) -> bool {
    setting.is_preference_setting()
}

/// Borrows the current setting value in a type-tagged form.
///
/// This is the main read path when a caller wants to branch on the runtime
/// [`SettingType`] without manually touching the raw engine unions.
pub fn setting_value_ref(setting: &Setting) -> SettingValueRef<'_> {
    match setting.get_type() {
        SettingType::Bool => SettingValueRef::Bool(setting.get_bool()),
        SettingType::Character => SettingValueRef::Character(setting.get_character()),
        SettingType::UnsignedCharacter => {
            SettingValueRef::UnsignedCharacter(setting.get_unsigned_character())
        }
        SettingType::Integer => SettingValueRef::Integer(setting.get_integer()),
        SettingType::UnsignedInteger => {
            SettingValueRef::UnsignedInteger(setting.get_unsigned_integer())
        }
        SettingType::Float => SettingValueRef::Float(setting.get_float()),
        SettingType::String => SettingValueRef::String(setting.get_string_as_str()),
        SettingType::ColorRGB => SettingValueRef::ColorRgb(setting.get_color()),
        SettingType::ColorRGBA => SettingValueRef::ColorRgba(setting.get_color_a()),
        SettingType::Unknown => SettingValueRef::Unknown,
    }
}

/// Clones the current setting value into an owned enum.
#[inline(always)]
pub fn setting_value(setting: &Setting) -> SettingValue {
    setting_value_ref(setting).into()
}

/// Returns the bool value when the setting is actually a bool.
#[inline(always)]
pub fn try_bool(setting: &Setting) -> Option<bool> {
    match setting_value_ref(setting) {
        SettingValueRef::Bool(value) => Some(value),
        _ => None,
    }
}

/// Returns the signed character value when the setting has that type.
#[inline(always)]
pub fn try_character(setting: &Setting) -> Option<i8> {
    match setting_value_ref(setting) {
        SettingValueRef::Character(value) => Some(value),
        _ => None,
    }
}

/// Returns the unsigned character value when the setting has that type.
#[inline(always)]
pub fn try_unsigned_character(setting: &Setting) -> Option<u8> {
    match setting_value_ref(setting) {
        SettingValueRef::UnsignedCharacter(value) => Some(value),
        _ => None,
    }
}

/// Returns the integer value when the setting has that type.
#[inline(always)]
pub fn try_integer(setting: &Setting) -> Option<i32> {
    match setting_value_ref(setting) {
        SettingValueRef::Integer(value) => Some(value),
        _ => None,
    }
}

/// Returns the unsigned integer value when the setting has that type.
#[inline(always)]
pub fn try_unsigned_integer(setting: &Setting) -> Option<u32> {
    match setting_value_ref(setting) {
        SettingValueRef::UnsignedInteger(value) => Some(value),
        _ => None,
    }
}

/// Returns the floating-point value when the setting has that type.
#[inline(always)]
pub fn try_float(setting: &Setting) -> Option<f32> {
    match setting_value_ref(setting) {
        SettingValueRef::Float(value) => Some(value),
        _ => None,
    }
}

/// Returns the string value when the setting has that type.
#[inline(always)]
pub fn try_string(setting: &Setting) -> Option<&str> {
    match setting_value_ref(setting) {
        SettingValueRef::String(value) => Some(value),
        _ => None,
    }
}

/// Returns the RGB color value when the setting has that type.
#[inline(always)]
pub fn try_color_rgb(setting: &Setting) -> Option<u32> {
    match setting_value_ref(setting) {
        SettingValueRef::ColorRgb(value) => Some(value),
        _ => None,
    }
}

/// Returns the RGBA color value when the setting has that type.
#[inline(always)]
pub fn try_color_rgba(setting: &Setting) -> Option<u32> {
    match setting_value_ref(setting) {
        SettingValueRef::ColorRgba(value) => Some(value),
        _ => None,
    }
}

/// Set a bool setting, returning `false` on type mismatch.
#[inline(always)]
pub fn set_bool(setting: &mut Setting, value: bool) -> bool {
    if setting.get_type() == SettingType::Bool {
        setting.set_bool(value);
        true
    } else {
        false
    }
}

/// Set a signed character setting, returning `false` on type mismatch.
#[inline(always)]
pub fn set_character(setting: &mut Setting, value: i8) -> bool {
    if setting.get_type() == SettingType::Character {
        setting.set_character(value);
        true
    } else {
        false
    }
}

/// Set an unsigned character setting, returning `false` on type mismatch.
#[inline(always)]
pub fn set_unsigned_character(setting: &mut Setting, value: u8) -> bool {
    if setting.get_type() == SettingType::UnsignedCharacter {
        setting.set_unsigned_character(value);
        true
    } else {
        false
    }
}

/// Set an integer setting, returning `false` on type mismatch.
#[inline(always)]
pub fn set_integer(setting: &mut Setting, value: i32) -> bool {
    if setting.get_type() == SettingType::Integer {
        setting.set_integer(value);
        true
    } else {
        false
    }
}

/// Set an unsigned integer setting, returning `false` on type mismatch.
#[inline(always)]
pub fn set_unsigned_integer(setting: &mut Setting, value: u32) -> bool {
    if setting.get_type() == SettingType::UnsignedInteger {
        setting.set_unsigned_integer(value);
        true
    } else {
        false
    }
}

/// Set a float setting, returning `false` on type mismatch.
#[inline(always)]
pub fn set_float(setting: &mut Setting, value: f32) -> bool {
    if setting.get_type() == SettingType::Float {
        setting.set_float(value);
        true
    } else {
        false
    }
}

/// Set a string setting, returning `false` on type mismatch.
#[inline(always)]
pub fn set_string(setting: &mut Setting, value: &str) -> bool {
    if setting.get_type() == SettingType::String {
        setting.set_string(value);
        true
    } else {
        false
    }
}

/// Set an RGB color setting, returning `false` on type mismatch.
#[inline(always)]
pub fn set_color_rgb(setting: &mut Setting, value: u32) -> bool {
    if setting.get_type() == SettingType::ColorRGB {
        setting.set_color(value);
        true
    } else {
        false
    }
}

/// Set an RGBA color setting, returning `false` on type mismatch.
#[inline(always)]
pub fn set_color_rgba(setting: &mut Setting, value: u32) -> bool {
    if setting.get_type() == SettingType::ColorRGBA {
        setting.set_color_a(value);
        true
    } else {
        false
    }
}

/// Applies a type-tagged value to a setting, returning `false` on mismatch.
///
/// This is the main generic write path used by higher-level setting-store
/// helpers once they have resolved a concrete setting by name.
pub fn set_setting_value(setting: &mut Setting, value: SettingValueRef<'_>) -> bool {
    match value {
        SettingValueRef::Bool(value) => set_bool(setting, value),
        SettingValueRef::Character(value) => set_character(setting, value),
        SettingValueRef::UnsignedCharacter(value) => set_unsigned_character(setting, value),
        SettingValueRef::Integer(value) => set_integer(setting, value),
        SettingValueRef::UnsignedInteger(value) => set_unsigned_integer(setting, value),
        SettingValueRef::Float(value) => set_float(setting, value),
        SettingValueRef::String(value) => set_string(setting, value),
        SettingValueRef::ColorRgb(value) => set_color_rgb(setting, value),
        SettingValueRef::ColorRgba(value) => set_color_rgba(setting, value),
        SettingValueRef::Unknown => false,
    }
}
