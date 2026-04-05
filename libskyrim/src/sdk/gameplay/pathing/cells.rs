use alloc::vec::Vec;

use crate::re::{
    BSNavmeshInfo, BSPathingCell, BSPathingLocation, BSTSmartPointer, NiPoint3, PathingCell,
    TESObjectCELL, TESObjectREFR, TESWorldSpace,
};
use crate::sdk::core::{GamePtr, snapshot_contiguous_cloned_named};

use super::runtime::singleton;
use super::types::{
    ConcretePathingCellState, LoadedPathingCellDescriptor, RecentPathingCellDescriptor,
    inspect_concrete_pathing_cell_base, inspect_pathing_cell_base,
};

/// Inspect the minimal readiness state for any `BSPathingCell`.
pub fn inspect_pathing_cell(cell: &BSPathingCell) -> super::types::PathingCellState {
    inspect_pathing_cell_base(cell)
}

#[inline(always)]
pub fn is_pathing_cell_ready(cell: &BSPathingCell) -> bool {
    inspect_pathing_cell(cell).ready()
}

/// Inspect the richer concrete state for one `PathingCell`.
pub fn inspect_concrete_pathing_cell(cell: &PathingCell) -> ConcretePathingCellState {
    inspect_concrete_pathing_cell_base(cell)
}

/// Attempt to downcast one abstract `BSPathingCell` owner into a concrete
/// `PathingCell`.
///
/// Use this when later code needs the richer concrete `PathingCell` surface
/// rather than the minimal abstract readiness/state checks.
pub fn concrete_pathing_cell(cell: &BSTSmartPointer<BSPathingCell>) -> GamePtr<PathingCell> {
    unsafe { GamePtr::from_raw(cell.get()) }.try_cast()
}

/// Inspect one smart-pointer owned `BSPathingCell` as a concrete `PathingCell`
/// when that downcast succeeds.
pub fn inspect_concrete_pathing_cell_ptr(
    cell: &BSTSmartPointer<BSPathingCell>,
) -> Option<ConcretePathingCellState> {
    let cell = concrete_pathing_cell(cell).into_option()?;
    Some(inspect_concrete_pathing_cell(cell.as_ref()))
}

/// Resolve the concrete `PathingCell` behind one `BSNavmeshInfo` entry.
pub fn navmesh_info_concrete_pathing_cell(info: &BSNavmeshInfo) -> GamePtr<PathingCell> {
    concrete_pathing_cell(&info.pathing_cell)
}

/// Inspect the concrete `PathingCell` state referenced by one `BSNavmeshInfo`
/// entry.
pub fn inspect_navmesh_info_concrete_pathing_cell(
    info: &BSNavmeshInfo,
) -> Option<ConcretePathingCellState> {
    inspect_concrete_pathing_cell_ptr(&info.pathing_cell)
}

/// Snapshot descriptors for all currently loaded pathing cells retained by the
/// runtime.
pub fn collect_loaded_pathing_cells() -> Vec<LoadedPathingCellDescriptor> {
    singleton().with(|pathing| {
        pathing
            .loaded_cells
            .iter()
            .map(|entry| LoadedPathingCellDescriptor {
                info: entry.first,
                cell: entry.second.clone(),
            })
            .filter(|entry| !entry.cell.is_null())
            .collect()
    })
}

/// Snapshot descriptors for recently used pathing cells retained by the
/// runtime.
pub fn collect_recent_pathing_cells() -> Vec<RecentPathingCellDescriptor> {
    singleton().with(|pathing| {
        snapshot_contiguous_cloned_named(
            &pathing.recent_cells,
            "sdk::gameplay::pathing::collect_recent_pathing_cells()",
            Default::default(),
        )
        .iter()
        .map(|entry| RecentPathingCellDescriptor {
            age_stamp: entry.first,
            cell: entry.second.clone(),
        })
        .filter(|entry| !entry.cell.is_null())
        .collect()
    })
}

/// Check whether two pathing cells refer to the same gameplay space.
///
/// This mirrors the engine's own notion of "same space" instead of trying to
/// infer it from worldspace/cell pointers manually.
pub fn pathing_cells_share_space(
    cell: &mut BSPathingCell,
    other: &BSTSmartPointer<BSPathingCell>,
) -> bool {
    let mut other = other.clone();
    cell.is_in_same_space(&mut other as *mut _)
}

/// Build one `BSPathingLocation` from a point and pathing cell.
///
/// Use this when pathing code already knows the owning `BSPathingCell` but
/// does not yet have a `BSNavmeshInfo`/triangle pairing.
pub fn make_pathing_location(
    point: NiPoint3,
    pathing_cell: BSTSmartPointer<BSPathingCell>,
) -> BSPathingLocation {
    BSPathingLocation {
        location: point,
        nav_mesh_info: core::ptr::null_mut(),
        nav_mesh_info_array: core::ptr::null_mut(),
        pathing_cell,
        triangle: 0,
        flags: 0,
        client_data: 0,
        pad2c: 0,
    }
}

/// Build one `BSPathingLocation` from a point, navmesh info, and pathing cell.
///
/// Prefer this over [`make_pathing_location`] when the caller already has a
/// concrete navmesh entry and triangle index to preserve.
pub fn make_pathing_location_with_info(
    point: NiPoint3,
    nav_mesh_info: GamePtr<BSNavmeshInfo>,
    pathing_cell: BSTSmartPointer<BSPathingCell>,
    triangle: Option<u16>,
) -> BSPathingLocation {
    BSPathingLocation {
        location: point,
        nav_mesh_info: nav_mesh_info.as_ptr(),
        nav_mesh_info_array: core::ptr::null_mut(),
        pathing_cell,
        triangle: triangle.unwrap_or(0),
        flags: 0,
        client_data: 0,
        pad2c: 0,
    }
}

/// Resolve the pathing cell that owns a world-space location/cell/worldspace
/// combination.
///
/// This is the main entry point when pathing workflows start from a point plus
/// world/cell context instead of an existing `TESObjectREFR`.
pub fn get_pathing_cell(
    location: NiPoint3,
    cell: GamePtr<TESObjectCELL>,
    world_space: GamePtr<TESWorldSpace>,
) -> Option<BSTSmartPointer<BSPathingCell>> {
    let mut cell_out = BSTSmartPointer::null();
    let found = unsafe {
        singleton().with_mut_unchecked(|pathing| {
            pathing.get_pathing_cell(
                &location,
                cell.as_ptr(),
                world_space.as_ptr(),
                &mut cell_out,
            )
        })
    };

    if found && !cell_out.is_null() {
        Some(cell_out)
    } else {
        None
    }
}

/// Resolve the pathing cell for one live reference.
///
/// This is the common bridge from gameplay/reference code into pathing code.
pub fn get_pathing_cell_for_reference(
    reference: &TESObjectREFR,
) -> Option<BSTSmartPointer<BSPathingCell>> {
    let location = reference.data.location;
    let cell = unsafe { GamePtr::from_raw(reference.get_parent_cell()) };
    let world_space = unsafe { GamePtr::from_raw(reference.get_worldspace()) };
    get_pathing_cell(location, cell, world_space)
}

/// Pointer-oriented variant of [`get_pathing_cell_for_reference`].
pub fn get_pathing_cell_for_reference_ptr(
    reference: GamePtr<TESObjectREFR>,
) -> Option<BSTSmartPointer<BSPathingCell>> {
    let reference = reference.into_option()?;
    get_pathing_cell_for_reference(reference.as_ref())
}

/// Find the closest navmesh-supported point for one pathing cell.
///
/// Use this to snap one world-space point back onto the local navmesh after
/// movement, spawn, or spatial scoring has produced a nearby candidate.
pub fn find_closest_point_on_navmesh(
    cell: &BSTSmartPointer<BSPathingCell>,
    location: NiPoint3,
) -> Option<NiPoint3> {
    if cell.is_null() {
        crate::defensive_sdk_warn!(
            "sdk::gameplay::pathing::find_closest_point_on_navmesh() received a null cell"
        );
        return None;
    }

    let mut point = location;
    let found = unsafe {
        singleton().with_mut_unchecked(|pathing| {
            pathing.find_closest_point_on_navmesh(cell, &location, &mut point)
        })
    };

    if found { Some(point) } else { None }
}
