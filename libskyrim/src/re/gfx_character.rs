#![allow(non_camel_case_types)]

use core_util::inherit;

use crate::re::{
    GASEnvironment, GFxLogBase, GFxResourceID, GFxSprite, GFxStatMovieViews, GMatrix2D,
    GRefCountBaseWeakSupport, GRendererCxform,
};

/// C++ `RE::GFxCharacter`
#[repr(C)]
pub struct GFxCharacter {
    pub weak_support:
        GRefCountBaseWeakSupport<GFxCharacter, { GFxStatMovieViews::MOVIE_CLIP_MEM as u32 }>, // 00
    pub log_base: GFxLogBase<GFxCharacter>, // 18
    pub resource_id: GFxResourceID,         // 20
    pub pad24: u32,                         // 24
    pub created_frame: i64,                 // 28
    pub parent_clip: *mut GFxSprite,        // 30
    pub color_transform: GRendererCxform,   // 38
    pub matrix: GMatrix2D,                  // 58
    pub unk70: u64,                         // 70
    pub unk78: u64,                         // 78
    pub unk80: u64,                         // 80
    pub unk88: u32,                         // 88
    pub unk8c: u32,                         // 8C
    pub unk90: u32,                         // 90
    pub unk94: u16,                         // 94
    pub unk96: u16,                         // 96
}

const _: () = assert!(core::mem::size_of::<GFxCharacter>() == 0x98);
const _: () = assert!(core::mem::offset_of!(GFxCharacter, weak_support) == 0x0);
const _: () = assert!(core::mem::offset_of!(GFxCharacter, log_base) == 0x18);
const _: () = assert!(core::mem::offset_of!(GFxCharacter, resource_id) == 0x20);

inherit!(GFxCharacter : GRefCountBaseWeakSupport<GFxCharacter, { GFxStatMovieViews::MOVIE_CLIP_MEM as u32 }>, weak_support);
inherit!(GFxCharacter => GFxLogBase<GFxCharacter>, log_base);

impl GFxCharacter {
    crate::virtual_method! { pub const VFUNC_DTOR: usize = 0x00; pub fn dtor() }
    crate::virtual_method! { pub const VFUNC_UNK_01: usize = 0x01; pub fn unk_01() }
    crate::virtual_method! { pub const VFUNC_UNK_02: usize = 0x02; pub fn unk_02() }
    crate::virtual_method! { pub const VFUNC_UNK_03: usize = 0x03; pub fn unk_03() }
    crate::virtual_method! { pub const VFUNC_UNK_04: usize = 0x04; pub fn unk_04() }
    crate::virtual_method! { pub const VFUNC_UNK_05: usize = 0x05; pub fn unk_05() }
    crate::virtual_method! { pub const VFUNC_UNK_06: usize = 0x06; pub fn unk_06() }
    crate::virtual_method! { pub const VFUNC_UNK_07: usize = 0x07; pub fn unk_07() }
    crate::virtual_method! { pub const VFUNC_UNK_08: usize = 0x08; pub fn unk_08() }
    crate::virtual_method! { pub const VFUNC_UNK_09: usize = 0x09; pub fn unk_09() }
    crate::virtual_method! { pub const VFUNC_UNK_0A: usize = 0x0A; pub fn unk_0a() }
    crate::virtual_method! { pub const VFUNC_UNK_0B: usize = 0x0B; pub fn unk_0b() }
    crate::virtual_method! { pub const VFUNC_UNK_0C: usize = 0x0C; pub fn unk_0c() }
    crate::virtual_method! { pub const VFUNC_UNK_0D: usize = 0x0D; pub fn unk_0d() }
    crate::virtual_method! { pub const VFUNC_UNK_0E: usize = 0x0E; pub fn unk_0e() }
    crate::virtual_method! { pub const VFUNC_UNK_0F: usize = 0x0F; pub fn unk_0f() }
    crate::virtual_method! { pub const VFUNC_UNK_10: usize = 0x10; pub fn unk_10() }
    crate::virtual_method! { pub const VFUNC_UNK_11: usize = 0x11; pub fn unk_11() }
    crate::virtual_method! { pub const VFUNC_UNK_12: usize = 0x12; pub fn unk_12() }
    crate::virtual_method! { pub const VFUNC_UNK_13: usize = 0x13; pub fn unk_13() }
    crate::virtual_method! { pub const VFUNC_UNK_14: usize = 0x14; pub fn unk_14() }
    crate::virtual_method! { pub const VFUNC_UNK_15: usize = 0x15; pub fn unk_15() }
    crate::virtual_method! { pub const VFUNC_UNK_16: usize = 0x16; pub fn unk_16() }
    crate::virtual_method! { pub const VFUNC_UNK_17: usize = 0x17; pub fn unk_17() }
    crate::virtual_method! { pub const VFUNC_UNK_18: usize = 0x18; pub fn unk_18() }
    crate::virtual_method! { pub const VFUNC_UNK_19: usize = 0x19; pub fn unk_19() }
    crate::virtual_method! { pub const VFUNC_UNK_1A: usize = 0x1A; pub fn unk_1a() }
    crate::virtual_method! { pub const VFUNC_UNK_1B: usize = 0x1B; pub fn unk_1b() }
    crate::virtual_method! { pub const VFUNC_GET_ENVIRONMENT: usize = 0x1C; pub fn get_environment() -> *mut GASEnvironment }
    crate::virtual_method! { pub const VFUNC_UNK_1D: usize = 0x1D; pub fn unk_1d() }
    crate::virtual_method! { pub const VFUNC_UNK_1E: usize = 0x1E; pub fn unk_1e() }
    crate::virtual_method! { pub const VFUNC_UNK_1F: usize = 0x1F; pub fn unk_1f() }
    crate::virtual_method! { pub const VFUNC_UNK_20: usize = 0x20; pub fn unk_20() }
    crate::virtual_method! { pub const VFUNC_UNK_21: usize = 0x21; pub fn unk_21() }
    crate::virtual_method! { pub const VFUNC_UNK_22: usize = 0x22; pub fn unk_22() }
    crate::virtual_method! { pub const VFUNC_UNK_23: usize = 0x23; pub fn unk_23() }
    crate::virtual_method! { pub const VFUNC_UNK_24: usize = 0x24; pub fn unk_24() }
    crate::virtual_method! { pub const VFUNC_UNK_25: usize = 0x25; pub fn unk_25() }
    crate::virtual_method! { pub const VFUNC_UNK_26: usize = 0x26; pub fn unk_26() }
    crate::virtual_method! { pub const VFUNC_UNK_27: usize = 0x27; pub fn unk_27() }
    crate::virtual_method! { pub const VFUNC_UNK_28: usize = 0x28; pub fn unk_28() }
    crate::virtual_method! { pub const VFUNC_UNK_29: usize = 0x29; pub fn unk_29() }
    crate::virtual_method! { pub const VFUNC_UNK_2A: usize = 0x2A; pub fn unk_2a() }
    crate::virtual_method! { pub const VFUNC_UNK_2B: usize = 0x2B; pub fn unk_2b() }
    crate::virtual_method! { pub const VFUNC_UNK_2C: usize = 0x2C; pub fn unk_2c() }
    crate::virtual_method! { pub const VFUNC_UNK_2D: usize = 0x2D; pub fn unk_2d() }
    crate::virtual_method! { pub const VFUNC_UNK_2E: usize = 0x2E; pub fn unk_2e() }
    crate::virtual_method! { pub const VFUNC_UNK_2F: usize = 0x2F; pub fn unk_2f() }
    crate::virtual_method! { pub const VFUNC_UNK_30: usize = 0x30; pub fn unk_30() }
}
