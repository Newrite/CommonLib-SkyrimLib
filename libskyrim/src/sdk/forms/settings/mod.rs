//! High-level access to game, ini, and preference settings.
//!
//! This module is intentionally split by workflow:
//!
//! - [`stores`] resolves settings by name and drives reload/save paths
//! - [`values`] exposes typed reads/writes for a single [`Setting`]
//! - [`iteration`] walks entire setting collections when plugins want to
//!   inspect or snapshot them
//!
//! Reach for this layer when you want ergonomic, type-aware setting access
//! without manually branching on `SettingType`.
//!
//! Decision guide:
//!
//! - use [`stores`] when the plugin starts from a setting name like
//!   `"fCombatDistance"` and wants lookup/reload/save helpers;
//! - use [`values`] when the plugin already has a concrete [`Setting`] pointer
//!   and wants typed reads or writes;
//! - use [`iteration`] when diagnostics, config sync, or tooling wants to walk
//!   whole setting collections.

mod iteration;
mod stores;
mod values;

/// Collection-iteration helpers for setting stores.
pub use iteration::{
    collect_game_settings, collect_ini_settings, collect_preference_settings, collect_settings,
    count_settings, for_each_game_setting, for_each_ini_setting, for_each_preference_setting,
    for_each_setting,
};

/// Store lookup, reload/save, and name-based set/get helpers.
pub use stores::{
    SettingStore, game_settings, has_game_setting, has_ini_setting, has_preference_setting,
    has_setting, ini_settings, lookup_game_setting, lookup_game_setting_value, lookup_ini_setting,
    lookup_ini_setting_value, lookup_preference_setting, lookup_preference_setting_value,
    lookup_setting, lookup_setting_value, preference_settings, reload_game_settings,
    reload_ini_settings, reload_preference_settings, reload_settings, save_game_settings,
    save_ini_settings, save_preference_settings, save_settings, set_game_setting, set_ini_setting,
    set_preference_setting, set_setting,
};

/// Typed views and typed mutation helpers for one concrete [`Setting`].
pub use values::{
    SettingValue, SettingValueRef, is_preference_setting, set_bool, set_character, set_color_rgb,
    set_color_rgba, set_float, set_integer, set_setting_value, set_string, set_unsigned_character,
    set_unsigned_integer, setting_name, setting_type, setting_value, setting_value_ref, try_bool,
    try_character, try_color_rgb, try_color_rgba, try_float, try_integer, try_string,
    try_unsigned_character, try_unsigned_integer,
};

#[cfg(test)]
mod tests;
