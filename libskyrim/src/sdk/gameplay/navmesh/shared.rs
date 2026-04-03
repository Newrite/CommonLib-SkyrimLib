use alloc::vec;
use alloc::vec::Vec;
use core::cmp::Ordering;

use crate::re::{
    BSNavmeshEdgeExtraInfoType, BSNavmeshExt, BSNavmeshTriangleFlag, BSTSmartPointer, NavMesh,
    NavMeshArray, NiPoint3, TESObjectCELL, TESObjectREFR,
};
use crate::sdk::core::{GamePtr, GameRef};

use super::types::{
    NavMeshCellSnapshot, NavMeshPointQuery, NavMeshReachabilityHeuristicsOptions,
    NavMeshTriangleSupport, NavMeshVertexSupport,
};

pub(super) const NAVMESH_TRIANGLE_NONE: u16 = 0xFFFF;

#[inline(always)]
pub(super) fn empty_point_query(origin: NiPoint3) -> NavMeshPointQuery {
    NavMeshPointQuery {
        origin,
        mesh_count: 0,
        vertex_count: 0,
        triangle_count: 0,
        nearest_vertex_support: None,
        nearest_vertex: None,
        nearest_vertex_distance: None,
        nearest_triangle_support: None,
        nearest_triangle_center: None,
        nearest_triangle_center_distance: None,
    }
}

#[inline(always)]
pub(super) fn is_valid_non_negative_distance(distance: f32, _caller: &'static str) -> bool {
    if !distance.is_finite() || distance < 0.0 {
        crate::defensive_sdk_warn!(
            "{} ignored negative or non-finite distance={}",
            _caller,
            distance
        );
        false
    } else {
        true
    }
}

#[inline(always)]
pub(super) fn is_valid_positive_radius(radius: f32, _caller: &'static str) -> bool {
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
pub(super) fn reference_from_ptr(
    reference: GamePtr<TESObjectREFR>,
    _caller: &'static str,
) -> Option<GameRef<TESObjectREFR>> {
    let Some(reference) = reference.into_option() else {
        crate::defensive_sdk_warn!("{} received a null reference", _caller);
        return None;
    };

    Some(reference)
}

#[inline(always)]
pub(super) fn nav_mesh_array(cell: &TESObjectCELL) -> Option<&NavMeshArray> {
    unsafe { cell.nav_meshes_raw().as_ref() }
}

#[inline(always)]
pub(super) fn nav_mesh_ref(nav_mesh: &BSTSmartPointer<NavMesh>) -> Option<&NavMesh> {
    unsafe { nav_mesh.get().as_ref() }
}

#[inline(always)]
pub(super) fn count_navmesh_vertices(nav_mesh: &BSTSmartPointer<NavMesh>) -> usize {
    nav_mesh_ref(nav_mesh).map_or(0, BSNavmeshExt::vertex_count)
}

#[inline(always)]
pub(super) fn count_navmesh_triangles(nav_mesh: &BSTSmartPointer<NavMesh>) -> usize {
    nav_mesh_ref(nav_mesh).map_or(0, BSNavmeshExt::triangle_count)
}

#[inline(always)]
pub(super) fn nav_point_distance(origin: NiPoint3, point: NiPoint3) -> f32 {
    origin.get_distance(point)
}

#[inline(always)]
pub(super) fn maybe_min_by_distance(
    current: Option<(usize, usize, NiPoint3, f32)>,
    candidate: (usize, usize, NiPoint3, f32),
) -> Option<(usize, usize, NiPoint3, f32)> {
    match current {
        Some(best) if best.3 <= candidate.3 => Some(best),
        _ => Some(candidate),
    }
}

#[inline(always)]
pub(super) fn within_range(origin: NiPoint3, point: NiPoint3, radius: f32) -> bool {
    origin.get_squared_distance(point) <= radius * radius
}

pub(super) fn query_point_in_snapshot(
    snapshot: &NavMeshCellSnapshot,
    origin: NiPoint3,
    minimum_offset: f32,
) -> NavMeshPointQuery {
    let mut nearest_vertex = None;
    let mut nearest_triangle_center = None;

    for (mesh_index, nav_mesh) in snapshot.meshes.iter().enumerate() {
        let Some(nav_mesh) = nav_mesh_ref(nav_mesh) else {
            continue;
        };

        for (vertex_index, vertex) in nav_mesh.vertices_slice().iter().enumerate() {
            let distance = nav_point_distance(origin, vertex.location);
            if distance < minimum_offset {
                continue;
            }

            nearest_vertex = maybe_min_by_distance(
                nearest_vertex,
                (mesh_index, vertex_index, vertex.location, distance),
            );
        }

        for triangle_index in 0..nav_mesh.triangle_count() {
            let Some(center) = nav_mesh.triangle_center(triangle_index) else {
                continue;
            };

            let distance = nav_point_distance(origin, center);
            if distance < minimum_offset {
                continue;
            }

            nearest_triangle_center = maybe_min_by_distance(
                nearest_triangle_center,
                (mesh_index, triangle_index, center, distance),
            );
        }
    }

    NavMeshPointQuery {
        origin,
        mesh_count: snapshot.mesh_count(),
        vertex_count: snapshot.vertex_count(),
        triangle_count: snapshot.triangle_count(),
        nearest_vertex_support: nearest_vertex.map(
            |(mesh_index, vertex_index, point, distance)| NavMeshVertexSupport {
                mesh_index,
                vertex_index,
                point,
                distance,
            },
        ),
        nearest_vertex: nearest_vertex.map(|(_, _, point, _)| point),
        nearest_vertex_distance: nearest_vertex.map(|(_, _, _, distance)| distance),
        nearest_triangle_support: nearest_triangle_center.map(
            |(mesh_index, triangle_index, center, distance)| NavMeshTriangleSupport {
                mesh_index,
                triangle_index,
                center,
                distance,
            },
        ),
        nearest_triangle_center: nearest_triangle_center.map(|(_, _, center, _)| center),
        nearest_triangle_center_distance: nearest_triangle_center
            .map(|(_, _, _, distance)| distance),
    }
}

#[inline(always)]
pub(super) fn validate_reachability_options(options: NavMeshReachabilityHeuristicsOptions) -> bool {
    if !is_valid_non_negative_distance(
        options.minimum_offset,
        "sdk::gameplay::navmesh::evaluate_reachability_in_cell()",
    ) {
        return false;
    }

    if !is_valid_non_negative_distance(
        options.maximum_support_distance,
        "sdk::gameplay::navmesh::evaluate_reachability_in_cell()",
    ) {
        return false;
    }

    if let Some(maximum_path_cost) = options.maximum_path_cost {
        if !maximum_path_cost.is_finite() || maximum_path_cost < 0.0 {
            crate::defensive_sdk_warn!(
                "sdk::gameplay::navmesh::evaluate_reachability_in_cell() ignored non-finite or negative maximum_path_cost={}",
                maximum_path_cost
            );
            return false;
        }
    }

    true
}

pub(super) fn approximate_same_mesh_triangle_path_cost(
    nav_mesh: &NavMesh,
    start_triangle: usize,
    end_triangle: usize,
) -> Option<(f32, u32)> {
    let triangle_count = nav_mesh.triangle_count();
    if start_triangle >= triangle_count || end_triangle >= triangle_count {
        return None;
    }

    if start_triangle == end_triangle {
        return Some((0.0, 0));
    }

    let mut centers = Vec::with_capacity(triangle_count);
    for triangle_index in 0..triangle_count {
        centers.push(nav_mesh.triangle_center(triangle_index)?);
    }

    let mut distances = vec![f32::INFINITY; triangle_count];
    let mut hops = vec![u32::MAX; triangle_count];
    let mut visited = vec![false; triangle_count];
    distances[start_triangle] = 0.0;
    hops[start_triangle] = 0;

    loop {
        let mut next: Option<(usize, f32)> = None;
        for (index, distance) in distances.iter().copied().enumerate() {
            if visited[index] {
                continue;
            }

            match next {
                Some((_, best_distance))
                    if best_distance
                        .partial_cmp(&distance)
                        .unwrap_or(Ordering::Equal)
                        != Ordering::Greater => {}
                _ => next = Some((index, distance)),
            }
        }
        let next = next.map(|(index, _)| index);
        let Some(current) = next else {
            break;
        };

        if !distances[current].is_finite() {
            break;
        }

        if current == end_triangle {
            return Some((distances[current], hops[current]));
        }

        visited[current] = true;
        let Some(triangle) = nav_mesh.triangle(current) else {
            continue;
        };

        for neighbor in triangle.triangles {
            if neighbor == NAVMESH_TRIANGLE_NONE {
                continue;
            }

            let neighbor = neighbor as usize;
            if neighbor >= triangle_count || visited[neighbor] {
                continue;
            }

            let edge_cost = centers[current].get_distance(centers[neighbor]);
            let next_cost = distances[current] + edge_cost;
            if next_cost < distances[neighbor] {
                distances[neighbor] = next_cost;
                hops[neighbor] = hops[current].saturating_add(1);
            }
        }
    }

    None
}

#[inline(always)]
pub(super) fn triangle_edge_link_flag(edge_index: usize) -> Option<u16> {
    Some(match edge_index {
        0 => BSNavmeshTriangleFlag::Edge0Link as u16,
        1 => BSNavmeshTriangleFlag::Edge1Link as u16,
        2 => BSNavmeshTriangleFlag::Edge2Link as u16,
        _ => return None,
    })
}

#[inline(always)]
pub(super) fn snapshot_mesh_index_for_navmesh_id(
    mesh_ids: &[u32],
    nav_mesh_id: u32,
) -> Option<usize> {
    mesh_ids
        .iter()
        .position(|candidate| *candidate == nav_mesh_id)
}

pub(super) fn approximate_cross_mesh_snapshot_path_cost(
    snapshot: &NavMeshCellSnapshot,
    from_support: NavMeshTriangleSupport,
    to_support: NavMeshTriangleSupport,
) -> Option<(f32, u32)> {
    if from_support.mesh_index == to_support.mesh_index {
        return None;
    }

    let mut mesh_ids = Vec::with_capacity(snapshot.meshes.len());
    let mut mesh_offsets = Vec::with_capacity(snapshot.meshes.len());
    let mut mesh_triangle_counts = Vec::with_capacity(snapshot.meshes.len());
    let mut triangle_centers = Vec::new();
    let mut node_to_triangle = Vec::new();

    for (mesh_index, nav_mesh) in snapshot.meshes.iter().enumerate() {
        let nav_mesh = nav_mesh_ref(nav_mesh)?;
        let triangle_count = nav_mesh.triangle_count();

        mesh_ids.push(nav_mesh.base.get_form_id());
        mesh_offsets.push(triangle_centers.len());
        mesh_triangle_counts.push(triangle_count);

        for triangle_index in 0..triangle_count {
            triangle_centers.push(nav_mesh.triangle_center(triangle_index)?);
            node_to_triangle.push((mesh_index, triangle_index));
        }
    }

    let from_offset = *mesh_offsets.get(from_support.mesh_index)?;
    let to_offset = *mesh_offsets.get(to_support.mesh_index)?;
    if from_support.triangle_index >= *mesh_triangle_counts.get(from_support.mesh_index)?
        || to_support.triangle_index >= *mesh_triangle_counts.get(to_support.mesh_index)?
    {
        return None;
    }

    let start_node = from_offset.checked_add(from_support.triangle_index)?;
    let goal_node = to_offset.checked_add(to_support.triangle_index)?;
    if start_node >= triangle_centers.len() || goal_node >= triangle_centers.len() {
        return None;
    }

    let mut distances = vec![f32::INFINITY; triangle_centers.len()];
    let mut hops = vec![u32::MAX; triangle_centers.len()];
    let mut visited = vec![false; triangle_centers.len()];
    distances[start_node] = 0.0;
    hops[start_node] = 0;

    loop {
        let mut next: Option<(usize, f32)> = None;
        for (index, distance) in distances.iter().copied().enumerate() {
            if visited[index] {
                continue;
            }

            match next {
                Some((_, best_distance))
                    if best_distance
                        .partial_cmp(&distance)
                        .unwrap_or(Ordering::Equal)
                        != Ordering::Greater => {}
                _ => next = Some((index, distance)),
            }
        }

        let Some(current) = next.map(|(index, _)| index) else {
            break;
        };

        if !distances[current].is_finite() {
            break;
        }

        if current == goal_node {
            return Some((distances[current], hops[current]));
        }

        visited[current] = true;
        let (mesh_index, triangle_index) = *node_to_triangle.get(current)?;
        let Some(nav_mesh) = snapshot.meshes.get(mesh_index).and_then(nav_mesh_ref) else {
            continue;
        };
        let Some(triangle) = nav_mesh.triangle(triangle_index) else {
            continue;
        };

        for neighbor in triangle.triangles {
            if neighbor == NAVMESH_TRIANGLE_NONE {
                continue;
            }

            let neighbor_triangle = neighbor as usize;
            if neighbor_triangle >= mesh_triangle_counts[mesh_index] {
                continue;
            }

            let Some(neighbor_node) = mesh_offsets[mesh_index].checked_add(neighbor_triangle)
            else {
                continue;
            };
            if neighbor_node >= triangle_centers.len() || visited[neighbor_node] {
                continue;
            }

            let edge_cost = triangle_centers[current].get_distance(triangle_centers[neighbor_node]);
            let next_cost = distances[current] + edge_cost;
            if next_cost < distances[neighbor_node] {
                distances[neighbor_node] = next_cost;
                hops[neighbor_node] = hops[current].saturating_add(1);
            }
        }

        let edge_infos = nav_mesh.extra_edge_info_slice();
        let flat_edge_base = triangle_index.saturating_mul(3);
        for edge_index in 0..3 {
            let Some(edge_flag) = triangle_edge_link_flag(edge_index) else {
                continue;
            };
            if !triangle.triangle_flags.all_underlying(edge_flag) {
                continue;
            }

            let Some(extra_info) = edge_infos.get(flat_edge_base + edge_index) else {
                continue;
            };
            if !extra_info
                .type_
                .all_underlying(BSNavmeshEdgeExtraInfoType::Portal as u32)
            {
                continue;
            }

            let Some(destination_mesh_index) =
                snapshot_mesh_index_for_navmesh_id(&mesh_ids, extra_info.portal.other_mesh_id)
            else {
                continue;
            };
            let destination_triangle = extra_info.portal.triangle as usize;
            if destination_triangle >= mesh_triangle_counts[destination_mesh_index] {
                continue;
            }

            let Some(destination_node) =
                mesh_offsets[destination_mesh_index].checked_add(destination_triangle)
            else {
                continue;
            };
            if destination_node >= triangle_centers.len() || visited[destination_node] {
                continue;
            }

            let edge_cost =
                triangle_centers[current].get_distance(triangle_centers[destination_node]);
            let next_cost = distances[current] + edge_cost;
            if next_cost < distances[destination_node] {
                distances[destination_node] = next_cost;
                hops[destination_node] = hops[current].saturating_add(1);
            }
        }
    }

    None
}
