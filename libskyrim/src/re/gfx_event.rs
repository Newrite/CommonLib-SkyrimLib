#![allow(non_camel_case_types)]

use core_util::EnumSet;

use crate::re::{GFxKeyCode, GFxSpecialKeysState};

/// C++ `RE::GFxEvent::EventType`
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GFxEventEventType {
    kNone = 0,
    kMouseMove = 1,
    kMouseDown = 2,
    kMouseUp = 3,
    kMouseWheel = 4,
    kKeyDown = 5,
    kKeyUp = 6,
    kSceneResize = 7,
    kSetFocus = 8,
    kKillFocus = 9,
    kDoShowMouse = 10,
    kDoHideMouse = 11,
    kDoSetMouseCursor = 12,
    kCharEvent = 13,
    kIMEEvent = 14,
}

core_util::impl_enumset_type!(GFxEventEventType => u32);

impl Default for GFxEventEventType {
    #[inline(always)]
    fn default() -> Self {
        Self::kNone
    }
}

/// C++ `RE::GFxEvent`
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct GFxEvent {
    pub type_: EnumSet<GFxEventEventType, u32>, // 00
}

const _: () = assert!(core::mem::size_of::<GFxEvent>() == 0x4);
const _: () = assert!(core::mem::offset_of!(GFxEvent, type_) == 0x0);

impl GFxEvent {
    #[inline(always)]
    pub fn new(event_type: GFxEventEventType) -> Self {
        Self {
            type_: EnumSet::from_underlying(event_type as u32),
        }
    }
}

impl Default for GFxEvent {
    #[inline(always)]
    fn default() -> Self {
        Self::new(GFxEventEventType::kNone)
    }
}

/// C++ `RE::GFxMouseEvent`
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GFxMouseEvent {
    pub base: GFxEvent,    // 00
    pub x: f32,            // 04
    pub y: f32,            // 08
    pub scroll_delta: f32, // 0C
    pub button: u32,       // 10
    pub mouse_index: u32,  // 14
}

const _: () = assert!(core::mem::size_of::<GFxMouseEvent>() == 0x18);
const _: () = assert!(core::mem::offset_of!(GFxMouseEvent, base) == 0x0);
const _: () = assert!(core::mem::offset_of!(GFxMouseEvent, x) == 0x4);
const _: () = assert!(core::mem::offset_of!(GFxMouseEvent, y) == 0x8);
const _: () = assert!(core::mem::offset_of!(GFxMouseEvent, scroll_delta) == 0xC);
const _: () = assert!(core::mem::offset_of!(GFxMouseEvent, button) == 0x10);
const _: () = assert!(core::mem::offset_of!(GFxMouseEvent, mouse_index) == 0x14);

impl GFxMouseEvent {
    #[inline(always)]
    pub fn new(
        event_type: GFxEventEventType,
        button: u32,
        x: f32,
        y: f32,
        scroll_delta: f32,
        mouse_index: u32,
    ) -> Self {
        Self {
            base: GFxEvent::new(event_type),
            x,
            y,
            scroll_delta,
            button,
            mouse_index,
        }
    }

    #[inline(always)]
    pub fn with_mouse_index(event_type: GFxEventEventType, mouse_index: u32) -> Self {
        Self::new(event_type, 0, 0.0, 0.0, 0.0, mouse_index)
    }
}

impl Default for GFxMouseEvent {
    #[inline(always)]
    fn default() -> Self {
        Self::new(GFxEventEventType::kNone, 0, 0.0, 0.0, 0.0, 0)
    }
}

/// C++ `RE::GFxKeyEvent`
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct GFxKeyEvent {
    pub base: GFxEvent,                         // 00
    pub key_code: GFxKeyCode,                   // 04
    pub ascii_code: u8,                         // 08
    pub pad09: u8,                              // 09
    pub pad0a: u16,                             // 0A
    pub wchar_code: u32,                        // 0C
    pub special_key_state: GFxSpecialKeysState, // 10
    pub keyboard_index: u8,                     // 11
    pub pad12: u16,                             // 12
}

const _: () = assert!(core::mem::size_of::<GFxKeyEvent>() == 0x14);
const _: () = assert!(core::mem::offset_of!(GFxKeyEvent, base) == 0x0);
const _: () = assert!(core::mem::offset_of!(GFxKeyEvent, key_code) == 0x4);
const _: () = assert!(core::mem::offset_of!(GFxKeyEvent, ascii_code) == 0x8);
const _: () = assert!(core::mem::offset_of!(GFxKeyEvent, wchar_code) == 0xC);
const _: () = assert!(core::mem::offset_of!(GFxKeyEvent, special_key_state) == 0x10);
const _: () = assert!(core::mem::offset_of!(GFxKeyEvent, keyboard_index) == 0x11);

impl GFxKeyEvent {
    #[inline(always)]
    pub fn new(
        event_type: GFxEventEventType,
        key_code: GFxKeyCode,
        ascii_code: u8,
        wchar_code: u32,
        special_key_state: GFxSpecialKeysState,
        keyboard_index: u8,
    ) -> Self {
        Self {
            base: GFxEvent::new(event_type),
            key_code,
            ascii_code,
            pad09: 0,
            pad0a: 0,
            wchar_code,
            special_key_state,
            keyboard_index,
            pad12: 0,
        }
    }
}

impl Default for GFxKeyEvent {
    #[inline(always)]
    fn default() -> Self {
        Self::new(
            GFxEventEventType::kNone,
            GFxKeyCode::kVoidSymbol,
            0,
            0,
            GFxSpecialKeysState::default(),
            0,
        )
    }
}

// TODO: `GFxEvent.h` also declares the VR-only `ScaleformEvent` relocation
// globals/helpers (`g_scaleformGFxEventData`, `QueueGFxMouseEvent`,
// `DispatchGFxEvent`, `CreateMouseEvent`, `SendMouseEvent`). The vendored tree
// does not provide source-backed relocation IDs/definitions for them yet, so
// the Rust translation intentionally omits that namespace until those VR
// addresses can be modeled honestly.
