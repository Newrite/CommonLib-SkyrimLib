use alloc::collections::VecDeque;
use alloc::vec::Vec;

use crate::re::{
    BSNavmeshInfo, BSNavmeshInfoMap, BSPrecomputedNavmeshInfoPathMap, BSTArray, FormID, NavMesh,
    NiPoint3,
};
use crate::sdk::core::{GamePtr, snapshot_contiguous_copied_named};

use super::shared::info_ptr;
use super::types::{NavMeshInfoGraphPath, approximate_info_path_cost};

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
    nav_mesh: &crate::re::BSTSmartPointer<NavMesh>,
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

#[inline(always)]
pub fn approximate_navmesh_info_path_cost(path: &[GamePtr<BSNavmeshInfo>]) -> Option<(f32, u32)> {
    approximate_info_path_cost(path)
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
