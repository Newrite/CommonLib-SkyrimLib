use crate::re::{BSNavmeshInfo, BSPathingDoor, NavMesh};
use crate::sdk::core::GamePtr;

#[inline(always)]
pub(super) fn info_ptr(info: *mut BSNavmeshInfo) -> GamePtr<BSNavmeshInfo> {
    unsafe { GamePtr::from_raw(info) }
}

#[inline(always)]
pub(super) fn door_ptr(door: *mut BSPathingDoor) -> GamePtr<BSPathingDoor> {
    unsafe { GamePtr::from_raw(door) }
}

#[inline(always)]
pub(super) fn navmesh_ptr(nav_mesh: *mut NavMesh) -> GamePtr<NavMesh> {
    unsafe { GamePtr::from_raw(nav_mesh) }
}
