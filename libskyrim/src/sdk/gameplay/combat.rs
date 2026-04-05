//! Combat-state and hit-processing helpers.
//!
//! This module focuses on a narrower set of combat-adjacent workflows than a
//! full combat framework:
//!
//! - query cached hostility state through [`are_hostile_actors_nearby`] and
//!   [`collect_hostile_actors_nearby`]
//! - clear native reaction caches with
//!   [`clear_cached_faction_fight_reactions`]
//! - stop combat on one actor, the player, or nearby groups
//!
//! Example:
//!
//! ```rust,ignore
//! use libskyrim::sdk::gameplay::combat;
//!
//! fn calm_nearby_hostiles() {
//!     if combat::are_hostile_actors_nearby() {
//!         combat::stop_combat_on_hostile_actors_nearby(true);
//!     }
//! }
//! ```

use alloc::vec::Vec;

use crate::re::{Actor, BSScrapArray, NiPointer, ProcessLists};
use crate::sdk::core::{
    ContiguousSequenceIterationOptions, GamePtr, GameRef, Resolved,
    for_each_contiguous_sequence_named,
};
use crate::sdk::gameplay::{actors, player};

const MAX_REASONABLE_HOSTILE_NEAR_HANDLES: u32 = 0x1000;

/// Return the combat-facing `ProcessLists` singleton.
///
/// Most consumers reach for higher-level helpers in this module, but the raw
/// singleton accessor stays available when a gameplay routine needs direct
/// `ProcessLists` state.
#[inline(always)]
pub fn singleton() -> GameRef<ProcessLists> {
    actors::process_lists()
}

// Query-ish validation around native-sensitive combat helpers.

#[inline(always)]
fn is_valid_radius(radius: f32, _caller: &'static str) -> bool {
    if !radius.is_finite() || radius <= 0.0 {
        crate::defensive_sdk_warn!(
            "{} ignored non-positive or non-finite radius={}",
            _caller,
            radius
        );
        false
    } else {
        true
    }
}

#[inline(always)]
fn can_stop_combat_on_actor(actor: &Actor) -> bool {
    if actor.base.is_dead(false) {
        return false;
    }

    if actor.base.is_disabled() {
        return false;
    }

    if actor.is_in_kill_move() {
        return false;
    }

    true
}

#[inline(always)]
fn actor_from_ptr(actor: GamePtr<Actor>, _caller: &'static str) -> Option<GameRef<Actor>> {
    let Some(actor) = actor.into_option() else {
        crate::defensive_sdk_warn!("{} received a null actor pointer", _caller);
        return None;
    };

    Some(actor)
}

/// Query the engine's cached "hostile actors near" state.
#[inline(always)]
pub fn are_hostile_actors_nearby() -> bool {
    let mut actors = BSScrapArray::new();
    unsafe {
        singleton()
            .with_mut_unchecked(|process_lists| process_lists.are_hostile_actors_near(&mut actors))
    }
}

/// Resolve the hostile actors near the player reported by `ProcessLists`.
///
/// This is the higher-level companion to [`are_hostile_actors_nearby`]: it
/// upgrades the cached handle set into resolved actor references when possible.
pub fn collect_hostile_actors_nearby() -> Vec<Resolved<Actor>> {
    let mut handles = BSScrapArray::new();
    let any_hostile = unsafe {
        singleton()
            .with_mut_unchecked(|process_lists| process_lists.are_hostile_actors_near(&mut handles))
    };

    if !any_hostile {
        return Vec::new();
    }

    let mut actors = Vec::new();
    let _ = for_each_contiguous_sequence_named(
        &handles,
        "sdk::gameplay::combat::collect_hostile_actors_nearby()",
        ContiguousSequenceIterationOptions::new()
            .with_max_reasonable_len(MAX_REASONABLE_HOSTILE_NEAR_HANDLES),
        |handle| {
            if let Some(actor) = Resolved::from_handle(*handle) {
                actors.push(actor);
            }
            core::ops::ControlFlow::Continue(())
        },
    );
    actors
}

/// Clear cached faction fight-reaction state inside `ProcessLists`.
#[inline(always)]
pub fn clear_cached_faction_fight_reactions() {
    singleton().clear_cached_faction_fight_reactions();
}

/// Whether the player is currently in combat.
#[inline(always)]
pub fn is_player_in_combat() -> bool {
    player::is_in_combat()
}

/// Stop combat on one actor when it is safe to do so.
#[inline(always)]
pub fn stop_combat_on_actor(actor: &mut Actor, suppress_alarm: bool) {
    if !can_stop_combat_on_actor(actor) {
        crate::defensive_sdk_warn!(
            "sdk::gameplay::combat::stop_combat_on_actor() skipped actor because it was dead, disabled, or in a kill move"
        );
        return;
    }

    unsafe {
        singleton().with_mut_unchecked(|process_lists| {
            process_lists.stop_combat_and_alarm_on_actor(actor, suppress_alarm)
        })
    };
}

/// Pointer-friendly variant of [`stop_combat_on_actor`].
#[inline(always)]
pub fn stop_combat_on_actor_ptr(actor: GamePtr<Actor>, suppress_alarm: bool) -> bool {
    let Some(actor) = actor_from_ptr(actor, "sdk::gameplay::combat::stop_combat_on_actor_ptr()")
    else {
        return false;
    };

    unsafe {
        actor.with_mut_unchecked(|actor| stop_combat_on_actor(actor, suppress_alarm));
    }
    true
}

/// `NiPointer`-friendly variant of [`stop_combat_on_actor`].
#[inline(always)]
pub fn stop_combat_on_actor_owner(actor: &NiPointer<Actor>, suppress_alarm: bool) -> bool {
    stop_combat_on_actor_ptr(unsafe { GamePtr::from_raw(actor.get()) }, suppress_alarm)
}

/// Stop combat on all resolved actors in the slice.
pub fn stop_combat_on_actors(actors: &mut [Resolved<Actor>], suppress_alarm: bool) {
    if actors.is_empty() {
        return;
    }

    for actor in actors {
        stop_combat_on_actor(actor, suppress_alarm);
    }
}

/// Stop combat on the player.
#[inline(always)]
pub fn stop_combat_on_player(suppress_alarm: bool) {
    unsafe {
        player::singleton()
            .with_mut_unchecked(|player| stop_combat_on_actor(player, suppress_alarm))
    };
}

/// Stop combat on actors currently reported as hostile and near.
///
/// This uses the engine's cached hostile-near query rather than a fresh
/// world-space radius scan.
pub fn stop_combat_on_hostile_actors_nearby(suppress_alarm: bool) {
    let mut actors = collect_hostile_actors_nearby();
    stop_combat_on_actors(&mut actors, suppress_alarm);
}

/// Stop combat on actors near the player within a supplied radius.
pub fn stop_combat_on_nearby_actors(radius: f32, suppress_alarm: bool) {
    if !is_valid_radius(
        radius,
        "sdk::gameplay::combat::stop_combat_on_nearby_actors()",
    ) {
        return;
    }

    let mut actors = actors::collect_nearby_player_actors(radius);
    stop_combat_on_actors(&mut actors, suppress_alarm);
}

/// Stop combat on hostile actors near the player within a supplied radius.
pub fn stop_combat_on_hostile_nearby_actors(radius: f32, suppress_alarm: bool) {
    if !is_valid_radius(
        radius,
        "sdk::gameplay::combat::stop_combat_on_hostile_nearby_actors()",
    ) {
        return;
    }

    let mut actors = actors::collect_hostile_nearby_player_actors(radius);
    stop_combat_on_actors(&mut actors, suppress_alarm);
}
