use crate::re::{
    BSPathingCell, CellID, FormID, NAVMESH_PORTAL, NavMesh, PathingCell, PathingCellInfo,
};
use crate::sdk::core::GamePtr;

/// Minimal pathing-cell readiness state shared by concrete and abstract cell
/// views.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PathingCellState {
    pub type_id: u32,
    pub valid: bool,
    pub attached: bool,
    pub loaded: bool,
}

impl PathingCellState {
    /// Whether the cell looks valid, attached, and loaded.
    #[inline(always)]
    pub fn ready(&self) -> bool {
        self.valid && self.attached && self.loaded
    }
}

/// Extended state snapshot for a concrete `PathingCell`.
///
/// Use this when plugin code needs to carry cell/world identifiers alongside
/// the shared readiness state from [`PathingCellState`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ConcretePathingCellState {
    pub base: PathingCellState,
    pub world_space_id: FormID,
    pub cell_form_id: FormID,
    pub cell_coordinates: CellID,
}

/// Flat descriptor for one navmesh portal edge.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NavMeshPortalDescriptor {
    pub triangle_index: u16,
    pub nav_mesh_form_id_bits: FormID,
    pub nav_mesh: GamePtr<NavMesh>,
}

/// Flat descriptor for a cross-edge navmesh transition.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NavMeshEdgeTransitionDescriptor {
    pub source_triangle_index: u16,
    pub source_edge_index: u8,
    pub destination_nav_mesh_id: FormID,
    pub destination_triangle_index: u16,
    pub destination_edge_index: i8,
}

/// Approximate path summary through the navmesh-info graph.
///
/// This stays intentionally heuristic and graph-level: it is useful for plugin
/// routing decisions, not as a promise of full engine path solvability.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct NavMeshInfoGraphPath {
    pub start_nav_mesh_id: FormID,
    pub goal_nav_mesh_id: FormID,
    pub hops: u32,
    pub approximate_cost: f32,
}

/// Descriptor for a currently loaded pathing cell.
///
/// This keeps the original `PathingCellInfo` side-by-side with the retained
/// smart pointer so pathing code can inspect either metadata or the live cell.
#[derive(Clone)]
pub struct LoadedPathingCellDescriptor {
    pub info: PathingCellInfo,
    pub cell: crate::re::BSTSmartPointer<PathingCell>,
}

/// Descriptor for a recently used pathing cell retained by the runtime.
///
/// The `age_stamp` is useful for heuristics that prefer fresh pathing cells
/// without assuming every recent entry is still loaded.
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
