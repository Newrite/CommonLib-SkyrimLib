//! Plugin-facing config helpers.
//!
//! This domain gathers the repetitive SKSE plugin patterns around typed INI
//! access, config-driven form lookup, and user-facing hotkey parsing.

use alloc::string::String;
use alloc::vec::Vec;
use core::fmt;
use core::str::FromStr;

use crate::ini::Section;
use crate::re::{
    BSInputDeviceManager, BSWin32KeyboardDevice, BSWin32MouseDevice, ButtonEvent, FormCastable,
    TESForm,
};
use crate::rex::W32;
use crate::sdk::core::GamePtr;
use crate::sdk::events::input::InputEvents;
use crate::sdk::forms::lookup;
use crate::skse::input_map;

pub use crate::ini::Ini;

/// Borrowed typed facade over one loaded [`Ini`].
#[derive(Clone, Copy)]
pub struct Config<'a> {
    ini: &'a Ini,
}

/// Failure to read one config field.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConfigValueError {
    MissingSection,
    MissingField,
    InvalidValue,
}

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

impl<'a> Config<'a> {
    #[inline(always)]
    pub const fn new(ini: &'a Ini) -> Self {
        Self { ini }
    }

    #[inline(always)]
    pub const fn ini(self) -> &'a Ini {
        self.ini
    }

    #[inline(always)]
    pub fn has_section(self, section: &str) -> bool {
        self.ini.section(section).is_ok()
    }

    #[inline(always)]
    pub fn has_field(self, section: &str, field: &str) -> bool {
        self.raw(section, field).is_some()
    }

    #[inline(always)]
    pub fn section(self, section: &str) -> Option<Section<'a>> {
        self.ini.section(section).ok()
    }

    #[inline(always)]
    pub fn raw(self, section: &str, field: &str) -> Option<&'a str> {
        self.ini.get(section, field)
    }

    #[inline]
    pub fn require(self, section: &str, field: &str) -> Result<&'a str, ConfigValueError> {
        let section = self
            .ini
            .section(section)
            .map_err(|_| ConfigValueError::MissingSection)?;
        let field = section
            .field(field)
            .map_err(|_| ConfigValueError::MissingField)?;
        field.value().ok_or(ConfigValueError::InvalidValue)
    }

    #[inline]
    pub fn parsed<T: FromStr>(self, section: &str, field: &str) -> Result<T, ConfigValueError> {
        self.require(section, field)?
            .trim()
            .parse::<T>()
            .map_err(|_| ConfigValueError::InvalidValue)
    }

    #[inline]
    pub fn parsed_or<T: FromStr + Copy>(self, section: &str, field: &str, default: T) -> T {
        self.parsed(section, field).unwrap_or(default)
    }

    #[inline]
    pub fn string_or(self, section: &str, field: &str, default: &'a str) -> &'a str {
        self.raw(section, field).unwrap_or(default)
    }

    #[inline]
    pub fn bool(self, section: &str, field: &str) -> Result<bool, ConfigValueError> {
        parse_bool_value(self.require(section, field)?).ok_or(ConfigValueError::InvalidValue)
    }

    #[inline]
    pub fn bool_or(self, section: &str, field: &str, default: bool) -> bool {
        self.bool(section, field).unwrap_or(default)
    }

    #[inline(always)]
    pub fn i32(self, section: &str, field: &str) -> Result<i32, ConfigValueError> {
        self.parsed(section, field)
    }

    #[inline(always)]
    pub fn i32_or(self, section: &str, field: &str, default: i32) -> i32 {
        self.parsed_or(section, field, default)
    }

    #[inline(always)]
    pub fn u32(self, section: &str, field: &str) -> Result<u32, ConfigValueError> {
        self.parsed(section, field)
    }

    #[inline(always)]
    pub fn u32_or(self, section: &str, field: &str, default: u32) -> u32 {
        self.parsed_or(section, field, default)
    }

    #[inline(always)]
    pub fn f32(self, section: &str, field: &str) -> Result<f32, ConfigValueError> {
        self.parsed(section, field)
    }

    #[inline(always)]
    pub fn f32_or(self, section: &str, field: &str, default: f32) -> f32 {
        self.parsed_or(section, field, default)
    }

    #[inline(always)]
    pub fn form(self, section: &str, field: &str) -> Result<GamePtr<TESForm>, ConfigValueError> {
        Ok(lookup::form_from_string(self.require(section, field)?))
    }

    #[inline(always)]
    pub fn form_typed<T: FormCastable>(
        self,
        section: &str,
        field: &str,
    ) -> Result<GamePtr<T>, ConfigValueError> {
        Ok(lookup::form_from_string_typed::<T>(
            self.require(section, field)?,
        ))
    }

    #[inline(always)]
    pub fn form_raw(
        self,
        section: &str,
        field: &str,
    ) -> Result<GamePtr<TESForm>, ConfigValueError> {
        Ok(lookup::form_from_string_raw(self.require(section, field)?))
    }

    #[inline(always)]
    pub fn form_raw_typed<T: FormCastable>(
        self,
        section: &str,
        field: &str,
    ) -> Result<GamePtr<T>, ConfigValueError> {
        Ok(lookup::form_from_string_raw_typed::<T>(
            self.require(section, field)?,
        ))
    }

    #[inline]
    pub fn hotkey(self, section: &str, field: &str) -> Result<HotkeyCombo, ConfigValueError> {
        HotkeyCombo::parse(self.require(section, field)?)
            .map_err(|_| ConfigValueError::InvalidValue)
    }

    #[inline]
    pub fn hotkey_or(self, section: &str, field: &str, default: &str) -> HotkeyCombo {
        self.hotkey(section, field)
            .unwrap_or_else(|_| HotkeyCombo::parse(default).unwrap_or_default())
    }
}

impl<'a> From<&'a Ini> for Config<'a> {
    #[inline(always)]
    fn from(value: &'a Ini) -> Self {
        Self::new(value)
    }
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

#[inline(always)]
pub fn config(ini: &Ini) -> Config<'_> {
    Config::new(ini)
}

#[inline(always)]
pub fn load_ini(path: &str) -> Result<Ini, ()> {
    Ini::from_path_str(path)
}

#[inline(always)]
pub fn write_ini(ini: &Ini, path: &str) -> Result<(), core::fmt::Error> {
    ini.write_file_str(path)
}

#[inline(always)]
pub fn has_section(ini: &Ini, section: &str) -> bool {
    config(ini).has_section(section)
}

#[inline(always)]
pub fn has_field(ini: &Ini, section: &str, field: &str) -> bool {
    config(ini).has_field(section, field)
}

#[inline(always)]
pub fn raw_value<'a>(ini: &'a Ini, section: &str, field: &str) -> Option<&'a str> {
    config(ini).raw(section, field)
}

#[inline(always)]
pub fn bool_value(ini: &Ini, section: &str, field: &str) -> Result<bool, ConfigValueError> {
    config(ini).bool(section, field)
}

#[inline(always)]
pub fn i32_value(ini: &Ini, section: &str, field: &str) -> Result<i32, ConfigValueError> {
    config(ini).i32(section, field)
}

#[inline(always)]
pub fn u32_value(ini: &Ini, section: &str, field: &str) -> Result<u32, ConfigValueError> {
    config(ini).u32(section, field)
}

#[inline(always)]
pub fn f32_value(ini: &Ini, section: &str, field: &str) -> Result<f32, ConfigValueError> {
    config(ini).f32(section, field)
}

#[inline(always)]
pub fn form_value(
    ini: &Ini,
    section: &str,
    field: &str,
) -> Result<GamePtr<TESForm>, ConfigValueError> {
    config(ini).form(section, field)
}

#[inline(always)]
pub fn form_value_typed<T: FormCastable>(
    ini: &Ini,
    section: &str,
    field: &str,
) -> Result<GamePtr<T>, ConfigValueError> {
    config(ini).form_typed::<T>(section, field)
}

#[inline(always)]
pub fn hotkey_value(
    ini: &Ini,
    section: &str,
    field: &str,
) -> Result<HotkeyCombo, ConfigValueError> {
    config(ini).hotkey(section, field)
}

#[inline]
fn parse_bool_value(value: &str) -> Option<bool> {
    let value = value.trim();
    if value.eq_ignore_ascii_case("true")
        || value.eq_ignore_ascii_case("yes")
        || value.eq_ignore_ascii_case("on")
        || value == "1"
    {
        Some(true)
    } else if value.eq_ignore_ascii_case("false")
        || value.eq_ignore_ascii_case("no")
        || value.eq_ignore_ascii_case("off")
        || value == "0"
    {
        Some(false)
    } else {
        None
    }
}

#[inline]
fn parse_hotkey_key(token: &str) -> Option<u32> {
    let token = token.trim();
    if token.is_empty() {
        return None;
    }

    if let Some(value) = parse_numeric_key_code(token) {
        return Some(value);
    }

    if token.len() == 1 {
        let ch = token.as_bytes()[0].to_ascii_uppercase();
        return match ch {
            b'A'..=b'Z' => Some((W32::DIK::DIK_A as u32) + u32::from(ch - b'A')),
            b'0'..=b'9' => Some((W32::DIK::DIK_0 as u32) - u32::from(b'9' - ch)),
            _ => None,
        };
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

#[cfg(test)]
mod tests {
    use super::{
        Config, ConfigValueError, HotkeyCombo, HotkeyParseError, config, parse_hotkey_key,
    };
    use crate::ini::Ini;
    use crate::rex::W32;
    use crate::skse::input_map;

    #[test]
    fn bool_values_accept_common_spellings() {
        let ini = Ini::from_str("[General]\nEnabled = yes\nDisabled = 0\n").unwrap();
        let config = config(&ini);

        assert_eq!(config.bool("General", "Enabled").unwrap(), true);
        assert_eq!(config.bool("General", "Disabled").unwrap(), false);
    }

    #[test]
    fn typed_values_report_missing_and_invalid_fields() {
        let ini = Ini::from_str("[General]\nCount = nope\n").unwrap();
        let config = Config::new(&ini);

        assert_eq!(
            config.i32("General", "Missing").unwrap_err(),
            ConfigValueError::MissingField
        );
        assert_eq!(
            config.i32("General", "Count").unwrap_err(),
            ConfigValueError::InvalidValue
        );
    }

    #[test]
    fn parses_keyboard_hotkeys() {
        let combo = HotkeyCombo::parse("Shift + E").unwrap();
        assert_eq!(
            combo.keys(),
            &[W32::DIK::DIK_LSHIFT as u32, W32::DIK::DIK_E as u32]
        );
    }

    #[test]
    fn parses_mouse_and_gamepad_hotkeys() {
        assert_eq!(
            parse_hotkey_key("Mouse Wheel Down"),
            Some(input_map::MACRO_MOUSE_WHEEL_OFFSET + 1)
        );
        assert_eq!(
            parse_hotkey_key("Gamepad A"),
            Some(input_map::GAMEPAD_BUTTON_OFFSET_A)
        );
    }

    #[test]
    fn rejects_invalid_hotkeys() {
        assert_eq!(HotkeyCombo::parse("").unwrap_err(), HotkeyParseError::Empty);
        assert_eq!(
            HotkeyCombo::parse("Shift + Shift").unwrap_err(),
            HotkeyParseError::DuplicateKey
        );
        assert_eq!(
            HotkeyCombo::parse("???").unwrap_err(),
            HotkeyParseError::UnknownKey
        );
    }
}
