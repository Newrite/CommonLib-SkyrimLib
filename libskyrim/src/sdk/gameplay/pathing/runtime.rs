use alloc::vec::Vec;

use crate::re::pathing::BSPathingCellManagerExt;
use crate::re::{
    BSNavmesh, BSNavmeshInfo, BSPathing, BSPathingDoor, BSTArray, FormID, NiAVObject, Pathing, TES,
};
use crate::sdk::core::{
    GamePtr, GameRef, snapshot_contiguous_cloned_named, snapshot_contiguous_copied_named,
};

use super::shared::{door_ptr, info_ptr};

/// Return the global pathing singleton.
///
/// Most consumers should still prefer the narrower helpers in this module, but
/// this accessor is the raw front door for plugins that need direct `Pathing`
/// methods.
pub fn singleton() -> GameRef<Pathing> {
    unsafe { GameRef::from_raw(Pathing::get_singleton()) }
}

/// Return the global navmesh-info map owned by `TES`.
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

/// Collect currently loaded navmesh objects through one mutable `BSPathing`
/// interface.
pub fn collect_loaded_navmeshes(
    pathing: &mut BSPathing,
) -> Vec<crate::re::BSTSmartPointer<BSNavmesh>> {
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

/// Collect currently loaded navmesh-info records through one mutable
/// `BSPathing` interface.
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

/// Resolve a `BSPathingDoor` from a collision object when the pathing runtime
/// recognizes that object as a pathing door.
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
