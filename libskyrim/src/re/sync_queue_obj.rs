use core_util::inherit;

use crate::offsets::offsets_rtti::RTTI_SyncQueueObj;
use crate::offsets::offsets_vtable::VTABLE_SyncQueueObj;
use crate::re::bs_intrusive_ref_counted::BSIntrusiveRefCounted;
use crate::re::bst_smart_pointer::BSTSmartPointerIntrusiveRefCountable;
use crate::relocation::{RttiType, VariantID};
use crate::virtual_method;

/// C++ `RE::SyncQueueObj`
#[repr(C)]
pub struct SyncQueueObj {
    pub vtable: *const usize,        // 00
    pub base: BSIntrusiveRefCounted, // 08
    pub unk0c: u32,                  // 0C
}

const _: () = assert!(core::mem::size_of::<SyncQueueObj>() == 0x10);
const _: () = assert!(core::mem::offset_of!(SyncQueueObj, base) == 0x08);
const _: () = assert!(core::mem::offset_of!(SyncQueueObj, unk0c) == 0x0C);

inherit!(SyncQueueObj : BSIntrusiveRefCounted);

impl RttiType for SyncQueueObj {
    const RTTI: VariantID = RTTI_SyncQueueObj;
}

impl BSTSmartPointerIntrusiveRefCountable for SyncQueueObj {
    #[inline(always)]
    fn bst_inc_ref(&self) {
        self.base.inc_ref();
    }

    #[inline(always)]
    fn bst_dec_ref(&self) -> u32 {
        self.base.dec_ref()
    }

    #[inline(always)]
    unsafe fn bst_delete(&self) {
        self.dtor();
    }
}

impl SyncQueueObj {
    pub const RTTI: VariantID = RTTI_SyncQueueObj;
    pub const VTABLE: &'static [VariantID] = &VTABLE_SyncQueueObj;

    virtual_method! {
        pub const VFUNC_DTOR: usize = 0x00;
        pub fn dtor()
    }

    virtual_method! {
        pub const VFUNC_UNK_01: usize = 0x01;
        pub fn unk_01()
    }
}
