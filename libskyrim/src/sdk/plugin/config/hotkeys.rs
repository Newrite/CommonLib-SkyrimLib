use alloc::string::String;
use alloc::vec::Vec;
use core::fmt;

use crate::re::{BSInputDeviceManager, BSWin32KeyboardDevice, BSWin32MouseDevice, ButtonEvent};
use crate::rex::W32;
use crate::sdk::events::input::InputEvents;
use crate::skse::input_map;

/// Failure to parse one user-facing hotkey spec.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HotkeyParseError {
    Empty,
    EmptyToken,
    DuplicateKey,
    TooManyKeys,
    UnknownKey,
}

/// Parsed config hotkey combo.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HotkeyCombo {
    keys: Vec<u32>,
}

impl HotkeyCombo {
    pub const MAX_KEYS: usize = 8;

    #[inline(always)]
    pub fn new(keys: Vec<u32>) -> Result<Self, HotkeyParseError> {
        if keys.is_empty() {
            return Err(HotkeyParseError::Empty);
        }
        if keys.len() > Self::MAX_KEYS {
            return Err(HotkeyParseError::TooManyKeys);
        }

        let mut dedup = Vec::with_capacity(keys.len());
        for key in keys {
            if dedup.contains(&key) {
                return Err(HotkeyParseError::DuplicateKey);
            }
            dedup.push(key);
        }

        Ok(Self { keys: dedup })
    }

    #[inline]
    pub fn parse(spec: &str) -> Result<Self, HotkeyParseError> {
        let spec = spec.trim();
        if spec.is_empty() {
            return Err(HotkeyParseError::Empty);
        }

        let mut keys = Vec::new();
        for token in spec.split('+') {
            let token = token.trim();
            if token.is_empty() {
                return Err(HotkeyParseError::EmptyToken);
            }

            let key = parse_hotkey_key(token).ok_or(HotkeyParseError::UnknownKey)?;
            if keys.contains(&key) {
                return Err(HotkeyParseError::DuplicateKey);
            }
            keys.push(key);
            if keys.len() > Self::MAX_KEYS {
                return Err(HotkeyParseError::TooManyKeys);
            }
        }

        Ok(Self { keys })
    }

    #[inline(always)]
    pub fn keys(&self) -> &[u32] {
        &self.keys
    }

    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        self.keys.is_empty()
    }

    #[inline(always)]
    pub fn len(&self) -> usize {
        self.keys.len()
    }

    #[inline(always)]
    pub fn primary_key(&self) -> Option<u32> {
        self.keys.last().copied()
    }

    #[inline(always)]
    pub fn modifiers(&self) -> &[u32] {
        let split = self.keys.len().saturating_sub(1);
        &self.keys[..split]
    }

    #[inline(always)]
    pub fn contains_key(&self, key_code: u32) -> bool {
        self.keys.contains(&key_code)
    }

    #[inline]
    pub fn display_names(&self) -> Vec<String> {
        self.keys
            .iter()
            .map(|&key| input_map::get_key_name(key))
            .collect()
    }

    #[inline]
    pub fn display_string(&self) -> String {
        let mut parts = self.display_names().into_iter();
        let mut text = parts.next().unwrap_or_default();
        for part in parts {
            text.push_str(" + ");
            text.push_str(&part);
        }
        text
    }

    #[inline]
    pub fn currently_pressed(&self) -> bool {
        self.keys.iter().copied().all(key_is_currently_pressed)
    }

    #[inline]
    pub fn just_pressed_in(&self, events: &InputEvents<'_>) -> bool {
        let Some(primary) = self.primary_key() else {
            return false;
        };

        key_is_pressed_in_events(events, primary)
            && self
                .modifiers()
                .iter()
                .copied()
                .all(key_is_currently_pressed)
    }

    #[inline]
    pub fn matches_events(&self, events: &InputEvents<'_>) -> bool {
        self.currently_pressed()
            && self
                .keys
                .iter()
                .copied()
                .any(|key| key_is_pressed_in_events(events, key))
    }
}

impl Default for HotkeyCombo {
    #[inline(always)]
    fn default() -> Self {
        Self { keys: Vec::new() }
    }
}

impl fmt::Display for HotkeyCombo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.display_string())
    }
}

#[inline]
pub(super) fn parse_hotkey_key(token: &str) -> Option<u32> {
    let token = token.trim();
    if token.is_empty() {
        return None;
    }

    if token.len() == 1 {
        return parse_single_char_key(token.as_bytes()[0]);
    }

    if let Some(value) = parse_numeric_key_code(token) {
        return Some(value);
    }

    let normalized = normalize_hotkey_token(token);
    match normalized.as_str() {
        "escape" | "esc" => Some(W32::DIK::DIK_ESCAPE as u32),
        "tab" => Some(W32::DIK::DIK_TAB as u32),
        "enter" | "return" => Some(W32::DIK::DIK_RETURN as u32),
        "space" | "spacebar" => Some(W32::DIK::DIK_SPACE as u32),
        "backspace" | "back" => Some(W32::DIK::DIK_BACK as u32),
        "delete" | "del" => Some(W32::DIK::DIK_DELETE as u32),
        "insert" | "ins" => Some(W32::DIK::DIK_INSERT as u32),
        "home" => Some(W32::DIK::DIK_HOME as u32),
        "end" => Some(W32::DIK::DIK_END as u32),
        "pageup" | "pgup" => Some(W32::DIK::DIK_PRIOR as u32),
        "pagedown" | "pgdown" => Some(W32::DIK::DIK_NEXT as u32),
        "up" | "uparrow" => Some(W32::DIK::DIK_UP as u32),
        "down" | "downarrow" => Some(W32::DIK::DIK_DOWN as u32),
        "left" | "leftarrow" => Some(W32::DIK::DIK_LEFT as u32),
        "right" | "rightarrow" => Some(W32::DIK::DIK_RIGHT as u32),
        "shift" | "leftshift" | "lshift" => Some(W32::DIK::DIK_LSHIFT as u32),
        "rightshift" | "rshift" => Some(W32::DIK::DIK_RSHIFT as u32),
        "ctrl" | "control" | "leftctrl" | "leftcontrol" | "lctrl" | "lcontrol" => {
            Some(W32::DIK::DIK_LCONTROL as u32)
        }
        "rightctrl" | "rightcontrol" | "rctrl" | "rcontrol" => Some(W32::DIK::DIK_RCONTROL as u32),
        "alt" | "leftalt" | "lalt" => Some(W32::DIK::DIK_LMENU as u32),
        "rightalt" | "ralt" => Some(W32::DIK::DIK_RMENU as u32),
        "capslock" => Some(W32::DIK::DIK_CAPITAL as u32),
        "numlock" => Some(W32::DIK::DIK_NUMLOCK as u32),
        "scrolllock" => Some(W32::DIK::DIK_SCROLL as u32),
        "minus" => Some(W32::DIK::DIK_MINUS as u32),
        "equals" | "equal" => Some(W32::DIK::DIK_EQUALS as u32),
        "comma" => Some(W32::DIK::DIK_COMMA as u32),
        "period" | "dot" => Some(W32::DIK::DIK_PERIOD as u32),
        "slash" | "forwardslash" => Some(W32::DIK::DIK_SLASH as u32),
        "backslash" => Some(W32::DIK::DIK_BACKSLASH as u32),
        "semicolon" => Some(W32::DIK::DIK_SEMICOLON as u32),
        "apostrophe" | "quote" => Some(W32::DIK::DIK_APOSTROPHE as u32),
        "grave" | "tilde" | "backtick" => Some(W32::DIK::DIK_GRAVE as u32),
        "lbracket" | "leftbracket" => Some(W32::DIK::DIK_LBRACKET as u32),
        "rbracket" | "rightbracket" => Some(W32::DIK::DIK_RBRACKET as u32),
        "leftmousebutton" | "leftmouse" | "mouse1" => Some(input_map::MACRO_MOUSE_BUTTON_OFFSET),
        "rightmousebutton" | "rightmouse" | "mouse2" => {
            Some(input_map::MACRO_MOUSE_BUTTON_OFFSET + 1)
        }
        "middlemousebutton" | "middlemouse" | "mouse3" => {
            Some(input_map::MACRO_MOUSE_BUTTON_OFFSET + 2)
        }
        "mousebutton3" | "mouse4" => Some(input_map::MACRO_MOUSE_BUTTON_OFFSET + 3),
        "mousebutton4" | "mouse5" => Some(input_map::MACRO_MOUSE_BUTTON_OFFSET + 4),
        "mousebutton5" | "mouse6" => Some(input_map::MACRO_MOUSE_BUTTON_OFFSET + 5),
        "mousebutton6" | "mouse7" => Some(input_map::MACRO_MOUSE_BUTTON_OFFSET + 6),
        "mousebutton7" | "mouse8" => Some(input_map::MACRO_MOUSE_BUTTON_OFFSET + 7),
        "mousewheelup" | "wheelup" => Some(input_map::MACRO_MOUSE_WHEEL_OFFSET),
        "mousewheeldown" | "wheeldown" => Some(input_map::MACRO_MOUSE_WHEEL_OFFSET + 1),
        "gamepaddpadup" | "dpadup" => Some(input_map::GAMEPAD_BUTTON_OFFSET_DPAD_UP),
        "gamepaddpaddown" | "dpaddown" => Some(input_map::GAMEPAD_BUTTON_OFFSET_DPAD_DOWN),
        "gamepaddpadleft" | "dpadleft" => Some(input_map::GAMEPAD_BUTTON_OFFSET_DPAD_LEFT),
        "gamepaddpadright" | "dpadright" => Some(input_map::GAMEPAD_BUTTON_OFFSET_DPAD_RIGHT),
        "gamepadstart" | "start" => Some(input_map::GAMEPAD_BUTTON_OFFSET_START),
        "gamepadback" | "backbutton" => Some(input_map::GAMEPAD_BUTTON_OFFSET_BACK),
        "gamepadleftthumb" | "leftthumb" | "l3" => {
            Some(input_map::GAMEPAD_BUTTON_OFFSET_LEFT_THUMB)
        }
        "gamepadrightthumb" | "rightthumb" | "r3" => {
            Some(input_map::GAMEPAD_BUTTON_OFFSET_RIGHT_THUMB)
        }
        "gamepadleftshoulder" | "leftshoulder" | "lb" | "l1" => {
            Some(input_map::GAMEPAD_BUTTON_OFFSET_LEFT_SHOULDER)
        }
        "gamepadrightshoulder" | "rightshoulder" | "rb" | "r1" => {
            Some(input_map::GAMEPAD_BUTTON_OFFSET_RIGHT_SHOULDER)
        }
        "gamepada" | "abutton" => Some(input_map::GAMEPAD_BUTTON_OFFSET_A),
        "gamepadb" | "bbutton" => Some(input_map::GAMEPAD_BUTTON_OFFSET_B),
        "gamepadx" | "xbutton" => Some(input_map::GAMEPAD_BUTTON_OFFSET_X),
        "gamepady" | "ybutton" => Some(input_map::GAMEPAD_BUTTON_OFFSET_Y),
        "gamepadlt" | "lt" | "l2" => Some(input_map::GAMEPAD_BUTTON_OFFSET_LT),
        "gamepadrt" | "rt" | "r2" => Some(input_map::GAMEPAD_BUTTON_OFFSET_RT),
        _ => parse_function_key(&normalized),
    }
}

#[inline]
fn parse_single_char_key(byte: u8) -> Option<u32> {
    match byte.to_ascii_uppercase() {
        b'1' => Some(W32::DIK::DIK_1 as u32),
        b'2' => Some(W32::DIK::DIK_2 as u32),
        b'3' => Some(W32::DIK::DIK_3 as u32),
        b'4' => Some(W32::DIK::DIK_4 as u32),
        b'5' => Some(W32::DIK::DIK_5 as u32),
        b'6' => Some(W32::DIK::DIK_6 as u32),
        b'7' => Some(W32::DIK::DIK_7 as u32),
        b'8' => Some(W32::DIK::DIK_8 as u32),
        b'9' => Some(W32::DIK::DIK_9 as u32),
        b'0' => Some(W32::DIK::DIK_0 as u32),
        b'A' => Some(W32::DIK::DIK_A as u32),
        b'B' => Some(W32::DIK::DIK_B as u32),
        b'C' => Some(W32::DIK::DIK_C as u32),
        b'D' => Some(W32::DIK::DIK_D as u32),
        b'E' => Some(W32::DIK::DIK_E as u32),
        b'F' => Some(W32::DIK::DIK_F as u32),
        b'G' => Some(W32::DIK::DIK_G as u32),
        b'H' => Some(W32::DIK::DIK_H as u32),
        b'I' => Some(W32::DIK::DIK_I as u32),
        b'J' => Some(W32::DIK::DIK_J as u32),
        b'K' => Some(W32::DIK::DIK_K as u32),
        b'L' => Some(W32::DIK::DIK_L as u32),
        b'M' => Some(W32::DIK::DIK_M as u32),
        b'N' => Some(W32::DIK::DIK_N as u32),
        b'O' => Some(W32::DIK::DIK_O as u32),
        b'P' => Some(W32::DIK::DIK_P as u32),
        b'Q' => Some(W32::DIK::DIK_Q as u32),
        b'R' => Some(W32::DIK::DIK_R as u32),
        b'S' => Some(W32::DIK::DIK_S as u32),
        b'T' => Some(W32::DIK::DIK_T as u32),
        b'U' => Some(W32::DIK::DIK_U as u32),
        b'V' => Some(W32::DIK::DIK_V as u32),
        b'W' => Some(W32::DIK::DIK_W as u32),
        b'X' => Some(W32::DIK::DIK_X as u32),
        b'Y' => Some(W32::DIK::DIK_Y as u32),
        b'Z' => Some(W32::DIK::DIK_Z as u32),
        _ => None,
    }
}

#[inline]
fn parse_numeric_key_code(token: &str) -> Option<u32> {
    token
        .strip_prefix("0x")
        .or_else(|| token.strip_prefix("0X"))
        .map(|value| u32::from_str_radix(value, 16).ok())
        .unwrap_or_else(|| token.parse::<u32>().ok())
}

#[inline]
fn parse_function_key(token: &str) -> Option<u32> {
    let number = token.strip_prefix('f')?.parse::<u32>().ok()?;
    match number {
        1..=10 => Some((W32::DIK::DIK_F1 as u32) + (number - 1)),
        11 => Some(W32::DIK::DIK_F11 as u32),
        12 => Some(W32::DIK::DIK_F12 as u32),
        _ => None,
    }
}

#[inline]
fn normalize_hotkey_token(token: &str) -> String {
    let mut normalized = String::with_capacity(token.len());
    for ch in token.chars() {
        if ch.is_ascii_alphanumeric() {
            normalized.push(ch.to_ascii_lowercase());
        }
    }
    normalized
}

#[inline]
fn key_is_pressed_in_events(events: &InputEvents<'_>, key_code: u32) -> bool {
    events
        .buttons()
        .any(|button| button_matches_key(button, key_code))
}

#[inline]
fn button_matches_key(button: &ButtonEvent, key_code: u32) -> bool {
    button.get_id_code() == key_code && button.is_down()
}

#[inline]
fn key_is_currently_pressed(key_code: u32) -> bool {
    let manager = BSInputDeviceManager::get_singleton();
    let Some(manager) = (unsafe { manager.as_ref() }) else {
        return false;
    };

    if key_code < input_map::MACRO_MOUSE_BUTTON_OFFSET {
        let keyboard = manager.get_keyboard();
        return unsafe { keyboard.as_ref() }
            .map(|keyboard: &BSWin32KeyboardDevice| keyboard.is_pressed(key_code))
            .unwrap_or(false);
    }

    if key_code < input_map::MACRO_GAMEPAD_OFFSET {
        let mouse = manager.get_mouse();
        return unsafe { mouse.as_ref() }
            .map(|mouse: &BSWin32MouseDevice| mouse.base.base.is_pressed(key_code))
            .unwrap_or(false);
    }

    let gamepad = manager.get_gamepad();
    unsafe { gamepad.as_ref() }
        .map(|gamepad| gamepad.base.base.is_pressed(key_code))
        .unwrap_or(false)
}
