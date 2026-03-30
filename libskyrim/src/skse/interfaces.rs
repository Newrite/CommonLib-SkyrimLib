use core::ffi::{c_char, c_void};

use crate::re::bs_core_types::FormID;
use crate::re::bs_core_types::VMHandle;
use crate::re::{IVirtualMachine, VirtualMachine};
use crate::version::Version;

core_util::abstract_type! {
    pub type SKSEDelayFunctorManager;
    pub type SKSEObjectRegistry;
    pub type SKSEPersistentObjectStorage;
}

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
    Total,
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
    pub query_interface: unsafe extern "system" fn(u32) -> *mut c_void,
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
    pub data: *mut c_void,
}

/// A callback function registered as a message listener.
pub type MessageCallback = unsafe extern "system" fn(*mut Message);
/// A raw callback function registered with the scaleform interface.
///
/// TODO: Replace the erased `c_void` parameters with `GFxMovieView`, `GFxValue`,
/// and `InventoryEntryData` callback signatures once those RE dependencies have
/// matching Rust translations in `libskyrim/src/re`.
pub type ScaleformRegCallback = unsafe extern "system" fn(*mut c_void, *mut c_void) -> bool;
/// A raw inventory callback function registered with the scaleform interface.
pub type ScaleformInventoryCallback =
    unsafe extern "system" fn(*mut c_void, *mut c_void, *mut c_void);

/// A callback function registered with the papyrus interface.
pub type PapyrusRegFunction1 = unsafe extern "system" fn(*mut VirtualMachine) -> bool;
/// A callback function registered with the papyrus interface.
pub type PapyrusRegFunction2 = unsafe extern "system" fn(*mut IVirtualMachine) -> bool;

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

/// The raw papyrus interface returned by SKSE.
#[repr(C)]
pub struct SksePapyrusInterface {
    pub interface_version: u32,
    pub register: unsafe extern "system" fn(*mut c_void) -> bool,
}

/// The raw scaleform interface returned by SKSE.
#[repr(C)]
pub struct SkseScaleformInterface {
    pub interface_version: u32,
    pub register: unsafe extern "system" fn(*const c_char, *mut c_void) -> bool,
    pub register_for_inventory: unsafe extern "system" fn(*mut c_void),
}

/// The raw object interface returned by SKSE.
#[repr(C)]
pub struct SkseObjectInterface {
    pub interface_version: u32,
    pub get_delay_functor_manager: unsafe extern "system" fn() -> *mut SKSEDelayFunctorManager,
    pub get_object_registry: unsafe extern "system" fn() -> *mut SKSEObjectRegistry,
    pub get_persistent_object_storage:
        unsafe extern "system" fn() -> *mut SKSEPersistentObjectStorage,
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

/// The raw task interface returned by SKSE.
#[repr(C)]
pub struct SkseTaskInterface {
    pub interface_version: u32,
    pub add_task: unsafe extern "system" fn(*mut c_void),
    pub add_ui_task: unsafe extern "system" fn(*mut c_void),
}

pub type LoadInterface = SkseInterface;
pub type PapyrusInterface = SksePapyrusInterface;
pub type ScaleformInterface = SkseScaleformInterface;
pub type MessagingInterface = SkseMessagingInterface;
pub type ObjectInterface = SkseObjectInterface;
pub type SerializationInterface = SkseSerializationInterface;
pub type TaskInterface = SkseTaskInterface;
pub type TrampolineInterface = SkseTrampolineInterface;

/// Messaging dispatchers exposed by `SKSE::MessagingInterface`.
#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum MessagingDispatcher {
    ModEvent = 0,
    CameraEvent = 1,
    CrosshairEvent = 2,
    ActionEvent = 3,
    NiNodeUpdateEvent = 4,
    Total = 5,
}

impl PluginInfo {
    pub const VERSION: u32 = 1;
}

impl PluginHandle {
    pub const INVALID: Self = Self(u32::MAX);
}

impl SkseInterface {
    #[inline(always)]
    pub fn skse_version(&self) -> u32 {
        self.skse_version
    }

    #[inline(always)]
    pub fn runtime_version(&self) -> Version {
        Version::from_packed(self.runtime_version)
    }

    #[inline(always)]
    pub fn editor_version(&self) -> u32 {
        self.editor_version
    }

    #[inline(always)]
    pub fn is_editor(&self) -> bool {
        self.is_editor != 0
    }

    #[inline(always)]
    pub fn get_plugin_handle(&self) -> PluginHandle {
        unsafe { (self.get_plugin_handle)() }
    }

    #[inline(always)]
    pub fn get_release_index(&self) -> u32 {
        unsafe { (self.get_release_index)() }
    }

    #[inline(always)]
    pub fn get_plugin_info(&self, name: *const c_char) -> *const PluginInfo {
        unsafe { (self.get_plugin_info)(name) }
    }

    #[inline(always)]
    pub fn query_interface_raw(&self, id: u32) -> *mut c_void {
        unsafe { (self.query_interface)(id) }
    }

    #[inline(always)]
    pub fn query_interface(&self, id: InterfaceId) -> *mut c_void {
        self.query_interface_raw(id as u32)
    }
}

impl Message {
    pub const SKSE_TOTAL: usize = 9;
    pub const SKSE_POST_LOAD: u32 = 0;
    pub const SKSE_POST_POST_LOAD: u32 = 1;
    pub const SKSE_PRE_LOAD_GAME: u32 = 2;
    pub const SKSE_POST_LOAD_GAME: u32 = 3;
    pub const SKSE_SAVE_GAME: u32 = 4;
    pub const SKSE_DELETE_GAME: u32 = 5;
    pub const SKSE_INPUT_LOADED: u32 = 6;
    pub const SKSE_NEW_GAME: u32 = 7;
    pub const SKSE_DATA_LOADED: u32 = 8;
    pub const SKSE_MAX: usize = Self::SKSE_TOTAL;
}

impl SkseMessagingInterface {
    pub const VERSION: u32 = 2;

    #[inline(always)]
    pub fn version(&self) -> u32 {
        self.interface_version
    }

    #[inline(always)]
    pub fn dispatch(
        &self,
        message_type: u32,
        data: *mut c_void,
        data_len: u32,
        receiver: *const c_char,
    ) -> bool {
        unsafe {
            (self.dispatch)(
                super::plugin_handle(),
                message_type,
                data,
                data_len,
                receiver,
            )
        }
    }

    #[inline(always)]
    pub fn get_event_dispatcher(&self, dispatcher: MessagingDispatcher) -> *mut c_void {
        unsafe { (self.get_event_dispatcher)(dispatcher as u32) }
    }

    #[inline(always)]
    pub fn register_listener(&self, callback: MessageCallback) -> bool {
        self.register_listener_for(b"SKSE\0".as_ptr().cast(), callback)
    }

    #[inline(always)]
    pub fn register_listener_for(&self, sender: *const c_char, callback: MessageCallback) -> bool {
        unsafe { (self.register_listener)(super::plugin_handle(), sender, callback) }
    }
}

impl SksePapyrusInterface {
    pub const VERSION: u32 = 1;

    #[inline(always)]
    pub fn version(&self) -> u32 {
        self.interface_version
    }

    #[inline(always)]
    pub fn register_internal(&self, callback: PapyrusRegFunction1) -> bool {
        let vm = VirtualMachine::get_singleton();
        if vm.is_null() {
            unsafe { (self.register)(callback as *mut c_void) }
        } else {
            unsafe {
                callback(vm);
            }
            true
        }
    }

    #[inline(always)]
    pub fn register_vm(&self, callback: PapyrusRegFunction2) -> bool {
        let vm = VirtualMachine::get_singleton();
        if vm.is_null() {
            unsafe { (self.register)(callback as *mut c_void) }
        } else {
            unsafe {
                callback(vm.cast());
            }
            true
        }
    }
}

impl SkseScaleformInterface {
    pub const VERSION: u32 = 2;

    #[inline(always)]
    pub fn version(&self) -> u32 {
        self.interface_version
    }

    #[inline(always)]
    pub fn register_raw(&self, callback: ScaleformRegCallback, name: *const c_char) -> bool {
        unsafe { (self.register)(name, callback as *mut c_void) }
    }

    #[inline(always)]
    pub fn register_for_inventory_raw(&self, callback: ScaleformInventoryCallback) {
        unsafe { (self.register_for_inventory)(callback as *mut c_void) }
    }
}

impl SkseObjectInterface {
    pub const VERSION: u32 = 1;

    #[inline(always)]
    pub fn version(&self) -> u32 {
        self.interface_version
    }

    #[inline(always)]
    pub fn get_delay_functor_manager(&self) -> *mut SKSEDelayFunctorManager {
        unsafe { (self.get_delay_functor_manager)() }
    }

    #[inline(always)]
    pub fn get_object_registry(&self) -> *mut SKSEObjectRegistry {
        unsafe { (self.get_object_registry)() }
    }

    #[inline(always)]
    pub fn get_persistent_object_storage(&self) -> *mut SKSEPersistentObjectStorage {
        unsafe { (self.get_persistent_object_storage)() }
    }
}

impl SkseSerializationInterface {
    pub const VERSION: u32 = 4;

    #[inline(always)]
    pub fn set_unique_id(&self, uid: u32) {
        unsafe {
            (self.set_unique_id)(super::plugin_handle(), uid);
        }
    }

    #[inline(always)]
    pub fn set_revert_callback(&self, callback: Option<SerializationEventCallback>) {
        unsafe {
            (self.set_revert_callback)(super::plugin_handle(), callback);
        }
    }

    #[inline(always)]
    pub fn set_save_callback(&self, callback: Option<SerializationEventCallback>) {
        unsafe {
            (self.set_save_callback)(super::plugin_handle(), callback);
        }
    }

    #[inline(always)]
    pub fn set_load_callback(&self, callback: Option<SerializationEventCallback>) {
        unsafe {
            (self.set_load_callback)(super::plugin_handle(), callback);
        }
    }

    #[inline(always)]
    pub fn set_form_delete_callback(&self, callback: Option<FormDeleteCallback>) {
        unsafe {
            (self.set_form_delete_callback)(super::plugin_handle(), callback);
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
    pub fn write_record_data_ex(&self, diff: &mut u32, buf: *const c_void, length: u32) -> bool {
        *diff = diff.saturating_add(length);
        self.write_record_data(buf, length)
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
    pub fn read_record_data_ex(&self, diff: &mut u32, buf: *mut c_void, length: u32) -> u32 {
        let result = self.read_record_data(buf, length);
        *diff = diff.saturating_sub(result);
        result
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
        unsafe { (self.allocate_from_branch_pool)(super::plugin_handle(), size) }
    }

    #[inline(always)]
    pub fn allocate_from_local_pool(&self, size: usize) -> *mut c_void {
        unsafe { (self.allocate_from_local_pool)(super::plugin_handle(), size) }
    }
}

impl SkseTaskInterface {
    pub const VERSION: u32 = 2;

    #[inline(always)]
    pub fn version(&self) -> u32 {
        self.interface_version
    }

    #[inline(always)]
    pub fn add_task_raw(&self, task: *mut c_void) {
        unsafe { (self.add_task)(task) }
    }

    #[inline(always)]
    pub fn add_ui_task_raw(&self, task: *mut c_void) {
        unsafe { (self.add_ui_task)(task) }
    }
}
