//! Camera-oriented gameplay helpers.

use crate::re::{Actor, CameraState, NiNode, NiPoint3, PlayerCamera, TESCameraState};
use crate::sdk::core::{GamePtr, GameRef, Resolved};

#[inline(always)]
pub fn singleton() -> GameRef<PlayerCamera> {
    unsafe { GameRef::from_raw(PlayerCamera::get_singleton()) }
}

#[inline(always)]
fn is_valid_camera_state(state: CameraState) -> bool {
    let total = if crate::runtime::is_vr() {
        crate::re::CameraStates::VR_TOTAL
    } else {
        crate::re::CameraStates::TOTAL
    };
    state.index() < total
}

#[inline(always)]
pub fn camera_root() -> GamePtr<NiNode> {
    unsafe { GamePtr::from_raw(singleton().base.camera_root.get()) }
}

#[inline(always)]
pub fn current_state() -> GamePtr<TESCameraState> {
    unsafe { GamePtr::from_raw(singleton().base.current_state.get()) }
}

#[inline(always)]
pub fn current_state_id() -> Option<CameraState> {
    current_state().with(|state| state.id)
}

#[inline(always)]
pub fn camera_target() -> Option<Resolved<Actor>> {
    Resolved::from_handle(singleton().camera_target)
}

#[inline(always)]
pub fn world_fov() -> f32 {
    singleton().runtime_data2().world_fov
}

#[inline(always)]
pub fn first_person_fov() -> f32 {
    singleton().runtime_data2().first_person_fov
}

#[inline(always)]
pub fn position() -> NiPoint3 {
    singleton().runtime_data2().pos
}

#[inline(always)]
pub fn yaw() -> f32 {
    singleton().runtime_data2().yaw
}

#[inline(always)]
pub fn allows_auto_vanity_mode() -> bool {
    singleton().runtime_data2().allow_auto_vanity_mode
}

#[inline(always)]
pub fn is_bow_zoomed_in() -> bool {
    singleton().runtime_data2().bow_zoomed_in
}

#[inline(always)]
pub fn is_weapon_sheathed() -> bool {
    singleton().runtime_data2().is_weap_sheathed
}

#[inline(always)]
pub fn is_processed() -> bool {
    singleton().runtime_data2().is_processed
}

#[inline(always)]
pub fn force_first_person() -> bool {
    let changed = unsafe { singleton().with_mut_unchecked(|camera| camera.force_first_person()) };
    if !changed && crate::runtime::is_vr() {
        crate::defensive_sdk_warn!(
            "sdk::gameplay::camera::force_first_person() is unsupported in VR"
        );
    }
    changed
}

#[inline(always)]
pub fn force_third_person() -> bool {
    let changed = unsafe { singleton().with_mut_unchecked(|camera| camera.force_third_person()) };
    if !changed && crate::runtime::is_vr() {
        crate::defensive_sdk_warn!(
            "sdk::gameplay::camera::force_third_person() is unsupported in VR"
        );
    }
    changed
}

#[inline(always)]
pub fn push_state(state: CameraState) -> bool {
    if !is_valid_camera_state(state) {
        crate::defensive_sdk_warn!(
            "sdk::gameplay::camera::push_state() ignored invalid camera state id={}",
            state.0
        );
        return false;
    }

    unsafe { singleton().with_mut_unchecked(|camera| camera.push_camera_state(state)) };
    true
}

#[inline(always)]
pub fn toggle_free_camera_mode(freeze_time: bool) {
    unsafe { singleton().with_mut_unchecked(|camera| camera.toggle_free_camera_mode(freeze_time)) };
}

#[inline(always)]
pub fn is_in_bleedout_mode() -> bool {
    singleton().is_in_bleedout_mode()
}

#[inline(always)]
pub fn is_in_first_person() -> bool {
    singleton().is_in_first_person()
}

#[inline(always)]
pub fn is_in_free_camera_mode() -> bool {
    singleton().is_in_free_camera_mode()
}

#[inline(always)]
pub fn is_in_third_person() -> bool {
    singleton().is_in_third_person()
}
