//! Pathing-oriented gameplay helpers over translated `BSPathing*` surfaces.
//!
//! This module intentionally stays thin and source-backed: it wraps the RE
//! entrypoints that are already translated without pretending that a complete
//! global `Pathing` singleton surface is available yet.

use alloc::collections::VecDeque;
use alloc::vec::Vec;

use crate::re::pathing::BSPathingCellManagerExt;
use crate::re::{
    BSNavmesh, BSNavmeshEdgeExtraInfoType, BSNavmeshInfo, BSNavmeshInfoMap, BSPathing,
    BSPathingCell, BSPathingDoor, BSPathingLocation, BSPrecomputedNavmeshInfoPathMap, BSTArray,
    BSTSmartPointer, CellID, ExtraNavMeshPortal, FormID, NAVMESH_PORTAL, NavMesh, NiAVObject,
    NiPoint3, Pathing, PathingCell, PathingCellInfo, TES, TESObjectCELL, TESObjectREFR,
    TESWorldSpace,
};
use crate::sdk::core::{
    GamePtr, GameRef, snapshot_contiguous_cloned_named, snapshot_contiguous_copied_named,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PathingCellState {
    pub type_id: u32,
    pub valid: bool,
    pub attached: bool,
    pub loaded: bool,
}

impl PathingCellState {
    #[inline(always)]
    pub fn ready(&self) -> bool {
        self.valid && self.attached && self.loaded
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ConcretePathingCellState {
    pub base: PathingCellState,
    pub world_space_id: FormID,
    pub cell_form_id: FormID,
    pub cell_coordinates: CellID,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NavMeshPortalDescriptor {
    pub triangle_index: u16,
    pub nav_mesh_form_id_bits: FormID,
    pub nav_mesh: GamePtr<NavMesh>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NavMeshEdgeTransitionDescriptor {
    pub source_triangle_index: u16,
    pub source_edge_index: u8,
    pub destination_nav_mesh_id: FormID,
    pub destination_triangle_index: u16,
    pub destination_edge_index: i8,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct NavMeshInfoGraphPath {
    pub start_nav_mesh_id: FormID,
    pub goal_nav_mesh_id: FormID,
    pub hops: u32,
    pub approximate_cost: f32,
}

#[derive(Clone)]
pub struct LoadedPathingCellDescriptor {
    pub info: PathingCellInfo,
    pub cell: BSTSmartPointer<PathingCell>,
}

#[derive(Clone)]
pub struct RecentPathingCellDescriptor {
    pub age_stamp: u64,
    pub cell: BSTSmartPointer<PathingCell>,
}

#[inline(always)]
fn info_ptr(info: *mut BSNavmeshInfo) -> GamePtr<BSNavmeshInfo> {
    unsafe { GamePtr::from_raw(info) }
}

#[inline(always)]
fn door_ptr(door: *mut BSPathingDoor) -> GamePtr<BSPathingDoor> {
    unsafe { GamePtr::from_raw(door) }
}

#[inline(always)]
fn navmesh_ptr(nav_mesh: *mut NavMesh) -> GamePtr<NavMesh> {
    unsafe { GamePtr::from_raw(nav_mesh) }
}

#[inline(always)]
fn describe_portal(portal: NAVMESH_PORTAL) -> NavMeshPortalDescriptor {
    NavMeshPortalDescriptor {
        triangle_index: portal.tri_index,
        nav_mesh_form_id_bits: unsafe { portal.nav.nav_mesh_id },
        nav_mesh: navmesh_ptr(unsafe { portal.nav.nav_mesh }),
    }
}

pub fn singleton() -> GameRef<Pathing> {
    unsafe { GameRef::from_raw(Pathing::get_singleton()) }
}

pub fn nav_mesh_info_map() -> GameRef<crate::re::NavMeshInfoMap> {
    unsafe {
        GameRef::from_raw(
            TES::get_singleton()
                .as_mut()
                .unwrap()
                .get_runtime_data2()
                .nav_mesh_info_map,
        )
    }
}

#[inline(always)]
pub fn exterior_cell_width() -> f32 {
    singleton().with(|pathing| pathing.get_exterior_cell_width())
}

pub fn inspect_pathing_cell(cell: &BSPathingCell) -> PathingCellState {
    PathingCellState {
        type_id: cell.get_type(),
        valid: cell.q_valid(),
        attached: cell.q_attached(),
        loaded: cell.q_loaded(),
    }
}

#[inline(always)]
pub fn is_pathing_cell_ready(cell: &BSPathingCell) -> bool {
    inspect_pathing_cell(cell).ready()
}

pub fn inspect_concrete_pathing_cell(cell: &PathingCell) -> ConcretePathingCellState {
    ConcretePathingCellState {
        base: inspect_pathing_cell(cell.as_ref()),
        world_space_id: cell.pathing_cell_info.world_space_id,
        cell_form_id: unsafe { cell.pathing_cell_info.cell_id.form_id },
        cell_coordinates: unsafe { cell.pathing_cell_info.cell_id.coordinates },
    }
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

pub fn lookup_navmesh_info(
    navmesh_info_map: &mut BSNavmeshInfoMap,
    nav_mesh_id: FormID,
) -> GamePtr<BSNavmeshInfo> {
    if nav_mesh_id == 0 {
        crate::defensive_sdk_warn!(
            "sdk::gameplay::pathing::lookup_navmesh_info() ignored nav_mesh_id=0"
        );
        return GamePtr::null();
    }

    info_ptr(navmesh_info_map.get_navmesh_info(nav_mesh_id))
}

pub fn collect_navmesh_infos(
    navmesh_info_map: &mut BSNavmeshInfoMap,
) -> Vec<GamePtr<BSNavmeshInfo>> {
    let mut infos = BSTArray::new();
    navmesh_info_map.get_all_nav_mesh_info(&mut infos);
    snapshot_contiguous_copied_named(
        &infos,
        "sdk::gameplay::pathing::collect_navmesh_infos()",
        Default::default(),
    )
    .into_iter()
    .map(info_ptr)
    .filter(|info| !info.is_null())
    .collect()
}

pub fn collect_connected_navmesh_infos(
    navmesh_info_map: &mut BSNavmeshInfoMap,
    info: &BSNavmeshInfo,
) -> Vec<GamePtr<BSNavmeshInfo>> {
    let mut infos = BSTArray::new();
    navmesh_info_map.build_list_of_connected_infos(info as *const _, &mut infos);
    snapshot_contiguous_copied_named(
        &infos,
        "sdk::gameplay::pathing::collect_connected_navmesh_infos()",
        Default::default(),
    )
    .into_iter()
    .map(info_ptr)
    .filter(|info| !info.is_null())
    .collect()
}

pub fn collect_connected_navmesh_ids(
    navmesh_info_map: &mut BSNavmeshInfoMap,
    info: &BSNavmeshInfo,
) -> Vec<FormID> {
    collect_connected_navmesh_infos(navmesh_info_map, info)
        .into_iter()
        .filter_map(|info| info.as_ref().map(|info| info.nav_mesh_id))
        .collect()
}

pub fn collect_adjacent_navmesh_ids(info: &BSNavmeshInfo) -> Vec<FormID> {
    info.adjacent_mesh_ids()
        .iter()
        .copied()
        .filter(|id| *id != 0)
        .collect()
}

pub fn collect_adjacent_navmesh_infos(
    navmesh_info_map: &mut BSNavmeshInfoMap,
    info: &BSNavmeshInfo,
) -> Vec<GamePtr<BSNavmeshInfo>> {
    collect_adjacent_navmesh_ids(info)
        .into_iter()
        .map(|nav_mesh_id| lookup_navmesh_info(navmesh_info_map, nav_mesh_id))
        .filter(|info| !info.is_null())
        .collect()
}

pub fn lookup_navmesh_info_for_navmesh(
    navmesh_info_map: &mut BSNavmeshInfoMap,
    nav_mesh: &NavMesh,
) -> GamePtr<BSNavmeshInfo> {
    lookup_navmesh_info(navmesh_info_map, nav_mesh.base.get_form_id())
}

pub fn lookup_navmesh_info_for_navmesh_ptr(
    navmesh_info_map: &mut BSNavmeshInfoMap,
    nav_mesh: &BSTSmartPointer<NavMesh>,
) -> GamePtr<BSNavmeshInfo> {
    let Some(nav_mesh) = (unsafe { nav_mesh.get().as_ref() }) else {
        return GamePtr::null();
    };
    lookup_navmesh_info_for_navmesh(navmesh_info_map, nav_mesh)
}

pub fn precomputed_navmesh_path_index(
    path_map: &BSPrecomputedNavmeshInfoPathMap,
    info: &BSNavmeshInfo,
) -> Option<usize> {
    path_map
        .info_to_index_map
        .get(&(info as *const BSNavmeshInfo))
        .map(|entry| entry.second as usize)
}

pub fn precomputed_navmesh_path_at(
    path_map: &BSPrecomputedNavmeshInfoPathMap,
    index: usize,
) -> Vec<GamePtr<BSNavmeshInfo>> {
    let Some(path) = path_map.all_paths_slice().get(index).copied() else {
        return Vec::new();
    };
    let Some(path) = (unsafe { path.as_ref() }) else {
        return Vec::new();
    };

    snapshot_contiguous_copied_named(
        path,
        "sdk::gameplay::pathing::precomputed_navmesh_path_at()",
        Default::default(),
    )
    .into_iter()
    .map(|info| unsafe { GamePtr::from_raw(info.cast_mut()) })
    .filter(|info| !info.is_null())
    .collect()
}

pub fn precomputed_navmesh_path_for_info(
    path_map: &BSPrecomputedNavmeshInfoPathMap,
    info: &BSNavmeshInfo,
) -> Vec<GamePtr<BSNavmeshInfo>> {
    let Some(index) = precomputed_navmesh_path_index(path_map, info) else {
        return Vec::new();
    };
    precomputed_navmesh_path_at(path_map, index)
}

pub fn collect_precomputed_navmesh_paths(
    path_map: &BSPrecomputedNavmeshInfoPathMap,
) -> Vec<Vec<GamePtr<BSNavmeshInfo>>> {
    (0..path_map.path_count())
        .map(|index| precomputed_navmesh_path_at(path_map, index))
        .collect()
}

pub fn approximate_navmesh_info_path_cost(path: &[GamePtr<BSNavmeshInfo>]) -> Option<(f32, u32)> {
    if path.is_empty() {
        return None;
    }

    let mut total = 0.0;
    let mut previous = path[0].as_ref()?.approx_location;
    let mut hops = 0u32;
    for info in path.iter().skip(1) {
        let info = info.as_ref()?;
        total += previous.get_distance(info.approx_location);
        previous = info.approx_location;
        hops = hops.saturating_add(1);
    }

    Some((total, hops))
}

pub fn approximate_navmesh_info_graph_path(
    navmesh_info_map: &mut BSNavmeshInfoMap,
    start: &BSNavmeshInfo,
    goal: &BSNavmeshInfo,
) -> Option<NavMeshInfoGraphPath> {
    if start.nav_mesh_id == 0 || goal.nav_mesh_id == 0 {
        return None;
    }

    if start.nav_mesh_id == goal.nav_mesh_id {
        return Some(NavMeshInfoGraphPath {
            start_nav_mesh_id: start.nav_mesh_id,
            goal_nav_mesh_id: goal.nav_mesh_id,
            hops: 0,
            approximate_cost: 0.0,
        });
    }

    let mut frontier: VecDeque<(FormID, NiPoint3, f32, u32)> = VecDeque::new();
    let mut seen = alloc::vec::Vec::<FormID>::new();
    frontier.push_back((start.nav_mesh_id, start.approx_location, 0.0f32, 0u32));
    seen.push(start.nav_mesh_id);

    while let Some((current_id, current_location, current_cost, current_hops)) =
        frontier.pop_front()
    {
        let current = lookup_navmesh_info(navmesh_info_map, current_id).into_option()?;
        let current = current.as_ref();
        for neighbor_id in collect_adjacent_navmesh_ids(current) {
            if seen.contains(&neighbor_id) {
                continue;
            }

            let neighbor = lookup_navmesh_info(navmesh_info_map, neighbor_id).into_option()?;
            let neighbor = neighbor.as_ref();
            let next_cost = current_cost + current_location.get_distance(neighbor.approx_location);
            let next_hops = current_hops.saturating_add(1);

            if neighbor.nav_mesh_id == goal.nav_mesh_id {
                return Some(NavMeshInfoGraphPath {
                    start_nav_mesh_id: start.nav_mesh_id,
                    goal_nav_mesh_id: goal.nav_mesh_id,
                    hops: next_hops,
                    approximate_cost: next_cost,
                });
            }

            seen.push(neighbor.nav_mesh_id);
            frontier.push_back((
                neighbor.nav_mesh_id,
                neighbor.approx_location,
                next_cost,
                next_hops,
            ));
        }
    }

    None
}

pub fn collect_potential_navmeshes_for_location(
    location: &mut BSPathingLocation,
    radius: f32,
) -> Vec<BSTSmartPointer<BSNavmesh>> {
    if !radius.is_finite() || radius <= 0.0 {
        crate::defensive_sdk_warn!(
            "sdk::gameplay::pathing::collect_potential_navmeshes_for_location() rejected invalid radius={radius}"
        );
        return Vec::new();
    }

    let mut nav_meshes: BSTArray<BSTSmartPointer<BSNavmesh>> = BSTArray::new();
    unsafe {
        singleton().with_mut_unchecked(|pathing| {
            pathing.find_potential_navmeshes_for_location2(location, radius, &mut nav_meshes)
        });
    }

    snapshot_contiguous_cloned_named(
        &nav_meshes,
        "sdk::gameplay::pathing::collect_potential_navmeshes_for_location()",
        Default::default(),
    )
    .into_iter()
    .filter(|nav_mesh| !nav_mesh.is_null())
    .collect()
}

pub fn collect_connected_navmesh_infos_for_location(
    location: &mut BSPathingLocation,
    radius: f32,
) -> Vec<GamePtr<BSNavmeshInfo>> {
    if !radius.is_finite() || radius <= 0.0 {
        crate::defensive_sdk_warn!(
            "sdk::gameplay::pathing::collect_connected_navmesh_infos_for_location() rejected invalid radius={radius}"
        );
        return Vec::new();
    }

    let mut infos: BSTArray<*const BSNavmeshInfo> = BSTArray::new();
    unsafe {
        singleton().with_mut_unchecked(|pathing| {
            pathing.find_connected_navmeshes_for_location4(location, radius, &mut infos)
        });
    }

    snapshot_contiguous_copied_named(
        &infos,
        "sdk::gameplay::pathing::collect_connected_navmesh_infos_for_location()",
        Default::default(),
    )
    .into_iter()
    .map(|info: *const BSNavmeshInfo| unsafe { GamePtr::from_raw(info.cast_mut()) })
    .filter(|info| !info.is_null())
    .collect()
}

pub fn collect_connected_navmeshes_for_location(
    location: &mut BSPathingLocation,
    radius: f32,
) -> Vec<BSTSmartPointer<BSNavmesh>> {
    if !radius.is_finite() || radius <= 0.0 {
        crate::defensive_sdk_warn!(
            "sdk::gameplay::pathing::collect_connected_navmeshes_for_location() rejected invalid radius={radius}"
        );
        return Vec::new();
    }

    let mut nav_meshes: BSTArray<BSTSmartPointer<BSNavmesh>> = BSTArray::new();
    unsafe {
        singleton().with_mut_unchecked(|pathing| {
            pathing.find_connected_navmeshes_for_location6(location, radius, &mut nav_meshes)
        });
    }

    snapshot_contiguous_cloned_named(
        &nav_meshes,
        "sdk::gameplay::pathing::collect_connected_navmeshes_for_location()",
        Default::default(),
    )
    .into_iter()
    .filter(|nav_mesh| !nav_mesh.is_null())
    .collect()
}

pub fn collect_loaded_navmeshes(pathing: &mut BSPathing) -> Vec<BSTSmartPointer<BSNavmesh>> {
    let mut nav_meshes = BSTArray::new();
    if !pathing.get_all_loaded_navmeshes5(&mut nav_meshes) {
        crate::defensive_sdk_warn!(
            "sdk::gameplay::pathing::collect_loaded_navmeshes() failed to query loaded navmeshes"
        );
        return Vec::new();
    }

    snapshot_contiguous_cloned_named(
        &nav_meshes,
        "sdk::gameplay::pathing::collect_loaded_navmeshes()",
        Default::default(),
    )
    .into_iter()
    .filter(|nav_mesh| !nav_mesh.is_null())
    .collect()
}

pub fn collect_loaded_navmesh_infos(pathing: &mut BSPathing) -> Vec<GamePtr<BSNavmeshInfo>> {
    let mut nav_meshes = BSTArray::new();
    if !pathing.get_all_loaded_navmeshes3(&mut nav_meshes) {
        crate::defensive_sdk_warn!(
            "sdk::gameplay::pathing::collect_loaded_navmesh_infos() failed to query loaded navmesh infos"
        );
        return Vec::new();
    }

    snapshot_contiguous_copied_named(
        &nav_meshes,
        "sdk::gameplay::pathing::collect_loaded_navmesh_infos()",
        Default::default(),
    )
    .into_iter()
    .map(info_ptr)
    .filter(|info| !info.is_null())
    .collect()
}

#[inline(always)]
pub fn selected_debug_ref(pathing: &mut BSPathing) -> FormID {
    pathing.get_selected_debug_ref()
}

pub fn pathing_door_from_collision(
    pathing: &mut BSPathing,
    object: GamePtr<NiAVObject>,
) -> GamePtr<BSPathingDoor> {
    if object.is_null() {
        crate::defensive_sdk_warn!(
            "sdk::gameplay::pathing::pathing_door_from_collision() received a null object"
        );
        return GamePtr::null();
    }

    let mut door = core::ptr::null_mut();
    if !pathing.get_pathing_door_from_collision(object.as_ptr(), &mut door) {
        return GamePtr::null();
    }

    door_ptr(door)
}

#[inline(always)]
pub fn describe_extra_nav_mesh_portal(extra: &ExtraNavMeshPortal) -> NavMeshPortalDescriptor {
    describe_portal(extra.portal)
}

pub fn collect_navmesh_edge_transitions(
    navmesh: &BSNavmesh,
) -> Vec<NavMeshEdgeTransitionDescriptor> {
    let triangles = navmesh.triangles_slice();
    navmesh
        .extra_edge_info_slice()
        .iter()
        .enumerate()
        .filter_map(|(flat_index, info)| {
            if !info
                .type_
                .all_underlying(BSNavmeshEdgeExtraInfoType::Portal as u32)
            {
                return None;
            }

            let source_triangle_index = (flat_index / 3) as u16;
            let source_edge_index = (flat_index % 3) as u8;
            let triangle = triangles.get(source_triangle_index as usize)?;
            if !triangle
                .triangle_flags
                .all_underlying(match source_edge_index {
                    0 => crate::re::BSNavmeshTriangleFlag::Edge0Link as u16,
                    1 => crate::re::BSNavmeshTriangleFlag::Edge1Link as u16,
                    2 => crate::re::BSNavmeshTriangleFlag::Edge2Link as u16,
                    _ => return None,
                })
            {
                return None;
            }

            Some(NavMeshEdgeTransitionDescriptor {
                source_triangle_index,
                source_edge_index,
                destination_nav_mesh_id: info.portal.other_mesh_id,
                destination_triangle_index: info.portal.triangle,
                destination_edge_index: info.portal.edge_index,
            })
        })
        .collect()
}

pub fn reference_nav_mesh_portal(reference: &TESObjectREFR) -> Option<NavMeshPortalDescriptor> {
    let extra = unsafe {
        reference
            .extra_list
            .get_by_type_typed::<ExtraNavMeshPortal>()
            .as_ref()
    }?;
    Some(describe_extra_nav_mesh_portal(extra))
}

pub fn reference_nav_mesh_portal_ptr(
    reference: GamePtr<TESObjectREFR>,
) -> Option<NavMeshPortalDescriptor> {
    let reference = reference.into_option()?;
    reference_nav_mesh_portal(reference.as_ref())
}
