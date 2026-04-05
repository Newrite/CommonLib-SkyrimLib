//! High-level player helpers over `PlayerCharacter` and the player's current
//! world state.
//!
//! This is the lightweight "player singleton" layer for gameplay code that
//! only needs the current player, current location/cell, or a handful of
//! common player-state queries.
//!
//! Example:
//!
//! ```rust,ignore
//! use libskyrim::sdk::gameplay::player;
//!
//! fn player_context() -> (libskyrim::re::NiPoint3, bool) {
//!     (player::position(), player::is_in_combat())
//! }
//! ```

use crate::re::{Actor, BGSLocation, NiPoint3, PlayerCharacter, TESObjectCELL, TESWorldSpace};
use crate::sdk::core::{GamePtr, GameRef, Resolved};

/// Return the global `PlayerCharacter` singleton.
#[inline(always)]
pub fn singleton() -> GameRef<PlayerCharacter> {
    unsafe { GameRef::from_raw(PlayerCharacter::get_singleton()) }
}

/// Current player position.
#[inline(always)]
pub fn position() -> NiPoint3 {
    singleton().get_position()
}

/// Current player angles.
#[inline(always)]
pub fn angle() -> NiPoint3 {
    singleton().get_angle()
}

/// Current parent cell of the player.
#[inline(always)]
pub fn current_cell() -> GamePtr<TESObjectCELL> {
    unsafe { GamePtr::from_raw(singleton().get_parent_cell()) }
}

/// Current worldspace of the player.
#[inline(always)]
pub fn current_worldspace() -> GamePtr<TESWorldSpace> {
    unsafe { GamePtr::from_raw(singleton().get_worldspace()) }
}

/// Current location of the player.
#[inline(always)]
pub fn current_location() -> GamePtr<BGSLocation> {
    unsafe { GamePtr::from_raw(singleton().get_current_location()) }
}

/// Whether the player is currently in combat.
#[inline(always)]
pub fn is_in_combat() -> bool {
    singleton().is_in_combat()
}

/// Actor currently carrying out a player-issued command, if any.
#[inline(always)]
pub fn actor_doing_player_command() -> Option<Resolved<Actor>> {
    Resolved::try_from_ptr(singleton().get_actor_doing_player_command().get())
}
