use core::ptr;

use crate::re::{BSTrackedControllerDevice, ControllerDeviceHand, INPUT_DEVICE};
use crate::relocation::{Offset, Relocation, RttiType, VariantID};

/// C++ `RE::BSOpenVRControllerDevice::Keys::Key`
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BSOpenVRControllerDeviceKey {
    kBY = 1,
    kGrip = 2,
    kXA = 7,
    kJoystickTrigger = 32,
    kTrigger = 33,
    kGripAlt = 34,
    kTouchpadClick = 35,
    kTouchpadAlt = 36,
}

const _: () = assert!(core::mem::size_of::<BSOpenVRControllerDeviceKey>() == 0x4);

/// C++ `RE::BSOpenVRControllerDevice`
#[repr(C)]
pub struct BSOpenVRControllerDevice {
    pub base: BSTrackedControllerDevice, // 000
    pub unk80: [u64; 0x16],              // 080
    pub unk130: u32,                     // 130
    pub unk134: u32,                     // 134
    pub unk138: u32,                     // 138
    pub unk13c: u32,                     // 13C
    pub unk140: u32,                     // 140
    pub unk144: u32,                     // 144
    pub unk148: u32,                     // 148
    pub unk14c: u32,                     // 14C
}

const _: () = assert!(core::mem::size_of::<BSOpenVRControllerDevice>() == 0x150);
const _: () = assert!(core::mem::offset_of!(BSOpenVRControllerDevice, base) == 0x000);
const _: () = assert!(core::mem::offset_of!(BSOpenVRControllerDevice, unk80) == 0x080);
const _: () = assert!(core::mem::offset_of!(BSOpenVRControllerDevice, unk130) == 0x130);
const _: () = assert!(core::mem::offset_of!(BSOpenVRControllerDevice, unk14c) == 0x14C);

impl RttiType for BSOpenVRControllerDevice {
    const RTTI: VariantID = VariantID::new(0, 0, 0x1F5FF28);
}

core_util::inherit!(BSOpenVRControllerDevice : BSTrackedControllerDevice, base);

impl BSOpenVRControllerDevice {
    pub const RTTI: VariantID = VariantID::new(0, 0, 0x1F5FF28);
    pub const VTABLE: &'static [VariantID] = &[VariantID::new(0, 0, 0x17E87C0)];

    #[inline(always)]
    fn left_handed_mode_ptr() -> *mut bool {
        if !crate::runtime::is_vr() {
            return ptr::null_mut();
        }
        unsafe {
            Relocation::<*mut bool>::try_new(Offset::new(0x1E71778))
                .ok()
                .map(|relocation| relocation.get())
                .unwrap_or(ptr::null_mut())
        }
    }

    #[inline(always)]
    pub fn is_grip_button(key_code: u32) -> bool {
        key_code == BSOpenVRControllerDeviceKey::kGrip as u32
            || key_code == BSOpenVRControllerDeviceKey::kGripAlt as u32
    }

    #[inline(always)]
    pub fn is_trigger_button(key_code: u32) -> bool {
        key_code == BSOpenVRControllerDeviceKey::kTrigger as u32
    }

    #[inline(always)]
    pub fn is_stick_click(key_code: u32) -> bool {
        key_code == BSOpenVRControllerDeviceKey::kJoystickTrigger as u32
    }

    #[inline(always)]
    pub fn is_a_button(key_code: u32) -> bool {
        key_code == BSOpenVRControllerDeviceKey::kXA as u32
    }

    #[inline(always)]
    pub fn is_b_button(key_code: u32) -> bool {
        key_code == BSOpenVRControllerDeviceKey::kBY as u32
    }

    #[inline(always)]
    pub fn is_x_button(key_code: u32) -> bool {
        key_code == BSOpenVRControllerDeviceKey::kXA as u32
    }

    #[inline(always)]
    pub fn is_y_button(key_code: u32) -> bool {
        key_code == BSOpenVRControllerDeviceKey::kBY as u32
    }

    #[inline(always)]
    pub fn is_touchpad_click(key_code: u32) -> bool {
        key_code == BSOpenVRControllerDeviceKey::kTouchpadClick as u32
            || key_code == BSOpenVRControllerDeviceKey::kTouchpadAlt as u32
    }

    #[inline(always)]
    pub fn is_left_handed_mode() -> bool {
        let left_handed_mode = Self::left_handed_mode_ptr();
        if left_handed_mode.is_null() {
            false
        } else {
            unsafe { *left_handed_mode }
        }
    }

    #[inline(always)]
    pub fn is_primary_controller(device: INPUT_DEVICE) -> bool {
        let is_phys_primary = device == INPUT_DEVICE::kVivePrimary
            || device == INPUT_DEVICE::kOculusPrimary
            || device == INPUT_DEVICE::kWMRPrimary;
        if Self::is_left_handed_mode() {
            !is_phys_primary
        } else {
            is_phys_primary
        }
    }

    #[inline(always)]
    pub fn is_secondary_controller(device: INPUT_DEVICE) -> bool {
        let is_phys_secondary = device == INPUT_DEVICE::kViveSecondary
            || device == INPUT_DEVICE::kOculusSecondary
            || device == INPUT_DEVICE::kWMRSecondary;
        if Self::is_left_handed_mode() {
            !is_phys_secondary
        } else {
            is_phys_secondary
        }
    }

    #[inline(always)]
    pub fn is_left_hand(&self) -> bool {
        self.base.hand == ControllerDeviceHand::kLeft
    }

    #[inline(always)]
    pub fn is_right_hand(&self) -> bool {
        self.base.hand == ControllerDeviceHand::kRight
    }
}

#[inline(always)]
pub fn get_openvr_button_name(key_code: u32) -> &'static str {
    match key_code {
        x if x == BSOpenVRControllerDeviceKey::kTrigger as u32 => "Trigger",
        x if x == BSOpenVRControllerDeviceKey::kGrip as u32 => "Grip",
        x if x == BSOpenVRControllerDeviceKey::kGripAlt as u32 => "Grip Alt",
        x if x == BSOpenVRControllerDeviceKey::kJoystickTrigger as u32 => "Joystick Click",
        x if x == BSOpenVRControllerDeviceKey::kTouchpadClick as u32 => "Touchpad Click",
        x if x == BSOpenVRControllerDeviceKey::kTouchpadAlt as u32 => "Touchpad Alt",
        x if x == BSOpenVRControllerDeviceKey::kXA as u32 => "A/X",
        x if x == BSOpenVRControllerDeviceKey::kBY as u32 => "B/Y",
        _ => {
            if BSOpenVRControllerDevice::is_trigger_button(key_code) {
                "Trigger"
            } else if BSOpenVRControllerDevice::is_grip_button(key_code) {
                "Grip"
            } else if BSOpenVRControllerDevice::is_stick_click(key_code) {
                "Joystick Click"
            } else if BSOpenVRControllerDevice::is_touchpad_click(key_code) {
                "Touchpad Click"
            } else if BSOpenVRControllerDevice::is_a_button(key_code) {
                "A"
            } else if BSOpenVRControllerDevice::is_b_button(key_code) {
                "B"
            } else if BSOpenVRControllerDevice::is_x_button(key_code) {
                "X"
            } else if BSOpenVRControllerDevice::is_y_button(key_code) {
                "Y"
            } else {
                "?"
            }
        }
    }
}
