use alloc::vec::Vec;
use core::ops::ControlFlow;

use crate::re::Setting;
use crate::sdk::core::GamePtr;
use crate::sdk::forms::shared::game_ptr_from_ref;

use super::stores::{SettingStore, game_settings, ini_settings, preference_settings};

pub fn for_each_game_setting(visit: impl FnMut(&Setting) -> ControlFlow<()>) -> ControlFlow<()> {
    let settings = game_settings();
    let Some(collection) = settings.as_ref() else {
        return ControlFlow::Continue(());
    };
    for_each_setting_ptrs(
        collection.base.settings_iter().map(|entry| entry.second),
        visit,
    )
}

pub fn for_each_ini_setting(visit: impl FnMut(&Setting) -> ControlFlow<()>) -> ControlFlow<()> {
    let settings = ini_settings();
    let Some(collection) = settings.as_ref() else {
        return ControlFlow::Continue(());
    };
    for_each_setting_ptrs(collection.base.settings_iter(), visit)
}

pub fn for_each_preference_setting(
    visit: impl FnMut(&Setting) -> ControlFlow<()>,
) -> ControlFlow<()> {
    let settings = preference_settings();
    let Some(collection) = settings.as_ref() else {
        return ControlFlow::Continue(());
    };
    for_each_setting_ptrs(collection.base.base.settings_iter(), visit)
}

#[inline(always)]
pub fn for_each_setting(
    store: SettingStore,
    visit: impl FnMut(&Setting) -> ControlFlow<()>,
) -> ControlFlow<()> {
    match store {
        SettingStore::Game => for_each_game_setting(visit),
        SettingStore::Ini => for_each_ini_setting(visit),
        SettingStore::Preference => for_each_preference_setting(visit),
    }
}

#[inline(always)]
pub fn collect_game_settings() -> Vec<GamePtr<Setting>> {
    collect_settings(SettingStore::Game)
}

#[inline(always)]
pub fn collect_ini_settings() -> Vec<GamePtr<Setting>> {
    collect_settings(SettingStore::Ini)
}

#[inline(always)]
pub fn collect_preference_settings() -> Vec<GamePtr<Setting>> {
    collect_settings(SettingStore::Preference)
}

pub fn collect_settings(store: SettingStore) -> Vec<GamePtr<Setting>> {
    let mut settings = Vec::new();
    let _ = for_each_setting(store, |setting| {
        settings.push(game_ptr_from_ref(setting));
        ControlFlow::Continue(())
    });
    settings
}

#[inline(always)]
pub fn count_settings(store: SettingStore) -> usize {
    let mut count = 0;
    let _ = for_each_setting(store, |_| {
        count += 1;
        ControlFlow::Continue(())
    });
    count
}

fn for_each_setting_ptrs(
    iter: impl IntoIterator<Item = *mut Setting>,
    mut visit: impl FnMut(&Setting) -> ControlFlow<()>,
) -> ControlFlow<()> {
    for setting in iter {
        let Some(setting) = (unsafe { setting.as_ref() }) else {
            continue;
        };

        let flow = visit(setting);
        if flow.is_break() {
            return flow;
        }
    }
    ControlFlow::Continue(())
}
