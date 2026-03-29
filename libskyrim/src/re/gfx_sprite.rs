#![allow(non_camel_case_types)]

use core_util::inherit;

use crate::re::{
    GASEnvironment, GASMovieClipObject, GArray, GFxASCharacter, GFxMovieRoot, GFxSpriteDef,
};

/// C++ `RE::GFxSprite`
#[repr(C)]
pub struct GFxSprite {
    pub base: GFxASCharacter,                   // 000
    pub sprite_def: *mut GFxSpriteDef,          // 110
    pub movie_root: *mut GFxMovieRoot,          // 118
    pub unk120: [u64; 5],                       // 120
    pub unk148: u32,                            // 148
    pub current_frame: u32,                     // 14C
    pub unk150: GArray<*mut core::ffi::c_void>, // 150
    pub environment: GASEnvironment,            // 168
    pub unk308: u64,                            // 308
    pub unk310: u64,                            // 310
    pub movie_clip: *mut GASMovieClipObject,    // 318
    pub unk320: u64,                            // 320
    pub unk328: u64,                            // 328
    pub unk330: u64,                            // 330
    pub unk338: u64,                            // 338
    pub unk340: u64,                            // 340
    pub unk348: u8,                             // 348
    pub unk349: u8,                             // 349
    pub unk34a: u8,                             // 34A
    pub unk34b: u8,                             // 34B
    pub unk34c: u32,                            // 34C
}

const _: () = assert!(core::mem::size_of::<GFxSprite>() == 0x350);
const _: () = assert!(core::mem::offset_of!(GFxSprite, base) == 0x0);
const _: () = assert!(core::mem::offset_of!(GFxSprite, sprite_def) == 0x110);
const _: () = assert!(core::mem::offset_of!(GFxSprite, environment) == 0x168);

inherit!(GFxSprite : GFxASCharacter, base);

impl GFxSprite {
    crate::virtual_method! { pub const VFUNC_UNK_58: usize = 0x58; pub fn unk_58() }
    crate::virtual_method! { pub const VFUNC_UNK_59: usize = 0x59; pub fn unk_59() }
    crate::virtual_method! { pub const VFUNC_UNK_5A: usize = 0x5A; pub fn unk_5a() }
    crate::virtual_method! { pub const VFUNC_UNK_5B: usize = 0x5B; pub fn unk_5b() }
    crate::virtual_method! { pub const VFUNC_UNK_5C: usize = 0x5C; pub fn unk_5c() }
    crate::virtual_method! { pub const VFUNC_UNK_5D: usize = 0x5D; pub fn unk_5d() }
    crate::virtual_method! { pub const VFUNC_UNK_5E: usize = 0x5E; pub fn unk_5e() }
}
