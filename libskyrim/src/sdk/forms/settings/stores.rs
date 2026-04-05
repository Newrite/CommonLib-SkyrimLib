use crate::re::{
    GameSettingCollection, INIPrefSettingCollection, INISettingCollection, Setting,
    SettingCollectionExt,
};
use crate::sdk::core::GamePtr;
use crate::sdk::forms::shared::trimmed_non_empty;

use super::values::{SettingValue, SettingValueRef, set_setting_value, setting_value};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SettingStore {
    /// `GameSettingCollection`, usually accessed through gameplay/game-setting
    /// names.
    Game,
    /// `INISettingCollection`, backed by the regular INI store.
    Ini,
    /// `INIPrefSettingCollection`, backed by preference INI values.
    Preference,
}

/// Return the live game-setting collection singleton.
#[inline(always)]
pub fn game_settings() -> GamePtr<GameSettingCollection> {
    unsafe { GamePtr::from_raw(GameSettingCollection::get_singleton()) }
}

/// Return the live INI-setting collection singleton.
#[inline(always)]
pub fn ini_settings() -> GamePtr<INISettingCollection> {
    unsafe { GamePtr::from_raw(INISettingCollection::get_singleton()) }
}

/// Return the live preference-setting collection singleton.
#[inline(always)]
pub fn preference_settings() -> GamePtr<INIPrefSettingCollection> {
    unsafe { GamePtr::from_raw(INIPrefSettingCollection::get_singleton()) }
}

/// Looks up a game setting by name.
#[inline(always)]
pub fn lookup_game_setting(name: &str) -> GamePtr<Setting> {
    lookup_setting(SettingStore::Game, name)
}

/// Looks up a regular INI setting by name.
#[inline(always)]
pub fn lookup_ini_setting(name: &str) -> GamePtr<Setting> {
    lookup_setting(SettingStore::Ini, name)
}

/// Looks up a preference setting by name.
#[inline(always)]
pub fn lookup_preference_setting(name: &str) -> GamePtr<Setting> {
    lookup_setting(SettingStore::Preference, name)
}

/// Look up a setting by name from a specific store.
///
/// The input is trimmed and empty names resolve to null.
pub fn lookup_setting(store: SettingStore, name: &str) -> GamePtr<Setting> {
    let Some(name) = trimmed_non_empty(name) else {
        return GamePtr::null();
    };

    match store {
        SettingStore::Game => {
            let settings = game_settings();
            let Some(collection) = settings.as_ref() else {
                return GamePtr::null();
            };
            unsafe { GamePtr::from_raw(collection.get_setting_str(name)) }
        }
        SettingStore::Ini => {
            let settings = ini_settings();
            let Some(collection) = settings.as_ref() else {
                return GamePtr::null();
            };
            unsafe { GamePtr::from_raw(collection.get_setting(name)) }
        }
        SettingStore::Preference => {
            let settings = preference_settings();
            let Some(collection) = settings.as_ref() else {
                return GamePtr::null();
            };
            unsafe { GamePtr::from_raw(collection.base.get_setting(name)) }
        }
    }
}

/// Returns `true` when the named game setting exists.
#[inline(always)]
pub fn has_game_setting(name: &str) -> bool {
    lookup_game_setting(name).is_some()
}

/// Returns `true` when the named INI setting exists.
#[inline(always)]
pub fn has_ini_setting(name: &str) -> bool {
    lookup_ini_setting(name).is_some()
}

/// Returns `true` when the named preference setting exists.
#[inline(always)]
pub fn has_preference_setting(name: &str) -> bool {
    lookup_preference_setting(name).is_some()
}

/// Returns `true` when the named setting exists in the selected store.
#[inline(always)]
pub fn has_setting(store: SettingStore, name: &str) -> bool {
    lookup_setting(store, name).is_some()
}

/// Look up a game setting and snapshot its typed value.
#[inline(always)]
pub fn lookup_game_setting_value(name: &str) -> Option<SettingValue> {
    lookup_game_setting(name).with(setting_value)
}

/// Looks up a regular INI setting and snapshots its typed value.
#[inline(always)]
pub fn lookup_ini_setting_value(name: &str) -> Option<SettingValue> {
    lookup_ini_setting(name).with(setting_value)
}

/// Looks up a preference setting and snapshots its typed value.
#[inline(always)]
pub fn lookup_preference_setting_value(name: &str) -> Option<SettingValue> {
    lookup_preference_setting(name).with(setting_value)
}

/// Looks up a setting in the selected store and snapshots its typed value.
#[inline(always)]
pub fn lookup_setting_value(store: SettingStore, name: &str) -> Option<SettingValue> {
    lookup_setting(store, name).with(setting_value)
}

/// Set a game setting if it exists and the provided value matches its runtime
/// type.
#[inline(always)]
pub fn set_game_setting(name: &str, value: SettingValueRef<'_>) -> bool {
    set_setting(SettingStore::Game, name, value)
}

/// Sets a regular INI setting when it exists and the type matches.
#[inline(always)]
pub fn set_ini_setting(name: &str, value: SettingValueRef<'_>) -> bool {
    set_setting(SettingStore::Ini, name, value)
}

/// Sets a preference setting when it exists and the type matches.
#[inline(always)]
pub fn set_preference_setting(name: &str, value: SettingValueRef<'_>) -> bool {
    set_setting(SettingStore::Preference, name, value)
}

/// Set a setting in the chosen store, returning `false` when the setting is
/// missing or the value type does not match.
pub fn set_setting(store: SettingStore, name: &str, value: SettingValueRef<'_>) -> bool {
    unsafe {
        lookup_setting(store, name)
            .with_mut_unchecked(|setting| set_setting_value(setting, value))
            .unwrap_or(false)
    }
}

/// Reloads the game-setting store from disk/default sources.
#[inline(always)]
pub fn reload_game_settings() -> bool {
    reload_settings(SettingStore::Game)
}

/// Flushes the game-setting store.
#[inline(always)]
pub fn save_game_settings() -> bool {
    save_settings(SettingStore::Game)
}

/// Reloads the regular INI-setting store.
#[inline(always)]
pub fn reload_ini_settings() -> bool {
    reload_settings(SettingStore::Ini)
}

/// Flushes the regular INI-setting store.
#[inline(always)]
pub fn save_ini_settings() -> bool {
    save_settings(SettingStore::Ini)
}

/// Reloads the preference-setting store.
#[inline(always)]
pub fn reload_preference_settings() -> bool {
    reload_settings(SettingStore::Preference)
}

/// Flushes the preference-setting store.
#[inline(always)]
pub fn save_preference_settings() -> bool {
    save_settings(SettingStore::Preference)
}

/// Reload all settings from the selected store.
pub fn reload_settings(store: SettingStore) -> bool {
    match store {
        SettingStore::Game => unsafe {
            game_settings()
                .with_mut_unchecked(|collection| collection.base.read_all_settings())
                .is_some()
        },
        SettingStore::Ini => unsafe {
            ini_settings()
                .with_mut_unchecked(|collection| collection.base.read_all_settings())
                .is_some()
        },
        SettingStore::Preference => unsafe {
            preference_settings()
                .with_mut_unchecked(|collection| collection.base.base.read_all_settings())
                .is_some()
        },
    }
}

/// Flush the selected store back to disk.
pub fn save_settings(store: SettingStore) -> bool {
    match store {
        SettingStore::Game => unsafe {
            game_settings()
                .with_mut_unchecked(|collection| collection.base.write_all_settings())
                .is_some()
        },
        SettingStore::Ini => unsafe {
            ini_settings()
                .with_mut_unchecked(|collection| collection.base.write_all_settings())
                .is_some()
        },
        SettingStore::Preference => unsafe {
            preference_settings()
                .with_mut_unchecked(|collection| collection.base.base.write_all_settings())
                .is_some()
        },
    }
}
