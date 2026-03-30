//! Combat-state and hit-processing helpers.

use alloc::vec::Vec;

use crate::re::{Actor, BSScrapArray, ProcessLists};
use crate::sdk::core::{GameRef, Resolved};
use crate::sdk::gameplay::{actors, player};

#[inline(always)]
pub fn singleton() -> GameRef<ProcessLists> {
    actors::process_lists()
}

#[inline(always)]
fn is_valid_radius(radius: f32, caller: &'static str) -> bool {
    if !radius.is_finite() || radius <= 0.0 {
        crate::defensive_sdk_warn!(
            "{} ignored non-positive or non-finite radius={}",
            caller,
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
pub fn are_hostile_actors_nearby() -> bool {
    let mut actors = BSScrapArray::new();
    unsafe {
        singleton()
            .with_mut_unchecked(|process_lists| process_lists.are_hostile_actors_near(&mut actors))
    }
}

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
    for handle in unsafe { handles.as_slice() } {
        if let Some(actor) = Resolved::from_handle(*handle) {
            actors.push(actor);
        }
    }
    actors
}

#[inline(always)]
pub fn clear_cached_faction_fight_reactions() {
    singleton().clear_cached_faction_fight_reactions();
}

#[inline(always)]
pub fn is_player_in_combat() -> bool {
    player::is_in_combat()
}

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

pub fn stop_combat_on_actors(actors: &mut [Resolved<Actor>], suppress_alarm: bool) {
    if actors.is_empty() {
        return;
    }

    for actor in actors {
        stop_combat_on_actor(actor, suppress_alarm);
    }
}

#[inline(always)]
pub fn stop_combat_on_player(suppress_alarm: bool) {
    unsafe {
        player::singleton()
            .with_mut_unchecked(|player| stop_combat_on_actor(player, suppress_alarm))
    };
}

pub fn stop_combat_on_hostile_actors_nearby(suppress_alarm: bool) {
    let mut actors = collect_hostile_actors_nearby();
    stop_combat_on_actors(&mut actors, suppress_alarm);
}

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
