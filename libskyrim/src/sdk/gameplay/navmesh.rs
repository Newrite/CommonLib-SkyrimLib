//! Navmesh-oriented gameplay helpers over translated `NavMesh` / `BSNavmesh`.
//!
//! This layer intentionally stays query-oriented. It exposes cell-local
//! snapshots, nearest-point helpers, and conservative same-mesh path-cost
//! heuristics without pretending that we already have a full Bethesda pathing
//! stack.

use alloc::vec;
use alloc::vec::Vec;
use core::cmp::Ordering;

use crate::re::{
    BSNavmeshEdgeExtraInfoType, BSNavmeshExt, BSNavmeshTriangleFlag, BSTSmartPointer, NavMesh,
    NavMeshArray, NiPoint3, TESObjectCELL, TESObjectREFR,
};
use crate::sdk::core::{GamePtr, GameRef, snapshot_contiguous_cloned_named};

const NAVMESH_TRIANGLE_NONE: u16 = 0xFFFF;

#[derive(Clone, Default)]
pub struct NavMeshCellSnapshot {
    pub meshes: Vec<BSTSmartPointer<NavMesh>>,
}

impl NavMeshCellSnapshot {
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        self.meshes.is_empty()
    }

    #[inline(always)]
    pub fn mesh_count(&self) -> usize {
        self.meshes.len()
    }

    #[inline(always)]
    pub fn vertex_count(&self) -> usize {
        self.meshes.iter().map(count_navmesh_vertices).sum()
    }

    #[inline(always)]
    pub fn triangle_count(&self) -> usize {
        self.meshes.iter().map(count_navmesh_triangles).sum()
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct NavMeshVertexSupport {
    pub mesh_index: usize,
    pub vertex_index: usize,
    pub point: NiPoint3,
    pub distance: f32,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct NavMeshTriangleSupport {
    pub mesh_index: usize,
    pub triangle_index: usize,
    pub center: NiPoint3,
    pub distance: f32,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct NavMeshPointQuery {
    pub origin: NiPoint3,
    pub mesh_count: usize,
    pub vertex_count: usize,
    pub triangle_count: usize,
    pub nearest_vertex_support: Option<NavMeshVertexSupport>,
    pub nearest_vertex: Option<NiPoint3>,
    pub nearest_vertex_distance: Option<f32>,
    pub nearest_triangle_support: Option<NavMeshTriangleSupport>,
    pub nearest_triangle_center: Option<NiPoint3>,
    pub nearest_triangle_center_distance: Option<f32>,
}

impl NavMeshPointQuery {
    #[inline(always)]
    pub fn has_support(&self) -> bool {
        self.nearest_vertex.is_some() || self.nearest_triangle_center.is_some()
    }

    #[inline(always)]
    pub fn nearest_support_distance(&self) -> Option<f32> {
        match (
            self.nearest_vertex_distance,
            self.nearest_triangle_center_distance,
        ) {
            (Some(vertex), Some(center)) => Some(vertex.min(center)),
            (Some(vertex), None) => Some(vertex),
            (None, Some(center)) => Some(center),
            (None, None) => None,
        }
    }
}

#[inline(always)]
fn empty_point_query(origin: NiPoint3) -> NavMeshPointQuery {
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

#[derive(Debug, Clone, Copy)]
pub struct NavMeshReachabilityHeuristicsOptions {
    pub minimum_offset: f32,
    pub maximum_support_distance: f32,
    pub maximum_path_cost: Option<f32>,
    pub allow_cross_mesh_graph: bool,
}

impl Default for NavMeshReachabilityHeuristicsOptions {
    fn default() -> Self {
        Self {
            minimum_offset: 0.0,
            maximum_support_distance: 128.0,
            maximum_path_cost: None,
            allow_cross_mesh_graph: true,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct NavMeshReachabilityHeuristics {
    pub from: NiPoint3,
    pub to: NiPoint3,
    pub direct_distance: f32,
    pub from_query: NavMeshPointQuery,
    pub to_query: NavMeshPointQuery,
    pub same_mesh: bool,
    pub same_triangle: bool,
    pub support_ok: bool,
    pub path_cost_ok: bool,
    pub triangle_hops: Option<u32>,
    pub mesh_hops: Option<u32>,
    pub approximate_path_cost: Option<f32>,
    pub used_cross_mesh_graph: bool,
    pub likely_reachable: bool,
}

#[inline(always)]
fn is_valid_non_negative_distance(distance: f32, caller: &'static str) -> bool {
    if !distance.is_finite() || distance < 0.0 {
        crate::defensive_sdk_warn!(
            "{} ignored negative or non-finite distance={}",
            caller,
            distance
        );
        false
    } else {
        true
    }
}

#[inline(always)]
fn is_valid_positive_radius(radius: f32, caller: &'static str) -> bool {
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
fn reference_from_ptr(
    reference: GamePtr<TESObjectREFR>,
    caller: &'static str,
) -> Option<GameRef<TESObjectREFR>> {
    let Some(reference) = reference.into_option() else {
        crate::defensive_sdk_warn!("{} received a null reference", caller);
        return None;
    };

    Some(reference)
}

#[inline(always)]
fn nav_mesh_array(cell: &TESObjectCELL) -> Option<&NavMeshArray> {
    unsafe { cell.nav_meshes_raw().as_ref() }
}

#[inline(always)]
fn nav_mesh_ref(nav_mesh: &BSTSmartPointer<NavMesh>) -> Option<&NavMesh> {
    unsafe { nav_mesh.get().as_ref() }
}

#[inline(always)]
fn count_navmesh_vertices(nav_mesh: &BSTSmartPointer<NavMesh>) -> usize {
    nav_mesh_ref(nav_mesh).map_or(0, BSNavmeshExt::vertex_count)
}

#[inline(always)]
fn count_navmesh_triangles(nav_mesh: &BSTSmartPointer<NavMesh>) -> usize {
    nav_mesh_ref(nav_mesh).map_or(0, BSNavmeshExt::triangle_count)
}

#[inline(always)]
fn nav_point_distance(origin: NiPoint3, point: NiPoint3) -> f32 {
    origin.get_distance(point)
}

#[inline(always)]
fn maybe_min_by_distance(
    current: Option<(usize, usize, NiPoint3, f32)>,
    candidate: (usize, usize, NiPoint3, f32),
) -> Option<(usize, usize, NiPoint3, f32)> {
    match current {
        Some(best) if best.3 <= candidate.3 => Some(best),
        _ => Some(candidate),
    }
}

#[inline(always)]
fn within_range(origin: NiPoint3, point: NiPoint3, radius: f32) -> bool {
    origin.get_squared_distance(point) <= radius * radius
}

pub fn snapshot_cell_navmeshes(cell: &TESObjectCELL) -> NavMeshCellSnapshot {
    let Some(nav_meshes) = nav_mesh_array(cell) else {
        return NavMeshCellSnapshot::default();
    };

    let meshes = snapshot_contiguous_cloned_named(
        &nav_meshes.nav_meshes,
        "sdk::gameplay::navmesh::snapshot_cell_navmeshes()",
        Default::default(),
    )
    .into_iter()
    .filter(|nav_mesh| !nav_mesh.is_null())
    .collect();

    NavMeshCellSnapshot { meshes }
}

fn query_point_in_snapshot(
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

pub fn snapshot_reference_navmeshes(reference: &TESObjectREFR) -> NavMeshCellSnapshot {
    let Some(cell) = (unsafe { reference.get_parent_cell().as_ref() }) else {
        crate::defensive_sdk_warn!(
            "sdk::gameplay::navmesh::snapshot_reference_navmeshes() skipped reference without parent cell"
        );
        return NavMeshCellSnapshot::default();
    };

    snapshot_cell_navmeshes(cell)
}

pub fn snapshot_reference_navmeshes_ptr(reference: GamePtr<TESObjectREFR>) -> NavMeshCellSnapshot {
    let Some(reference) = reference_from_ptr(
        reference,
        "sdk::gameplay::navmesh::snapshot_reference_navmeshes_ptr()",
    ) else {
        return NavMeshCellSnapshot::default();
    };

    snapshot_reference_navmeshes(reference.as_ref())
}

pub fn collect_navmesh_vertices_in_cell_range(
    cell: &TESObjectCELL,
    origin: NiPoint3,
    radius: f32,
) -> Vec<NiPoint3> {
    if !is_valid_positive_radius(
        radius,
        "sdk::gameplay::navmesh::collect_navmesh_vertices_in_cell_range()",
    ) {
        return Vec::new();
    }

    let snapshot = snapshot_cell_navmeshes(cell);
    let mut vertices = Vec::new();
    for nav_mesh in &snapshot.meshes {
        let Some(nav_mesh) = nav_mesh_ref(nav_mesh) else {
            continue;
        };

        for vertex in nav_mesh.vertices_slice() {
            if within_range(origin, vertex.location, radius) {
                vertices.push(vertex.location);
            }
        }
    }

    vertices
}

pub fn collect_navmesh_triangle_centers_in_cell_range(
    cell: &TESObjectCELL,
    origin: NiPoint3,
    radius: f32,
) -> Vec<NiPoint3> {
    if !is_valid_positive_radius(
        radius,
        "sdk::gameplay::navmesh::collect_navmesh_triangle_centers_in_cell_range()",
    ) {
        return Vec::new();
    }

    let snapshot = snapshot_cell_navmeshes(cell);
    let mut centers = Vec::new();
    for nav_mesh in &snapshot.meshes {
        let Some(nav_mesh) = nav_mesh_ref(nav_mesh) else {
            continue;
        };

        for triangle_index in 0..nav_mesh.triangle_count() {
            let Some(center) = nav_mesh.triangle_center(triangle_index) else {
                continue;
            };
            if within_range(origin, center, radius) {
                centers.push(center);
            }
        }
    }

    centers
}

pub fn query_point_in_cell(
    cell: &TESObjectCELL,
    origin: NiPoint3,
    minimum_offset: f32,
) -> NavMeshPointQuery {
    if !is_valid_non_negative_distance(
        minimum_offset,
        "sdk::gameplay::navmesh::query_point_in_cell()",
    ) {
        return empty_point_query(origin);
    }

    let snapshot = snapshot_cell_navmeshes(cell);
    query_point_in_snapshot(&snapshot, origin, minimum_offset)
}

pub fn query_point_from_reference(
    reference: &TESObjectREFR,
    minimum_offset: f32,
) -> Option<NavMeshPointQuery> {
    let cell = unsafe { reference.get_parent_cell().as_ref() }?;
    Some(query_point_in_cell(
        cell,
        reference.get_position(),
        minimum_offset,
    ))
}

pub fn query_point_from_reference_ptr(
    reference: GamePtr<TESObjectREFR>,
    minimum_offset: f32,
) -> Option<NavMeshPointQuery> {
    let reference = reference_from_ptr(
        reference,
        "sdk::gameplay::navmesh::query_point_from_reference_ptr()",
    )?;
    query_point_from_reference(reference.as_ref(), minimum_offset)
}

#[inline(always)]
pub fn nearest_vertex_in_cell(
    cell: &TESObjectCELL,
    origin: NiPoint3,
    minimum_offset: f32,
) -> Option<NiPoint3> {
    query_point_in_cell(cell, origin, minimum_offset).nearest_vertex
}

#[inline(always)]
pub fn nearest_triangle_center_in_cell(
    cell: &TESObjectCELL,
    origin: NiPoint3,
    minimum_offset: f32,
) -> Option<NiPoint3> {
    query_point_in_cell(cell, origin, minimum_offset).nearest_triangle_center
}

#[inline(always)]
pub fn nearest_vertex_from_reference(
    reference: &TESObjectREFR,
    minimum_offset: f32,
) -> Option<NiPoint3> {
    query_point_from_reference(reference, minimum_offset)?.nearest_vertex
}

#[inline(always)]
pub fn nearest_triangle_center_from_reference(
    reference: &TESObjectREFR,
    minimum_offset: f32,
) -> Option<NiPoint3> {
    query_point_from_reference(reference, minimum_offset)?.nearest_triangle_center
}

#[inline(always)]
pub fn has_navmesh_support_in_cell(
    cell: &TESObjectCELL,
    origin: NiPoint3,
    maximum_distance: f32,
) -> bool {
    if !is_valid_non_negative_distance(
        maximum_distance,
        "sdk::gameplay::navmesh::has_navmesh_support_in_cell()",
    ) {
        return false;
    }

    query_point_in_cell(cell, origin, 0.0)
        .nearest_support_distance()
        .is_some_and(|distance| distance <= maximum_distance)
}

#[inline(always)]
pub fn has_navmesh_support_for_reference(reference: &TESObjectREFR, maximum_distance: f32) -> bool {
    query_point_from_reference(reference, 0.0)
        .and_then(|query| query.nearest_support_distance())
        .is_some_and(|distance| distance <= maximum_distance)
}

#[inline(always)]
fn validate_reachability_options(options: NavMeshReachabilityHeuristicsOptions) -> bool {
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

fn approximate_same_mesh_triangle_path_cost(
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
fn triangle_edge_link_flag(edge_index: usize) -> Option<u16> {
    Some(match edge_index {
        0 => BSNavmeshTriangleFlag::Edge0Link as u16,
        1 => BSNavmeshTriangleFlag::Edge1Link as u16,
        2 => BSNavmeshTriangleFlag::Edge2Link as u16,
        _ => return None,
    })
}

#[inline(always)]
fn snapshot_mesh_index_for_navmesh_id(mesh_ids: &[u32], nav_mesh_id: u32) -> Option<usize> {
    mesh_ids
        .iter()
        .position(|candidate| *candidate == nav_mesh_id)
}

fn approximate_cross_mesh_snapshot_path_cost(
    snapshot: &NavMeshCellSnapshot,
    from_support: NavMeshTriangleSupport,
    to_support: NavMeshTriangleSupport,
) -> Option<(f32, u32)> {
    // Keep runtime reachability on the already-snapshotted cell-local navmesh
    // graph. The `TES::RUNTIME_DATA2` `NavMeshInfoMap*` seam is only partially
    // named/source-backed (`unk2A8` in CommonLib headers) and has proven too
    // fragile for hot gameplay heuristics.
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

pub fn evaluate_reachability_in_cell(
    cell: &TESObjectCELL,
    from: NiPoint3,
    to: NiPoint3,
    options: NavMeshReachabilityHeuristicsOptions,
) -> NavMeshReachabilityHeuristics {
    let direct_distance = from.get_distance(to);
    if !validate_reachability_options(options) {
        return NavMeshReachabilityHeuristics {
            from,
            to,
            direct_distance,
            from_query: empty_point_query(from),
            to_query: empty_point_query(to),
            same_mesh: false,
            same_triangle: false,
            support_ok: false,
            path_cost_ok: false,
            triangle_hops: None,
            mesh_hops: None,
            approximate_path_cost: None,
            used_cross_mesh_graph: false,
            likely_reachable: false,
        };
    }

    let snapshot = snapshot_cell_navmeshes(cell);
    let from_query = query_point_in_snapshot(&snapshot, from, options.minimum_offset);
    let to_query = query_point_in_snapshot(&snapshot, to, options.minimum_offset);
    let support_ok = from_query
        .nearest_support_distance()
        .is_some_and(|distance| distance <= options.maximum_support_distance)
        && to_query
            .nearest_support_distance()
            .is_some_and(|distance| distance <= options.maximum_support_distance);

    let same_mesh = matches!(
        (from_query.nearest_triangle_support, to_query.nearest_triangle_support),
        (Some(from_support), Some(to_support)) if from_support.mesh_index == to_support.mesh_index
    );
    let same_triangle = matches!(
        (from_query.nearest_triangle_support, to_query.nearest_triangle_support),
        (Some(from_support), Some(to_support))
            if from_support.mesh_index == to_support.mesh_index
                && from_support.triangle_index == to_support.triangle_index
    );

    let (approximate_path_cost, triangle_hops, mesh_hops, used_cross_mesh_graph) = match (
        from_query.nearest_triangle_support,
        to_query.nearest_triangle_support,
    ) {
        (Some(from_support), Some(to_support))
            if from_support.mesh_index == to_support.mesh_index =>
        {
            match snapshot
                .meshes
                .get(from_support.mesh_index)
                .and_then(nav_mesh_ref)
                .and_then(|nav_mesh| {
                    approximate_same_mesh_triangle_path_cost(
                        nav_mesh,
                        from_support.triangle_index,
                        to_support.triangle_index,
                    )
                }) {
                Some((triangle_center_cost, hops)) => (
                    Some(from_support.distance + triangle_center_cost + to_support.distance),
                    Some(hops),
                    None,
                    false,
                ),
                None => (None, None, None, false),
            }
        }
        (Some(from_support), Some(to_support)) if options.allow_cross_mesh_graph => {
            match approximate_cross_mesh_snapshot_path_cost(&snapshot, from_support, to_support) {
                Some((graph_cost, hops)) => (
                    Some(from_support.distance + graph_cost + to_support.distance),
                    None,
                    Some(hops),
                    true,
                ),
                None => (None, None, None, false),
            }
        }
        _ => (None, None, None, false),
    };

    let path_cost_ok = match (options.maximum_path_cost, approximate_path_cost) {
        (Some(maximum_path_cost), Some(path_cost)) => path_cost <= maximum_path_cost,
        (Some(_), None) => false,
        (None, Some(_)) => true,
        (None, None) => false,
    };
    let likely_reachable = support_ok && approximate_path_cost.is_some() && path_cost_ok;

    NavMeshReachabilityHeuristics {
        from,
        to,
        direct_distance,
        from_query,
        to_query,
        same_mesh,
        same_triangle,
        support_ok,
        path_cost_ok,
        triangle_hops,
        mesh_hops,
        approximate_path_cost,
        used_cross_mesh_graph,
        likely_reachable,
    }
}

pub fn evaluate_reachability_from_reference(
    reference: &TESObjectREFR,
    to: NiPoint3,
    options: NavMeshReachabilityHeuristicsOptions,
) -> Option<NavMeshReachabilityHeuristics> {
    let cell = unsafe { reference.get_parent_cell().as_ref() }?;
    Some(evaluate_reachability_in_cell(
        cell,
        reference.get_position(),
        to,
        options,
    ))
}

pub fn evaluate_reachability_from_reference_ptr(
    reference: GamePtr<TESObjectREFR>,
    to: NiPoint3,
    options: NavMeshReachabilityHeuristicsOptions,
) -> Option<NavMeshReachabilityHeuristics> {
    let reference = reference_from_ptr(
        reference,
        "sdk::gameplay::navmesh::evaluate_reachability_from_reference_ptr()",
    )?;
    evaluate_reachability_from_reference(reference.as_ref(), to, options)
}
