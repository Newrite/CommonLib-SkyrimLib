#![allow(non_camel_case_types)]

use bitflags::bitflags;

use crate::offsets::offsets_rtti::RTTI_BSPCOrbisGamepadDevice;
use crate::offsets::offsets_vtable::VTABLE_BSPCOrbisGamepadDevice;
use crate::re::BSPCGamepadDeviceDelegate;
use crate::relocation::{RttiType, VariantID};
use crate::rex::PS4;

/// C++ `RE::BSPCOrbisGamepadDevice::Keys::Key`
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BSPCOrbisGamepadDeviceKey {
    kUp = PS4::SCE_PAD_BUTTON::SCE_PAD_BUTTON_UP as u32,
    kDown = PS4::SCE_PAD_BUTTON::SCE_PAD_BUTTON_DOWN as u32,
    kLeft = PS4::SCE_PAD_BUTTON::SCE_PAD_BUTTON_LEFT as u32,
    kRight = PS4::SCE_PAD_BUTTON::SCE_PAD_BUTTON_RIGHT as u32,
    kPS3_Start = PS4::SCE_PAD_BUTTON::SCE_PAD_BUTTON_OPTIONS as u32,
    kPS3_Back = PS4::SCE_PAD_BUTTON::SCE_PAD_BUTTON_TOUCH_PAD as u32,
    kPS3_L3 = PS4::SCE_PAD_BUTTON::SCE_PAD_BUTTON_L3 as u32,
    kPS3_R3 = PS4::SCE_PAD_BUTTON::SCE_PAD_BUTTON_R3 as u32,
    kPS3_LB = PS4::SCE_PAD_BUTTON::SCE_PAD_BUTTON_L1 as u32,
    kPS3_RB = PS4::SCE_PAD_BUTTON::SCE_PAD_BUTTON_R1 as u32,
    kPS3_A = PS4::SCE_PAD_BUTTON::SCE_PAD_BUTTON_CROSS as u32,
    kPS3_B = PS4::SCE_PAD_BUTTON::SCE_PAD_BUTTON_CIRCLE as u32,
    kPS3_X = PS4::SCE_PAD_BUTTON::SCE_PAD_BUTTON_SQUARE as u32,
    kPS3_Y = PS4::SCE_PAD_BUTTON::SCE_PAD_BUTTON_TRIANGLE as u32,
    kPS4_Share = PS4::SCE_PAD_BUTTON::SCE_PAD_BUTTON_SHARE as u32,
    kPS4_L2 = PS4::SCE_PAD_BUTTON::SCE_PAD_BUTTON_L2 as u32,
    kPS4_R2 = PS4::SCE_PAD_BUTTON::SCE_PAD_BUTTON_R2 as u32,
    kPS4_PSBtn = PS4::SCE_PAD_BUTTON::SCE_PAD_BUTTON_PLAYSTATION as u32,
    kPS3_LT = 0x0009,
    kPS3_RT = 0x000A,
    kPS3_LS = 0x000B,
    kPS3_RS = 0x000C,
}

bitflags! {
    /// C++ `RE::BSPCOrbisGamepadDevice::ButtonState`
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct BSPCOrbisGamepadButtonState: u32 {
        const SHARE = PS4::SCE_PAD_BUTTON::SCE_PAD_BUTTON_SHARE as u32;
        const L3 = PS4::SCE_PAD_BUTTON::SCE_PAD_BUTTON_L3 as u32;
        const R3 = PS4::SCE_PAD_BUTTON::SCE_PAD_BUTTON_R3 as u32;
        const START = PS4::SCE_PAD_BUTTON::SCE_PAD_BUTTON_OPTIONS as u32;
        const UP = PS4::SCE_PAD_BUTTON::SCE_PAD_BUTTON_UP as u32;
        const RIGHT = PS4::SCE_PAD_BUTTON::SCE_PAD_BUTTON_RIGHT as u32;
        const DOWN = PS4::SCE_PAD_BUTTON::SCE_PAD_BUTTON_DOWN as u32;
        const LEFT = PS4::SCE_PAD_BUTTON::SCE_PAD_BUTTON_LEFT as u32;
        const L2BTN = PS4::SCE_PAD_BUTTON::SCE_PAD_BUTTON_L2 as u32;
        const R2BTN = PS4::SCE_PAD_BUTTON::SCE_PAD_BUTTON_R2 as u32;
        const LB = PS4::SCE_PAD_BUTTON::SCE_PAD_BUTTON_L1 as u32;
        const RB = PS4::SCE_PAD_BUTTON::SCE_PAD_BUTTON_R1 as u32;
        const TRIANGLE_Y = PS4::SCE_PAD_BUTTON::SCE_PAD_BUTTON_TRIANGLE as u32;
        const CIRCLE_B = PS4::SCE_PAD_BUTTON::SCE_PAD_BUTTON_CIRCLE as u32;
        const CROSS_A = PS4::SCE_PAD_BUTTON::SCE_PAD_BUTTON_CROSS as u32;
        const SQUARE_X = PS4::SCE_PAD_BUTTON::SCE_PAD_BUTTON_SQUARE as u32;
        const PSBTN = PS4::SCE_PAD_BUTTON::SCE_PAD_BUTTON_PLAYSTATION as u32;
        const TOUCHPAD = PS4::SCE_PAD_BUTTON::SCE_PAD_BUTTON_TOUCH_PAD as u32;
        const INTERCEPTED = PS4::SCE_PAD_BUTTON::SCE_PAD_BUTTON_INTERCEPTED as u32;
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct BSPCOrbisGamepadDeviceVector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

const _: () = assert!(core::mem::size_of::<BSPCOrbisGamepadDeviceVector3>() == 0x0C);

#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct BSPCOrbisGamepadDeviceVector4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

const _: () = assert!(core::mem::size_of::<BSPCOrbisGamepadDeviceVector4>() == 0x10);

#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct BSPCOrbisGamepadDevicePadTouch {
    pub x: u16,
    pub y: u16,
    pub touch_id: u8,
    pub pad05: [u8; 3],
}

const _: () = assert!(core::mem::size_of::<BSPCOrbisGamepadDevicePadTouch>() == 0x08);

#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct BSPCOrbisGamepadDeviceTouchPadData {
    pub touch_num: u8,
    pub pad01: [u8; 3],
    pub pad04: u32,
    pub touch: [BSPCOrbisGamepadDevicePadTouch; 2],
}

const _: () = assert!(core::mem::size_of::<BSPCOrbisGamepadDeviceTouchPadData>() == 0x18);

#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct BSPCOrbisGamepadDeviceUnusedExtensionData {
    pub ext_unit_id: u32,
    pub pad04: u8,
    pub data_length: u8,
    pub data: [u8; 10],
}

const _: () = assert!(core::mem::size_of::<BSPCOrbisGamepadDeviceUnusedExtensionData>() == 0x10);

#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct BSPCOrbisGamepadDeviceGamepadData {
    pub button_state: u32,
    pub raw_left_stick_x: u8,
    pub raw_left_stick_y: u8,
    pub raw_right_stick_x: u8,
    pub raw_right_stick_y: u8,
    pub raw_left_trigger: u8,
    pub raw_right_trigger: u8,
    pub pad0a: [u8; 2],
    pub orientation: BSPCOrbisGamepadDeviceVector4,
    pub acceleration: BSPCOrbisGamepadDeviceVector3,
    pub angular_velocity: BSPCOrbisGamepadDeviceVector3,
    pub touch_pad_data: BSPCOrbisGamepadDeviceTouchPadData,
    pub pad_connected: bool,
    pub pad4d: [u8; 3],
    pub timestamp: u64,
    pub unused_ext_data: BSPCOrbisGamepadDeviceUnusedExtensionData,
    pub connected_count: u8,
    pub pad69: [u8; 2],
    pub special_data_len: u8,
    pub special_data: [u8; 12],
}

const _: () = assert!(core::mem::size_of::<BSPCOrbisGamepadDeviceGamepadData>() == 0x78);

/// C++ `RE::BSPCOrbisGamepadDevice`
#[repr(C)]
pub struct BSPCOrbisGamepadDevice {
    pub base: BSPCGamepadDeviceDelegate,                       // 000
    pub previous_pad_state: BSPCOrbisGamepadDeviceGamepadData, // 0D8
    pub previous_lt: f32,                                      // 150
    pub previous_rt: f32,                                      // 154
    pub previous_lx: f32,                                      // 158
    pub previous_ly: f32,                                      // 15C
    pub previous_rx: f32,                                      // 160
    pub previous_ry: f32,                                      // 164
    pub current_pad_state: BSPCOrbisGamepadDeviceGamepadData,  // 168
    pub current_lt: f32,                                       // 1E0
    pub current_rt: f32,                                       // 1E4
    pub current_lx: f32,                                       // 1E8
    pub current_ly: f32,                                       // 1EC
    pub current_rx: f32,                                       // 1F0
    pub current_ry: f32,                                       // 1F4
}

const _: () = assert!(core::mem::size_of::<BSPCOrbisGamepadDevice>() == 0x1F8);
const _: () = assert!(core::mem::offset_of!(BSPCOrbisGamepadDevice, previous_pad_state) == 0x0D8);
const _: () = assert!(core::mem::offset_of!(BSPCOrbisGamepadDevice, current_pad_state) == 0x168);

impl RttiType for BSPCOrbisGamepadDevice {
    const RTTI: VariantID = RTTI_BSPCOrbisGamepadDevice;
}

core_util::inherit!(BSPCOrbisGamepadDevice : BSPCGamepadDeviceDelegate, base);

impl BSPCOrbisGamepadDevice {
    pub const RTTI: VariantID = RTTI_BSPCOrbisGamepadDevice;
    pub const VTABLE: &'static [VariantID] = &VTABLE_BSPCOrbisGamepadDevice;

    #[inline(always)]
    pub fn previous_button_state(&self) -> BSPCOrbisGamepadButtonState {
        BSPCOrbisGamepadButtonState::from_bits_retain(self.previous_pad_state.button_state)
    }

    #[inline(always)]
    pub fn current_button_state(&self) -> BSPCOrbisGamepadButtonState {
        BSPCOrbisGamepadButtonState::from_bits_retain(self.current_pad_state.button_state)
    }
}
