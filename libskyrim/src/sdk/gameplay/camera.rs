//! Camera-oriented gameplay helpers.
//!
//! This module is the lightweight entry point for current player-camera state
//! and a few deliberate camera transitions. It is meant for gameplay/UI code
//! that needs FOV, target, camera state, or controlled first/third-person
//! switching without diving into the raw `PlayerCamera` surface.
//!
//! Example:
//!
//! ```rust,ignore
//! use libskyrim::sdk::gameplay::camera;
//!
//! fn ensure_third_person() {
//!     if !camera::is_in_third_person() {
//!         let _ = camera::force_third_person();
//!     }
//! }
//! ```

use crate::re::{Actor, CameraState, NiNode, NiPoint3, PlayerCamera, TESCameraState};
use crate::sdk::core::{GamePtr, GameRef, Resolved};

/// Return the global `PlayerCamera` singleton.
///
/// Prefer the narrower helper functions below when possible; this accessor
/// exists for code that still needs direct `PlayerCamera` access.
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

/// Return the camera root scene node.
#[inline(always)]
pub fn camera_root() -> GamePtr<NiNode> {
    unsafe { GamePtr::from_raw(singleton().base.camera_root.get()) }
}

/// Return the current camera-state object.
#[inline(always)]
pub fn current_state() -> GamePtr<TESCameraState> {
    unsafe { GamePtr::from_raw(singleton().base.current_state.get()) }
}

/// Return the current camera-state ID when available.
#[inline(always)]
pub fn current_state_id() -> Option<CameraState> {
    current_state().with(|state| state.id)
}

/// Return the current actor camera target when one exists.
#[inline(always)]
pub fn camera_target() -> Option<Resolved<Actor>> {
    Resolved::from_handle(singleton().camera_target)
}

/// World FOV currently used by the player camera.
#[inline(always)]
pub fn world_fov() -> f32 {
    singleton().runtime_data2().world_fov
}

/// First-person FOV currently used by the player camera.
#[inline(always)]
pub fn first_person_fov() -> f32 {
    singleton().runtime_data2().first_person_fov
}

/// Current camera position.
#[inline(always)]
pub fn position() -> NiPoint3 {
    singleton().runtime_data2().pos
}

/// Current camera yaw.
#[inline(always)]
pub fn yaw() -> f32 {
    singleton().runtime_data2().yaw
}

/// Whether auto-vanity mode is currently allowed.
#[inline(always)]
pub fn allows_auto_vanity_mode() -> bool {
    singleton().runtime_data2().allow_auto_vanity_mode
}

/// Whether bow zoom is currently active.
#[inline(always)]
pub fn is_bow_zoomed_in() -> bool {
    singleton().runtime_data2().bow_zoomed_in
}

/// Whether the camera thinks the current weapon is sheathed.
#[inline(always)]
pub fn is_weapon_sheathed() -> bool {
    singleton().runtime_data2().is_weap_sheathed
}

/// Whether the player camera has been processed this frame.
#[inline(always)]
pub fn is_processed() -> bool {
    singleton().runtime_data2().is_processed
}

/// Try to force first-person mode.
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

/// Try to force third-person mode.
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

/// Push a concrete camera state onto the camera stack.
///
/// This is lower-level than [`force_first_person`] / [`force_third_person`]
/// and should usually be reserved for code that already understands Skyrim's
/// concrete camera-state IDs.
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

/// Toggle free camera mode.
#[inline(always)]
pub fn toggle_free_camera_mode(freeze_time: bool) {
    unsafe { singleton().with_mut_unchecked(|camera| camera.toggle_free_camera_mode(freeze_time)) };
}

/// Whether the camera is in bleedout mode.
#[inline(always)]
pub fn is_in_bleedout_mode() -> bool {
    singleton().is_in_bleedout_mode()
}

/// Whether the camera is currently in first person.
#[inline(always)]
pub fn is_in_first_person() -> bool {
    singleton().is_in_first_person()
}

/// Whether the camera is currently in free camera mode.
#[inline(always)]
pub fn is_in_free_camera_mode() -> bool {
    singleton().is_in_free_camera_mode()
}

/// Whether the camera is currently in third person.
#[inline(always)]
pub fn is_in_third_person() -> bool {
    singleton().is_in_third_person()
}
