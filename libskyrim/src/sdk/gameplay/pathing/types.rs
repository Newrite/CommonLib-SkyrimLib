use crate::re::{
    BSPathingCell, CellID, FormID, NAVMESH_PORTAL, NavMesh, PathingCell, PathingCellInfo,
};
use crate::sdk::core::GamePtr;

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
    pub cell: crate::re::BSTSmartPointer<PathingCell>,
}

#[derive(Clone)]
pub struct RecentPathingCellDescriptor {
    pub age_stamp: u64,
    pub cell: crate::re::BSTSmartPointer<PathingCell>,
}

#[inline(always)]
pub(super) fn describe_portal(portal: NAVMESH_PORTAL) -> NavMeshPortalDescriptor {
    NavMeshPortalDescriptor {
        triangle_index: portal.tri_index,
        nav_mesh_form_id_bits: unsafe { portal.nav.nav_mesh_id },
        nav_mesh: super::shared::navmesh_ptr(unsafe { portal.nav.nav_mesh }),
    }
}

#[inline(always)]
pub(super) fn inspect_pathing_cell_base(cell: &BSPathingCell) -> PathingCellState {
    PathingCellState {
        type_id: cell.get_type(),
        valid: cell.q_valid(),
        attached: cell.q_attached(),
        loaded: cell.q_loaded(),
    }
}

#[inline(always)]
pub(super) fn inspect_concrete_pathing_cell_base(cell: &PathingCell) -> ConcretePathingCellState {
    ConcretePathingCellState {
        base: inspect_pathing_cell_base(cell.as_ref()),
        world_space_id: cell.pathing_cell_info.world_space_id,
        cell_form_id: unsafe { cell.pathing_cell_info.cell_id.form_id },
        cell_coordinates: unsafe { cell.pathing_cell_info.cell_id.coordinates },
    }
}

#[inline(always)]
pub(super) fn approximate_info_path_cost(
    path: &[GamePtr<crate::re::BSNavmeshInfo>],
) -> Option<(f32, u32)> {
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

#[inline(always)]
pub(super) fn is_valid_positive_radius(radius: f32, _caller: &'static str) -> bool {
    if !radius.is_finite() || radius <= 0.0 {
        crate::defensive_sdk_warn!("{} rejected invalid radius={}", _caller, radius);
        false
    } else {
        true
    }
}
