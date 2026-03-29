#![allow(non_camel_case_types)]

use core_util::inherit;

use crate::re::{GASMovieClipObject, GASObjectInterface, GFxCharacter, GFxMovieDef};

/// C++ `RE::GFxASCharacter`
#[repr(C)]
pub struct GFxASCharacter {
    pub character: GFxCharacter,              // 000
    pub object_interface: GASObjectInterface, // 098
    pub unk0b0: u64,                          // 0B0
    pub unk0b8: u64,                          // 0B8
    pub unk0c0: u64,                          // 0C0
    pub movie_def: *mut GFxMovieDef,          // 0C8
    pub unk0d0: u64,                          // 0D0
    pub unk0d8: u64,                          // 0D8
    pub unk0e0: u64,                          // 0E0
    pub unk0e8: u64,                          // 0E8
    pub unk0f0: u64,                          // 0F0
    pub unk0f8: u64,                          // 0F8
    pub unk100: u64,                          // 100
    pub unk108: u64,                          // 108
}

const _: () = assert!(core::mem::size_of::<GFxASCharacter>() == 0x110);
const _: () = assert!(core::mem::offset_of!(GFxASCharacter, character) == 0x0);
const _: () = assert!(core::mem::offset_of!(GFxASCharacter, object_interface) == 0x98);
const _: () = assert!(core::mem::offset_of!(GFxASCharacter, movie_def) == 0xC8);

inherit!(GFxASCharacter : GFxCharacter, character);
inherit!(GFxASCharacter => GASObjectInterface, object_interface);

impl GFxASCharacter {
    crate::virtual_method! { pub const VFUNC_UNK_31: usize = 0x31; pub fn unk_31() }
    crate::virtual_method! { pub const VFUNC_UNK_32: usize = 0x32; pub fn unk_32() }
    crate::virtual_method! { pub const VFUNC_UNK_33: usize = 0x33; pub fn unk_33() }
    crate::virtual_method! { pub const VFUNC_UNK_34: usize = 0x34; pub fn unk_34() }
    crate::virtual_method! { pub const VFUNC_UNK_35: usize = 0x35; pub fn unk_35() }
    crate::virtual_method! { pub const VFUNC_UNK_36: usize = 0x36; pub fn unk_36() }
    crate::virtual_method! { pub const VFUNC_UNK_37: usize = 0x37; pub fn unk_37() }
    crate::virtual_method! { pub const VFUNC_UNK_38: usize = 0x38; pub fn unk_38() }
    crate::virtual_method! { pub const VFUNC_UNK_39: usize = 0x39; pub fn unk_39() }
    crate::virtual_method! { pub const VFUNC_UNK_3A: usize = 0x3A; pub fn unk_3a() }
    crate::virtual_method! { pub const VFUNC_UNK_3B: usize = 0x3B; pub fn unk_3b() }
    crate::virtual_method! { pub const VFUNC_UNK_3C: usize = 0x3C; pub fn unk_3c() }
    crate::virtual_method! { pub const VFUNC_UNK_3D: usize = 0x3D; pub fn unk_3d() }
    crate::virtual_method! { pub const VFUNC_UNK_3E: usize = 0x3E; pub fn unk_3e() }
    crate::virtual_method! { pub const VFUNC_UNK_3F: usize = 0x3F; pub fn unk_3f() }
    crate::virtual_method! { pub const VFUNC_UNK_40: usize = 0x40; pub fn unk_40() }
    crate::virtual_method! { pub const VFUNC_GET_MOVIE_CLIP: usize = 0x41; pub fn get_movie_clip() -> *mut GASMovieClipObject }
    crate::virtual_method! { pub const VFUNC_UNK_42: usize = 0x42; pub fn unk_42() }
    crate::virtual_method! { pub const VFUNC_UNK_43: usize = 0x43; pub fn unk_43() }
    crate::virtual_method! { pub const VFUNC_UNK_44: usize = 0x44; pub fn unk_44() }
    crate::virtual_method! { pub const VFUNC_UNK_45: usize = 0x45; pub fn unk_45() }
    crate::virtual_method! { pub const VFUNC_UNK_46: usize = 0x46; pub fn unk_46() }
    crate::virtual_method! { pub const VFUNC_UNK_47: usize = 0x47; pub fn unk_47() }
    crate::virtual_method! { pub const VFUNC_UNK_48: usize = 0x48; pub fn unk_48() }
    crate::virtual_method! { pub const VFUNC_UNK_49: usize = 0x49; pub fn unk_49() }
    crate::virtual_method! { pub const VFUNC_UNK_4A: usize = 0x4A; pub fn unk_4a() }
    crate::virtual_method! { pub const VFUNC_UNK_4B: usize = 0x4B; pub fn unk_4b() }
    crate::virtual_method! { pub const VFUNC_UNK_4C: usize = 0x4C; pub fn unk_4c() }
    crate::virtual_method! { pub const VFUNC_UNK_4D: usize = 0x4D; pub fn unk_4d() }
    crate::virtual_method! { pub const VFUNC_UNK_4E: usize = 0x4E; pub fn unk_4e() }
    crate::virtual_method! { pub const VFUNC_UNK_4F: usize = 0x4F; pub fn unk_4f() }
    crate::virtual_method! { pub const VFUNC_UNK_50: usize = 0x50; pub fn unk_50() }
    crate::virtual_method! { pub const VFUNC_UNK_51: usize = 0x51; pub fn unk_51() }
    crate::virtual_method! { pub const VFUNC_UNK_52: usize = 0x52; pub fn unk_52() }
    crate::virtual_method! { pub const VFUNC_UNK_53: usize = 0x53; pub fn unk_53() }
    crate::virtual_method! { pub const VFUNC_UNK_54: usize = 0x54; pub fn unk_54() }
    crate::virtual_method! { pub const VFUNC_UNK_55: usize = 0x55; pub fn unk_55() }
    crate::virtual_method! { pub const VFUNC_UNK_56: usize = 0x56; pub fn unk_56() }
    crate::virtual_method! { pub const VFUNC_UNK_57: usize = 0x57; pub fn unk_57() }
}

impl AsRef<GFxASCharacter> for GFxASCharacter {
    #[inline(always)]
    fn as_ref(&self) -> &GFxASCharacter {
        self
    }
}

impl AsMut<GFxASCharacter> for GFxASCharacter {
    #[inline(always)]
    fn as_mut(&mut self) -> &mut GFxASCharacter {
        self
    }
}

pub trait GFxASCharacterExt: AsRef<GFxASCharacter> + AsMut<GFxASCharacter> {
    #[inline(always)]
    fn unk_31(&mut self) {
        self.as_mut().unk_31()
    }

    #[inline(always)]
    fn unk_32(&mut self) {
        self.as_mut().unk_32()
    }

    #[inline(always)]
    fn unk_33(&mut self) {
        self.as_mut().unk_33()
    }

    #[inline(always)]
    fn unk_34(&mut self) {
        self.as_mut().unk_34()
    }

    #[inline(always)]
    fn unk_35(&mut self) {
        self.as_mut().unk_35()
    }

    #[inline(always)]
    fn unk_36(&mut self) {
        self.as_mut().unk_36()
    }

    #[inline(always)]
    fn unk_37(&mut self) {
        self.as_mut().unk_37()
    }

    #[inline(always)]
    fn unk_38(&mut self) {
        self.as_mut().unk_38()
    }

    #[inline(always)]
    fn unk_39(&mut self) {
        self.as_mut().unk_39()
    }

    #[inline(always)]
    fn unk_3a(&mut self) {
        self.as_mut().unk_3a()
    }

    #[inline(always)]
    fn unk_3b(&mut self) {
        self.as_mut().unk_3b()
    }

    #[inline(always)]
    fn unk_3c(&mut self) {
        self.as_mut().unk_3c()
    }

    #[inline(always)]
    fn unk_3d(&mut self) {
        self.as_mut().unk_3d()
    }

    #[inline(always)]
    fn unk_3e(&mut self) {
        self.as_mut().unk_3e()
    }

    #[inline(always)]
    fn unk_3f(&mut self) {
        self.as_mut().unk_3f()
    }

    #[inline(always)]
    fn unk_40(&mut self) {
        self.as_mut().unk_40()
    }

    #[inline(always)]
    fn get_movie_clip(&mut self) -> *mut GASMovieClipObject {
        self.as_mut().get_movie_clip()
    }

    #[inline(always)]
    fn unk_42(&mut self) {
        self.as_mut().unk_42()
    }

    #[inline(always)]
    fn unk_43(&mut self) {
        self.as_mut().unk_43()
    }

    #[inline(always)]
    fn unk_44(&mut self) {
        self.as_mut().unk_44()
    }

    #[inline(always)]
    fn unk_45(&mut self) {
        self.as_mut().unk_45()
    }

    #[inline(always)]
    fn unk_46(&mut self) {
        self.as_mut().unk_46()
    }

    #[inline(always)]
    fn unk_47(&mut self) {
        self.as_mut().unk_47()
    }

    #[inline(always)]
    fn unk_48(&mut self) {
        self.as_mut().unk_48()
    }

    #[inline(always)]
    fn unk_49(&mut self) {
        self.as_mut().unk_49()
    }

    #[inline(always)]
    fn unk_4a(&mut self) {
        self.as_mut().unk_4a()
    }

    #[inline(always)]
    fn unk_4b(&mut self) {
        self.as_mut().unk_4b()
    }

    #[inline(always)]
    fn unk_4c(&mut self) {
        self.as_mut().unk_4c()
    }

    #[inline(always)]
    fn unk_4d(&mut self) {
        self.as_mut().unk_4d()
    }

    #[inline(always)]
    fn unk_4e(&mut self) {
        self.as_mut().unk_4e()
    }

    #[inline(always)]
    fn unk_4f(&mut self) {
        self.as_mut().unk_4f()
    }

    #[inline(always)]
    fn unk_50(&mut self) {
        self.as_mut().unk_50()
    }

    #[inline(always)]
    fn unk_51(&mut self) {
        self.as_mut().unk_51()
    }

    #[inline(always)]
    fn unk_52(&mut self) {
        self.as_mut().unk_52()
    }

    #[inline(always)]
    fn unk_53(&mut self) {
        self.as_mut().unk_53()
    }

    #[inline(always)]
    fn unk_54(&mut self) {
        self.as_mut().unk_54()
    }

    #[inline(always)]
    fn unk_55(&mut self) {
        self.as_mut().unk_55()
    }

    #[inline(always)]
    fn unk_56(&mut self) {
        self.as_mut().unk_56()
    }

    #[inline(always)]
    fn unk_57(&mut self) {
        self.as_mut().unk_57()
    }
}

impl<T> GFxASCharacterExt for T where T: AsRef<GFxASCharacter> + AsMut<GFxASCharacter> {}
