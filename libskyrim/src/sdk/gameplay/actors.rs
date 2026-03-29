//! Actor-oriented gameplay helpers over `ProcessLists` and common player-facing
//! actor categorizations.

use alloc::vec::Vec;
use core::ops::ControlFlow;

use crate::re::{Actor, BSContainerForEachResult, NiPoint3, NiPointer, ProcessLists};
use crate::sdk::core::{GameRef, Resolved};
use crate::sdk::gameplay::player;

#[inline(always)]
pub fn process_lists() -> GameRef<ProcessLists> {
    unsafe { GameRef::from_raw(ProcessLists::get_singleton()) }
}

#[inline(always)]
pub fn distance_to_point(actor: &Actor, point: NiPoint3) -> f32 {
    actor.get_position().get_distance(point)
}

#[inline(always)]
pub fn squared_distance_to_point(actor: &Actor, point: NiPoint3) -> f32 {
    actor.get_position().get_squared_distance(point)
}

#[inline(always)]
pub fn distance_to_player(actor: &Actor) -> f32 {
    distance_to_point(actor, player::position())
}

#[inline(always)]
pub fn squared_distance_to_player(actor: &Actor) -> f32 {
    squared_distance_to_point(actor, player::position())
}

#[inline(always)]
pub fn is_within_radius(actor: &Actor, origin: NiPoint3, radius: f32) -> bool {
    squared_distance_to_point(actor, origin) <= radius * radius
}

#[inline(always)]
pub fn is_within_player_radius(actor: &Actor, radius: f32) -> bool {
    is_within_radius(actor, player::position(), radius)
}

#[inline(always)]
pub fn commanding_actor(actor: &Actor) -> NiPointer<Actor> {
    actor.get_commanding_actor()
}

#[inline(always)]
pub fn commanding_actor_resolved(actor: &Actor) -> Option<Resolved<Actor>> {
    Resolved::try_from_ptr(commanding_actor(actor).get())
}

#[inline(always)]
pub fn is_hostile_to(actor: &Actor, target: &Actor) -> bool {
    actor.is_hostile_to_actor(target as *const Actor as *mut Actor)
}

#[inline(always)]
pub fn is_hostile_to_player(actor: &Actor) -> bool {
    actor.is_hostile_to_actor(player::singleton().as_ptr().cast())
}

#[inline(always)]
pub fn is_summon_or_commanded_actor(actor: &Actor) -> bool {
    actor.is_summoned() || actor.is_commanded_actor()
}

#[inline(always)]
pub fn is_commanded_by_player(actor: &Actor) -> bool {
    let commanding_actor = commanding_actor(actor);
    !commanding_actor.is_null() && commanding_actor.is_player_ref()
}

#[inline(always)]
pub fn is_player_follower(actor: &Actor) -> bool {
    actor.is_player_teammate() || is_commanded_by_player(actor)
}

#[inline(always)]
pub fn is_player_ally(actor: &Actor) -> bool {
    actor.is_player_ref() || is_player_follower(actor) || !is_hostile_to_player(actor)
}

pub fn for_each_loaded_actor(mut visit: impl FnMut(&Actor) -> ControlFlow<()>) -> ControlFlow<()> {
    let mut flow = ControlFlow::Continue(());
    process_lists().for_all_actors(|actor| {
        let Some(actor) = (unsafe { actor.as_ref() }) else {
            return BSContainerForEachResult::Continue;
        };

        flow = visit(actor);
        flow_to_engine_result(flow)
    });
    flow
}

pub fn for_each_high_actor(mut visit: impl FnMut(&Actor) -> ControlFlow<()>) -> ControlFlow<()> {
    let mut flow = ControlFlow::Continue(());
    process_lists().for_each_high_actor(|actor| {
        let Some(actor) = (unsafe { actor.as_ref() }) else {
            return BSContainerForEachResult::Continue;
        };

        flow = visit(actor);
        flow_to_engine_result(flow)
    });
    flow
}

pub fn collect_loaded_actors() -> Vec<Resolved<Actor>> {
    collect_loaded_actors_matching(|_| true)
}

pub fn collect_loaded_actors_matching(
    mut predicate: impl FnMut(&Actor) -> bool,
) -> Vec<Resolved<Actor>> {
    let mut actors = Vec::new();
    let _ = for_each_loaded_actor(|actor| {
        if predicate(actor) {
            push_resolved_actor(&mut actors, actor);
        }
        ControlFlow::Continue(())
    });
    actors
}

pub fn collect_high_actors() -> Vec<Resolved<Actor>> {
    collect_high_actors_matching(|_| true)
}

pub fn collect_high_actors_matching(
    mut predicate: impl FnMut(&Actor) -> bool,
) -> Vec<Resolved<Actor>> {
    let mut actors = Vec::new();
    let _ = for_each_high_actor(|actor| {
        if predicate(actor) {
            push_resolved_actor(&mut actors, actor);
        }
        ControlFlow::Continue(())
    });
    actors
}

pub fn collect_nearby_actors(origin: NiPoint3, radius: f32) -> Vec<Resolved<Actor>> {
    collect_loaded_actors_matching(|actor| is_within_radius(actor, origin, radius))
}

#[inline(always)]
pub fn collect_nearby_player_actors(radius: f32) -> Vec<Resolved<Actor>> {
    collect_nearby_actors(player::position(), radius)
}

#[inline(always)]
pub fn collect_player_commanded_actors() -> Vec<Resolved<Actor>> {
    collect_loaded_actors_matching(is_commanded_by_player)
}

#[inline(always)]
pub fn collect_player_allies() -> Vec<Resolved<Actor>> {
    collect_loaded_actors_matching(is_player_ally)
}

#[inline(always)]
pub fn collect_hostile_actors() -> Vec<Resolved<Actor>> {
    collect_loaded_actors_matching(is_hostile_to_player)
}

pub fn collect_hostile_nearby_actors(origin: NiPoint3, radius: f32) -> Vec<Resolved<Actor>> {
    collect_loaded_actors_matching(|actor| {
        is_hostile_to_player(actor) && is_within_radius(actor, origin, radius)
    })
}

#[inline(always)]
pub fn collect_hostile_nearby_player_actors(radius: f32) -> Vec<Resolved<Actor>> {
    collect_hostile_nearby_actors(player::position(), radius)
}

#[inline(always)]
pub fn collect_player_summons() -> Vec<Resolved<Actor>> {
    collect_loaded_actors_matching(Actor::is_summoned_by_player)
}

#[inline(always)]
pub fn collect_player_followers() -> Vec<Resolved<Actor>> {
    collect_loaded_actors_matching(is_player_follower)
}

#[inline(always)]
fn push_resolved_actor(out: &mut Vec<Resolved<Actor>>, actor: &Actor) {
    if let Some(actor) = Resolved::try_from_ref(actor) {
        out.push(actor);
    }
}

#[inline(always)]
fn flow_to_engine_result(flow: ControlFlow<()>) -> BSContainerForEachResult {
    match flow {
        ControlFlow::Continue(()) => BSContainerForEachResult::Continue,
        ControlFlow::Break(()) => BSContainerForEachResult::Stop,
    }
}
