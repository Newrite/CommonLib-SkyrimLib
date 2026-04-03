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

pub fn inspect_pathing_cell(cell: &BSPathingCell) -> super::types::PathingCellState {
    inspect_pathing_cell_base(cell)
}

#[inline(always)]
pub fn is_pathing_cell_ready(cell: &BSPathingCell) -> bool {
    inspect_pathing_cell(cell).ready()
}

pub fn inspect_concrete_pathing_cell(cell: &PathingCell) -> ConcretePathingCellState {
    inspect_concrete_pathing_cell_base(cell)
}

pub fn concrete_pathing_cell(cell: &BSTSmartPointer<BSPathingCell>) -> GamePtr<PathingCell> {
    unsafe { GamePtr::from_raw(cell.get()) }.try_cast()
}

pub fn inspect_concrete_pathing_cell_ptr(
    cell: &BSTSmartPointer<BSPathingCell>,
) -> Option<ConcretePathingCellState> {
    let cell = concrete_pathing_cell(cell).into_option()?;
    Some(inspect_concrete_pathing_cell(cell.as_ref()))
}

pub fn navmesh_info_concrete_pathing_cell(info: &BSNavmeshInfo) -> GamePtr<PathingCell> {
    concrete_pathing_cell(&info.pathing_cell)
}

pub fn inspect_navmesh_info_concrete_pathing_cell(
    info: &BSNavmeshInfo,
) -> Option<ConcretePathingCellState> {
    inspect_concrete_pathing_cell_ptr(&info.pathing_cell)
}

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

pub fn pathing_cells_share_space(
    cell: &mut BSPathingCell,
    other: &BSTSmartPointer<BSPathingCell>,
) -> bool {
    let mut other = other.clone();
    cell.is_in_same_space(&mut other as *mut _)
}

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

pub fn get_pathing_cell_for_reference(
    reference: &TESObjectREFR,
) -> Option<BSTSmartPointer<BSPathingCell>> {
    let location = reference.data.location;
    let cell = unsafe { GamePtr::from_raw(reference.get_parent_cell()) };
    let world_space = unsafe { GamePtr::from_raw(reference.get_worldspace()) };
    get_pathing_cell(location, cell, world_space)
}

pub fn get_pathing_cell_for_reference_ptr(
    reference: GamePtr<TESObjectREFR>,
) -> Option<BSTSmartPointer<BSPathingCell>> {
    let reference = reference.into_option()?;
    get_pathing_cell_for_reference(reference.as_ref())
}

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
