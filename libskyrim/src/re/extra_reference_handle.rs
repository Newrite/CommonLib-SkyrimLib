use core_util::inherit;

use crate::offsets::offsets_rtti::RTTI_ExtraReferenceHandle;
use crate::offsets::offsets_vtable::VTABLE_ExtraReferenceHandle;
use crate::re::{
    BSExtraData, ExtraDataType, ExtraDataTyped, NiPointer, ObjectRefHandle, TESObjectREFR,
};
use crate::relocation::{RttiType, VariantID};

/// C++ `RE::ExtraReferenceHandle`
#[repr(C)]
pub struct ExtraReferenceHandle {
    pub base: BSExtraData,              // 00
    pub container_ref: ObjectRefHandle, // 10
    pub pad14: u32,                     // 14
}

const _: () = assert!(core::mem::size_of::<ExtraReferenceHandle>() == 0x18);
const _: () = assert!(core::mem::offset_of!(ExtraReferenceHandle, container_ref) == 0x10);
const _: () = assert!(core::mem::offset_of!(ExtraReferenceHandle, pad14) == 0x14);

impl RttiType for ExtraReferenceHandle {
    const RTTI: VariantID = RTTI_ExtraReferenceHandle;
}

impl ExtraDataTyped for ExtraReferenceHandle {
    const EXTRADATATYPE: ExtraDataType = ExtraDataType::ReferenceHandle;
}

inherit!(ExtraReferenceHandle : BSExtraData);

impl ExtraReferenceHandle {
    pub const RTTI: VariantID = RTTI_ExtraReferenceHandle;
    pub const VTABLE: &'static [VariantID] = &VTABLE_ExtraReferenceHandle;
    pub const EXTRADATATYPE: ExtraDataType = ExtraDataType::ReferenceHandle;

    // override (BSExtraData)
    // ExtraDataType GetType() const override;  // 01 - { return kReferenceHandle; }
    // bool IsNotEqual(const BSExtraData* a_rhs) const override;  // 02

    #[inline(always)]
    pub fn new(container_ref: ObjectRefHandle) -> Self {
        Self {
            base: BSExtraData {
                vtable: Self::VTABLE[0].address() as *const usize,
                next: core::ptr::null_mut(),
            },
            container_ref,
            pad14: 0,
        }
    }

    #[inline(always)]
    pub fn is_not_equal_impl(&self, rhs: *const BSExtraData) -> bool {
        let rhs = rhs.cast::<Self>();
        rhs.is_null() || self.container_ref != unsafe { (*rhs).container_ref }
    }

    #[inline(always)]
    pub fn get_original_reference(&self) -> NiPointer<TESObjectREFR> {
        self.container_ref.get()
    }
}
