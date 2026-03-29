#![allow(non_camel_case_types)]

use core_util::inherit;

use crate::re::{
    GASStringContext, GFxEventEventType, GFxKey, GFxSpecialKeysState, GFxStatMovieViews,
    GRefCountBaseNTS,
};

/// C++ `RE::GFxKeyboardState::IListener`
#[repr(C)]
pub struct GFxKeyboardStateIListener {
    pub vtable: *const usize, // 00
}

const _: () = assert!(core::mem::size_of::<GFxKeyboardStateIListener>() == 0x8);

impl GFxKeyboardStateIListener {
    crate::virtual_method! { pub const VFUNC_DTOR: usize = 0x00; pub fn dtor() }
    crate::virtual_method! { pub const VFUNC_ON_KEY_DOWN: usize = 0x01; pub fn on_key_down(string_context: *mut GASStringContext, code: i32, ascii: u8, char_code: u32, keyboard_index: u8) }
    crate::virtual_method! { pub const VFUNC_ON_KEY_UP: usize = 0x02; pub fn on_key_up(string_context: *mut GASStringContext, code: i32, ascii: u8, char_code: u32, keyboard_index: u8) }
    crate::virtual_method! { pub const VFUNC_UPDATE: usize = 0x03; pub fn update(code: i32, ascii: u8, char_code: u32, keyboard_index: u8) }
}

/// C++ `RE::GFxKeyboardState::KeyQueue::KeyRecord`
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct GFxKeyboardStateKeyQueueKeyRecord {
    pub char_code: u32,                          // 00
    pub code: u16,                               // 04
    pub pad06: u16,                              // 06
    pub event: GFxEventEventType,                // 08
    pub ascii: u8,                               // 0C
    pub special_keys_state: GFxSpecialKeysState, // 0D
    pub pad0e: u8,                               // 0E
    pub pad0f: u8,                               // 0F
}

const _: () = assert!(core::mem::size_of::<GFxKeyboardStateKeyQueueKeyRecord>() == 0x10);

/// C++ `RE::GFxKeyboardState::KeyQueue`
#[repr(C)]
pub struct GFxKeyboardStateKeyQueue {
    pub buffer: [GFxKeyboardStateKeyQueueKeyRecord; Self::kKeyQueueSize], // 000
    pub put_idx: u32,                                                     // 640
    pub get_idx: u32,                                                     // 644
    pub count: u32,                                                       // 648
}

impl GFxKeyboardStateKeyQueue {
    pub const kKeyQueueSize: usize = 100;
}

const _: () = assert!(core::mem::size_of::<GFxKeyboardStateKeyQueue>() == 0x64C);

/// C++ `RE::GFxKeyboardState`
#[repr(C)]
pub struct GFxKeyboardState {
    pub base:
        GRefCountBaseNTS<GFxKeyboardState, { GFxStatMovieViews::kGFxStatMV_Other_Mem as u32 }>, // 00
    pub listener: *mut GFxKeyboardStateIListener, // 10
    pub key_queue: GFxKeyboardStateKeyQueue,      // 18
    pub keyboard_index: u8,                       // 664
    pub keymap: [u8; GFxKey::kTotal / 8 + 1],     // 665
    pub toggled: [bool; 3],                       // 682
    pub pad685: u8,                               // 685
    pub pad686: u8,                               // 686
    pub pad687: u8,                               // 687
}

const _: () = assert!(core::mem::size_of::<GFxKeyboardState>() == 0x688);
const _: () = assert!(core::mem::offset_of!(GFxKeyboardState, listener) == 0x10);
const _: () = assert!(core::mem::offset_of!(GFxKeyboardState, key_queue) == 0x18);

inherit!(GFxKeyboardState : GRefCountBaseNTS<GFxKeyboardState, { GFxStatMovieViews::kGFxStatMV_Other_Mem as u32 }>, base);
