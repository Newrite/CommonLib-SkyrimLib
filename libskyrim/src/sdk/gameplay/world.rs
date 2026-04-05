//! World/reference traversal helpers over `TES`.
//!
//! This SDK layer keeps `TES` singleton access strict while softening nullable
//! origin/radius seams around reference scans.
//!
//! Use this module when plugin code wants global or radius-bounded reference
//! traversal without dropping to raw `TES::for_each_reference*` callbacks.
//!
//! Common workflows:
//!
//! - collect or visit all tracked references with [`for_each_reference`] and
//!   [`collect_references`]
//! - scan around one origin reference with [`for_each_reference_in_range`] and
//!   [`collect_references_in_range`]
//! - build lightweight world-space snapshots with [`snapshot_scene_in_range`]
//!   and [`snapshot_scene_in_cell_range`]
//!
//! Example:
//!
//! ```rust,ignore
//! use libskyrim::sdk::gameplay::world;
//!
//! fn nearby_reference_count(origin: &libskyrim::re::TESObjectREFR) -> usize {
//!     world::snapshot_reference_handles_in_range(origin, 1024.0).len()
//! }
//! ```

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
    /// All references captured in the scene snapshot, as handles.
    pub reference_handles: Vec<ObjectRefHandle>,
    /// Positions of actor references found in the scene.
    pub actor_positions: Vec<NiPoint3>,
    /// Positions of hostile actor references found in the scene.
    pub hostile_actor_positions: Vec<NiPoint3>,
}

/// Return the global `TES` singleton.
///
/// Prefer the traversal helpers in this module for ordinary scans. This raw
/// accessor exists for code that still needs direct `TES` methods.
#[inline(always)]
pub fn singleton() -> GameRef<TES> {
    unsafe { GameRef::from_raw(TES::get_singleton()) }
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

#[inline(always)]
fn origin_from_ptr(
    origin: GamePtr<TESObjectREFR>,
    _caller: &'static str,
) -> Option<GameRef<TESObjectREFR>> {
    let Some(origin) = origin.into_option() else {
        crate::defensive_sdk_warn!("{} received a null origin reference", _caller);
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

/// Visit every reference currently tracked by the global `TES` singleton.
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

/// Visit references within a radius around a reference origin.
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

/// Visit references within a radius around an arbitrary point inside a cell.
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

/// Pointer-friendly variant of [`for_each_reference_in_range`].
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

/// Snapshot handles for all tracked world references.
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

/// Snapshot handles for references within a radius around an origin reference.
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

/// Snapshot handles for references within a radius around a point inside a
/// cell.
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

/// Pointer-friendly variant of [`snapshot_reference_handles_in_range`].
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

/// Resolve all tracked world references.
pub fn collect_references() -> Vec<Resolved<TESObjectREFR>> {
    snapshot_reference_handles()
        .into_iter()
        .filter_map(Resolved::from_handle)
        .collect()
}

/// Resolve references within a radius around an origin reference.
pub fn collect_references_in_range(
    origin: &TESObjectREFR,
    radius: f32,
) -> Vec<Resolved<TESObjectREFR>> {
    snapshot_reference_handles_in_range(origin, radius)
        .into_iter()
        .filter_map(Resolved::from_handle)
        .collect()
}

/// Resolve references within a radius around a point inside a cell.
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

/// Pointer-friendly variant of [`collect_references_in_range`].
pub fn collect_references_in_range_ptr(
    origin: GamePtr<TESObjectREFR>,
    radius: f32,
) -> Vec<Resolved<TESObjectREFR>> {
    snapshot_reference_handles_in_range_ptr(origin, radius)
        .into_iter()
        .filter_map(Resolved::from_handle)
        .collect()
}

/// Snapshot nearby reference and actor positions around a reference origin.
///
/// This is the highest-level world scan in the module: it collects reference
/// handles plus actor/hostile positions in one pass for plugins that need a
/// coarse scene summary.
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

/// Snapshot nearby reference and actor positions around a point inside a cell.
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

/// Pointer-friendly variant of [`snapshot_scene_in_range`].
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
