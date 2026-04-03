use crate::re::{
    GameSettingCollection, INIPrefSettingCollection, INISettingCollection, Setting,
    SettingCollectionExt,
};
use crate::sdk::core::GamePtr;
use crate::sdk::forms::shared::trimmed_non_empty;

use super::values::{SettingValue, SettingValueRef, set_setting_value, setting_value};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SettingStore {
    Game,
    Ini,
    Preference,
}

#[inline(always)]
pub fn game_settings() -> GamePtr<GameSettingCollection> {
    unsafe { GamePtr::from_raw(GameSettingCollection::get_singleton()) }
}

#[inline(always)]
pub fn ini_settings() -> GamePtr<INISettingCollection> {
    unsafe { GamePtr::from_raw(INISettingCollection::get_singleton()) }
}

#[inline(always)]
pub fn preference_settings() -> GamePtr<INIPrefSettingCollection> {
    unsafe { GamePtr::from_raw(INIPrefSettingCollection::get_singleton()) }
}

#[inline(always)]
pub fn lookup_game_setting(name: &str) -> GamePtr<Setting> {
    lookup_setting(SettingStore::Game, name)
}

#[inline(always)]
pub fn lookup_ini_setting(name: &str) -> GamePtr<Setting> {
    lookup_setting(SettingStore::Ini, name)
}

#[inline(always)]
pub fn lookup_preference_setting(name: &str) -> GamePtr<Setting> {
    lookup_setting(SettingStore::Preference, name)
}

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

#[inline(always)]
pub fn has_game_setting(name: &str) -> bool {
    lookup_game_setting(name).is_some()
}

#[inline(always)]
pub fn has_ini_setting(name: &str) -> bool {
    lookup_ini_setting(name).is_some()
}

#[inline(always)]
pub fn has_preference_setting(name: &str) -> bool {
    lookup_preference_setting(name).is_some()
}

#[inline(always)]
pub fn has_setting(store: SettingStore, name: &str) -> bool {
    lookup_setting(store, name).is_some()
}

#[inline(always)]
pub fn lookup_game_setting_value(name: &str) -> Option<SettingValue> {
    lookup_game_setting(name).with(setting_value)
}

#[inline(always)]
pub fn lookup_ini_setting_value(name: &str) -> Option<SettingValue> {
    lookup_ini_setting(name).with(setting_value)
}

#[inline(always)]
pub fn lookup_preference_setting_value(name: &str) -> Option<SettingValue> {
    lookup_preference_setting(name).with(setting_value)
}

#[inline(always)]
pub fn lookup_setting_value(store: SettingStore, name: &str) -> Option<SettingValue> {
    lookup_setting(store, name).with(setting_value)
}

#[inline(always)]
pub fn set_game_setting(name: &str, value: SettingValueRef<'_>) -> bool {
    set_setting(SettingStore::Game, name, value)
}

#[inline(always)]
pub fn set_ini_setting(name: &str, value: SettingValueRef<'_>) -> bool {
    set_setting(SettingStore::Ini, name, value)
}

#[inline(always)]
pub fn set_preference_setting(name: &str, value: SettingValueRef<'_>) -> bool {
    set_setting(SettingStore::Preference, name, value)
}

pub fn set_setting(store: SettingStore, name: &str, value: SettingValueRef<'_>) -> bool {
    unsafe {
        lookup_setting(store, name)
            .with_mut_unchecked(|setting| set_setting_value(setting, value))
            .unwrap_or(false)
    }
}

#[inline(always)]
pub fn reload_game_settings() -> bool {
    reload_settings(SettingStore::Game)
}

#[inline(always)]
pub fn save_game_settings() -> bool {
    save_settings(SettingStore::Game)
}

#[inline(always)]
pub fn reload_ini_settings() -> bool {
    reload_settings(SettingStore::Ini)
}

#[inline(always)]
pub fn save_ini_settings() -> bool {
    save_settings(SettingStore::Ini)
}

#[inline(always)]
pub fn reload_preference_settings() -> bool {
    reload_settings(SettingStore::Preference)
}

#[inline(always)]
pub fn save_preference_settings() -> bool {
    save_settings(SettingStore::Preference)
}

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
