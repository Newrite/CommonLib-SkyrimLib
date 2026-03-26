use core_util::inherit;

use crate::offsets::offsets_rtti::RTTI_BGSLocAlias;
use crate::offsets::offsets_vtable::VTABLE_BGSLocAlias;
use crate::re::{BGSBaseAlias, TESCondition, VMTypeID};
use crate::relocation::{RttiType, VariantID};
use crate::virtual_method;

/// C++ `RE::BGSLocAlias`
#[repr(C)]
pub struct BGSLocAlias {
    pub base: BGSBaseAlias,            // 00
    pub unk28: u64,                    // 28
    pub unk30: u64,                    // 30
    pub unk38: u64,                    // 38
    pub unk40: u64,                    // 40
    pub unk48: u64,                    // 48
    pub unk50: u64,                    // 50
    pub conditions: *mut TESCondition, // 58
}

const _: () = assert!(core::mem::size_of::<BGSLocAlias>() == 0x60);
const _: () = assert!(core::mem::offset_of!(BGSLocAlias, conditions) == 0x58);

impl RttiType for BGSLocAlias {
    const RTTI: VariantID = RTTI_BGSLocAlias;
}

inherit!(BGSLocAlias : BGSBaseAlias);

impl BGSLocAlias {
    pub const RTTI: VariantID = RTTI_BGSLocAlias;
    pub const VTABLE: &'static [VariantID] = &VTABLE_BGSLocAlias;
    pub const VM_TYPE_ID: VMTypeID = 141;

    virtual_method! {
        pub const VFUNC_DTOR: usize = 0x00;
        pub fn dtor(&mut self)
    }
}
