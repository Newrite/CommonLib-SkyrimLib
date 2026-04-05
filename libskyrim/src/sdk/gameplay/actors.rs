//! Actor-oriented gameplay helpers over `ProcessLists` and common player-facing
//! actor categorizations.
//!
//! This module is the broad actor-query companion to [`crate::sdk::gameplay::player`]:
//! it exposes repeated loaded-actor workflows such as hostility, ally/follower
//! classification, distance/radius checks, and loaded/high-process actor
//! collection.
//!
//! Example:
//!
//! ```rust,ignore
//! use libskyrim::sdk::gameplay::actors;
//!
//! fn nearby_hostile_count(radius: f32) -> usize {
//!     actors::collect_hostile_nearby_player_actors(radius).len()
//! }
//! ```

use alloc::vec::Vec;
use core::ops::ControlFlow;

use crate::re::{
    Actor, ActorHandle, BSContainerForEachResult, BSTArray, NiPoint3, NiPointer, ProcessLists,
};
use crate::sdk::core::{
    ContiguousSequenceIterationOptions, GamePtr, GameRef, Resolved,
    for_each_contiguous_sequence_named,
};
use crate::sdk::gameplay::player;

const MAX_REASONABLE_PROCESS_ACTOR_HANDLES: u32 = 0x1_0000;

/// Return the global `ProcessLists` singleton.
///
/// Most gameplay code should prefer the collection helpers below, but the raw
/// accessor remains useful when a plugin needs direct `ProcessLists` state.
#[inline(always)]
pub fn process_lists() -> GameRef<ProcessLists> {
    unsafe { GameRef::from_raw(ProcessLists::get_singleton()) }
}

// Query-ish helpers over already-available actor state.

/// Distance from the actor to a world-space point.
#[inline(always)]
pub fn distance_to_point(actor: &Actor, point: NiPoint3) -> f32 {
    actor.get_position().get_distance(point)
}

/// Squared distance from the actor to a world-space point.
#[inline(always)]
pub fn squared_distance_to_point(actor: &Actor, point: NiPoint3) -> f32 {
    actor.get_position().get_squared_distance(point)
}

/// Distance from the actor to the player.
#[inline(always)]
pub fn distance_to_player(actor: &Actor) -> f32 {
    distance_to_point(actor, player::position())
}

/// Squared distance from the actor to the player.
#[inline(always)]
pub fn squared_distance_to_player(actor: &Actor) -> f32 {
    squared_distance_to_point(actor, player::position())
}

/// Whether the actor lies within the supplied radius around `origin`.
#[inline(always)]
pub fn is_within_radius(actor: &Actor, origin: NiPoint3, radius: f32) -> bool {
    squared_distance_to_point(actor, origin) <= radius * radius
}

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

/// Whether the actor lies within the supplied radius around the player.
#[inline(always)]
pub fn is_within_player_radius(actor: &Actor, radius: f32) -> bool {
    is_within_radius(actor, player::position(), radius)
}

/// Return the actor currently commanding this actor, if any.
#[inline(always)]
pub fn commanding_actor(actor: &Actor) -> NiPointer<Actor> {
    actor.get_commanding_actor()
}

/// Nullable pointer form of [`commanding_actor`].
#[inline(always)]
pub fn commanding_actor_ptr(actor: &Actor) -> GamePtr<Actor> {
    unsafe { GamePtr::from_raw(commanding_actor(actor).get()) }
}

/// Resolved-handle form of [`commanding_actor`].
#[inline(always)]
pub fn commanding_actor_resolved(actor: &Actor) -> Option<Resolved<Actor>> {
    Resolved::try_from_ptr(commanding_actor_ptr(actor).as_ptr())
}

/// Whether `actor` is hostile to `target`.
#[inline(always)]
pub fn is_hostile_to(actor: &Actor, target: &Actor) -> bool {
    actor.is_hostile_to_actor(target as *const Actor as *mut Actor)
}

#[inline(always)]
pub fn is_hostile_to_ptr(actor: &Actor, target: GamePtr<Actor>) -> bool {
    let Some(target) = target.as_ref() else {
        crate::defensive_sdk_warn!(
            "sdk::gameplay::actors::is_hostile_to_ptr() received a null target actor"
        );
        return false;
    };

    is_hostile_to(actor, target)
}

#[inline(always)]
pub fn is_hostile_to_owner(actor: &Actor, target: &NiPointer<Actor>) -> bool {
    is_hostile_to_ptr(actor, unsafe { GamePtr::from_raw(target.get()) })
}

/// Whether the actor is hostile to the player.
#[inline(always)]
pub fn is_hostile_to_player(actor: &Actor) -> bool {
    actor.is_hostile_to_actor(player::singleton().as_ptr().cast())
}

/// Whether the actor is a summon or otherwise commanded.
#[inline(always)]
pub fn is_summon_or_commanded_actor(actor: &Actor) -> bool {
    actor.is_summoned() || actor.is_commanded_actor()
}

/// Whether the actor is currently commanded by the player.
#[inline(always)]
pub fn is_commanded_by_player(actor: &Actor) -> bool {
    commanding_actor_ptr(actor)
        .as_ref()
        .is_some_and(|actor| actor.base.base.is_player_ref())
}

/// Whether the actor behaves like a player follower.
#[inline(always)]
pub fn is_player_follower(actor: &Actor) -> bool {
    actor.is_player_teammate() || is_commanded_by_player(actor)
}

/// Whether the actor should be treated as a player ally.
#[inline(always)]
pub fn is_player_ally(actor: &Actor) -> bool {
    actor.is_player_ref() || is_player_follower(actor) || !is_hostile_to_player(actor)
}

// Native-sensitive helpers: container traversal, handle resolution, and
// hostility checks over global actor state.

/// Visit all currently loaded actors.
pub fn for_each_loaded_actor(mut visit: impl FnMut(&Actor) -> ControlFlow<()>) -> ControlFlow<()> {
    let mut flow = ControlFlow::Continue(());
    for_each_loaded_actor_owner(|_, owner| {
        let Some(actor) = owner_actor_ref(owner) else {
            return BSContainerForEachResult::Continue;
        };
        flow = visit(actor);
        flow_to_engine_result(flow)
    });
    flow
}

/// Visit the high-process actor set.
pub fn for_each_high_actor(mut visit: impl FnMut(&Actor) -> ControlFlow<()>) -> ControlFlow<()> {
    let mut flow = ControlFlow::Continue(());
    for_each_high_actor_owner(|_, owner| {
        let Some(actor) = owner_actor_ref(owner) else {
            return BSContainerForEachResult::Continue;
        };
        flow = visit(actor);
        flow_to_engine_result(flow)
    });
    flow
}

/// Collect all currently loaded actors.
///
/// This is the widest actor snapshot in the module and the usual starting
/// point for plugin-side filtering.
pub fn collect_loaded_actors() -> Vec<Resolved<Actor>> {
    collect_loaded_actors_matching(|_| true)
}

/// Collect loaded actors matching a predicate.
pub fn collect_loaded_actors_matching(
    mut predicate: impl FnMut(&Actor) -> bool,
) -> Vec<Resolved<Actor>> {
    let mut actors = Vec::new();
    for (handle, owner) in snapshot_loaded_actor_entries() {
        let Some(actor) = owner_actor_ref(&owner) else {
            continue;
        };
        if predicate(actor) {
            if let Some(actor) = Resolved::from_handle_owner(handle, owner.clone()) {
                actors.push(actor);
            }
        }
    }
    actors
}

/// Collect all currently high-process actors.
pub fn collect_high_actors() -> Vec<Resolved<Actor>> {
    collect_high_actors_matching(|_| true)
}

/// Collect high-process actors matching a predicate.
pub fn collect_high_actors_matching(
    mut predicate: impl FnMut(&Actor) -> bool,
) -> Vec<Resolved<Actor>> {
    let mut actors = Vec::new();
    for (handle, owner) in snapshot_high_actor_entries() {
        let Some(actor) = owner_actor_ref(&owner) else {
            continue;
        };
        if predicate(actor) {
            if let Some(actor) = Resolved::from_handle_owner(handle, owner.clone()) {
                actors.push(actor);
            }
        }
    }
    actors
}

/// Collect loaded actors near an arbitrary point.
pub fn collect_nearby_actors(origin: NiPoint3, radius: f32) -> Vec<Resolved<Actor>> {
    if !is_valid_radius(radius, "sdk::gameplay::actors::collect_nearby_actors()") {
        return Vec::new();
    }

    collect_loaded_actors_matching(|actor| is_within_radius(actor, origin, radius))
}

/// Collect loaded actors near the player.
#[inline(always)]
pub fn collect_nearby_player_actors(radius: f32) -> Vec<Resolved<Actor>> {
    collect_nearby_actors(player::position(), radius)
}

/// Collect actors currently commanded by the player.
#[inline(always)]
pub fn collect_player_commanded_actors() -> Vec<Resolved<Actor>> {
    collect_loaded_actors_matching(is_commanded_by_player)
}

/// Collect player allies from the loaded actor set.
#[inline(always)]
pub fn collect_player_allies() -> Vec<Resolved<Actor>> {
    collect_loaded_actors_matching(is_player_ally)
}

/// Collect actors currently hostile to the player.
#[inline(always)]
pub fn collect_hostile_actors() -> Vec<Resolved<Actor>> {
    collect_loaded_actors_matching(is_hostile_to_player)
}

/// Collect hostile actors near an arbitrary point.
pub fn collect_hostile_nearby_actors(origin: NiPoint3, radius: f32) -> Vec<Resolved<Actor>> {
    if !is_valid_radius(
        radius,
        "sdk::gameplay::actors::collect_hostile_nearby_actors()",
    ) {
        return Vec::new();
    }

    collect_loaded_actors_matching(|actor| {
        is_hostile_to_player(actor) && is_within_radius(actor, origin, radius)
    })
}

/// Snapshot positions of loaded actors within range.
pub fn collect_actor_positions_in_range(origin: NiPoint3, radius: f32) -> Vec<NiPoint3> {
    if !is_valid_radius(
        radius,
        "sdk::gameplay::actors::collect_actor_positions_in_range()",
    ) {
        return Vec::new();
    }

    let mut positions = Vec::new();
    let _ = for_each_loaded_actor(|actor| {
        if is_within_radius(actor, origin, radius) {
            positions.push(actor.get_position());
        }
        ControlFlow::Continue(())
    });
    positions
}

/// Snapshot positions of hostile loaded actors within range.
pub fn collect_hostile_positions_in_range(origin: NiPoint3, radius: f32) -> Vec<NiPoint3> {
    if !is_valid_radius(
        radius,
        "sdk::gameplay::actors::collect_hostile_positions_in_range()",
    ) {
        return Vec::new();
    }

    let mut positions = Vec::new();
    let _ = for_each_loaded_actor(|actor| {
        if is_hostile_to_player(actor) && is_within_radius(actor, origin, radius) {
            positions.push(actor.get_position());
        }
        ControlFlow::Continue(())
    });
    positions
}

/// Collect hostile actors near the player.
#[inline(always)]
pub fn collect_hostile_nearby_player_actors(radius: f32) -> Vec<Resolved<Actor>> {
    collect_hostile_nearby_actors(player::position(), radius)
}

/// Collect summons currently owned by the player.
#[inline(always)]
pub fn collect_player_summons() -> Vec<Resolved<Actor>> {
    collect_loaded_actors_matching(Actor::is_summoned_by_player)
}

/// Collect actors behaving as player followers.
#[inline(always)]
pub fn collect_player_followers() -> Vec<Resolved<Actor>> {
    collect_loaded_actors_matching(is_player_follower)
}

#[inline(always)]
fn flow_to_engine_result(flow: ControlFlow<()>) -> BSContainerForEachResult {
    match flow {
        ControlFlow::Continue(()) => BSContainerForEachResult::Continue,
        ControlFlow::Break(()) => BSContainerForEachResult::Stop,
    }
}

#[inline(always)]
pub fn snapshot_loaded_actor_owners() -> Vec<NiPointer<Actor>> {
    snapshot_loaded_actor_entries()
        .into_iter()
        .map(|(_, owner)| owner)
        .collect()
}

#[inline(always)]
pub fn snapshot_high_actor_owners() -> Vec<NiPointer<Actor>> {
    snapshot_high_actor_entries()
        .into_iter()
        .map(|(_, owner)| owner)
        .collect()
}

#[inline(always)]
fn snapshot_loaded_actor_entries() -> Vec<(ActorHandle, NiPointer<Actor>)> {
    let process_lists = process_lists();
    let mut entries = Vec::new();
    snapshot_actor_handle_array(&process_lists.high_actor_handles, &mut entries);
    snapshot_actor_handle_array(&process_lists.middle_high_actor_handles, &mut entries);
    snapshot_actor_handle_array(&process_lists.middle_low_actor_handles, &mut entries);
    snapshot_actor_handle_array(&process_lists.low_actor_handles, &mut entries);
    entries
}

#[inline(always)]
fn snapshot_high_actor_entries() -> Vec<(ActorHandle, NiPointer<Actor>)> {
    let process_lists = process_lists();
    let mut entries = Vec::new();
    snapshot_actor_handle_array(&process_lists.high_actor_handles, &mut entries);
    entries
}

#[inline(always)]
fn for_each_loaded_actor_owner(
    mut visit: impl FnMut(ActorHandle, &NiPointer<Actor>) -> BSContainerForEachResult,
) {
    let process_lists = process_lists();
    for actor_handles in [
        &process_lists.high_actor_handles,
        &process_lists.middle_high_actor_handles,
        &process_lists.middle_low_actor_handles,
        &process_lists.low_actor_handles,
    ] {
        if for_each_actor_handle_owner(actor_handles, &mut visit) == BSContainerForEachResult::Stop
        {
            return;
        }
    }
}

#[inline(always)]
fn for_each_high_actor_owner(
    mut visit: impl FnMut(ActorHandle, &NiPointer<Actor>) -> BSContainerForEachResult,
) {
    let process_lists = process_lists();
    let _ = for_each_actor_handle_owner(&process_lists.high_actor_handles, &mut visit);
}

#[inline(always)]
fn for_each_actor_handle_owner(
    actor_handles: &BSTArray<ActorHandle>,
    visit: &mut impl FnMut(ActorHandle, &NiPointer<Actor>) -> BSContainerForEachResult,
) -> BSContainerForEachResult {
    let mut result = BSContainerForEachResult::Continue;
    let _ = for_each_contiguous_sequence_named(
        actor_handles,
        "sdk::gameplay::actors::for_each_actor_handle_owner()",
        actor_handle_iteration_options(),
        |actor_handle| {
            let actor_handle = *actor_handle;
            let Some(actor_owner) = resolve_actor_handle_owner(actor_handle) else {
                return ControlFlow::Continue(());
            };

            if visit(actor_handle, &actor_owner) == BSContainerForEachResult::Stop {
                result = BSContainerForEachResult::Stop;
                ControlFlow::Break(())
            } else {
                ControlFlow::Continue(())
            }
        },
    );
    result
}

#[inline(always)]
fn snapshot_actor_handle_array(
    actor_handles: &BSTArray<ActorHandle>,
    out: &mut Vec<(ActorHandle, NiPointer<Actor>)>,
) {
    let _ = for_each_contiguous_sequence_named(
        actor_handles,
        "sdk::gameplay::actors::snapshot_actor_handle_array()",
        actor_handle_iteration_options(),
        |actor_handle| {
            let actor_handle = *actor_handle;
            let Some(actor_owner) = resolve_actor_handle_owner(actor_handle) else {
                return ControlFlow::Continue(());
            };

            out.push((actor_handle, actor_owner));
            ControlFlow::Continue(())
        },
    );
}

#[inline(always)]
fn resolve_actor_handle_owner(actor_handle: ActorHandle) -> Option<NiPointer<Actor>> {
    let actor_owner = actor_handle.get();
    if actor_owner.is_null() {
        return None;
    }

    Some(actor_owner)
}

#[inline(always)]
fn owner_actor_ref(owner: &NiPointer<Actor>) -> Option<&Actor> {
    unsafe { owner.get().as_ref() }
}

#[inline(always)]
fn actor_handle_iteration_options() -> ContiguousSequenceIterationOptions {
    ContiguousSequenceIterationOptions::new()
        .with_max_reasonable_len(MAX_REASONABLE_PROCESS_ACTOR_HANDLES)
}
