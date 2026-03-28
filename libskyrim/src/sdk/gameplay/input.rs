//! Gameplay-facing input helpers.

use crate::re::{ControlMap, INPUT_CONTEXT_ID, PlayerControls, USER_EVENT_FLAG};
use crate::sdk::core::GameRef;

pub const GAMEPLAY_CONTROL_FLAGS: [USER_EVENT_FLAG; 10] = [
    USER_EVENT_FLAG::kMovement,
    USER_EVENT_FLAG::kLooking,
    USER_EVENT_FLAG::kActivate,
    USER_EVENT_FLAG::kPOVSwitch,
    USER_EVENT_FLAG::kFighting,
    USER_EVENT_FLAG::kSneaking,
    USER_EVENT_FLAG::kMainFour,
    USER_EVENT_FLAG::kWheelZoom,
    USER_EVENT_FLAG::kJumping,
    USER_EVENT_FLAG::kVATS,
];

#[inline(always)]
pub fn control_map() -> GameRef<ControlMap> {
    unsafe { GameRef::from_raw(ControlMap::get_singleton()) }
}

#[inline(always)]
pub fn player_controls() -> GameRef<PlayerControls> {
    unsafe { GameRef::from_raw(PlayerControls::get_singleton()) }
}

#[inline(always)]
pub fn push_context(context: INPUT_CONTEXT_ID) {
    ControlMap::push_input_context(context);
}

#[inline(always)]
pub fn pop_context(context: INPUT_CONTEXT_ID) {
    ControlMap::pop_input_context(context);
}

#[inline(always)]
pub fn allow_text_input(allow: bool) -> i8 {
    unsafe { control_map().with_mut_unchecked(|control_map| control_map.allow_text_input(allow)) }
}

#[inline(always)]
pub fn are_controls_enabled(flags: USER_EVENT_FLAG) -> bool {
    control_map().are_controls_enabled(flags)
}

#[inline(always)]
pub fn toggle_control(flags: USER_EVENT_FLAG, enable: bool, store_state: bool) {
    unsafe {
        control_map().with_mut_unchecked(|control_map| {
            control_map.toggle_controls(flags, enable, store_state)
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
    unsafe {
        control_map().with_mut_unchecked(|control_map| {
            for flag in GAMEPLAY_CONTROL_FLAGS {
                control_map.toggle_controls(flag, enable, store_state);
            }
        })
    };
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
