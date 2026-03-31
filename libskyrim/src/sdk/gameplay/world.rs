//! World/reference traversal helpers over `TES`.
//!
//! This SDK layer keeps `TES` singleton access strict while softening nullable
//! origin/radius seams around reference scans.

use alloc::vec::Vec;
use core::ops::ControlFlow;

use crate::re::{
    Actor, BSContainerForEachResult, NiPoint3, ObjectRefHandle, TES, TESObjectCELL, TESObjectREFR,
};
use crate::relocation::skyrim_cast_const;
use crate::sdk::core::{GamePtr, GameRef, ResolvableHandle, Resolved};
use crate::sdk::gameplay::actors;

#[derive(Debug, Default, Clone)]
pub struct WorldSceneSnapshot {
    pub reference_handles: Vec<ObjectRefHandle>,
    pub actor_positions: Vec<NiPoint3>,
    pub hostile_actor_positions: Vec<NiPoint3>,
}

#[inline(always)]
pub fn singleton() -> GameRef<TES> {
    unsafe { GameRef::from_raw(TES::get_singleton()) }
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
fn origin_from_ptr(
    origin: GamePtr<TESObjectREFR>,
    caller: &'static str,
) -> Option<GameRef<TESObjectREFR>> {
    let Some(origin) = origin.into_option() else {
        crate::defensive_sdk_warn!("{} received a null origin reference", caller);
        return None;
    };

    Some(origin)
}

#[inline(always)]
fn flow_to_engine_result(flow: ControlFlow<()>) -> BSContainerForEachResult {
    match flow {
        ControlFlow::Continue(()) => BSContainerForEachResult::Continue,
        ControlFlow::Break(()) => BSContainerForEachResult::Stop,
    }
}

#[inline(always)]
fn snapshot_reference_handle(reference: &TESObjectREFR) -> Option<ObjectRefHandle> {
    let handle = reference.get_handle();
    (!handle.is_null()).then_some(handle)
}

#[inline(always)]
fn reference_as_actor(reference: &TESObjectREFR) -> Option<&Actor> {
    if !reference.base.is_actor() {
        return None;
    }

    let actor = unsafe { skyrim_cast_const::<TESObjectREFR, Actor>(reference) };
    unsafe { actor.as_ref() }
}

#[inline(always)]
fn snapshot_scene_reference(reference: &TESObjectREFR, snapshot: &mut WorldSceneSnapshot) {
    if let Some(handle) = snapshot_reference_handle(reference) {
        snapshot.reference_handles.push(handle);
    }

    let Some(actor) = reference_as_actor(reference) else {
        return;
    };

    let position = reference.get_position();
    snapshot.actor_positions.push(position);
    if actors::is_hostile_to_player(actor) {
        snapshot.hostile_actor_positions.push(position);
    }
}

// Native-sensitive traversal helpers over the global TES reference set.

pub fn for_each_reference(
    mut visit: impl FnMut(&TESObjectREFR) -> ControlFlow<()>,
) -> ControlFlow<()> {
    let mut flow = ControlFlow::Continue(());
    singleton().for_each_reference(|reference| {
        let Some(reference) = (unsafe { reference.as_ref() }) else {
            return BSContainerForEachResult::Continue;
        };
        flow = visit(reference);
        flow_to_engine_result(flow)
    });
    flow
}

pub fn for_each_reference_in_range(
    origin: &TESObjectREFR,
    radius: f32,
    mut visit: impl FnMut(&TESObjectREFR) -> ControlFlow<()>,
) -> ControlFlow<()> {
    if !is_valid_radius(
        radius,
        "sdk::gameplay::world::for_each_reference_in_range()",
    ) {
        return ControlFlow::Continue(());
    }

    let mut flow = ControlFlow::Continue(());
    singleton().for_each_reference_in_range(
        origin as *const TESObjectREFR as *mut TESObjectREFR,
        radius,
        |reference| {
            let Some(reference) = (unsafe { reference.as_ref() }) else {
                return BSContainerForEachResult::Continue;
            };
            flow = visit(reference);
            flow_to_engine_result(flow)
        },
    );
    flow
}

pub fn for_each_reference_in_cell_range(
    cell: &TESObjectCELL,
    origin: NiPoint3,
    radius: f32,
    mut visit: impl FnMut(&TESObjectREFR) -> ControlFlow<()>,
) -> ControlFlow<()> {
    if !is_valid_radius(
        radius,
        "sdk::gameplay::world::for_each_reference_in_cell_range()",
    ) {
        return ControlFlow::Continue(());
    }

    let mut flow = ControlFlow::Continue(());
    cell.for_each_reference_in_range(&origin, radius, |reference| {
        let Some(reference) = (unsafe { reference.as_ref() }) else {
            return BSContainerForEachResult::Continue;
        };
        flow = visit(reference);
        flow_to_engine_result(flow)
    });
    flow
}

pub fn for_each_reference_in_range_ptr(
    origin: GamePtr<TESObjectREFR>,
    radius: f32,
    visit: impl FnMut(&TESObjectREFR) -> ControlFlow<()>,
) -> bool {
    if !is_valid_radius(
        radius,
        "sdk::gameplay::world::for_each_reference_in_range_ptr()",
    ) {
        return false;
    }

    let Some(origin) = origin_from_ptr(
        origin,
        "sdk::gameplay::world::for_each_reference_in_range_ptr()",
    ) else {
        return false;
    };

    let _ = for_each_reference_in_range(origin.as_ref(), radius, visit);
    true
}

pub fn snapshot_reference_handles() -> Vec<ObjectRefHandle> {
    let mut handles = Vec::new();
    let _ = for_each_reference(|reference| {
        if let Some(handle) = snapshot_reference_handle(reference) {
            handles.push(handle);
        }
        ControlFlow::Continue(())
    });
    handles
}

pub fn snapshot_reference_handles_in_range(
    origin: &TESObjectREFR,
    radius: f32,
) -> Vec<ObjectRefHandle> {
    let mut handles = Vec::new();
    let _ = for_each_reference_in_range(origin, radius, |reference| {
        if let Some(handle) = snapshot_reference_handle(reference) {
            handles.push(handle);
        }
        ControlFlow::Continue(())
    });
    handles
}

pub fn snapshot_reference_handles_in_cell_range(
    cell: &TESObjectCELL,
    origin: NiPoint3,
    radius: f32,
) -> Vec<ObjectRefHandle> {
    let mut handles = Vec::new();
    let _ = for_each_reference_in_cell_range(cell, origin, radius, |reference| {
        if let Some(handle) = snapshot_reference_handle(reference) {
            handles.push(handle);
        }
        ControlFlow::Continue(())
    });
    handles
}

pub fn snapshot_reference_handles_in_range_ptr(
    origin: GamePtr<TESObjectREFR>,
    radius: f32,
) -> Vec<ObjectRefHandle> {
    if !is_valid_radius(
        radius,
        "sdk::gameplay::world::snapshot_reference_handles_in_range_ptr()",
    ) {
        return Vec::new();
    }

    let Some(origin) = origin_from_ptr(
        origin,
        "sdk::gameplay::world::snapshot_reference_handles_in_range_ptr()",
    ) else {
        return Vec::new();
    };

    snapshot_reference_handles_in_range(origin.as_ref(), radius)
}

pub fn collect_references() -> Vec<Resolved<TESObjectREFR>> {
    snapshot_reference_handles()
        .into_iter()
        .filter_map(Resolved::from_handle)
        .collect()
}

pub fn collect_references_in_range(
    origin: &TESObjectREFR,
    radius: f32,
) -> Vec<Resolved<TESObjectREFR>> {
    snapshot_reference_handles_in_range(origin, radius)
        .into_iter()
        .filter_map(Resolved::from_handle)
        .collect()
}

pub fn collect_references_in_cell_range(
    cell: &TESObjectCELL,
    origin: NiPoint3,
    radius: f32,
) -> Vec<Resolved<TESObjectREFR>> {
    snapshot_reference_handles_in_cell_range(cell, origin, radius)
        .into_iter()
        .filter_map(Resolved::from_handle)
        .collect()
}

pub fn collect_references_in_range_ptr(
    origin: GamePtr<TESObjectREFR>,
    radius: f32,
) -> Vec<Resolved<TESObjectREFR>> {
    snapshot_reference_handles_in_range_ptr(origin, radius)
        .into_iter()
        .filter_map(Resolved::from_handle)
        .collect()
}

pub fn snapshot_scene_in_range(origin: &TESObjectREFR, radius: f32) -> WorldSceneSnapshot {
    if !is_valid_radius(radius, "sdk::gameplay::world::snapshot_scene_in_range()") {
        return WorldSceneSnapshot::default();
    }

    let mut snapshot = WorldSceneSnapshot::default();
    let _ = for_each_reference_in_range(origin, radius, |reference| {
        snapshot_scene_reference(reference, &mut snapshot);
        ControlFlow::Continue(())
    });
    snapshot
}

pub fn snapshot_scene_in_cell_range(
    cell: &TESObjectCELL,
    origin: NiPoint3,
    radius: f32,
) -> WorldSceneSnapshot {
    if !is_valid_radius(
        radius,
        "sdk::gameplay::world::snapshot_scene_in_cell_range()",
    ) {
        return WorldSceneSnapshot::default();
    }

    let mut snapshot = WorldSceneSnapshot::default();
    let _ = for_each_reference_in_cell_range(cell, origin, radius, |reference| {
        snapshot_scene_reference(reference, &mut snapshot);
        ControlFlow::Continue(())
    });
    snapshot
}

pub fn snapshot_scene_in_range_ptr(
    origin: GamePtr<TESObjectREFR>,
    radius: f32,
) -> WorldSceneSnapshot {
    if !is_valid_radius(
        radius,
        "sdk::gameplay::world::snapshot_scene_in_range_ptr()",
    ) {
        return WorldSceneSnapshot::default();
    }

    let Some(origin) = origin_from_ptr(
        origin,
        "sdk::gameplay::world::snapshot_scene_in_range_ptr()",
    ) else {
        return WorldSceneSnapshot::default();
    };

    snapshot_scene_in_range(origin.as_ref(), radius)
}
