#![allow(non_camel_case_types)]

use core_util::inherit;

use crate::re::{GImageBase, GImageBaseImageFormat, GPtrTarget, GRefCountBaseNTS, GStatGroup};

pub type GImageImageFormat = GImageBaseImageFormat;

/// C++ `RE::GImage`
#[repr(C)]
pub struct GImage {
    pub base: GRefCountBaseNTS<GImage, { GStatGroup::IMAGE_MEM as u32 }>, // 00
    pub image_base: GImageBase,                                            // 10
}

const _: () = assert!(core::mem::size_of::<GImage>() == 0x48);
const _: () = assert!(core::mem::offset_of!(GImage, base) == 0x0);
const _: () = assert!(core::mem::offset_of!(GImage, image_base) == 0x10);

inherit!(GImage : GRefCountBaseNTS<GImage, { GStatGroup::IMAGE_MEM as u32 }>, base);
inherit!(GImage => GImageBase, image_base);

impl GPtrTarget for GImage {
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
