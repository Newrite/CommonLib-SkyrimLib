use alloc::vec::Vec;

use crate::re::{BSTSmartPointer, NavMesh, NiPoint3};

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
        self.meshes
            .iter()
            .map(super::shared::count_navmesh_vertices)
            .sum()
    }

    #[inline(always)]
    pub fn triangle_count(&self) -> usize {
        self.meshes
            .iter()
            .map(super::shared::count_navmesh_triangles)
            .sum()
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
