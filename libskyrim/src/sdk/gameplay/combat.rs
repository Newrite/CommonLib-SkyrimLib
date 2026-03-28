//! Combat-state and hit-processing helpers.

use crate::re::{Actor, BSScrapArray, ProcessLists};
use crate::sdk::core::GameRef;
use crate::sdk::gameplay::player;

#[inline(always)]
pub fn singleton() -> GameRef<ProcessLists> {
    unsafe { GameRef::from_raw(ProcessLists::get_singleton()) }
}

#[inline(always)]
pub fn are_hostile_actors_nearby() -> bool {
    let mut actors = BSScrapArray::new();
    unsafe {
        singleton()
            .with_mut_unchecked(|process_lists| process_lists.are_hostile_actors_near(&mut actors))
    }
}

#[inline(always)]
pub fn stop_combat_on_actor(actor: &mut Actor, suppress_alarm: bool) {
    unsafe {
        singleton().with_mut_unchecked(|process_lists| {
            process_lists.stop_combat_and_alarm_on_actor(actor, suppress_alarm)
        })
    };
}

#[inline(always)]
pub fn stop_combat_on_player(suppress_alarm: bool) {
    unsafe {
        player::singleton()
            .with_mut_unchecked(|player| stop_combat_on_actor(player, suppress_alarm))
    };
}
