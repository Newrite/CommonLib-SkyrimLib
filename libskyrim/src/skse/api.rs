use crate::re::BSTEventSource;

use super::events::{
    ActionEvent, CameraEvent, CrosshairRefEvent, ModCallbackEvent, NiNodeUpdateEvent,
};
use super::interfaces::{
    LoadInterface, MessagingInterface, ObjectInterface, PapyrusInterface, SKSEDelayFunctorManager,
    SKSEObjectRegistry, SKSEPersistentObjectStorage, ScaleformInterface, SerializationInterface,
    TaskInterface, TrampolineInterface,
};

#[inline(always)]
pub unsafe fn init(load_interface: *const LoadInterface) {
    unsafe {
        crate::ffi::init_commonlib(load_interface.cast());
    }
}

#[inline(always)]
pub unsafe fn init_with_log(load_interface: *const LoadInterface, log: bool) {
    unsafe {
        crate::ffi::init_commonlib_with_log(load_interface.cast(), log);
    }
}

#[inline(always)]
pub fn alloc_trampoline(size: usize) {
    unsafe {
        crate::ffi::commonlib_alloc_trampoline(size);
    }
}

#[inline(always)]
pub fn get_scaleform_interface() -> *mut ScaleformInterface {
    unsafe { crate::ffi::commonlib_skse_get_scaleform_interface().cast() }
}

#[inline(always)]
pub fn get_serialization_interface() -> *mut SerializationInterface {
    unsafe { crate::ffi::commonlib_skse_get_serialization_interface().cast() }
}

#[inline(always)]
pub fn get_papyrus_interface() -> *mut PapyrusInterface {
    unsafe { crate::ffi::commonlib_skse_get_papyrus_interface().cast() }
}

#[inline(always)]
pub fn get_task_interface() -> *mut TaskInterface {
    unsafe { crate::ffi::commonlib_skse_get_task_interface().cast() }
}

#[inline(always)]
pub fn get_messaging_interface() -> *mut MessagingInterface {
    unsafe { crate::ffi::commonlib_skse_get_messaging_interface().cast() }
}

#[inline(always)]
pub fn get_object_interface() -> *mut ObjectInterface {
    unsafe { crate::ffi::commonlib_skse_get_object_interface().cast() }
}

#[inline(always)]
pub fn get_trampoline_interface() -> *mut TrampolineInterface {
    unsafe { crate::ffi::commonlib_skse_get_trampoline_interface().cast() }
}

#[inline(always)]
pub fn get_delay_functor_manager() -> *mut SKSEDelayFunctorManager {
    unsafe { crate::ffi::commonlib_skse_get_delay_functor_manager().cast() }
}

#[inline(always)]
pub fn get_object_registry() -> *mut SKSEObjectRegistry {
    unsafe { crate::ffi::commonlib_skse_get_object_registry().cast() }
}

#[inline(always)]
pub fn get_persistent_object_storage() -> *mut SKSEPersistentObjectStorage {
    unsafe { crate::ffi::commonlib_skse_get_persistent_object_storage().cast() }
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

// TODO: `SKSE::RegisterForAPIInitEvent(std::function<void()>)` is still omitted
// here because libskyrim does not yet have a retained callback bridge for
// stored `std::function` registrations. Expose it once the callback lifetime
// and ownership model is implemented through the shared FFI bridge.
