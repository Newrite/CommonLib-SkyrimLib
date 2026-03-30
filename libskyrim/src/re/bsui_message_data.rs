use alloc::ffi::CString;
use core::ffi::c_void;

use core_util::inherit;

use crate::offsets::offsets_rtti::RTTI_BSUIMessageData;
use crate::offsets::offsets_vtable::VTABLE_BSUIMessageData;
use crate::re::{BSFixedString, BSString, IUIMessageData, UIMessageQueue, UIMessageType};
use crate::relocation::{RttiType, VariantID};

/// C++ `RE::BSUIMessageData::Data`
#[repr(C)]
#[derive(Clone, Copy)]
pub union BSUIMessageDataData {
    pub b: bool,
    pub u: u32,
    pub f: f32,
    pub p: *mut c_void,
}

const _: () = assert!(core::mem::size_of::<BSUIMessageDataData>() == 0x8);

impl Default for BSUIMessageDataData {
    #[inline(always)]
    fn default() -> Self {
        Self {
            p: core::ptr::null_mut(),
        }
    }
}

/// C++ `RE::BSUIMessageData`
#[repr(C)]
pub struct BSUIMessageData {
    pub base: IUIMessageData,      // 00
    pub str_: *mut BSString,       // 10
    pub fixed_str: BSFixedString,  // 18
    pub data: BSUIMessageDataData, // 20
}

const _: () = assert!(core::mem::size_of::<BSUIMessageData>() == 0x28);
const _: () = assert!(core::mem::offset_of!(BSUIMessageData, base) == 0x00);
const _: () = assert!(core::mem::offset_of!(BSUIMessageData, str_) == 0x10);
const _: () = assert!(core::mem::offset_of!(BSUIMessageData, fixed_str) == 0x18);
const _: () = assert!(core::mem::offset_of!(BSUIMessageData, data) == 0x20);

inherit!(BSUIMessageData : IUIMessageData);

impl RttiType for BSUIMessageData {
    const RTTI: VariantID = RTTI_BSUIMessageData;
}

impl BSUIMessageData {
    pub const RTTI: VariantID = RTTI_BSUIMessageData;
    pub const VTABLE: &'static [VariantID] = &VTABLE_BSUIMessageData;
    pub const CLASS_NAME: &'static str = "BSUIMessageData";

    #[inline(always)]
    fn create_message_data() -> *mut Self {
        let Ok(class_name) = CString::new(Self::CLASS_NAME) else {
            return core::ptr::null_mut();
        };

        unsafe { crate::ffi::commonlib_create_ui_message_data(class_name.as_ptr()).cast() }
    }

    #[inline(always)]
    fn send_ui_message_with<F>(menu: &BSFixedString, type_: UIMessageType, init: F)
    where
        F: FnOnce(&mut Self),
    {
        let Some(ui_message_queue) = (unsafe { UIMessageQueue::get_singleton().as_mut() }) else {
            return;
        };

        let msg_data_ptr = Self::create_message_data();
        let data_ptr = {
            let Some(msg_data) = (unsafe { msg_data_ptr.as_mut() }) else {
                return;
            };
            init(msg_data);
            msg_data as *mut Self as *mut IUIMessageData
        };

        ui_message_queue.add_message(menu, type_, data_ptr);
    }

    #[inline(always)]
    pub fn send_ui_bool_message(menu: &BSFixedString, type_: UIMessageType, data: bool) {
        Self::send_ui_message_with(menu, type_, |msg_data| {
            msg_data.data = BSUIMessageDataData { b: data };
        });
    }

    #[inline(always)]
    pub fn send_ui_message(menu: &BSFixedString, type_: UIMessageType, data: u32) {
        Self::send_ui_message_with(menu, type_, |msg_data| {
            msg_data.data = BSUIMessageDataData { u: data };
        });
    }

    #[inline(always)]
    pub fn send_ui_ptr_message(menu: &BSFixedString, type_: UIMessageType, data: *mut c_void) {
        Self::send_ui_message_with(menu, type_, |msg_data| {
            msg_data.data = BSUIMessageDataData { p: data };
        });
    }

    #[inline(always)]
    pub fn send_ui_string_bool_message(
        menu: &BSFixedString,
        type_: UIMessageType,
        str_: &BSFixedString,
        data: bool,
    ) {
        Self::send_ui_message_with(menu, type_, |msg_data| {
            msg_data.fixed_str = str_.clone();
            msg_data.data = BSUIMessageDataData { b: data };
        });
    }

    #[inline(always)]
    pub fn send_ui_string_float_message(
        menu: &BSFixedString,
        type_: UIMessageType,
        str_: &BSFixedString,
        data: f32,
    ) {
        Self::send_ui_message_with(menu, type_, |msg_data| {
            msg_data.fixed_str = str_.clone();
            msg_data.data = BSUIMessageDataData { f: data };
        });
    }

    #[inline(always)]
    pub fn send_ui_string_message(
        menu: &BSFixedString,
        type_: UIMessageType,
        str_: &BSFixedString,
    ) {
        Self::send_ui_message_with(menu, type_, |msg_data| {
            msg_data.fixed_str = str_.clone();
        });
    }

    #[inline(always)]
    pub fn send_ui_string_uint_message(
        menu: &BSFixedString,
        type_: UIMessageType,
        str_: &BSFixedString,
        data: u32,
    ) {
        Self::send_ui_message_with(menu, type_, |msg_data| {
            msg_data.fixed_str = str_.clone();
            msg_data.data = BSUIMessageDataData { u: data };
        });
    }

    // override (IUIMessageData)
    // ~BSUIMessageData() override; // 00
}
