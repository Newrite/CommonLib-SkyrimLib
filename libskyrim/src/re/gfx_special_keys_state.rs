#![allow(non_camel_case_types)]

/// C++ `RE::GFxSpecialKeysState::Key`
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GFxSpecialKeysStateKey {
    kShiftPressed = 1 << 0,
    kCtrlPressed = 1 << 1,
    kAltPressed = 1 << 2,
    kCapsToggled = 1 << 3,
    kNumToggled = 1 << 4,
    kScrollToggled = 1 << 5,
    kInitializedBit = 1 << 7,
    kInitializedMask = 0xFF,
}

/// C++ `RE::GFxSpecialKeysState`
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct GFxSpecialKeysState {
    pub states: u8, // 00
}

const _: () = assert!(core::mem::size_of::<GFxSpecialKeysState>() == 0x1);
const _: () = assert!(core::mem::offset_of!(GFxSpecialKeysState, states) == 0x0);
