use core_util::inherit;

use crate::offsets::offsets_rtti::RTTI_ExtraOwnership;
use crate::offsets::offsets_vtable::VTABLE_ExtraOwnership;
use crate::re::{BSExtraData, ExtraDataType, ExtraDataTyped, TESForm};
use crate::relocation::{RttiType, VariantID};

/// C++ `RE::ExtraOwnership`
#[repr(C)]
pub struct ExtraOwnership {
    pub base: BSExtraData,   // 00
    pub owner: *mut TESForm, // 10
}

const _: () = assert!(core::mem::size_of::<ExtraOwnership>() == 0x18);
const _: () = assert!(core::mem::offset_of!(ExtraOwnership, owner) == 0x10);

impl RttiType for ExtraOwnership {
    const RTTI: VariantID = RTTI_ExtraOwnership;
}

impl ExtraDataTyped for ExtraOwnership {
    const EXTRADATATYPE: ExtraDataType = ExtraDataType::Ownership;
}

inherit!(ExtraOwnership : BSExtraData);

impl ExtraOwnership {
    pub const RTTI: VariantID = RTTI_ExtraOwnership;
    pub const VTABLE: &'static [VariantID] = &VTABLE_ExtraOwnership;
    pub const EXTRADATATYPE: ExtraDataType = ExtraDataType::Ownership;

    // override (BSExtraData)
    // ExtraDataType GetType() const override;  // 01 - { return kOwnership; }
    // bool IsNotEqual(const BSExtraData* a_rhs) const override;  // 02

    #[inline(always)]
    pub fn new(owner: *mut TESForm) -> Self {
        Self {
            base: BSExtraData {
                vtable: Self::VTABLE[0].address() as *const usize,
                next: core::ptr::null_mut(),
            },
            owner,
        }
    }

    #[inline(always)]
    pub fn is_not_equal_impl(&self, rhs: *const BSExtraData) -> bool {
        let rhs = rhs.cast::<Self>();
        rhs.is_null() || self.owner != unsafe { (*rhs).owner }
    }
}
