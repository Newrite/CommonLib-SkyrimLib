#![allow(non_camel_case_types)]

use core_util::inherit;

use crate::re::{GPtrTarget, GRefCountBase, GStatGroup};

/// C++ `RE::GFxMovieDefBindStates`
#[repr(C)]
pub struct GFxMovieDefBindStates {
    pub base: GRefCountBase<GFxMovieDefBindStates, { GStatGroup::kGStat_Default_Mem as u32 }>, // 00
    // TODO: `GFxMovieDefBindStates.h` only forward-declares these helper types at
    // this layer. Restore the named pointer fields once their reusable RE
    // translations exist.
    pub file_opener: *mut core::ffi::c_void,           // 10
    pub url_builder: *mut core::ffi::c_void,           // 18
    pub image_creator: *mut core::ffi::c_void,         // 20
    pub import_visitor: *mut core::ffi::c_void,        // 28
    pub gradient_params: *mut core::ffi::c_void,       // 30
    pub font_pack_params: *mut core::ffi::c_void,      // 38
    pub preprocess_params: *mut core::ffi::c_void,     // 40
    pub font_compactor_params: *mut core::ffi::c_void, // 48
    pub image_packer_params: *mut core::ffi::c_void,   // 50
}

const _: () = assert!(core::mem::size_of::<GFxMovieDefBindStates>() == 0x58);
const _: () = assert!(core::mem::offset_of!(GFxMovieDefBindStates, base) == 0x0);
const _: () = assert!(core::mem::offset_of!(GFxMovieDefBindStates, file_opener) == 0x10);

inherit!(GFxMovieDefBindStates : GRefCountBase<GFxMovieDefBindStates, { GStatGroup::kGStat_Default_Mem as u32 }>, base);

impl GPtrTarget for GFxMovieDefBindStates {
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
