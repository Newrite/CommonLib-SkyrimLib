use core_util::EnumSet;

use crate::re::{ControlMap, USER_EVENT_FLAG, UserEventEnabled};

use super::access::{control_map, player_controls};
use super::shared::{
    has_invalid_control_flag, warn_invalid_control_flags, warn_skipped_invalid_control_flags,
};
use super::types::{GAMEPLAY_CONTROL_FLAGS, InputStateGuard, InputStateSnapshot, TextInputGuard};

#[inline(always)]
pub fn text_entry_count() -> i8 {
    control_map().runtime_data().text_entry_count
}

#[inline(always)]
pub fn has_text_input_requests() -> bool {
    text_entry_count() > 0
}

#[inline(always)]
pub fn allow_text_input(allow: bool) -> i8 {
    unsafe { control_map().with_mut_unchecked(|control_map| control_map.allow_text_input(allow)) }
}

#[inline(always)]
pub fn allow_text_input_scoped() -> TextInputGuard {
    let _ = allow_text_input(true);
    TextInputGuard::new(true)
}

#[inline(always)]
pub fn ignores_keyboard_mouse() -> bool {
    control_map().runtime_data().ignore_keyboard_mouse
}

#[inline(always)]
pub fn set_keyboard_mouse_ignored(ignored: bool) -> bool {
    unsafe {
        control_map().with_mut_unchecked(|control_map| {
            let runtime_data = control_map.runtime_data_mut();
            let previous = runtime_data.ignore_keyboard_mouse;
            runtime_data.ignore_keyboard_mouse = ignored;
            previous
        })
    }
}

#[inline(always)]
pub fn ignores_activate_disabled_events() -> bool {
    control_map().runtime_data().ignore_activate_disabled_events
}

#[inline(always)]
pub fn set_activate_disabled_events_ignored(ignored: bool) -> bool {
    unsafe {
        control_map().with_mut_unchecked(|control_map| {
            let runtime_data = control_map.runtime_data_mut();
            let previous = runtime_data.ignore_activate_disabled_events;
            runtime_data.ignore_activate_disabled_events = ignored;
            previous
        })
    }
}

#[inline(always)]
pub fn enabled_controls() -> EnumSet<USER_EVENT_FLAG, u32> {
    control_map().runtime_data().enabled_controls
}

#[inline(always)]
pub fn stored_controls() -> Option<EnumSet<USER_EVENT_FLAG, u32>> {
    let stored = control_map().runtime_data().stored_controls;
    (stored.underlying() != USER_EVENT_FLAG::kInvalid as u32).then_some(stored)
}

pub fn set_enabled_controls(enabled: EnumSet<USER_EVENT_FLAG, u32>) {
    unsafe {
        control_map().with_mut_unchecked(|control_map| {
            let old_state = {
                let runtime_data = control_map.runtime_data_mut();
                let old_state = runtime_data.enabled_controls;
                if old_state == enabled {
                    return;
                }
                runtime_data.enabled_controls = enabled;
                old_state
            };

            let event = UserEventEnabled {
                new_user_event_flag: enabled,
                old_user_event_flag: old_state,
            };
            control_map.event_source.send_event(&event);
        })
    }
}

#[inline(always)]
pub fn are_controls_enabled(flags: USER_EVENT_FLAG) -> bool {
    control_map().are_controls_enabled(flags)
}

#[inline(always)]
pub fn set_control_enabled(flags: USER_EVENT_FLAG, enable: bool, store_state: bool) {
    if flags == USER_EVENT_FLAG::kNone {
        return;
    }
    if has_invalid_control_flag(flags) {
        warn_invalid_control_flags("set_control_enabled", flags);
        return;
    }

    unsafe {
        control_map().with_mut_unchecked(|control_map| {
            control_map.toggle_controls(flags, enable, store_state)
        })
    };
}

#[inline(always)]
pub fn toggle_control(flags: USER_EVENT_FLAG, enable: bool, store_state: bool) {
    set_control_enabled(flags, enable, store_state);
}

pub fn set_control_group_enabled(flags: &[USER_EVENT_FLAG], enable: bool, store_state: bool) {
    if flags.is_empty() {
        return;
    }

    unsafe {
        control_map().with_mut_unchecked(|control_map| {
            for &flag in flags {
                if flag == USER_EVENT_FLAG::kNone {
                    continue;
                }
                if has_invalid_control_flag(flag) {
                    warn_skipped_invalid_control_flags("set_control_group_enabled", flag);
                    continue;
                }
                control_map.toggle_controls(flag, enable, store_state);
            }
        })
    };
}

#[inline(always)]
pub fn store_controls() {
    unsafe { control_map().with_mut_unchecked(ControlMap::store_controls) };
}

#[inline(always)]
pub fn restore_stored_controls() {
    unsafe { control_map().with_mut_unchecked(ControlMap::load_stored_controls) };
}

#[inline(always)]
pub fn set_gameplay_controls_enabled(enable: bool, store_state: bool) {
    set_control_group_enabled(&GAMEPLAY_CONTROL_FLAGS, enable, store_state);
}

#[inline(always)]
pub fn is_player_input_blocked() -> bool {
    player_controls().block_player_input
}

#[inline(always)]
pub fn set_player_input_blocked(blocked: bool) -> bool {
    unsafe {
        player_controls().with_mut_unchecked(|controls| {
            let previous = controls.block_player_input;
            controls.block_player_input = blocked;
            previous
        })
    }
}

#[inline(always)]
pub fn snapshot() -> InputStateSnapshot {
    let control_map = control_map();
    let runtime_data = control_map.runtime_data();
    InputStateSnapshot::new(
        runtime_data.enabled_controls,
        runtime_data.stored_controls,
        runtime_data.text_entry_count,
        runtime_data.ignore_keyboard_mouse,
        runtime_data.ignore_activate_disabled_events,
        is_player_input_blocked(),
    )
}

pub fn restore(snapshot: InputStateSnapshot) {
    unsafe {
        control_map().with_mut_unchecked(|control_map| {
            let runtime_data = control_map.runtime_data_mut();
            runtime_data.stored_controls = snapshot.stored_controls_raw();
            runtime_data.text_entry_count = snapshot.text_entry_count();
            runtime_data.ignore_keyboard_mouse = snapshot.ignore_keyboard_mouse();
            runtime_data.ignore_activate_disabled_events =
                snapshot.ignore_activate_disabled_events();
        })
    };
    let _ = set_player_input_blocked(snapshot.player_input_blocked());
    set_enabled_controls(snapshot.enabled_controls());
}

#[inline(always)]
pub fn scoped_state() -> InputStateGuard {
    InputStateGuard::new()
}

pub fn scoped_gameplay_controls_disabled() -> InputStateGuard {
    let guard = scoped_state();
    set_gameplay_controls_enabled(false, false);
    guard
}

pub fn scoped_player_input_blocked() -> InputStateGuard {
    let guard = scoped_state();
    let _ = set_player_input_blocked(true);
    guard
}

pub fn scoped_gameplay_input_suppressed() -> InputStateGuard {
    let guard = scoped_state();
    set_gameplay_controls_enabled(false, false);
    let _ = set_player_input_blocked(true);
    guard
}
