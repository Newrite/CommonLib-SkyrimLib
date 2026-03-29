//! High-level player helpers over `PlayerCharacter` and the player's current
//! world state.

use crate::re::{Actor, BGSLocation, NiPoint3, PlayerCharacter, TESObjectCELL, TESWorldSpace};
use crate::sdk::core::{GamePtr, GameRef, Resolved};

#[inline(always)]
pub fn singleton() -> GameRef<PlayerCharacter> {
    unsafe { GameRef::from_raw(PlayerCharacter::get_singleton()) }
}

#[inline(always)]
pub fn position() -> NiPoint3 {
    singleton().get_position()
}

#[inline(always)]
pub fn angle() -> NiPoint3 {
    singleton().get_angle()
}

#[inline(always)]
pub fn current_cell() -> GameRef<TESObjectCELL> {
    unsafe { GameRef::from_raw(singleton().get_parent_cell()) }
}

#[inline(always)]
pub fn current_worldspace() -> GamePtr<TESWorldSpace> {
    unsafe { GamePtr::from_raw(singleton().get_worldspace()) }
}

#[inline(always)]
pub fn current_location() -> GamePtr<BGSLocation> {
    unsafe { GamePtr::from_raw(singleton().get_current_location()) }
}

#[inline(always)]
pub fn is_in_combat() -> bool {
    singleton().is_in_combat()
}

#[inline(always)]
pub fn actor_doing_player_command() -> Option<Resolved<Actor>> {
    Resolved::try_from_ptr(singleton().get_actor_doing_player_command().get())
}
