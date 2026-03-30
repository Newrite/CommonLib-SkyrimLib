#![allow(non_camel_case_types)]

use core_util::inherit;

use crate::re::{GFxLogBase, GFxStateBag, GPtrTarget, GRefCountBase, GStatGroups};

/// C++ `RE::GFxStateBagImpl`
#[repr(C)]
pub struct GFxStateBagImpl {
    pub base: GRefCountBase<GFxStateBagImpl, { GStatGroups::DEFAULT_MEM as u32 }>, // 00
    pub state_bag: GFxStateBag,                                                    // 10
    pub pad18: u64,                                                                // 18
    pub log_base: GFxLogBase<GFxStateBagImpl>,                                     // 20
    pub unk28: [u64; 7],                                                           // 28
}

const _: () = assert!(core::mem::size_of::<GFxStateBagImpl>() == 0x60);
const _: () = assert!(core::mem::offset_of!(GFxStateBagImpl, base) == 0x0);
const _: () = assert!(core::mem::offset_of!(GFxStateBagImpl, state_bag) == 0x10);
const _: () = assert!(core::mem::offset_of!(GFxStateBagImpl, log_base) == 0x20);

inherit!(GFxStateBagImpl : GRefCountBase<GFxStateBagImpl, { GStatGroups::DEFAULT_MEM as u32 }>, base);
inherit!(GFxStateBagImpl => GFxStateBag, state_bag);
inherit!(GFxStateBagImpl => GFxLogBase<GFxStateBagImpl>, log_base);

impl GPtrTarget for GFxStateBagImpl {
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
