use core::ffi::c_void;

use crate::offsets::offsets_rtti::RTTI_BSVirtualKeyboardDevice;
use crate::offsets::offsets_vtable::VTABLE_BSVirtualKeyboardDevice;
use crate::re::BSKeyboardDevice;
use crate::relocation::{RttiType, VariantID};

pub type BSVirtualKeyboardDoneCallback = unsafe extern "C" fn(*mut c_void, *const i8);
pub type BSVirtualKeyboardCancelCallback = unsafe extern "C" fn();

/// C++ `RE::BSVirtualKeyboardDevice::kbInfo`
#[repr(C)]
pub struct BSVirtualKeyboardInfo {
    pub starting_text: *const i8,                                 // 00
    pub done_callback: Option<BSVirtualKeyboardDoneCallback>,     // 08
    pub cancel_callback: Option<BSVirtualKeyboardCancelCallback>, // 10
    pub user_param: *mut c_void,                                  // 18
    pub max_chars: u32,                                           // 20
}

const _: () = assert!(core::mem::size_of::<BSVirtualKeyboardInfo>() == 0x28);

/// C++ `RE::BSVirtualKeyboardDevice`
#[repr(C)]
pub struct BSVirtualKeyboardDevice {
    pub base: BSKeyboardDevice, // 00
}

const _: () = assert!(core::mem::size_of::<BSVirtualKeyboardDevice>() == 0x70);
const _: () = assert!(core::mem::offset_of!(BSVirtualKeyboardDevice, base) == 0x00);

impl RttiType for BSVirtualKeyboardDevice {
    const RTTI: VariantID = RTTI_BSVirtualKeyboardDevice;
}

core_util::inherit!(BSVirtualKeyboardDevice : BSKeyboardDevice, base);

impl BSVirtualKeyboardDevice {
    pub const RTTI: VariantID = RTTI_BSVirtualKeyboardDevice;
    pub const VTABLE: &'static [VariantID] = &VTABLE_BSVirtualKeyboardDevice;

    crate::virtual_method! {
        pub const VFUNC_START: usize = 0x0B;
        pub fn start(&mut self, info: *const BSVirtualKeyboardInfo)
    }

    crate::virtual_method! {
        pub const VFUNC_STOP: usize = 0x0C;
        pub fn stop(&mut self)
    }

    crate::virtual_method! {
        pub const VFUNC_USER_CANCELLED: usize = 0x0D;
        pub fn user_cancelled(&mut self)
    }
}

pub trait BSVirtualKeyboardDeviceExt {
    fn start(&mut self, info: *const BSVirtualKeyboardInfo);
    fn stop(&mut self);
    fn user_cancelled(&mut self);
}

impl<T> BSVirtualKeyboardDeviceExt for T
where
    T: AsRef<BSVirtualKeyboardDevice> + AsMut<BSVirtualKeyboardDevice>,
{
    #[inline(always)]
    fn start(&mut self, info: *const BSVirtualKeyboardInfo) {
        BSVirtualKeyboardDevice::start(self.as_mut(), info)
    }

    #[inline(always)]
    fn stop(&mut self) {
        BSVirtualKeyboardDevice::stop(self.as_mut())
    }

    #[inline(always)]
    fn user_cancelled(&mut self) {
        BSVirtualKeyboardDevice::user_cancelled(self.as_mut())
    }
}
