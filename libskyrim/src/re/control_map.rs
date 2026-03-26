#![allow(non_camel_case_types, non_upper_case_globals)]

use core_util::EnumSet;

use crate::re::bst_singleton::BSTSingletonSDM;
use crate::re::{
    BSFixedString, BSInputDeviceManager, BSTArray, BSTEventSink, BSTEventSource, INPUT_CONTEXT_ID,
    INPUT_DEVICE, PC_GAMEPAD_TYPE, USER_EVENT_FLAG, UserEventEnabled,
};
use crate::version::RUNTIME_SSE_1_6_1130;

pub type ControlMapInputContextID = INPUT_CONTEXT_ID;
pub type ControlMapUEFlag = USER_EVENT_FLAG;

pub const CONTROL_MAP_kInvalid: u32 = u8::MAX as u32;

/// C++ `RE::ControlMap::UserEventMapping`
#[repr(C)]
pub struct ControlMapUserEventMapping {
    pub event_id: BSFixedString,                              // 00
    pub input_key: u16,                                       // 08
    pub modifier: u16,                                        // 0A
    pub index_in_context: i8,                                 // 0C
    pub remappable: bool,                                     // 0D
    pub linked: bool,                                         // 0E
    pub pad0f: u8,                                            // 0F
    pub user_event_group_flag: EnumSet<USER_EVENT_FLAG, u32>, // 10
    pub pad14: u32,                                           // 14
}

const _: () = assert!(core::mem::size_of::<ControlMapUserEventMapping>() == 0x18);

/// Honest partial translation of C++ `RE::ControlMap::InputContext`.
///
/// The only source-backed member is an array of `BSTArray<UserEventMapping>` at offset `0x00`,
/// but the array length is runtime-varying (`4` flat, `10` VR). This helper keeps the real type
/// name and exposes the mappings through accessors instead of faking one universal layout.
#[repr(C)]
pub struct ControlMapInputContext {
    pub device_mappings_flat_prefix:
        [BSTArray<ControlMapUserEventMapping>; INPUT_DEVICE::kFlatTotal as usize], // 00
}

const _: () = assert!(core::mem::size_of::<ControlMapInputContext>() == 0x60);

impl ControlMapInputContext {
    #[inline(always)]
    pub fn num_device_mappings() -> usize {
        if crate::runtime::is_vr() {
            INPUT_DEVICE::total() as usize
        } else {
            INPUT_DEVICE::kFlatTotal as usize
        }
    }

    #[inline(always)]
    pub fn device_mappings(&self, device: INPUT_DEVICE) -> &BSTArray<ControlMapUserEventMapping> {
        let device_index = device as i32;
        assert!(device_index >= 0);
        assert!((device_index as usize) < Self::num_device_mappings());
        unsafe {
            &*((self as *const Self as *const BSTArray<ControlMapUserEventMapping>)
                .add(device_index as usize))
        }
    }

    #[inline(always)]
    pub fn device_mappings_mut(
        &mut self,
        device: INPUT_DEVICE,
    ) -> &mut BSTArray<ControlMapUserEventMapping> {
        let device_index = device as i32;
        assert!(device_index >= 0);
        assert!((device_index as usize) < Self::num_device_mappings());
        unsafe {
            &mut *((self as *mut Self as *mut BSTArray<ControlMapUserEventMapping>)
                .add(device_index as usize))
        }
    }
}

/// C++ `RE::ControlMap::LinkedMapping`
#[repr(C)]
pub struct ControlMapLinkedMapping {
    pub linked_mapping_name: BSFixedString,       // 00
    pub linked_mapping_context: INPUT_CONTEXT_ID, // 08
    pub device: INPUT_DEVICE,                     // 0C
    pub link_from_context: INPUT_CONTEXT_ID,      // 10
    pub pad14: u32,                               // 14
    pub link_from_name: BSFixedString,            // 18
}

const _: () = assert!(core::mem::size_of::<ControlMapLinkedMapping>() == 0x20);

/// C++ `RE::ControlMap::RUNTIME_DATA`
#[repr(C)]
pub struct ControlMapRuntimeData {
    pub linked_mappings: BSTArray<ControlMapLinkedMapping>, // 00
    pub context_priority_stack: BSTArray<INPUT_CONTEXT_ID>, // 18
    pub enabled_controls: EnumSet<USER_EVENT_FLAG, u32>,    // 30
    pub stored_controls: EnumSet<USER_EVENT_FLAG, u32>,     // 34
    pub text_entry_count: i8,                               // 38
    pub ignore_keyboard_mouse: bool,                        // 39
    pub ignore_activate_disabled_events: bool,              // 3A
    pub pad3b: u8,                                          // 3B
    pub game_pad_map_type: EnumSet<PC_GAMEPAD_TYPE, u32>,   // 3C
}

const _: () = assert!(core::mem::size_of::<ControlMapRuntimeData>() == 0x40);

/// Honest common prefix of C++ `RE::ControlMap`.
#[repr(C)]
pub struct ControlMap {
    pub base: BSTSingletonSDM<ControlMap>,              // 00
    pub pad001: [u8; 7],                                // 01
    pub event_source: BSTEventSource<UserEventEnabled>, // 08
}

const _: () = assert!(core::mem::size_of::<ControlMap>() == 0x60);
const _: () = assert!(core::mem::offset_of!(ControlMap, event_source) == 0x08);

core_util::inherit!(ControlMap => BSTEventSource<UserEventEnabled>, event_source);

impl ControlMap {
    pub const kInvalid: u32 = CONTROL_MAP_kInvalid;

    crate::relocation_variable! {
        fn singleton() -> *mut ControlMap => crate::relocation::RelocationID::new(514705, 400863), is_ptr
    }

    crate::runtime_data_accessor! {
        pub fn runtime_data() -> ControlMapRuntimeData {
            version: RUNTIME_SSE_1_6_1130,
            se: 0xE8,
            ae: 0xF0,
            vr: 0x108
        }
    }

    crate::runtime_data_mut_accessor! {
        pub fn runtime_data_mut() -> ControlMapRuntimeData {
            version: RUNTIME_SSE_1_6_1130,
            se: 0xE8,
            ae: 0xF0,
            vr: 0x108
        }
    }

    #[inline(always)]
    pub fn get_singleton() -> *mut ControlMap {
        Self::singleton()
    }

    #[inline(always)]
    pub fn context_count() -> usize {
        INPUT_CONTEXT_ID::kTotal() as usize
    }

    #[inline(always)]
    fn control_maps_base(&self) -> *const *mut ControlMapInputContext {
        unsafe { (self as *const Self as *const u8).add(0x60).cast() }
    }

    #[inline(always)]
    fn control_maps_base_mut(&mut self) -> *mut *mut ControlMapInputContext {
        unsafe { (self as *mut Self as *mut u8).add(0x60).cast() }
    }

    #[inline(always)]
    pub fn input_context(&self, context: INPUT_CONTEXT_ID) -> *mut ControlMapInputContext {
        let index = context.get() as usize;
        assert!(index < Self::context_count());
        unsafe { *self.control_maps_base().add(index) }
    }

    #[inline(always)]
    pub fn input_context_mut(&mut self, context: INPUT_CONTEXT_ID) -> *mut ControlMapInputContext {
        let index = context.get() as usize;
        assert!(index < Self::context_count());
        unsafe { *self.control_maps_base_mut().add(index) }
    }

    crate::relocation_func! {
        pub fn pop_input_context(context: INPUT_CONTEXT_ID) => crate::relocation::RelocationID::new(67244, 68544)
    }

    crate::relocation_func! {
        pub fn push_input_context(context: INPUT_CONTEXT_ID) => crate::relocation::RelocationID::new(67243, 68543)
    }

    #[inline(always)]
    pub fn allow_text_input(&mut self, allow: bool) -> i8 {
        let runtime_data = self.runtime_data_mut();
        if allow {
            if runtime_data.text_entry_count != -1 {
                runtime_data.text_entry_count += 1;
            }
        } else if runtime_data.text_entry_count != 0 {
            runtime_data.text_entry_count -= 1;
        }
        runtime_data.text_entry_count
    }

    #[inline(always)]
    pub fn are_controls_enabled(&self, flags: USER_EVENT_FLAG) -> bool {
        self.runtime_data().enabled_controls.all(flags)
    }

    #[inline(always)]
    pub fn get_game_pad_type(&self) -> PC_GAMEPAD_TYPE {
        unsafe { core::mem::transmute(self.runtime_data().game_pad_map_type.underlying()) }
    }

    #[inline(always)]
    pub fn is_activate_controls_enabled(&self) -> bool {
        self.are_controls_enabled(USER_EVENT_FLAG::kActivate)
    }

    #[inline(always)]
    pub fn is_console_controls_enabled(&self) -> bool {
        self.are_controls_enabled(USER_EVENT_FLAG::kConsole)
    }

    #[inline(always)]
    pub fn is_fighting_controls_enabled(&self) -> bool {
        self.are_controls_enabled(USER_EVENT_FLAG::kFighting)
    }

    #[inline(always)]
    pub fn is_jumping_controls_enabled(&self) -> bool {
        self.are_controls_enabled(USER_EVENT_FLAG::kJumping)
    }

    #[inline(always)]
    pub fn is_looking_controls_enabled(&self) -> bool {
        self.are_controls_enabled(USER_EVENT_FLAG::kLooking)
    }

    #[inline(always)]
    pub fn is_menu_controls_enabled(&self) -> bool {
        self.are_controls_enabled(USER_EVENT_FLAG::kMenu)
    }

    #[inline(always)]
    pub fn is_main_four_controls_enabled(&self) -> bool {
        self.are_controls_enabled(USER_EVENT_FLAG::kMainFour)
    }

    #[inline(always)]
    pub fn is_movement_controls_enabled(&self) -> bool {
        self.are_controls_enabled(USER_EVENT_FLAG::kMovement)
    }

    #[inline(always)]
    pub fn is_pov_switch_controls_enabled(&self) -> bool {
        self.are_controls_enabled(USER_EVENT_FLAG::kPOVSwitch)
    }

    #[inline(always)]
    pub fn is_sneaking_controls_enabled(&self) -> bool {
        self.are_controls_enabled(USER_EVENT_FLAG::kSneaking)
    }

    #[inline(always)]
    pub fn is_vats_controls_enabled(&self) -> bool {
        self.are_controls_enabled(USER_EVENT_FLAG::kVATS)
    }

    #[inline(always)]
    pub fn is_wheel_zoom_controls_enabled(&self) -> bool {
        self.are_controls_enabled(USER_EVENT_FLAG::kWheelZoom)
    }

    pub fn get_button_name_from_user_event(
        &self,
        event_id: &BSFixedString,
        device: INPUT_DEVICE,
        button_name: &mut BSFixedString,
    ) -> bool {
        for index in 0..Self::context_count() {
            let input_context = self.input_context(INPUT_CONTEXT_ID(index as u32));
            if input_context.is_null() {
                continue;
            }

            for mapping in unsafe { (*input_context).device_mappings(device).as_slice() } {
                if mapping.event_id == *event_id {
                    if mapping.input_key == 0xFF {
                        break;
                    }

                    let manager = BSInputDeviceManager::get_singleton();
                    if manager.is_null() {
                        return false;
                    }

                    unsafe {
                        (*manager).get_button_name_from_id(
                            device,
                            mapping.input_key as i32,
                            button_name,
                        );
                    }
                    return true;
                }
            }
        }

        false
    }

    pub fn get_mapped_key(
        &self,
        event_id: &str,
        device: INPUT_DEVICE,
        context: INPUT_CONTEXT_ID,
    ) -> u32 {
        let device_index = device as i32;
        assert!(device_index >= 0);
        assert!((device_index as usize) < ControlMapInputContext::num_device_mappings());
        assert!(context.get() < INPUT_CONTEXT_ID::kTotal());

        let input_context = self.input_context(context);
        if input_context.is_null() {
            return Self::kInvalid;
        }

        let event_id = BSFixedString::from_str(event_id);
        for mapping in unsafe { (*input_context).device_mappings(device).as_slice() } {
            if mapping.event_id == event_id {
                return mapping.input_key as u32;
            }
        }

        Self::kInvalid
    }

    pub fn get_mapping_from_event_name(
        &self,
        event_id: &BSFixedString,
        context: INPUT_CONTEXT_ID,
        device: INPUT_DEVICE,
        mapping_out: &mut ControlMapUserEventMapping,
    ) -> bool {
        let input_context = self.input_context(context);
        if input_context.is_null() {
            return false;
        }

        for mapping in unsafe { (*input_context).device_mappings(device).as_slice() } {
            if mapping.event_id == *event_id {
                *mapping_out = ControlMapUserEventMapping {
                    event_id: mapping.event_id.clone(),
                    input_key: mapping.input_key,
                    modifier: mapping.modifier,
                    index_in_context: mapping.index_in_context,
                    remappable: mapping.remappable,
                    linked: mapping.linked,
                    pad0f: mapping.pad0f,
                    user_event_group_flag: mapping.user_event_group_flag,
                    pad14: mapping.pad14,
                };
                return true;
            }
        }

        false
    }

    pub fn get_user_event_name(
        &self,
        button_id: u32,
        device: INPUT_DEVICE,
        context: INPUT_CONTEXT_ID,
    ) -> Option<&BSFixedString> {
        let device_index = device as i32;
        assert!(device_index >= 0);
        assert!((device_index as usize) < ControlMapInputContext::num_device_mappings());
        assert!(context.get() < INPUT_CONTEXT_ID::kTotal());

        let input_context = self.input_context(context);
        if input_context.is_null() {
            return None;
        }

        let mut found = None;
        for mapping in unsafe { (*input_context).device_mappings(device).as_slice() } {
            if mapping.input_key as u32 == button_id {
                if found.is_some() {
                    return None;
                }
                found = Some(&mapping.event_id);
            }
        }

        found
    }

    #[inline(always)]
    pub fn store_controls(&mut self) {
        let runtime_data = self.runtime_data_mut();
        if runtime_data.stored_controls.underlying() == USER_EVENT_FLAG::kInvalid as u32 {
            runtime_data.stored_controls = runtime_data.enabled_controls;
        }
    }

    #[inline(always)]
    pub fn load_stored_controls(&mut self) {
        let runtime_data = self.runtime_data_mut();
        if runtime_data.stored_controls.underlying() != USER_EVENT_FLAG::kInvalid as u32 {
            runtime_data.enabled_controls = runtime_data.stored_controls;
            runtime_data.stored_controls = EnumSet::from(USER_EVENT_FLAG::kInvalid);
        }
    }

    pub fn toggle_controls(&mut self, flags: USER_EVENT_FLAG, enable: bool, store_state: bool) {
        let runtime_data = self.runtime_data_mut();
        let old_state = runtime_data.enabled_controls;

        if enable {
            runtime_data.enabled_controls.set(flags);
        } else {
            runtime_data.enabled_controls.reset(flags);
        }

        if store_state
            && runtime_data.stored_controls.underlying() != USER_EVENT_FLAG::kInvalid as u32
        {
            if enable {
                runtime_data.stored_controls.set(flags);
            } else {
                runtime_data.stored_controls.reset(flags);
            }
        }

        let event = UserEventEnabled {
            new_user_event_flag: runtime_data.enabled_controls,
            old_user_event_flag: old_state,
        };
        unsafe { self.event_source.send_event(&event) };
    }

    #[inline(always)]
    pub unsafe fn add_user_event_enabled_sink(
        &mut self,
        sink: *mut BSTEventSink<UserEventEnabled>,
    ) {
        unsafe { self.event_source.add_event_sink(sink) }
    }

    #[inline(always)]
    pub unsafe fn remove_user_event_enabled_sink(
        &mut self,
        sink: *mut BSTEventSink<UserEventEnabled>,
    ) {
        unsafe { self.event_source.remove_event_sink(sink) }
    }
}
