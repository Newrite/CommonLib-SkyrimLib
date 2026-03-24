use core_util::inherit;

use crate::offsets::offsets_rtti::RTTI_ExtraPoison;
use crate::offsets::offsets_vtable::VTABLE_ExtraPoison;
use crate::re::{AlchemyItem, BSExtraData, ExtraDataType, ExtraDataTyped};
use crate::relocation::{RttiType, VariantID};

/// C++ `RE::ExtraPoison`
#[repr(C)]
pub struct ExtraPoison {
    pub base: BSExtraData,        // 00
    pub poison: *mut AlchemyItem, // 10
    pub count: u32,               // 18
    pub pad1c: u32,               // 1C
}

const _: () = assert!(core::mem::size_of::<ExtraPoison>() == 0x20);
const _: () = assert!(core::mem::offset_of!(ExtraPoison, base) == 0x00);
const _: () = assert!(core::mem::offset_of!(ExtraPoison, poison) == 0x10);
const _: () = assert!(core::mem::offset_of!(ExtraPoison, count) == 0x18);
const _: () = assert!(core::mem::offset_of!(ExtraPoison, pad1c) == 0x1C);

impl RttiType for ExtraPoison {
    const RTTI: VariantID = RTTI_ExtraPoison;
}

impl ExtraDataTyped for ExtraPoison {
    const EXTRADATATYPE: ExtraDataType = ExtraDataType::Poison;
}

inherit!(ExtraPoison : BSExtraData);

impl ExtraPoison {
    pub const RTTI: VariantID = RTTI_ExtraPoison;
    pub const VTABLE: &'static [VariantID] = &VTABLE_ExtraPoison;
    pub const EXTRADATATYPE: ExtraDataType = ExtraDataType::Poison;

    // override (BSExtraData)
    // ExtraDataType GetType() const override;  // 01 - { return kPoison; }
    // bool IsNotEqual(const BSExtraData* a_rhs) const override;  // 02

    #[inline(always)]
    pub fn new(poison: *mut AlchemyItem, count: i32) -> Self {
        Self {
            base: BSExtraData {
                vtable: Self::VTABLE[0].address() as *const usize,
                next: core::ptr::null_mut(),
            },
            poison,
            count: count as u32,
            pad1c: 0,
        }
    }

    #[inline(always)]
    pub fn is_not_equal_impl(&self, rhs: *const BSExtraData) -> bool {
        let rhs = rhs.cast::<Self>();
        rhs.is_null()
            || self.poison != unsafe { (*rhs).poison }
            || self.count != unsafe { (*rhs).count }
    }
}
