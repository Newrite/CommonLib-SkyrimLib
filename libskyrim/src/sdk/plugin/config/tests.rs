use super::hotkeys::parse_hotkey_key;
use super::{Config, ConfigValueError, HotkeyCombo, HotkeyParseError, config};
use crate::ini::Ini;
use crate::rex::W32;
use crate::skse::input_map;

#[test]
fn bool_values_accept_common_spellings() {
    let ini = Ini::from_str("[General]\nEnabled = yes\nDisabled = 0\n").unwrap();
    let config = config(&ini);

    assert!(config.bool("General", "Enabled").unwrap());
    assert!(!config.bool("General", "Disabled").unwrap());
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

    let digit = HotkeyCombo::parse("1").unwrap();
    assert_eq!(digit.keys(), &[W32::DIK::DIK_1 as u32]);
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
