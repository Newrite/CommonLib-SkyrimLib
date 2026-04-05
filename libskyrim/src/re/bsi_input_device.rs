use crate::offsets::offsets_rtti::RTTI_BSIInputDevice;
use crate::offsets::offsets_vtable::VTABLE_BSIInputDevice;
use crate::re::BSFixedString;
use crate::relocation::{RttiType, VariantID};

/// C++ `RE::BSIInputDevice`
#[repr(C)]
pub struct BSIInputDevice {
    pub vtable: *const usize, // 00
}

const _: () = assert!(core::mem::size_of::<BSIInputDevice>() == 0x08);
const _: () = assert!(core::mem::offset_of!(BSIInputDevice, vtable) == 0x00);

impl RttiType for BSIInputDevice {
    const RTTI: VariantID = RTTI_BSIInputDevice;
}

impl BSIInputDevice {
    pub const RTTI: VariantID = RTTI_BSIInputDevice;
    pub const VTABLE: &'static [VariantID] = &VTABLE_BSIInputDevice;

    crate::virtual_method! {
        pub const VFUNC_DTOR: usize = 0x00;
        pub fn dtor(&mut self)
    }

    crate::virtual_method! {
        pub const VFUNC_INITIALIZE: usize = 0x01;
        pub fn initialize(&mut self)
    }

    crate::virtual_method! {
        pub const VFUNC_POLL: usize = 0x02;
        pub fn poll(&mut self, time_delta: f32)
    }

    crate::virtual_method! {
        pub const VFUNC_SHUTDOWN: usize = 0x03;
        pub fn shutdown(&mut self)
    }

    crate::virtual_method! {
        pub const VFUNC_GET_BUTTON_NAME_FROM_ID: usize = 0x04;
        pub fn get_button_name_from_id(&mut self, id: i32, button_name: &mut BSFixedString) -> bool
    }

    crate::virtual_method! {
        pub const VFUNC_GET_MAPPING_KEY: usize = 0x05;
        fn get_mapping_key_impl(&mut self, mapping: &BSFixedString) -> u32
    }

    crate::virtual_method! {
        pub const VFUNC_GET_KEY_CODE_FROM_ID: usize = 0x06;
        pub fn get_key_code_from_id(&mut self, id: i32, key_code: &mut u32) -> bool
    }

    crate::virtual_method! {
        pub const VFUNC_IS_ENABLED: usize = 0x07;
        pub fn is_enabled() -> bool
    }

    crate::virtual_method! {
        pub const VFUNC_CLEAR_INPUT_STATE: usize = 0x08;
        pub fn clear_input_state(&mut self)
    }

    #[inline(always)]
    pub fn try_get_button_name_from_id(&mut self, id: i32) -> Option<BSFixedString> {
        let mut button_name = BSFixedString::empty();
        if self.get_button_name_from_id(id, &mut button_name) {
            Some(button_name)
        } else {
            None
        }
    }

    #[inline(always)]
    pub fn get_mapping_key(&mut self, mapping: BSFixedString) -> u32 {
        self.get_mapping_key_ref(&mapping)
    }

    #[inline(always)]
    pub fn get_mapping_key_ref(&mut self, mapping: &BSFixedString) -> u32 {
        self.get_mapping_key_impl(mapping)
    }

    #[inline(always)]
    pub fn get_mapping_key_from_str(&mut self, mapping: &str) -> u32 {
        let mapping = BSFixedString::from_str(mapping);
        self.get_mapping_key_ref(&mapping)
    }

    #[inline(always)]
    pub fn try_get_key_code_from_id(&mut self, id: i32) -> Option<u32> {
        let mut key_code = 0;
        if self.get_key_code_from_id(id, &mut key_code) {
            Some(key_code)
        } else {
            None
        }
    }
}

impl AsRef<BSIInputDevice> for BSIInputDevice {
    #[inline(always)]
    fn as_ref(&self) -> &BSIInputDevice {
        self
    }
}

impl AsMut<BSIInputDevice> for BSIInputDevice {
    #[inline(always)]
    fn as_mut(&mut self) -> &mut BSIInputDevice {
        self
    }
}

pub trait BSIInputDeviceExt {
    fn dtor(&mut self);
    fn initialize(&mut self);
    fn poll(&mut self, time_delta: f32);
    fn shutdown(&mut self);
    fn get_button_name_from_id(&mut self, id: i32, button_name: &mut BSFixedString) -> bool;
    fn try_get_button_name_from_id(&mut self, id: i32) -> Option<BSFixedString>;
    fn get_mapping_key(&mut self, mapping: BSFixedString) -> u32;
    fn get_mapping_key_ref(&mut self, mapping: &BSFixedString) -> u32;
    fn get_mapping_key_from_str(&mut self, mapping: &str) -> u32;
    fn get_key_code_from_id(&mut self, id: i32, key_code: &mut u32) -> bool;
    fn try_get_key_code_from_id(&mut self, id: i32) -> Option<u32>;
    fn is_enabled(&self) -> bool;
    fn clear_input_state(&mut self);
}

impl<T> BSIInputDeviceExt for T
where
    T: AsRef<BSIInputDevice> + AsMut<BSIInputDevice>,
{
    #[inline(always)]
    fn dtor(&mut self) {
        BSIInputDevice::dtor(self.as_mut())
    }

    #[inline(always)]
    fn initialize(&mut self) {
        BSIInputDevice::initialize(self.as_mut())
    }

    #[inline(always)]
    fn poll(&mut self, time_delta: f32) {
        BSIInputDevice::poll(self.as_mut(), time_delta)
    }

    #[inline(always)]
    fn shutdown(&mut self) {
        BSIInputDevice::shutdown(self.as_mut())
    }

    #[inline(always)]
    fn get_button_name_from_id(&mut self, id: i32, button_name: &mut BSFixedString) -> bool {
        BSIInputDevice::get_button_name_from_id(self.as_mut(), id, button_name)
    }

    #[inline(always)]
    fn try_get_button_name_from_id(&mut self, id: i32) -> Option<BSFixedString> {
        BSIInputDevice::try_get_button_name_from_id(self.as_mut(), id)
    }

    #[inline(always)]
    fn get_mapping_key(&mut self, mapping: BSFixedString) -> u32 {
        BSIInputDevice::get_mapping_key(self.as_mut(), mapping)
    }

    #[inline(always)]
    fn get_mapping_key_ref(&mut self, mapping: &BSFixedString) -> u32 {
        BSIInputDevice::get_mapping_key_ref(self.as_mut(), mapping)
    }

    #[inline(always)]
    fn get_mapping_key_from_str(&mut self, mapping: &str) -> u32 {
        BSIInputDevice::get_mapping_key_from_str(self.as_mut(), mapping)
    }

    #[inline(always)]
    fn get_key_code_from_id(&mut self, id: i32, key_code: &mut u32) -> bool {
        BSIInputDevice::get_key_code_from_id(self.as_mut(), id, key_code)
    }

    #[inline(always)]
    fn try_get_key_code_from_id(&mut self, id: i32) -> Option<u32> {
        BSIInputDevice::try_get_key_code_from_id(self.as_mut(), id)
    }

    #[inline(always)]
    fn is_enabled(&self) -> bool {
        BSIInputDevice::is_enabled(self.as_ref())
    }

    #[inline(always)]
    fn clear_input_state(&mut self) {
        BSIInputDevice::clear_input_state(self.as_mut())
    }
}
