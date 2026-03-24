use core_util::{EnumSet, inherit};

use crate::offsets::offsets_rtti::RTTI_ExtraLevCreaModifier;
use crate::offsets::offsets_vtable::VTABLE_ExtraLevCreaModifier;
use crate::re::{BSExtraData, ExtraDataType, ExtraDataTyped};
use crate::relocation::{RttiType, VariantID};

/// C++ `RE::LEV_CREA_MODIFIER`
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum LevCreaModifier {
    Easy = 0,
    Medium = 1,
    Hard = 2,
    VeryHard = 3,
    None = 4,
}

core_util::impl_enumset_type!(LevCreaModifier => u32);

#[allow(non_camel_case_types)]
pub type LEV_CREA_MODIFIER = LevCreaModifier;

/// C++ `RE::ExtraLevCreaModifier`
#[repr(C)]
pub struct ExtraLevCreaModifier {
    pub base: BSExtraData,                       // 00
    pub modifier: EnumSet<LevCreaModifier, u32>, // 10
    pub pad14: u32,                              // 14
}

const _: () = assert!(core::mem::size_of::<ExtraLevCreaModifier>() == 0x18);
const _: () = assert!(core::mem::offset_of!(ExtraLevCreaModifier, modifier) == 0x10);

impl RttiType for ExtraLevCreaModifier {
    const RTTI: VariantID = RTTI_ExtraLevCreaModifier;
}

impl ExtraDataTyped for ExtraLevCreaModifier {
    const EXTRADATATYPE: ExtraDataType = ExtraDataType::LevCreaModifier;
}

inherit!(ExtraLevCreaModifier : BSExtraData);

impl ExtraLevCreaModifier {
    pub const RTTI: VariantID = RTTI_ExtraLevCreaModifier;
    pub const VTABLE: &'static [VariantID] = &VTABLE_ExtraLevCreaModifier;
    pub const EXTRADATATYPE: ExtraDataType = ExtraDataType::LevCreaModifier;

    // override (BSExtraData)
    // ExtraDataType GetType() const override;  // 01 - { return kLevCreaModifier; }
    // bool IsNotEqual(const BSExtraData* a_rhs) const override;  // 02

    #[inline(always)]
    pub fn new(modifier: LEV_CREA_MODIFIER) -> Self {
        Self {
            base: BSExtraData {
                vtable: Self::VTABLE[0].address() as *const usize,
                next: core::ptr::null_mut(),
            },
            modifier: EnumSet::from_underlying(modifier as u32),
            pad14: 0,
        }
    }

    #[inline(always)]
    pub fn is_not_equal_impl(&self, rhs: *const BSExtraData) -> bool {
        let rhs = rhs.cast::<Self>();
        rhs.is_null() || self.modifier != unsafe { (*rhs).modifier }
    }
}
