use alloc::string::String;

use crate::re::{ControlMap, PC_GAMEPAD_TYPE};
use crate::rex::{PS4, W32};
use windows_sys::Win32::Foundation::MAX_PATH;
use windows_sys::Win32::UI::Input::KeyboardAndMouse::GetKeyNameTextW;

pub const MACRO_KEYBOARD_OFFSET: u32 = 0;
pub const NUM_KEYBOARD_KEYS: u32 = 256;

pub const MACRO_MOUSE_BUTTON_OFFSET: u32 = NUM_KEYBOARD_KEYS;
pub const NUM_MOUSE_BUTTONS: u32 = 8;

pub const MACRO_MOUSE_WHEEL_OFFSET: u32 = MACRO_MOUSE_BUTTON_OFFSET + NUM_MOUSE_BUTTONS;
pub const MOUSE_WHEEL_DIRECTIONS: u32 = 2;

pub const MACRO_GAMEPAD_OFFSET: u32 = MACRO_MOUSE_WHEEL_OFFSET + MOUSE_WHEEL_DIRECTIONS;
pub const NUM_GAMEPAD_BUTTONS: u32 = 16;

pub const MAX_MACROS: u32 = MACRO_GAMEPAD_OFFSET + NUM_GAMEPAD_BUTTONS;

pub const GAMEPAD_BUTTON_OFFSET_DPAD_UP: u32 = MACRO_GAMEPAD_OFFSET;
pub const GAMEPAD_BUTTON_OFFSET_DPAD_DOWN: u32 = GAMEPAD_BUTTON_OFFSET_DPAD_UP + 1;
pub const GAMEPAD_BUTTON_OFFSET_DPAD_LEFT: u32 = GAMEPAD_BUTTON_OFFSET_DPAD_UP + 2;
pub const GAMEPAD_BUTTON_OFFSET_DPAD_RIGHT: u32 = GAMEPAD_BUTTON_OFFSET_DPAD_UP + 3;
pub const GAMEPAD_BUTTON_OFFSET_START: u32 = GAMEPAD_BUTTON_OFFSET_DPAD_UP + 4;
pub const GAMEPAD_BUTTON_OFFSET_BACK: u32 = GAMEPAD_BUTTON_OFFSET_DPAD_UP + 5;
pub const GAMEPAD_BUTTON_OFFSET_LEFT_THUMB: u32 = GAMEPAD_BUTTON_OFFSET_DPAD_UP + 6;
pub const GAMEPAD_BUTTON_OFFSET_RIGHT_THUMB: u32 = GAMEPAD_BUTTON_OFFSET_DPAD_UP + 7;
pub const GAMEPAD_BUTTON_OFFSET_LEFT_SHOULDER: u32 = GAMEPAD_BUTTON_OFFSET_DPAD_UP + 8;
pub const GAMEPAD_BUTTON_OFFSET_RIGHT_SHOULDER: u32 = GAMEPAD_BUTTON_OFFSET_DPAD_UP + 9;
pub const GAMEPAD_BUTTON_OFFSET_A: u32 = GAMEPAD_BUTTON_OFFSET_DPAD_UP + 10;
pub const GAMEPAD_BUTTON_OFFSET_B: u32 = GAMEPAD_BUTTON_OFFSET_DPAD_UP + 11;
pub const GAMEPAD_BUTTON_OFFSET_X: u32 = GAMEPAD_BUTTON_OFFSET_DPAD_UP + 12;
pub const GAMEPAD_BUTTON_OFFSET_Y: u32 = GAMEPAD_BUTTON_OFFSET_DPAD_UP + 13;
pub const GAMEPAD_BUTTON_OFFSET_LT: u32 = GAMEPAD_BUTTON_OFFSET_DPAD_UP + 14;
pub const GAMEPAD_BUTTON_OFFSET_RT: u32 = GAMEPAD_BUTTON_OFFSET_DPAD_UP + 15;

const LEFT_TRIGGER_MASK: u32 = 0x9;
const RIGHT_TRIGGER_MASK: u32 = 0xA;
const INVALID_GAMEPAD_MASK: u32 = 0xFF;

#[inline(always)]
pub fn xinput_to_sce_pad_offset(key_mask: u32) -> u32 {
    match key_mask as u16 {
        W32::XINPUT_GAMEPAD_DPAD_UP => PS4::SCE_PAD_BUTTON::SCE_PAD_BUTTON_UP as u32,
        W32::XINPUT_GAMEPAD_DPAD_DOWN => PS4::SCE_PAD_BUTTON::SCE_PAD_BUTTON_DOWN as u32,
        W32::XINPUT_GAMEPAD_DPAD_LEFT => PS4::SCE_PAD_BUTTON::SCE_PAD_BUTTON_LEFT as u32,
        W32::XINPUT_GAMEPAD_DPAD_RIGHT => PS4::SCE_PAD_BUTTON::SCE_PAD_BUTTON_RIGHT as u32,
        W32::XINPUT_GAMEPAD_START => PS4::SCE_PAD_BUTTON::SCE_PAD_BUTTON_OPTIONS as u32,
        W32::XINPUT_GAMEPAD_BACK => PS4::SCE_PAD_BUTTON::SCE_PAD_BUTTON_TOUCH_PAD as u32,
        W32::XINPUT_GAMEPAD_LEFT_THUMB => PS4::SCE_PAD_BUTTON::SCE_PAD_BUTTON_L3 as u32,
        W32::XINPUT_GAMEPAD_RIGHT_THUMB => PS4::SCE_PAD_BUTTON::SCE_PAD_BUTTON_R3 as u32,
        W32::XINPUT_GAMEPAD_LEFT_SHOULDER => PS4::SCE_PAD_BUTTON::SCE_PAD_BUTTON_L1 as u32,
        W32::XINPUT_GAMEPAD_RIGHT_SHOULDER => PS4::SCE_PAD_BUTTON::SCE_PAD_BUTTON_R1 as u32,
        W32::XINPUT_GAMEPAD_A => PS4::SCE_PAD_BUTTON::SCE_PAD_BUTTON_CROSS as u32,
        W32::XINPUT_GAMEPAD_B => PS4::SCE_PAD_BUTTON::SCE_PAD_BUTTON_CIRCLE as u32,
        W32::XINPUT_GAMEPAD_X => PS4::SCE_PAD_BUTTON::SCE_PAD_BUTTON_SQUARE as u32,
        W32::XINPUT_GAMEPAD_Y => PS4::SCE_PAD_BUTTON::SCE_PAD_BUTTON_TRIANGLE as u32,
        _ => key_mask,
    }
}

#[inline(always)]
pub fn sce_pad_offset_to_xinput(key_mask: u32) -> u32 {
    match key_mask {
        x if x == PS4::SCE_PAD_BUTTON::SCE_PAD_BUTTON_UP as u32 => {
            W32::XINPUT_GAMEPAD_DPAD_UP as u32
        }
        x if x == PS4::SCE_PAD_BUTTON::SCE_PAD_BUTTON_DOWN as u32 => {
            W32::XINPUT_GAMEPAD_DPAD_DOWN as u32
        }
        x if x == PS4::SCE_PAD_BUTTON::SCE_PAD_BUTTON_LEFT as u32 => {
            W32::XINPUT_GAMEPAD_DPAD_LEFT as u32
        }
        x if x == PS4::SCE_PAD_BUTTON::SCE_PAD_BUTTON_RIGHT as u32 => {
            W32::XINPUT_GAMEPAD_DPAD_RIGHT as u32
        }
        x if x == PS4::SCE_PAD_BUTTON::SCE_PAD_BUTTON_OPTIONS as u32 => {
            W32::XINPUT_GAMEPAD_START as u32
        }
        x if x == PS4::SCE_PAD_BUTTON::SCE_PAD_BUTTON_TOUCH_PAD as u32 => {
            W32::XINPUT_GAMEPAD_BACK as u32
        }
        x if x == PS4::SCE_PAD_BUTTON::SCE_PAD_BUTTON_L3 as u32 => {
            W32::XINPUT_GAMEPAD_LEFT_THUMB as u32
        }
        x if x == PS4::SCE_PAD_BUTTON::SCE_PAD_BUTTON_R3 as u32 => {
            W32::XINPUT_GAMEPAD_RIGHT_THUMB as u32
        }
        x if x == PS4::SCE_PAD_BUTTON::SCE_PAD_BUTTON_L1 as u32 => {
            W32::XINPUT_GAMEPAD_LEFT_SHOULDER as u32
        }
        x if x == PS4::SCE_PAD_BUTTON::SCE_PAD_BUTTON_R1 as u32 => {
            W32::XINPUT_GAMEPAD_RIGHT_SHOULDER as u32
        }
        x if x == PS4::SCE_PAD_BUTTON::SCE_PAD_BUTTON_CROSS as u32 => W32::XINPUT_GAMEPAD_A as u32,
        x if x == PS4::SCE_PAD_BUTTON::SCE_PAD_BUTTON_CIRCLE as u32 => W32::XINPUT_GAMEPAD_B as u32,
        x if x == PS4::SCE_PAD_BUTTON::SCE_PAD_BUTTON_SQUARE as u32 => W32::XINPUT_GAMEPAD_X as u32,
        x if x == PS4::SCE_PAD_BUTTON::SCE_PAD_BUTTON_TRIANGLE as u32 => {
            W32::XINPUT_GAMEPAD_Y as u32
        }
        _ => key_mask,
    }
}

#[inline(always)]
pub fn gamepad_mask_to_keycode(mut key_mask: u32) -> u32 {
    let control_map = ControlMap::get_singleton();
    if let Some(control_map) = unsafe { control_map.as_ref() } {
        if control_map.get_game_pad_type() == PC_GAMEPAD_TYPE::kOrbis {
            key_mask = sce_pad_offset_to_xinput(key_mask);
        }
    }

    match key_mask as u16 {
        W32::XINPUT_GAMEPAD_DPAD_UP => GAMEPAD_BUTTON_OFFSET_DPAD_UP,
        W32::XINPUT_GAMEPAD_DPAD_DOWN => GAMEPAD_BUTTON_OFFSET_DPAD_DOWN,
        W32::XINPUT_GAMEPAD_DPAD_LEFT => GAMEPAD_BUTTON_OFFSET_DPAD_LEFT,
        W32::XINPUT_GAMEPAD_DPAD_RIGHT => GAMEPAD_BUTTON_OFFSET_DPAD_RIGHT,
        W32::XINPUT_GAMEPAD_START => GAMEPAD_BUTTON_OFFSET_START,
        W32::XINPUT_GAMEPAD_BACK => GAMEPAD_BUTTON_OFFSET_BACK,
        W32::XINPUT_GAMEPAD_LEFT_THUMB => GAMEPAD_BUTTON_OFFSET_LEFT_THUMB,
        W32::XINPUT_GAMEPAD_RIGHT_THUMB => GAMEPAD_BUTTON_OFFSET_RIGHT_THUMB,
        W32::XINPUT_GAMEPAD_LEFT_SHOULDER => GAMEPAD_BUTTON_OFFSET_LEFT_SHOULDER,
        W32::XINPUT_GAMEPAD_RIGHT_SHOULDER => GAMEPAD_BUTTON_OFFSET_RIGHT_SHOULDER,
        W32::XINPUT_GAMEPAD_A => GAMEPAD_BUTTON_OFFSET_A,
        W32::XINPUT_GAMEPAD_B => GAMEPAD_BUTTON_OFFSET_B,
        W32::XINPUT_GAMEPAD_X => GAMEPAD_BUTTON_OFFSET_X,
        W32::XINPUT_GAMEPAD_Y => GAMEPAD_BUTTON_OFFSET_Y,
        _ if key_mask == LEFT_TRIGGER_MASK => GAMEPAD_BUTTON_OFFSET_LT,
        _ if key_mask == RIGHT_TRIGGER_MASK => GAMEPAD_BUTTON_OFFSET_RT,
        _ => MAX_MACROS,
    }
}

#[inline(always)]
pub fn gamepad_keycode_to_mask(key_code: u32) -> u32 {
    let mut key_mask = match key_code {
        GAMEPAD_BUTTON_OFFSET_DPAD_UP => W32::XINPUT_GAMEPAD_DPAD_UP as u32,
        GAMEPAD_BUTTON_OFFSET_DPAD_DOWN => W32::XINPUT_GAMEPAD_DPAD_DOWN as u32,
        GAMEPAD_BUTTON_OFFSET_DPAD_LEFT => W32::XINPUT_GAMEPAD_DPAD_LEFT as u32,
        GAMEPAD_BUTTON_OFFSET_DPAD_RIGHT => W32::XINPUT_GAMEPAD_DPAD_RIGHT as u32,
        GAMEPAD_BUTTON_OFFSET_START => W32::XINPUT_GAMEPAD_START as u32,
        GAMEPAD_BUTTON_OFFSET_BACK => W32::XINPUT_GAMEPAD_BACK as u32,
        GAMEPAD_BUTTON_OFFSET_LEFT_THUMB => W32::XINPUT_GAMEPAD_LEFT_THUMB as u32,
        GAMEPAD_BUTTON_OFFSET_RIGHT_THUMB => W32::XINPUT_GAMEPAD_RIGHT_THUMB as u32,
        GAMEPAD_BUTTON_OFFSET_LEFT_SHOULDER => W32::XINPUT_GAMEPAD_LEFT_SHOULDER as u32,
        GAMEPAD_BUTTON_OFFSET_RIGHT_SHOULDER => W32::XINPUT_GAMEPAD_RIGHT_SHOULDER as u32,
        GAMEPAD_BUTTON_OFFSET_A => W32::XINPUT_GAMEPAD_A as u32,
        GAMEPAD_BUTTON_OFFSET_B => W32::XINPUT_GAMEPAD_B as u32,
        GAMEPAD_BUTTON_OFFSET_X => W32::XINPUT_GAMEPAD_X as u32,
        GAMEPAD_BUTTON_OFFSET_Y => W32::XINPUT_GAMEPAD_Y as u32,
        GAMEPAD_BUTTON_OFFSET_LT => LEFT_TRIGGER_MASK,
        GAMEPAD_BUTTON_OFFSET_RT => RIGHT_TRIGGER_MASK,
        _ => INVALID_GAMEPAD_MASK,
    };

    let control_map = ControlMap::get_singleton();
    if let Some(control_map) = unsafe { control_map.as_ref() } {
        if control_map.get_game_pad_type() == PC_GAMEPAD_TYPE::kOrbis {
            key_mask = xinput_to_sce_pad_offset(key_mask);
        }
    }

    key_mask
}

#[inline(always)]
pub fn get_key_name(key_code: u32) -> String {
    if (MACRO_MOUSE_BUTTON_OFFSET..MACRO_GAMEPAD_OFFSET).contains(&key_code) {
        get_mouse_button_name(key_code)
    } else if (MACRO_GAMEPAD_OFFSET..MAX_MACROS).contains(&key_code) {
        get_gamepad_button_name(key_code)
    } else {
        get_keyboard_key_name(key_code)
    }
}

pub fn get_keyboard_key_name(key_code: u32) -> String {
    let mut scancode = (key_code & 0xFF) as i32;

    match key_code {
        x if x == W32::DIK::DIK_NUMPADENTER as u32 => scancode = 0x11C,
        x if x == W32::DIK::DIK_RCONTROL as u32 => scancode = 0x11D,
        x if x == W32::DIK::DIK_DIVIDE as u32 => scancode = 0x135,
        x if x == W32::DIK::DIK_RMENU as u32 => scancode = 0x138,
        x if x == W32::DIK::DIK_HOME as u32 => scancode = 0x147,
        x if x == W32::DIK::DIK_UP as u32 => scancode = 0x148,
        x if x == W32::DIK::DIK_PRIOR as u32 => scancode = 0x149,
        x if x == W32::DIK::DIK_LEFT as u32 => scancode = 0x14B,
        x if x == W32::DIK::DIK_RIGHT as u32 => scancode = 0x14D,
        x if x == W32::DIK::DIK_END as u32 => scancode = 0x14F,
        x if x == W32::DIK::DIK_DOWN as u32 => scancode = 0x150,
        x if x == W32::DIK::DIK_NEXT as u32 => scancode = 0x151,
        x if x == W32::DIK::DIK_INSERT as u32 => scancode = 0x152,
        x if x == W32::DIK::DIK_DELETE as u32 => scancode = 0x153,
        _ => {}
    }

    let mut l_param = scancode << 16;
    if scancode == 0x45 {
        l_param |= 1 << 24;
    }

    let mut buffer = [0u16; MAX_PATH as usize];
    let length = unsafe { GetKeyNameTextW(l_param, buffer.as_mut_ptr(), MAX_PATH as i32) };
    if length <= 0 {
        return String::new();
    }

    String::from_utf16_lossy(&buffer[..length as usize])
}

#[inline(always)]
pub fn get_mouse_button_name(key_code: u32) -> String {
    match key_code {
        256 => String::from("Left Mouse Button"),
        257 => String::from("Right Mouse Button"),
        258 => String::from("Middle Mouse Button"),
        259 => String::from("Mouse Button 3"),
        260 => String::from("Mouse Button 4"),
        261 => String::from("Mouse Button 5"),
        262 => String::from("Mouse Button 6"),
        263 => String::from("Mouse Button 7"),
        264 => String::from("Mouse Wheel Up"),
        265 => String::from("Mouse Wheel Down"),
        _ => String::new(),
    }
}

#[inline(always)]
pub fn get_gamepad_button_name(key_code: u32) -> String {
    match key_code {
        GAMEPAD_BUTTON_OFFSET_DPAD_UP => String::from("Gamepad DPad Up"),
        GAMEPAD_BUTTON_OFFSET_DPAD_DOWN => String::from("Gamepad DPad Down"),
        GAMEPAD_BUTTON_OFFSET_DPAD_LEFT => String::from("Gamepad DPad Left"),
        GAMEPAD_BUTTON_OFFSET_DPAD_RIGHT => String::from("Gamepad DPad Right"),
        GAMEPAD_BUTTON_OFFSET_START => String::from("Gamepad Start"),
        GAMEPAD_BUTTON_OFFSET_BACK => String::from("Gamepad Back"),
        GAMEPAD_BUTTON_OFFSET_LEFT_THUMB => String::from("Gamepad Left Thumb"),
        GAMEPAD_BUTTON_OFFSET_RIGHT_THUMB => String::from("Gamepad Right Thumb"),
        GAMEPAD_BUTTON_OFFSET_LEFT_SHOULDER => String::from("Gamepad Left Shoulder"),
        GAMEPAD_BUTTON_OFFSET_RIGHT_SHOULDER => String::from("Gamepad Right Shoulder"),
        GAMEPAD_BUTTON_OFFSET_A => String::from("Gamepad A"),
        GAMEPAD_BUTTON_OFFSET_B => String::from("Gamepad B"),
        GAMEPAD_BUTTON_OFFSET_X => String::from("Gamepad X"),
        GAMEPAD_BUTTON_OFFSET_Y => String::from("Gamepad Y"),
        GAMEPAD_BUTTON_OFFSET_LT => String::from("Gamepad LT"),
        GAMEPAD_BUTTON_OFFSET_RT => String::from("Gamepad RT"),
        _ => String::new(),
    }
}
