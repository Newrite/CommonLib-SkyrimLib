//! High-level access to game, ini, and preference settings.

mod iteration;
mod stores;
mod values;

pub use iteration::{
    collect_game_settings, collect_ini_settings, collect_preference_settings, collect_settings,
    count_settings, for_each_game_setting, for_each_ini_setting, for_each_preference_setting,
    for_each_setting,
};
pub use stores::{
    SettingStore, game_settings, has_game_setting, has_ini_setting, has_preference_setting,
    has_setting, ini_settings, lookup_game_setting, lookup_game_setting_value, lookup_ini_setting,
    lookup_ini_setting_value, lookup_preference_setting, lookup_preference_setting_value,
    lookup_setting, lookup_setting_value, preference_settings, reload_game_settings,
    reload_ini_settings, reload_preference_settings, reload_settings, save_game_settings,
    save_ini_settings, save_preference_settings, save_settings, set_game_setting, set_ini_setting,
    set_preference_setting, set_setting,
};
pub use values::{
    SettingValue, SettingValueRef, is_preference_setting, set_bool, set_character, set_color_rgb,
    set_color_rgba, set_float, set_integer, set_setting_value, set_string, set_unsigned_character,
    set_unsigned_integer, setting_name, setting_type, setting_value, setting_value_ref, try_bool,
    try_character, try_color_rgb, try_color_rgba, try_float, try_integer, try_string,
    try_unsigned_character, try_unsigned_integer,
};

#[cfg(test)]
mod tests;
