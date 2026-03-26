#![allow(non_camel_case_types)]

use crate::offsets::offsets_rtti::RTTI_BSWin32MouseDevice;
use crate::offsets::offsets_vtable::VTABLE_BSWin32MouseDevice;
use crate::re::{BSMouseDevice, BSSpinLock};
use crate::relocation::{RttiType, VariantID};
use crate::rex::W32;

/// C++ `RE::BSWin32MouseDevice::Keys::Key`
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BSWin32MouseDeviceKey {
    kLeftButton = 0,
    kRightButton = 1,
    kMiddleButton = 2,
    kButton3 = 3,
    kButton4 = 4,
    kButton5 = 5,
    kButton6 = 6,
    kButton7 = 7,
    kWheelUp = 8,
    kWheelDown = 9,
}

/// C++ `RE::BSWin32MouseDevice`
#[repr(C)]
pub struct BSWin32MouseDevice {
    pub base: BSMouseDevice,                            // 00
    pub d_input_device: *mut W32::IDirectInputDevice8A, // 78
    pub d_input_prev_state: W32::DIMOUSESTATE2,         // 80
    pub d_input_next_state: W32::DIMOUSESTATE2,         // 94
    pub not_initialized: bool,                          // A8
    pub reinitialize_lock: BSSpinLock,                  // AC
}

const _: () = assert!(core::mem::size_of::<BSWin32MouseDevice>() == 0xB8);
const _: () = assert!(core::mem::offset_of!(BSWin32MouseDevice, d_input_device) == 0x78);
const _: () = assert!(core::mem::offset_of!(BSWin32MouseDevice, d_input_prev_state) == 0x80);
const _: () = assert!(core::mem::offset_of!(BSWin32MouseDevice, d_input_next_state) == 0x94);
const _: () = assert!(core::mem::offset_of!(BSWin32MouseDevice, not_initialized) == 0xA8);
const _: () = assert!(core::mem::offset_of!(BSWin32MouseDevice, reinitialize_lock) == 0xAC);

impl RttiType for BSWin32MouseDevice {
    const RTTI: VariantID = RTTI_BSWin32MouseDevice;
}

core_util::inherit!(BSWin32MouseDevice : BSMouseDevice, base);

impl BSWin32MouseDevice {
    pub const RTTI: VariantID = RTTI_BSWin32MouseDevice;
    pub const VTABLE: &'static [VariantID] = &VTABLE_BSWin32MouseDevice;
}
