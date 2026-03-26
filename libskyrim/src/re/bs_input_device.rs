#![allow(non_camel_case_types)]

use alloc::collections::BTreeMap;
use alloc::string::String;
use alloc::vec::Vec;
use core::ops::{Index, IndexMut};

use crate::offsets::offsets_rtti::RTTI_BSInputDevice;
use crate::offsets::offsets_vtable::VTABLE_BSInputDevice;
use crate::re::{BSFixedString, BSIInputDevice, BSTHashMap, INPUT_DEVICE};
use crate::relocation::{RelocationID, RttiType, VariantID};

/// C++ `RE::BSInputDevice::InputButton`
#[repr(C)]
pub struct InputButton {
    pub name: BSFixedString, // 00
    pub held_down_secs: f32, // 08
    pub keycode: u32,        // 0C
}

const _: () = assert!(core::mem::size_of::<InputButton>() == 0x10);

/// C++ `RE::ControllerRole`
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ControllerRole {
    Primary = 0,
    Secondary = 1,
    Count = 2,
}

impl ControllerRole {
    pub const COUNT: usize = Self::Count as usize;
}

/// C++ `RE::ButtonState`
#[derive(Debug, Clone, Copy, Default)]
pub struct ButtonState {
    pub is_pressed: bool,
    pub last_press_time: f64,
    pub last_release_time: f64,
    pub hold_duration: f64,
}

impl ButtonState {
    #[inline(always)]
    pub fn on_event(&mut self, pressed: bool, event_time: f64) {
        if pressed && !self.is_pressed {
            self.is_pressed = true;
            self.last_press_time = event_time;
        } else if !pressed && self.is_pressed {
            self.is_pressed = false;
            self.last_release_time = event_time;
            self.hold_duration = self.last_release_time - self.last_press_time;
        }
    }

    #[inline(always)]
    pub fn is_click(&self, threshold: f64) -> bool {
        !self.is_pressed && self.hold_duration < threshold
    }

    #[inline(always)]
    pub fn is_hold(&self, threshold: f64) -> bool {
        !self.is_pressed && self.hold_duration >= threshold
    }

    #[inline(always)]
    pub fn current_held_time(&self, now: f64) -> f64 {
        if self.is_pressed {
            now - self.last_press_time
        } else {
            self.hold_duration
        }
    }
}

/// C++ `RE::ThumbstickState`
#[derive(Debug, Clone, Copy, Default)]
pub struct ThumbstickState {
    pub x: f32,
    pub y: f32,
}

impl ThumbstickState {
    #[inline(always)]
    pub fn on_event(&mut self, new_x: f32, new_y: f32) {
        self.x = new_x;
        self.y = new_y;
    }
}

/// C++ `RE::TriggerState`
#[derive(Debug, Clone, Copy, Default)]
pub struct TriggerState {
    pub value: f32,
}

impl TriggerState {
    #[inline(always)]
    pub fn on_event(&mut self, new_value: f32) {
        self.value = new_value;
    }
}

/// C++ `RE::InputDeviceState`
#[derive(Debug, Clone)]
pub struct InputDeviceState {
    pub device_type: INPUT_DEVICE,
    pub buttons: [ButtonState; Self::BUTTON_ARRAY_SIZE],
    pub thumbsticks: [ThumbstickState; ControllerRole::COUNT],
    pub triggers: [TriggerState; ControllerRole::COUNT],
    pub custom_data: BTreeMap<String, f64>,
}

impl Default for InputDeviceState {
    fn default() -> Self {
        Self {
            device_type: INPUT_DEVICE::kNone,
            buttons: [ButtonState::default(); Self::BUTTON_ARRAY_SIZE],
            thumbsticks: [ThumbstickState::default(); ControllerRole::COUNT],
            triggers: [TriggerState::default(); ControllerRole::COUNT],
            custom_data: BTreeMap::new(),
        }
    }
}

impl InputDeviceState {
    pub const BUTTON_ARRAY_SIZE: usize = 256;

    #[inline(always)]
    pub fn get_active_buttons(&self) -> Vec<(u32, &ButtonState)> {
        let mut result = Vec::new();
        for (i, state) in self.buttons.iter().enumerate() {
            if state.is_pressed || state.last_press_time > 0.0 {
                result.push((i as u32, state));
            }
        }
        result
    }

    #[inline(always)]
    pub fn clear(&mut self) {
        self.buttons.fill(ButtonState::default());
        self.thumbsticks.fill(ThumbstickState::default());
        self.triggers.fill(TriggerState::default());
    }

    #[inline(always)]
    pub fn active_button_count(&self) -> usize {
        self.buttons
            .iter()
            .filter(|button| button.is_pressed || button.last_press_time > 0.0)
            .count()
    }
}

impl Index<u32> for InputDeviceState {
    type Output = ButtonState;

    #[inline(always)]
    fn index(&self, index: u32) -> &Self::Output {
        &self.buttons[usize::min(index as usize, Self::BUTTON_ARRAY_SIZE - 1)]
    }
}

impl IndexMut<u32> for InputDeviceState {
    #[inline(always)]
    fn index_mut(&mut self, index: u32) -> &mut Self::Output {
        &mut self.buttons[usize::min(index as usize, Self::BUTTON_ARRAY_SIZE - 1)]
    }
}

pub type VRControllerState = InputDeviceState;
pub type GamepadState = InputDeviceState;
pub type KeyboardState = InputDeviceState;
pub type MouseState = InputDeviceState;

#[inline(always)]
pub fn get_quadrant_name(x: f32, y: f32) -> &'static str {
    if x > 0.0 && y > 0.0 {
        "Top-Right"
    } else if x < 0.0 && y > 0.0 {
        "Top-Left"
    } else if x < 0.0 && y < 0.0 {
        "Bottom-Left"
    } else if x > 0.0 && y < 0.0 {
        "Bottom-Right"
    } else if x == 0.0 && y == 0.0 {
        "Center"
    } else if y == 0.0 {
        if x > 0.0 { "Right" } else { "Left" }
    } else if x == 0.0 {
        if y > 0.0 { "Top" } else { "Bottom" }
    } else {
        ""
    }
}

/// C++ `RE::ButtonMapping`
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ButtonMapping {
    pub key_code: u32,
    pub logical_button: i32,
    pub is_key_event: bool,
    pub key: i32,
    pub is_shift: bool,
}

/// C++ `RE::BSInputDevice`
#[repr(C)]
pub struct BSInputDevice {
    pub base: BSIInputDevice,                               // 00
    pub device: INPUT_DEVICE,                               // 08
    pub pad0c: u32,                                         // 0C
    pub device_buttons: BSTHashMap<u32, *mut InputButton>,  // 10
    pub button_name_id_map: BSTHashMap<BSFixedString, u32>, // 40
}

const _: () = assert!(core::mem::size_of::<BSInputDevice>() == 0x70);
const _: () = assert!(core::mem::offset_of!(BSInputDevice, base) == 0x00);
const _: () = assert!(core::mem::offset_of!(BSInputDevice, device) == 0x08);
const _: () = assert!(core::mem::offset_of!(BSInputDevice, device_buttons) == 0x10);
const _: () = assert!(core::mem::offset_of!(BSInputDevice, button_name_id_map) == 0x40);

impl RttiType for BSInputDevice {
    const RTTI: VariantID = RTTI_BSInputDevice;
}

core_util::inherit!(BSInputDevice : BSIInputDevice, base);

impl BSInputDevice {
    pub const RTTI: VariantID = RTTI_BSInputDevice;
    pub const VTABLE: &'static [VariantID] = &VTABLE_BSInputDevice;

    crate::relocation_func! {
        pub fn load_controls_definition_file(&mut self, file_name: *const i8) -> bool => RelocationID::new(67438, 68745)
    }

    crate::relocation_func! {
        pub fn set_button_state(
            &mut self,
            button_id: u32,
            time_since_last_poll: f32,
            button_was_pressed: bool,
            button_is_pressed: bool
        ) => RelocationID::new(67441, 68748)
    }

    #[inline(always)]
    pub fn is_keyboard(&self) -> bool {
        self.device == INPUT_DEVICE::kKeyboard
    }

    #[inline(always)]
    pub fn is_mouse(&self) -> bool {
        self.device == INPUT_DEVICE::kMouse
    }

    #[inline(always)]
    pub fn is_gamepad(&self) -> bool {
        self.device == INPUT_DEVICE::kGamepad
    }

    #[inline(always)]
    pub fn is_pressed(&self, key_code: u32) -> bool {
        let button = self.device_buttons.find(&key_code);
        if button.is_null() {
            false
        } else {
            let button = unsafe { (*button).second };
            !button.is_null() && unsafe { (*button).held_down_secs > 0.0 }
        }
    }

    #[inline(always)]
    pub fn reset_button_maps(&mut self) {
        unsafe {
            self.button_name_id_map.clear();
            self.device_buttons.clear();
        }
    }
}

pub trait BSInputDeviceExt {
    fn load_controls_definition_file(&mut self, file_name: *const i8) -> bool;
    fn set_button_state(
        &mut self,
        button_id: u32,
        time_since_last_poll: f32,
        button_was_pressed: bool,
        button_is_pressed: bool,
    );
    fn is_keyboard(&self) -> bool;
    fn is_mouse(&self) -> bool;
    fn is_gamepad(&self) -> bool;
    fn is_pressed(&self, key_code: u32) -> bool;
    fn reset_button_maps(&mut self);
}

impl<T> BSInputDeviceExt for T
where
    T: AsRef<BSInputDevice> + AsMut<BSInputDevice>,
{
    #[inline(always)]
    fn load_controls_definition_file(&mut self, file_name: *const i8) -> bool {
        BSInputDevice::load_controls_definition_file(self.as_mut(), file_name)
    }

    #[inline(always)]
    fn set_button_state(
        &mut self,
        button_id: u32,
        time_since_last_poll: f32,
        button_was_pressed: bool,
        button_is_pressed: bool,
    ) {
        BSInputDevice::set_button_state(
            self.as_mut(),
            button_id,
            time_since_last_poll,
            button_was_pressed,
            button_is_pressed,
        )
    }

    #[inline(always)]
    fn is_keyboard(&self) -> bool {
        BSInputDevice::is_keyboard(self.as_ref())
    }

    #[inline(always)]
    fn is_mouse(&self) -> bool {
        BSInputDevice::is_mouse(self.as_ref())
    }

    #[inline(always)]
    fn is_gamepad(&self) -> bool {
        BSInputDevice::is_gamepad(self.as_ref())
    }

    #[inline(always)]
    fn is_pressed(&self, key_code: u32) -> bool {
        BSInputDevice::is_pressed(self.as_ref(), key_code)
    }

    #[inline(always)]
    fn reset_button_maps(&mut self) {
        BSInputDevice::reset_button_maps(self.as_mut())
    }
}
