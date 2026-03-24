use core_util::inherit;

use crate::offsets::offsets_rtti::RTTI_ExtraLinkedRef;
use crate::offsets::offsets_vtable::VTABLE_ExtraLinkedRef;
use crate::re::{
    BGSKeyword, BSExtraData, BSTSmallArray, ExtraDataType, ExtraDataTyped, TESObjectREFR,
};
use crate::relocation::{RelocationID, RttiType, VariantID};

/// C++ `RE::ExtraLinkedRef::LinkedRef`
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct ExtraLinkedRefEntry {
    pub keyword: *mut BGSKeyword, // 00
    pub refr: *mut TESObjectREFR, // 08
}

const _: () = assert!(core::mem::size_of::<ExtraLinkedRefEntry>() == 0x10);
const _: () = assert!(core::mem::offset_of!(ExtraLinkedRefEntry, keyword) == 0x00);
const _: () = assert!(core::mem::offset_of!(ExtraLinkedRefEntry, refr) == 0x08);

/// C++ `RE::ExtraLinkedRef`
#[repr(C)]
pub struct ExtraLinkedRef {
    pub base: BSExtraData,                                     // 00
    pub linked_refs: BSTSmallArray<ExtraLinkedRefEntry, 0x10>, // 10
}

const _: () = assert!(core::mem::size_of::<ExtraLinkedRef>() == 0x30);
const _: () = assert!(core::mem::offset_of!(ExtraLinkedRef, linked_refs) == 0x10);

impl RttiType for ExtraLinkedRef {
    const RTTI: VariantID = RTTI_ExtraLinkedRef;
}

impl ExtraDataTyped for ExtraLinkedRef {
    const EXTRADATATYPE: ExtraDataType = ExtraDataType::LinkedRef;
}

inherit!(ExtraLinkedRef : BSExtraData);

impl ExtraLinkedRef {
    pub const RTTI: VariantID = RTTI_ExtraLinkedRef;
    pub const VTABLE: &'static [VariantID] = &VTABLE_ExtraLinkedRef;
    pub const EXTRADATATYPE: ExtraDataType = ExtraDataType::LinkedRef;

    // override (BSExtraData)
    // ExtraDataType GetType() const override;  // 01 - { return kLinkedRef; }
    // bool IsNotEqual(const BSExtraData* a_rhs) const override;  // 02

    crate::relocation_func! {
        pub fn is_not_equal_impl(&self, rhs: *const BSExtraData) -> bool => RelocationID::new(12405, 12539)
    }

    #[inline(always)]
    pub fn linked_refs_slice(&self) -> &[ExtraLinkedRefEntry] {
        unsafe { self.linked_refs.as_slice() }
    }

    #[inline(always)]
    pub fn linked_refs_slice_mut(&mut self) -> &mut [ExtraLinkedRefEntry] {
        unsafe { self.linked_refs.as_mut_slice() }
    }
}
