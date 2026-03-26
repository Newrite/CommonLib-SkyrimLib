#![allow(non_camel_case_types)]

use bitflags::bitflags;

use crate::offsets::offsets_rtti::RTTI_BSWin32GamepadDevice;
use crate::offsets::offsets_vtable::VTABLE_BSWin32GamepadDevice;
use crate::re::BSPCGamepadDeviceDelegate;
use crate::relocation::{RttiType, VariantID};
use crate::rex::W32;

/// C++ `RE::BSWin32GamepadDevice::Keys::Key`
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BSWin32GamepadDeviceKey {
    kUp = W32::XINPUT_GAMEPAD_DPAD_UP as u32,
    kDown = W32::XINPUT_GAMEPAD_DPAD_DOWN as u32,
    kLeft = W32::XINPUT_GAMEPAD_DPAD_LEFT as u32,
    kRight = W32::XINPUT_GAMEPAD_DPAD_RIGHT as u32,
    kStart = W32::XINPUT_GAMEPAD_START as u32,
    kBack = W32::XINPUT_GAMEPAD_BACK as u32,
    kLeftThumb = W32::XINPUT_GAMEPAD_LEFT_THUMB as u32,
    kRightThumb = W32::XINPUT_GAMEPAD_RIGHT_THUMB as u32,
    kLeftShoulder = W32::XINPUT_GAMEPAD_LEFT_SHOULDER as u32,
    kRightShoulder = W32::XINPUT_GAMEPAD_RIGHT_SHOULDER as u32,
    kA = W32::XINPUT_GAMEPAD_A as u32,
    kB = W32::XINPUT_GAMEPAD_B as u32,
    kX = W32::XINPUT_GAMEPAD_X as u32,
    kY = W32::XINPUT_GAMEPAD_Y as u32,
    kLeftTrigger = 0x0009,
    kRightTrigger = 0x000A,
    kLeftStick = 0x000B,
    kRightStick = 0x000C,
}

bitflags! {
    /// C++ `RE::BSWin32GamepadDevice::ButtonState`
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct BSWin32GamepadButtonState: u16 {
        const UP = W32::XINPUT_GAMEPAD_DPAD_UP;
        const DOWN = W32::XINPUT_GAMEPAD_DPAD_DOWN;
        const LEFT = W32::XINPUT_GAMEPAD_DPAD_LEFT;
        const RIGHT = W32::XINPUT_GAMEPAD_DPAD_RIGHT;
        const START = W32::XINPUT_GAMEPAD_START;
        const BACK = W32::XINPUT_GAMEPAD_BACK;
        const LEFT_THUMB = W32::XINPUT_GAMEPAD_LEFT_THUMB;
        const RIGHT_THUMB = W32::XINPUT_GAMEPAD_RIGHT_THUMB;
        const LEFT_SHOULDER = W32::XINPUT_GAMEPAD_LEFT_SHOULDER;
        const RIGHT_SHOULDER = W32::XINPUT_GAMEPAD_RIGHT_SHOULDER;
        const A = W32::XINPUT_GAMEPAD_A;
        const B = W32::XINPUT_GAMEPAD_B;
        const X = W32::XINPUT_GAMEPAD_X;
        const Y = W32::XINPUT_GAMEPAD_Y;
    }
}

/// C++ `RE::BSWin32GamepadDevice`
#[repr(C)]
pub struct BSWin32GamepadDevice {
    pub base: BSPCGamepadDeviceDelegate,   // 000
    pub previous_state: W32::XINPUT_STATE, // 0D8
    pub previous_lt: f32,                  // 0E8
    pub previous_rt: f32,                  // 0EC
    pub previous_lx: f32,                  // 0F0
    pub previous_ly: f32,                  // 0F4
    pub previous_rx: f32,                  // 0F8
    pub previous_ry: f32,                  // 0FC
    pub current_state: W32::XINPUT_STATE,  // 100
    pub current_lt: f32,                   // 110
    pub current_rt: f32,                   // 114
    pub current_lx: f32,                   // 118
    pub current_ly: f32,                   // 11C
    pub current_rx: f32,                   // 120
    pub current_ry: f32,                   // 124
}

const _: () = assert!(core::mem::size_of::<BSWin32GamepadDevice>() == 0x128);
const _: () = assert!(core::mem::offset_of!(BSWin32GamepadDevice, previous_state) == 0x0D8);
const _: () = assert!(core::mem::offset_of!(BSWin32GamepadDevice, current_state) == 0x100);

impl RttiType for BSWin32GamepadDevice {
    const RTTI: VariantID = RTTI_BSWin32GamepadDevice;
}

core_util::inherit!(BSWin32GamepadDevice : BSPCGamepadDeviceDelegate, base);

impl BSWin32GamepadDevice {
    pub const RTTI: VariantID = RTTI_BSWin32GamepadDevice;
    pub const VTABLE: &'static [VariantID] = &VTABLE_BSWin32GamepadDevice;

    #[inline(always)]
    pub fn previous_button_state(&self) -> BSWin32GamepadButtonState {
        BSWin32GamepadButtonState::from_bits_retain(
            self.previous_state.gamepad.buttons & W32::XINPUT_GAMEPAD_BUTTON_MASK,
        )
    }

    #[inline(always)]
    pub fn current_button_state(&self) -> BSWin32GamepadButtonState {
        BSWin32GamepadButtonState::from_bits_retain(
            self.current_state.gamepad.buttons & W32::XINPUT_GAMEPAD_BUTTON_MASK,
        )
    }
}
