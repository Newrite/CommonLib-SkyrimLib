use crate::offsets::offsets_rtti::RTTI_MagicItemTraversalFunctor;
use crate::offsets::offsets_vtable::VTABLE_MagicItemTraversalFunctor;
use crate::re::Effect;
use crate::re::bs_container::BSContainerForEachResult;
use crate::relocation::{RttiType, VariantID};
use crate::virtual_method;

/// C++ `RE::MagicItemTraversalFunctor`
#[repr(C)]
pub struct MagicItemTraversalFunctor {
    pub vtable: *const usize, // 00
    pub index: u32,           // 08
    pub pad0c: u32,           // 0C
}

const _: () = assert!(core::mem::size_of::<MagicItemTraversalFunctor>() == 0x10);
const _: () = assert!(core::mem::offset_of!(MagicItemTraversalFunctor, index) == 0x08);
const _: () = assert!(core::mem::offset_of!(MagicItemTraversalFunctor, pad0c) == 0x0C);

impl RttiType for MagicItemTraversalFunctor {
    const RTTI: VariantID = RTTI_MagicItemTraversalFunctor;
}

impl MagicItemTraversalFunctor {
    pub const RTTI: VariantID = RTTI_MagicItemTraversalFunctor;
    pub const VTABLE: &'static [VariantID] = &VTABLE_MagicItemTraversalFunctor;

    virtual_method! {
        pub const VFUNC_DTOR: usize = 0x00;
        pub fn dtor()
    }

    virtual_method! {
        pub const VFUNC_CALL: usize = 0x01;
        pub fn call(effect: *mut Effect) -> BSContainerForEachResult
    }
}
