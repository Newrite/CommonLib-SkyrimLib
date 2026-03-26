use core::ffi::{c_char, c_void};

use crate::re::bs_core_types::VMHandle;
use crate::re::tes_form::FormID;

/// Plugin interface IDs.
#[repr(u32)]
pub enum InterfaceId {
    Invalid,
    Scaleform,
    Papyrus,
    Serialization,
    Task,
    Messaging,
    Object,
    Trampoline,
    Max,
}

/// The ID assigned to a loaded plugin.
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct PluginHandle(u32);

/// Plugin query info returned to SKSE.
#[repr(C)]
pub struct PluginInfo {
    pub info_version: u32,
    pub name: *const c_char,
    pub version: u32,
}

/// Raw load interface passed to `SKSEPlugin_Query` and `SKSEPlugin_Load`.
#[repr(C)]
pub struct SkseInterface {
    pub skse_version: u32,
    pub runtime_version: u32,
    pub editor_version: u32,
    pub is_editor: u32,
    pub query_interface: unsafe extern "system" fn(InterfaceId) -> *mut c_void,
    pub get_plugin_handle: unsafe extern "system" fn() -> PluginHandle,
    pub get_release_index: unsafe extern "system" fn() -> u32,
    pub get_plugin_info: unsafe extern "system" fn(*const c_char) -> *const PluginInfo,
}

/// A message which can be received from/sent to other SKSE plugins.
#[repr(C)]
pub struct Message {
    pub sender: *const c_char,
    pub msg_type: u32,
    pub data_len: u32,
    pub data: *mut u8,
}

/// A callback function registered as a message listener.
pub type MessageCallback = unsafe extern "system" fn(*mut Message);

/// The raw messaging interface returned by SKSE.
#[repr(C)]
pub struct SkseMessagingInterface {
    pub interface_version: u32,
    pub register_listener:
        unsafe extern "system" fn(PluginHandle, *const c_char, MessageCallback) -> bool,
    pub dispatch:
        unsafe extern "system" fn(PluginHandle, u32, *mut c_void, u32, *const c_char) -> bool,
    pub get_event_dispatcher: unsafe extern "system" fn(u32) -> *mut c_void,
}

/// A callback function registered with the serialization interface.
pub type SerializationEventCallback = unsafe extern "system" fn(*mut SerializationInterface);
/// A callback function registered for form-delete notifications.
pub type FormDeleteCallback = unsafe extern "system" fn(VMHandle);

/// The raw serialization interface returned by SKSE.
#[repr(C)]
pub struct SkseSerializationInterface {
    pub interface_version: u32,
    pub set_unique_id: unsafe extern "system" fn(PluginHandle, u32),
    pub set_revert_callback:
        unsafe extern "system" fn(PluginHandle, Option<SerializationEventCallback>),
    pub set_save_callback:
        unsafe extern "system" fn(PluginHandle, Option<SerializationEventCallback>),
    pub set_load_callback:
        unsafe extern "system" fn(PluginHandle, Option<SerializationEventCallback>),
    pub set_form_delete_callback:
        unsafe extern "system" fn(PluginHandle, Option<FormDeleteCallback>),
    pub write_record: unsafe extern "system" fn(u32, u32, *const c_void, u32) -> bool,
    pub open_record: unsafe extern "system" fn(u32, u32) -> bool,
    pub write_record_data: unsafe extern "system" fn(*const c_void, u32) -> bool,
    pub get_next_record_info: unsafe extern "system" fn(*mut u32, *mut u32, *mut u32) -> bool,
    pub read_record_data: unsafe extern "system" fn(*mut c_void, u32) -> u32,
    pub resolve_handle: unsafe extern "system" fn(VMHandle, *mut VMHandle) -> bool,
    pub resolve_form_id: unsafe extern "system" fn(FormID, *mut FormID) -> bool,
}

/// The raw trampoline interface returned by SKSE.
#[repr(C)]
pub struct SkseTrampolineInterface {
    pub interface_version: u32,
    pub allocate_from_branch_pool: unsafe extern "system" fn(PluginHandle, usize) -> *mut c_void,
    pub allocate_from_local_pool: unsafe extern "system" fn(PluginHandle, usize) -> *mut c_void,
}

pub type LoadInterface = SkseInterface;
pub type MessagingInterface = SkseMessagingInterface;
pub type SerializationInterface = SkseSerializationInterface;
pub type TrampolineInterface = SkseTrampolineInterface;

impl PluginInfo {
    pub const VERSION: u32 = 1;
}

impl PluginHandle {
    pub const INVALID: Self = Self(u32::MAX);
}

impl Message {
    pub const SKSE_POST_LOAD: u32 = 0;
    pub const SKSE_POST_POST_LOAD: u32 = 1;
    pub const SKSE_PRE_LOAD_GAME: u32 = 2;
    pub const SKSE_POST_LOAD_GAME: u32 = 3;
    pub const SKSE_SAVE_GAME: u32 = 4;
    pub const SKSE_DELETE_GAME: u32 = 5;
    pub const SKSE_INPUT_LOADED: u32 = 6;
    pub const SKSE_NEW_GAME: u32 = 7;
    pub const SKSE_DATA_LOADED: u32 = 8;
    pub const SKSE_MAX: usize = 9;
}

impl SkseMessagingInterface {
    pub const VERSION: u32 = 2;
}

impl SkseSerializationInterface {
    pub const VERSION: u32 = 4;

    #[inline(always)]
    pub fn set_unique_id(&self, uid: u32) {
        unsafe {
            (self.set_unique_id)(crate::plugin_api::handle(), uid);
        }
    }

    #[inline(always)]
    pub fn set_revert_callback(&self, callback: Option<SerializationEventCallback>) {
        unsafe {
            (self.set_revert_callback)(crate::plugin_api::handle(), callback);
        }
    }

    #[inline(always)]
    pub fn set_save_callback(&self, callback: Option<SerializationEventCallback>) {
        unsafe {
            (self.set_save_callback)(crate::plugin_api::handle(), callback);
        }
    }

    #[inline(always)]
    pub fn set_load_callback(&self, callback: Option<SerializationEventCallback>) {
        unsafe {
            (self.set_load_callback)(crate::plugin_api::handle(), callback);
        }
    }

    #[inline(always)]
    pub fn set_form_delete_callback(&self, callback: Option<FormDeleteCallback>) {
        unsafe {
            (self.set_form_delete_callback)(crate::plugin_api::handle(), callback);
        }
    }

    #[inline(always)]
    pub fn write_record(&self, ty: u32, version: u32, buf: *const c_void, length: u32) -> bool {
        unsafe { (self.write_record)(ty, version, buf, length) }
    }

    #[inline(always)]
    pub fn open_record(&self, ty: u32, version: u32) -> bool {
        unsafe { (self.open_record)(ty, version) }
    }

    #[inline(always)]
    pub fn write_record_data(&self, buf: *const c_void, length: u32) -> bool {
        unsafe { (self.write_record_data)(buf, length) }
    }

    #[inline(always)]
    pub fn get_next_record_info(&self, ty: &mut u32, version: &mut u32, length: &mut u32) -> bool {
        unsafe { (self.get_next_record_info)(ty, version, length) }
    }

    #[inline(always)]
    pub fn read_record_data(&self, buf: *mut c_void, length: u32) -> u32 {
        unsafe { (self.read_record_data)(buf, length) }
    }

    #[inline(always)]
    pub fn resolve_handle(&self, old_handle: VMHandle, new_handle: &mut VMHandle) -> bool {
        unsafe { (self.resolve_handle)(old_handle, new_handle) }
    }

    #[inline(always)]
    pub fn resolve_form_id(&self, old_form_id: FormID, new_form_id: &mut FormID) -> bool {
        unsafe { (self.resolve_form_id)(old_form_id, new_form_id) }
    }
}

impl SkseTrampolineInterface {
    pub const VERSION: u32 = 1;

    #[inline(always)]
    pub fn allocate_from_branch_pool(&self, size: usize) -> *mut c_void {
        unsafe { (self.allocate_from_branch_pool)(crate::plugin_api::handle(), size) }
    }

    #[inline(always)]
    pub fn allocate_from_local_pool(&self, size: usize) -> *mut c_void {
        unsafe { (self.allocate_from_local_pool)(crate::plugin_api::handle(), size) }
    }
}
