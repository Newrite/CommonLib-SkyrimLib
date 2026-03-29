#![allow(non_camel_case_types)]

use crate::offsets::offsets_rtti::RTTI_GRefCountImplCore;
use crate::offsets::offsets_vtable::VTABLE_GRefCountImplCore;
use crate::relocation::{RttiType, VariantID};

/// C++ `RE::GRefCountImplCore`
#[repr(C)]
pub struct GRefCountImplCore {
    pub vtable: *const usize, // 00
    pub ref_count: u32,       // 08
    pub pad0c: u32,           // 0C
}

const _: () = assert!(core::mem::size_of::<GRefCountImplCore>() == 0x10);
const _: () = assert!(core::mem::offset_of!(GRefCountImplCore, vtable) == 0x0);
const _: () = assert!(core::mem::offset_of!(GRefCountImplCore, ref_count) == 0x8);
const _: () = assert!(core::mem::offset_of!(GRefCountImplCore, pad0c) == 0xC);

impl RttiType for GRefCountImplCore {
    const RTTI: VariantID = RTTI_GRefCountImplCore;
}

impl GRefCountImplCore {
    pub const RTTI: VariantID = RTTI_GRefCountImplCore;
    pub const VTABLE: &'static [VariantID] = &VTABLE_GRefCountImplCore;

    crate::virtual_method! {
        pub const VFUNC_DTOR: usize = 0x0;
        pub fn dtor()
    }

    #[inline(always)]
    pub fn check_invalid_delete(_: *mut Self) {}

    #[inline(always)]
    pub const fn get_ref_count(&self) -> u32 {
        self.ref_count
    }
}
