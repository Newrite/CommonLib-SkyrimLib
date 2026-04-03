//! Plugin-facing config helpers.
//!
//! This domain gathers the repetitive SKSE plugin patterns around typed INI
//! access, config-driven form lookup, and user-facing hotkey parsing.

mod facade;
mod hotkeys;
#[cfg(test)]
mod tests;

pub use crate::ini::Ini;
pub use facade::{
    Config, ConfigValueError, bool_value, config, f32_value, form_value, form_value_typed,
    has_field, has_section, hotkey_value, i32_value, load_ini, raw_value, u32_value, write_ini,
};
pub use hotkeys::{HotkeyCombo, HotkeyParseError};
