#![allow(non_camel_case_types)]

use core_util::inherit;

use crate::re::{GRefCountImplCore, GRefCountImplOps};

/// C++ `RE::GRefCountNTSImpl`
#[repr(C)]
pub struct GRefCountNTSImpl {
    pub base: GRefCountImplCore, // 00
}

const _: () = assert!(core::mem::size_of::<GRefCountNTSImpl>() == 0x10);
const _: () = assert!(core::mem::offset_of!(GRefCountNTSImpl, base) == 0x0);

inherit!(GRefCountNTSImpl : GRefCountImplCore, base);

impl GRefCountNTSImpl {
    #[inline(always)]
    pub fn add_ref(&mut self) {
        self.base.ref_count = self.base.ref_count.wrapping_add(1);
    }

    #[inline(always)]
    pub fn release(&mut self) {
        self.base.ref_count = self.base.ref_count.wrapping_sub(1);
        if self.base.ref_count == 0 {
            self.dtor();
        }
    }
}

impl GRefCountImplOps for GRefCountNTSImpl {
    #[inline(always)]
    fn add_ref_impl(&mut self) {
        self.add_ref();
    }

    #[inline(always)]
    fn release_impl(&mut self) {
        self.release();
    }
}
