//! Translation of `RE::InputDevices.h`.

#![allow(non_camel_case_types, non_upper_case_globals)]

/// C++ `RE::INPUT_DEVICES`
pub struct INPUT_DEVICES;

/// C++ `RE::INPUT_DEVICES::INPUT_DEVICE`
#[libskyrim_macros::open_enum]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum INPUT_DEVICE {
    kNone = -1,
    kKeyboard = 0,
    kMouse = 1,
    kGamepad = 2,
    kFlatVirtualKeyboard = 3,
    kFlatTotal = 4,
    kOculusPrimary = 5,
    kOculusSecondary = 6,
    kWMRPrimary = 7,
    kWMRSecondary = 8,
    kVRVirtualKeyboard = 9,
    kVRTotal = 10,
}

const _: () = assert!(core::mem::size_of::<INPUT_DEVICE>() == 0x4);

core_util::impl_enumset_type!(INPUT_DEVICE => u32);

impl INPUT_DEVICES {
    #[inline(always)]
    pub fn virtual_keyboard() -> INPUT_DEVICE {
        if crate::runtime::is_vr() {
            INPUT_DEVICE::kVRVirtualKeyboard
        } else {
            INPUT_DEVICE::kFlatVirtualKeyboard
        }
    }

    #[inline(always)]
    pub fn total() -> u32 {
        if crate::runtime::is_vr() {
            INPUT_DEVICE::kVRTotal as u32
        } else {
            INPUT_DEVICE::kFlatTotal as u32
        }
    }
}

impl INPUT_DEVICE {
    pub const kVivePrimary: Self = Self::kFlatVirtualKeyboard;
    pub const kViveSecondary: Self = Self::kFlatTotal;

    #[inline(always)]
    pub fn virtual_keyboard() -> Self {
        INPUT_DEVICES::virtual_keyboard()
    }

    #[inline(always)]
    pub fn total() -> u32 {
        INPUT_DEVICES::total()
    }
}
