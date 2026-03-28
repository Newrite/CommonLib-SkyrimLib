#![allow(non_camel_case_types)]

use crate::re::bst_singleton::BSTSingletonSDM;
use crate::re::{
    BSFixedString, BSIInputDevice, BSInputDeviceFactory, BSPCGamepadDeviceDelegate,
    BSPCGamepadDeviceHandler, BSRemoteGamepadEvent, BSTEventSink, BSTEventSource,
    BSTrackedControllerDevice, BSWin32KeyboardDevice, BSWin32MouseDevice,
    BSWin32VirtualKeyboardDevice, INPUT_DEVICE, INPUT_DEVICES, InputEvent,
};
use crate::relocation::{RelocationID, VariantOffset};

/// Minimal source-backed partial translation of C++ `RE::BSInputDeviceManager::RUNTIME_DATA`.
#[repr(C)]
pub struct BSInputDeviceManagerRuntimeData {
    pub queued_gamepad_enable_value: bool, // 00
    pub value_queued: bool,                // 01
    pub polling_enabled: bool,             // 02
    pub pad03: u8,                         // 03
    pub pad04: u32,                        // 04
    pub remote_gamepad_event_source: BSTEventSource<BSRemoteGamepadEvent>, // 08
    pub unk60: u8,                         // 60
    pub unk61: u8,                         // 61
    pub unk62: u16,                        // 62
    pub unk64: u32,                        // 64
    pub unk68: u64,                        // 68
}

const _: () = assert!(core::mem::size_of::<BSInputDeviceManagerRuntimeData>() == 0x70);
const _: () = assert!(
    core::mem::offset_of!(BSInputDeviceManagerRuntimeData, remote_gamepad_event_source) == 0x08
);

/// Honest common prefix of C++ `RE::BSInputDeviceManager`.
#[repr(C)]
pub struct BSInputDeviceManager {
    pub event_source: BSTEventSource<*mut InputEvent>, // 00
    pub base: BSTSingletonSDM<BSInputDeviceManager>,   // 58
    pub pad59: u8,                                     // 59
    pub pad5a: u16,                                    // 5A
    pub pad5c: u32,                                    // 5C
    pub devices: [*mut BSIInputDevice; 4],             // 60
}

const _: () = assert!(core::mem::size_of::<BSInputDeviceManager>() == 0x80);
const _: () = assert!(core::mem::offset_of!(BSInputDeviceManager, event_source) == 0x00);
const _: () = assert!(core::mem::offset_of!(BSInputDeviceManager, base) == 0x58);
const _: () = assert!(core::mem::offset_of!(BSInputDeviceManager, devices) == 0x60);

core_util::inherit!(BSInputDeviceManager => BSTEventSource<*mut InputEvent>, event_source);
core_util::inherit!(BSInputDeviceManager => BSTSingletonSDM<BSInputDeviceManager>, base);

impl BSInputDeviceManager {
    pub const RUNTIME_DATA_OFFSET: VariantOffset = VariantOffset::new(0x80, 0x80, 0x98);

    crate::relocation_variable! {
        fn singleton() -> *mut BSInputDeviceManager => RelocationID::new(516574, 402776), is_ptr
    }

    crate::relocation_func! {
        pub fn poll_input_devices(&mut self, secs_since_last_frame: f32) => RelocationID::new(67315, 68617)
    }

    crate::runtime_data_accessor! {
        pub fn runtime_data() -> BSInputDeviceManagerRuntimeData {
            offset: Self::RUNTIME_DATA_OFFSET
        }
    }

    crate::runtime_data_mut_accessor! {
        pub fn runtime_data_mut() -> BSInputDeviceManagerRuntimeData {
            offset: Self::RUNTIME_DATA_OFFSET
        }
    }

    #[inline(always)]
    pub fn get_singleton() -> *mut BSInputDeviceManager {
        Self::singleton()
    }

    #[inline(always)]
    fn device_by_type(&self, device: INPUT_DEVICE) -> *mut BSIInputDevice {
        match device {
            INPUT_DEVICE::kKeyboard => self.devices[INPUT_DEVICE::kKeyboard as usize],
            INPUT_DEVICE::kMouse => self.devices[INPUT_DEVICE::kMouse as usize],
            INPUT_DEVICE::kGamepad => self.devices[INPUT_DEVICE::kGamepad as usize],
            INPUT_DEVICE::kFlatVirtualKeyboard if !crate::runtime::is_vr() => {
                self.devices[INPUT_DEVICE::kFlatVirtualKeyboard as usize]
            }
            _ => {
                // TODO: `BSInputDeviceManager` only has a source-backed common prefix in Rust.
                // The vendored CommonLib header exposes `devices[4]` in the prefix and a VR-only
                // tail with additional tracked-controller / virtual-keyboard storage, but it does
                // not prove a single universal cross-runtime device-array layout for indices >= 4.
                // Replace this branch with honest runtime-tail accessors when the full VR storage
                // surface is translated instead of keeping VR-only device lookups unavailable here.
                core::ptr::null_mut()
            }
        }
    }

    #[inline(always)]
    pub fn get_button_name_from_id(
        &self,
        device: INPUT_DEVICE,
        id: i32,
        button_name: &mut BSFixedString,
    ) -> bool {
        let device = self.device_by_type(device);
        !device.is_null() && unsafe { (*device).get_button_name_from_id(id, button_name) }
    }

    #[inline(always)]
    pub fn try_get_button_name_from_id(
        &self,
        device: INPUT_DEVICE,
        id: i32,
    ) -> Option<BSFixedString> {
        let mut button_name = BSFixedString::empty();
        if self.get_button_name_from_id(device, id, &mut button_name) {
            Some(button_name)
        } else {
            None
        }
    }

    #[inline(always)]
    pub fn get_gamepad(&self) -> *mut BSPCGamepadDeviceDelegate {
        let handler = self.get_gamepad_handler();
        if handler.is_null() {
            core::ptr::null_mut()
        } else {
            unsafe { (*handler).runtime_data().current_pc_game_pad_delegate }
        }
    }

    #[inline(always)]
    pub fn get_gamepad_handler(&self) -> *mut BSPCGamepadDeviceHandler {
        self.devices[INPUT_DEVICE::kGamepad as usize].cast()
    }

    #[inline(always)]
    pub fn get_keyboard(&self) -> *mut BSWin32KeyboardDevice {
        self.devices[INPUT_DEVICE::kKeyboard as usize].cast()
    }

    #[inline(always)]
    pub fn get_mouse(&self) -> *mut BSWin32MouseDevice {
        self.devices[INPUT_DEVICE::kMouse as usize].cast()
    }

    #[inline(always)]
    pub fn get_vr_controller_right(&self) -> *mut BSTrackedControllerDevice {
        // TODO: `GetVRControllerRight()` indexes VR-only controller storage beyond the common
        // `devices[4]` prefix using the manager's untranslated VR tail plus
        // `BSOpenVR::GetSingleton()->GetHMDDeviceType()`. `BSOpenVR` now exists in Rust, but the
        // owner-side VR storage layout here still does not. Replace this null-returning
        // compromise once `BSInputDeviceManager` gains honest runtime-tail accessors for the
        // additional tracked-controller slots instead of fake flat indexing.
        core::ptr::null_mut()
    }

    #[inline(always)]
    pub fn get_vr_controller_left(&self) -> *mut BSTrackedControllerDevice {
        // TODO: `GetVRControllerLeft()` has the same untranslated manager VR-tail storage
        // dependency as `GetVRControllerRight()`. Replace this null-returning compromise once
        // those extra controller slots are modeled honestly instead of treating the common prefix
        // as a universal array.
        core::ptr::null_mut()
    }

    #[inline(always)]
    pub fn get_virtual_keyboard(&self) -> *mut BSWin32VirtualKeyboardDevice {
        if crate::runtime::is_vr() {
            // TODO: VR routes the virtual keyboard through manager storage outside the common
            // `devices[4]` prefix. Keep this unavailable until the true VR tail layout is
            // translated honestly instead of inventing a flat `devices[kTotal]` array.
            core::ptr::null_mut()
        } else {
            self.devices[INPUT_DEVICES::virtual_keyboard() as usize].cast()
        }
    }

    #[inline(always)]
    pub fn is_gamepad_connected(&self) -> bool {
        let handler = self.get_gamepad_handler();
        !handler.is_null()
            && unsafe {
                !(*handler)
                    .runtime_data()
                    .current_pc_game_pad_delegate
                    .is_null()
            }
    }

    #[inline(always)]
    pub fn is_gamepad_enabled(&self) -> bool {
        let gamepad = self.get_gamepad();
        !gamepad.is_null() && unsafe { (*gamepad).is_enabled() }
    }

    #[inline(always)]
    pub fn is_mouse_background(&self) -> bool {
        let mouse = self.get_mouse();
        !mouse.is_null() && unsafe { (&*mouse).background_mouse }
    }

    #[inline(always)]
    pub fn get_device_button_name_from_id(
        &self,
        device: INPUT_DEVICE,
        key: u32,
        mapping: &mut BSFixedString,
    ) -> bool {
        let device = self.device_by_type(device);
        !device.is_null() && unsafe { (*device).get_button_name_from_id(key as i32, mapping) }
    }

    #[inline(always)]
    pub fn try_get_device_button_name_from_id(
        &self,
        device: INPUT_DEVICE,
        key: u32,
    ) -> Option<BSFixedString> {
        let mut mapping = BSFixedString::empty();
        if self.get_device_button_name_from_id(device, key, &mut mapping) {
            Some(mapping)
        } else {
            None
        }
    }

    #[inline(always)]
    pub fn get_device_key_code_from_id(
        &self,
        device: INPUT_DEVICE,
        key: u32,
        out_key_code: &mut u32,
    ) -> bool {
        let device = self.device_by_type(device);
        !device.is_null() && unsafe { (*device).get_key_code_from_id(key as i32, out_key_code) }
    }

    #[inline(always)]
    pub fn try_get_device_key_code_from_id(&self, device: INPUT_DEVICE, key: u32) -> Option<u32> {
        let mut out_key_code = 0;
        if self.get_device_key_code_from_id(device, key, &mut out_key_code) {
            Some(out_key_code)
        } else {
            None
        }
    }

    #[inline(always)]
    pub fn process_gamepad_enabled_change(&mut self) {
        if self.runtime_data().value_queued {
            let gamepad_enable = RelocationID::new(511901, 388465).address() as *mut bool;
            unsafe {
                *gamepad_enable = true;
            }
            self.runtime_data_mut().value_queued = false;
        }
    }

    #[inline(always)]
    pub fn reinitialize_mouse(&mut self) {
        let mouse = self.get_mouse();
        if !mouse.is_null() {
            unsafe {
                (*mouse).reinitialize();
            }
        }
    }

    #[inline(always)]
    pub fn create_input_devices(&mut self) {
        for i in 0..self.devices.len() {
            let device = BSInputDeviceFactory::create_input_device(unsafe {
                core::mem::transmute(i as i32)
            });
            self.devices[i] = device;
            if !device.is_null() {
                unsafe {
                    (*device).initialize();
                }
            }
        }

        if crate::runtime::is_vr() {
            // TODO: CommonLib's VR `CreateInputDevices()` iterates `INPUT_DEVICE::kTotal`, but the
            // translated manager still keeps only the honest `devices[4]` common prefix. Extend
            // this method once the remaining VR-only device storage is translated instead of
            // pretending the flat prefix covers all controller slots.
        }
    }

    #[inline(always)]
    pub fn reset_input_devices(&mut self) {
        for device in &mut self.devices {
            if !device.is_null() {
                unsafe {
                    (**device).clear_input_state();
                }
            }
        }

        if crate::runtime::is_vr() {
            // TODO: VR reset walks additional device storage beyond the common prefix. Keep the
            // flat devices reset limited to the source-backed prefix until the real VR tail is
            // translated.
        }
    }

    #[inline(always)]
    pub fn destroy_input_devices(&mut self) {
        for device in &mut self.devices {
            if !device.is_null() {
                unsafe {
                    (**device).shutdown();
                    BSInputDeviceFactory::destroy_input_device(*device);
                }
                *device = core::ptr::null_mut();
            }
        }

        if crate::runtime::is_vr() {
            // TODO: VR destroy also covers device storage outside the common `devices[4]` prefix.
            // Keep the translated destroy path limited to the verified flat slots until the VR tail
            // storage is modeled honestly.
        }
    }

    #[inline(always)]
    pub unsafe fn add_input_event_sink(&mut self, sink: *mut BSTEventSink<*mut InputEvent>) {
        unsafe { self.event_source.add_event_sink(sink) }
    }

    #[inline(always)]
    pub unsafe fn remove_input_event_sink(&mut self, sink: *mut BSTEventSink<*mut InputEvent>) {
        unsafe { self.event_source.remove_event_sink(sink) }
    }

    #[inline(always)]
    pub unsafe fn add_remote_gamepad_event_sink(
        &mut self,
        sink: *mut BSTEventSink<BSRemoteGamepadEvent>,
    ) {
        unsafe {
            self.runtime_data_mut()
                .remote_gamepad_event_source
                .add_event_sink(sink)
        }
    }

    #[inline(always)]
    pub unsafe fn remove_remote_gamepad_event_sink(
        &mut self,
        sink: *mut BSTEventSink<BSRemoteGamepadEvent>,
    ) {
        unsafe {
            self.runtime_data_mut()
                .remote_gamepad_event_source
                .remove_event_sink(sink)
        }
    }
}
