#![allow(non_camel_case_types)]

use core_util::inherit;

use crate::re::{GFxStatMovieViews, GMemoryHeap, GPtrTarget, GRefCountBase, GStringHash};

/// C++ `RE::GASString`
#[repr(C)]
pub struct GASString {
    pub str_: *const i8,      // 00
    pub next: *mut GASString, // 08
    pub ref_count: i32,       // 10
    pub hash: i32,            // 14
    pub size: u32,            // 18
    pub pad1c: u32,           // 1C
}

const _: () = assert!(core::mem::size_of::<GASString>() == 0x20);
const _: () = assert!(core::mem::offset_of!(GASString, str_) == 0x0);
const _: () = assert!(core::mem::offset_of!(GASString, next) == 0x8);
const _: () = assert!(core::mem::offset_of!(GASString, ref_count) == 0x10);

/// C++ `RE::GASStringManager`
#[repr(C)]
pub struct GASStringManager {
    pub base:
        GRefCountBase<GASStringManager, { GFxStatMovieViews::kGFxStatMV_ActionScript_Mem as u32 }>, // 00
    pub unk10: GStringHash<GASString>, // 10
    pub heap: *mut GMemoryHeap,        // 18
    pub unk20: u64,                    // 20
    pub unk28: *mut core::ffi::c_void, // 28
    pub unk30: u64,                    // 30
    pub unk38: u64,                    // 38
    pub unk40: u64,                    // 40
    pub unk48: u64,                    // 48
    pub unk50: u64,                    // 50
}

const _: () = assert!(core::mem::size_of::<GASStringManager>() == 0x58);
const _: () = assert!(core::mem::offset_of!(GASStringManager, base) == 0x0);
const _: () = assert!(core::mem::offset_of!(GASStringManager, unk10) == 0x10);
const _: () = assert!(core::mem::offset_of!(GASStringManager, heap) == 0x18);

inherit!(GASStringManager : GRefCountBase<GASStringManager, { GFxStatMovieViews::kGFxStatMV_ActionScript_Mem as u32 }>, base);

impl GASStringManager {
    // override (GRefCountImpl)
    crate::virtual_method! { pub const VFUNC_DTOR: usize = 0x00; pub fn dtor() }
}

impl GPtrTarget for GASStringManager {
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
