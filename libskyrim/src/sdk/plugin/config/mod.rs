//! Plugin-facing config helpers.
//!
//! This domain gathers the repetitive SKSE plugin patterns around typed INI
//! access, config-driven form lookup, and user-facing hotkey parsing.
//!
//! Typical recipe:
//!
//! ```rust,ignore
//! use libskyrim::re::TESGlobal;
//! use libskyrim::sdk::plugin::config;
//!
//! fn load_global() -> Result<libskyrim::sdk::core::GamePtr<TESGlobal>, ()> {
//!     let ini = config::load_ini("Data/SKSE/Plugins/MyPlugin.ini")?;
//!     config::form_value_typed::<TESGlobal>(&ini, "Gameplay", "MultiplierGlobal")
//!         .map_err(|_| ())
//! }
//! ```
//!
//! Reach for this module when config parsing is part of plugin bootstrap or a
//! reloadable runtime workflow:
//!
//! - [`Config`] when several values come from one loaded INI and you want one
//!   borrowed typed facade
//! - free functions such as [`form_value_typed`] or [`hotkey_value`] when a
//!   small module needs only one field
//! - [`HotkeyCombo`] when user-facing bindings must be parsed once and matched
//!   against input events later
//!
//! This module intentionally composes with [`crate::sdk::forms`]: config values
//! may stay as raw strings when desired, but typed helpers can also resolve
//! form specs straight into [`crate::sdk::core::GamePtr`] values.
//!
//! Decision guide:
//!
//! - use free functions such as [`form_value_typed`] or [`hotkey_value`] when
//!   a small install path only needs one or two fields
//! - use [`Config`] when one module owns several related fields and should keep
//!   one borrowed view over the loaded INI
//! - use [`HotkeyCombo`] when parsing belongs to bootstrap/reload time and
//!   later input code should only match already-validated combinations
//! - combine this module with [`crate::sdk::forms::persistent`] when resolved
//!   forms become stable plugin-owned state instead of one-shot install values
//!
//! Common install flow:
//!
//! 1. load the INI through [`load_ini`]
//! 2. validate required booleans, numbers, form specs, and hotkeys here
//! 3. convert long-lived resolved forms into persistent wrappers or other
//!    plugin-owned state
//! 4. hand the validated values into `sdk::gameplay`, `sdk::papyrus`, or
//!    `sdk::ui` installation code

mod facade;
mod hotkeys;
#[cfg(test)]
mod tests;

/// Borrowed typed INI facade plus one-shot free-function helpers.
pub use crate::ini::Ini;

/// Typed field access, form-spec resolution, and load/write helpers for INI
/// files.
pub use facade::{
    Config, ConfigValueError, bool_value, config, f32_value, form_value, form_value_typed,
    has_field, has_section, hotkey_value, i32_value, load_ini, raw_value, u32_value, write_ini,
};

/// User-facing hotkey parsing primitives.
pub use hotkeys::{HotkeyCombo, HotkeyParseError};
