#![allow(non_camel_case_types)]

use core_util::inherit;

use crate::re::{GFxStatMovieData, GMutex, GPtrTarget, GRefCountBase};

/// C++ `RE::GFxLoadUpdateSync`
#[repr(C)]
pub struct GFxLoadUpdateSync {
    pub base: GRefCountBase<GFxLoadUpdateSync, { GFxStatMovieData::kGFxStatMD_Other_Mem as u32 }>, // 00
    pub mutex: GMutex,                 // 10
    pub unk38: *mut core::ffi::c_void, // 38
    pub unk40: *mut core::ffi::c_void, // 40
}

const _: () = assert!(core::mem::size_of::<GFxLoadUpdateSync>() == 0x48);
const _: () = assert!(core::mem::offset_of!(GFxLoadUpdateSync, base) == 0x0);
const _: () = assert!(core::mem::offset_of!(GFxLoadUpdateSync, mutex) == 0x10);
const _: () = assert!(core::mem::offset_of!(GFxLoadUpdateSync, unk38) == 0x38);

inherit!(GFxLoadUpdateSync : GRefCountBase<GFxLoadUpdateSync, { GFxStatMovieData::kGFxStatMD_Other_Mem as u32 }>, base);

impl GFxLoadUpdateSync {
    // override (GRefCountImpl)
    crate::virtual_method! { pub const VFUNC_DTOR: usize = 0x00; pub fn dtor() }
}

impl GPtrTarget for GFxLoadUpdateSync {
    #[inline(always)]
    fn gptr_add_ref(&self) {
        unsafe {
            (*(core::ptr::from_ref(self).cast_mut())).base.add_ref();
        }
    }

    #[inline(always)]
    fn gptr_release(&self) {
        unsafe {
            (*(core::ptr::from_ref(self).cast_mut())).base.release();
        }
    }
}
