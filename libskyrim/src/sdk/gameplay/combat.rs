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
pub fn are_hostile_actors_nearby() -> bool {
    let mut actors = BSScrapArray::new();
    unsafe {
        singleton()
            .with_mut_unchecked(|process_lists| process_lists.are_hostile_actors_near(&mut actors))
    }
}

pub fn collect_hostile_actors_nearby() -> Vec<Resolved<Actor>> {
    let mut handles = BSScrapArray::new();
    let _ = unsafe {
        singleton()
            .with_mut_unchecked(|process_lists| process_lists.are_hostile_actors_near(&mut handles))
    };

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
    unsafe {
        singleton().with_mut_unchecked(|process_lists| {
            process_lists.stop_combat_and_alarm_on_actor(actor, suppress_alarm)
        })
    };
}

pub fn stop_combat_on_actors(actors: &mut [Resolved<Actor>], suppress_alarm: bool) {
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
    let mut actors = actors::collect_nearby_player_actors(radius);
    stop_combat_on_actors(&mut actors, suppress_alarm);
}

pub fn stop_combat_on_hostile_nearby_actors(radius: f32, suppress_alarm: bool) {
    let mut actors = actors::collect_hostile_nearby_player_actors(radius);
    stop_combat_on_actors(&mut actors, suppress_alarm);
}
