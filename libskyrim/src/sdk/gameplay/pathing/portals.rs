use alloc::vec::Vec;

use crate::re::{
    BSNavmesh, BSNavmeshEdgeExtraInfoType, BSNavmeshTriangleFlag, ExtraNavMeshPortal, TESObjectREFR,
};
use crate::sdk::core::GamePtr;

use super::types::{NavMeshEdgeTransitionDescriptor, NavMeshPortalDescriptor, describe_portal};

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
                    0 => BSNavmeshTriangleFlag::Edge0Link as u16,
                    1 => BSNavmeshTriangleFlag::Edge1Link as u16,
                    2 => BSNavmeshTriangleFlag::Edge2Link as u16,
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
