use crate::re::BSTEventSource;

use super::events::{
    ActionEvent, CameraEvent, CrosshairRefEvent, ModCallbackEvent, NiNodeUpdateEvent,
};
use super::interfaces::{LoadInterface, SerializationInterface, TrampolineInterface};

#[inline(always)]
pub unsafe fn init(load_interface: *const LoadInterface) {
    unsafe {
        crate::ffi::init_commonlib(load_interface.cast());
    }
}

#[inline(always)]
pub fn alloc_trampoline(size: usize) {
    unsafe {
        crate::ffi::commonlib_alloc_trampoline(size);
    }
}

#[inline(always)]
pub fn get_serialization_interface() -> *mut SerializationInterface {
    unsafe { crate::ffi::commonlib_skse_get_serialization_interface().cast() }
}

#[inline(always)]
pub fn get_trampoline_interface() -> *mut TrampolineInterface {
    unsafe { crate::ffi::commonlib_skse_get_trampoline_interface().cast() }
}

#[inline(always)]
pub fn get_mod_callback_event_source() -> *mut BSTEventSource<ModCallbackEvent> {
    unsafe { crate::ffi::commonlib_skse_get_mod_callback_event_source().cast() }
}

#[inline(always)]
pub fn get_camera_event_source() -> *mut BSTEventSource<CameraEvent> {
    unsafe { crate::ffi::commonlib_skse_get_camera_event_source().cast() }
}

#[inline(always)]
pub fn get_crosshair_ref_event_source() -> *mut BSTEventSource<CrosshairRefEvent> {
    unsafe { crate::ffi::commonlib_skse_get_crosshair_ref_event_source().cast() }
}

#[inline(always)]
pub fn get_action_event_source() -> *mut BSTEventSource<ActionEvent> {
    unsafe { crate::ffi::commonlib_skse_get_action_event_source().cast() }
}

#[inline(always)]
pub fn get_ni_node_update_event_source() -> *mut BSTEventSource<NiNodeUpdateEvent> {
    unsafe { crate::ffi::commonlib_skse_get_ni_node_update_event_source().cast() }
}
