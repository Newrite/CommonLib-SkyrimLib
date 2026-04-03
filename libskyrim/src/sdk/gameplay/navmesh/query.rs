use alloc::vec::Vec;

use crate::re::{BSNavmeshExt, TESObjectCELL, TESObjectREFR};
use crate::sdk::core::{GamePtr, snapshot_contiguous_cloned_named};

use super::shared::{
    empty_point_query, is_valid_non_negative_distance, is_valid_positive_radius, nav_mesh_array,
    nav_mesh_ref, query_point_in_snapshot, reference_from_ptr, within_range,
};
use super::types::{NavMeshCellSnapshot, NavMeshPointQuery};

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
    origin: crate::re::NiPoint3,
    radius: f32,
) -> Vec<crate::re::NiPoint3> {
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
    origin: crate::re::NiPoint3,
    radius: f32,
) -> Vec<crate::re::NiPoint3> {
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
    origin: crate::re::NiPoint3,
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
    origin: crate::re::NiPoint3,
    minimum_offset: f32,
) -> Option<crate::re::NiPoint3> {
    query_point_in_cell(cell, origin, minimum_offset).nearest_vertex
}

#[inline(always)]
pub fn nearest_triangle_center_in_cell(
    cell: &TESObjectCELL,
    origin: crate::re::NiPoint3,
    minimum_offset: f32,
) -> Option<crate::re::NiPoint3> {
    query_point_in_cell(cell, origin, minimum_offset).nearest_triangle_center
}

#[inline(always)]
pub fn nearest_vertex_from_reference(
    reference: &TESObjectREFR,
    minimum_offset: f32,
) -> Option<crate::re::NiPoint3> {
    query_point_from_reference(reference, minimum_offset)?.nearest_vertex
}

#[inline(always)]
pub fn nearest_triangle_center_from_reference(
    reference: &TESObjectREFR,
    minimum_offset: f32,
) -> Option<crate::re::NiPoint3> {
    query_point_from_reference(reference, minimum_offset)?.nearest_triangle_center
}

#[inline(always)]
pub fn has_navmesh_support_in_cell(
    cell: &TESObjectCELL,
    origin: crate::re::NiPoint3,
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
