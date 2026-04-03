use alloc::vec::Vec;

use crate::re::pathing::BSPathingCellManagerExt;
use crate::re::{BSNavmesh, BSNavmeshInfo, BSPathingLocation, BSTArray};
use crate::sdk::core::{
    GamePtr, snapshot_contiguous_cloned_named, snapshot_contiguous_copied_named,
};

use super::runtime::singleton;
use super::types::is_valid_positive_radius;

pub fn collect_potential_navmeshes_for_location(
    location: &mut BSPathingLocation,
    radius: f32,
) -> Vec<crate::re::BSTSmartPointer<BSNavmesh>> {
    if !is_valid_positive_radius(
        radius,
        "sdk::gameplay::pathing::collect_potential_navmeshes_for_location()",
    ) {
        return Vec::new();
    }

    let mut nav_meshes: BSTArray<crate::re::BSTSmartPointer<BSNavmesh>> = BSTArray::new();
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
    if !is_valid_positive_radius(
        radius,
        "sdk::gameplay::pathing::collect_connected_navmesh_infos_for_location()",
    ) {
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
) -> Vec<crate::re::BSTSmartPointer<BSNavmesh>> {
    if !is_valid_positive_radius(
        radius,
        "sdk::gameplay::pathing::collect_connected_navmeshes_for_location()",
    ) {
        return Vec::new();
    }

    let mut nav_meshes: BSTArray<crate::re::BSTSmartPointer<BSNavmesh>> = BSTArray::new();
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
