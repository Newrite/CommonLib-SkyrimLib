use crate::re::{ControlMap, PlayerControls, UserEvents};
use crate::sdk::core::GameRef;

#[inline(always)]
pub fn control_map() -> GameRef<ControlMap> {
    unsafe { GameRef::from_raw(ControlMap::get_singleton()) }
}

#[inline(always)]
pub fn player_controls() -> GameRef<PlayerControls> {
    unsafe { GameRef::from_raw(PlayerControls::get_singleton()) }
}

#[inline(always)]
pub fn user_events() -> GameRef<UserEvents> {
    unsafe { GameRef::from_raw(UserEvents::get_singleton()) }
}
