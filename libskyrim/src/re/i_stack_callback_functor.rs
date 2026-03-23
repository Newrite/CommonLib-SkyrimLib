use core_util::inherit;

use crate::offsets::offsets_rtti::RTTI_BSScript__IStackCallbackFunctor;
use crate::offsets::offsets_vtable::VTABLE_BSScript__IStackCallbackFunctor;
use crate::re::bs_intrusive_ref_counted::BSIntrusiveRefCounted;
use crate::re::bsscript_object::Object;
use crate::re::bst_smart_pointer::{BSTSmartPointer, BSTSmartPointerIntrusiveRefCountable};
use crate::re::variable::Variable;
use crate::relocation::{RttiType, VariantID};
use crate::virtual_method;

/// C++ `RE::BSScript::IStackCallbackFunctor`
#[repr(C)]
pub struct IStackCallbackFunctor {
    pub vtable: *const usize,        // 00
    pub base: BSIntrusiveRefCounted, // 08
    pub pad0c: u32,                  // 0C
}

const _: () = assert!(core::mem::size_of::<IStackCallbackFunctor>() == 0x10);
const _: () = assert!(core::mem::offset_of!(IStackCallbackFunctor, base) == 0x8);
const _: () = assert!(core::mem::offset_of!(IStackCallbackFunctor, pad0c) == 0xC);

inherit!(IStackCallbackFunctor : BSIntrusiveRefCounted);

impl RttiType for IStackCallbackFunctor {
    const RTTI: VariantID = RTTI_BSScript__IStackCallbackFunctor;
}

impl BSTSmartPointerIntrusiveRefCountable for IStackCallbackFunctor {
    #[inline(always)]
    fn bst_inc_ref(&self) {
        self.base.inc_ref();
    }

    #[inline(always)]
    fn bst_dec_ref(&self) -> u32 {
        self.base.dec_ref()
    }

    #[inline]
    unsafe fn bst_delete(&self) {
        self.dtor();
    }
}

impl IStackCallbackFunctor {
    pub const RTTI: VariantID = RTTI_BSScript__IStackCallbackFunctor;
    pub const VTABLE: &'static [VariantID] = &VTABLE_BSScript__IStackCallbackFunctor;

    virtual_method! {
        pub const VFUNC_DTOR: usize = 0x00;
        pub fn dtor()
    }

    virtual_method! {
        pub const VFUNC_CALL: usize = 0x01;
        pub fn call(result: Variable)
    }

    virtual_method! {
        pub const VFUNC_CAN_SAVE: usize = 0x02;
        pub fn can_save() -> bool
    }

    virtual_method! {
        pub const VFUNC_SET_OBJECT: usize = 0x03;
        pub fn set_object(object: &BSTSmartPointer<Object>)
    }
}
