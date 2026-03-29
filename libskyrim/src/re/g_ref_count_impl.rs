#![allow(non_camel_case_types)]

use core::sync::atomic::{AtomicU32, Ordering};

use core_util::inherit;

use crate::offsets::offsets_rtti::RTTI_GRefCountImpl;
use crate::offsets::offsets_vtable::VTABLE_GRefCountImpl;
use crate::re::GRefCountImplCore;
use crate::relocation::{RttiType, VariantID};

pub trait GRefCountImplOps {
    fn add_ref_impl(&mut self);
    fn release_impl(&mut self);
}

/// C++ `RE::GRefCountImpl`
#[repr(C)]
pub struct GRefCountImpl {
    pub base: GRefCountImplCore, // 00
}

const _: () = assert!(core::mem::size_of::<GRefCountImpl>() == 0x10);
const _: () = assert!(core::mem::offset_of!(GRefCountImpl, base) == 0x0);

inherit!(GRefCountImpl : GRefCountImplCore, base);

impl RttiType for GRefCountImpl {
    const RTTI: VariantID = RTTI_GRefCountImpl;
}

impl GRefCountImpl {
    pub const RTTI: VariantID = RTTI_GRefCountImpl;
    pub const VTABLE: &'static [VariantID] = &VTABLE_GRefCountImpl;

    #[inline(always)]
    pub fn add_ref(&mut self) {
        let counter = unsafe { &*(core::ptr::addr_of!(self.base.ref_count).cast::<AtomicU32>()) };
        counter.fetch_add(1, Ordering::AcqRel);
    }

    #[inline(always)]
    pub fn release(&mut self) {
        let counter = unsafe { &*(core::ptr::addr_of!(self.base.ref_count).cast::<AtomicU32>()) };
        if counter.fetch_sub(1, Ordering::AcqRel) == 1 {
            self.dtor();
        }
    }
}

impl GRefCountImplOps for GRefCountImpl {
    #[inline(always)]
    fn add_ref_impl(&mut self) {
        self.add_ref();
    }

    #[inline(always)]
    fn release_impl(&mut self) {
        self.release();
    }
}
