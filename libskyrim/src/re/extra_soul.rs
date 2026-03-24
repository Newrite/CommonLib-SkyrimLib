use core_util::{EnumSet, inherit};

use crate::offsets::offsets_rtti::RTTI_ExtraSoul;
use crate::offsets::offsets_vtable::VTABLE_ExtraSoul;
use crate::re::{BSExtraData, ExtraDataType, ExtraDataTyped, SOUL_LEVEL};
use crate::relocation::{RttiType, VariantID};

/// C++ `RE::ExtraSoul`
#[repr(C)]
pub struct ExtraSoul {
    pub base: BSExtraData,             // 00
    pub soul: EnumSet<SOUL_LEVEL, u8>, // 10
    pub pad11: u8,                     // 11
    pub pad12: u16,                    // 12
    pub pad14: u32,                    // 14
}

const _: () = assert!(core::mem::size_of::<ExtraSoul>() == 0x18);
const _: () = assert!(core::mem::offset_of!(ExtraSoul, soul) == 0x10);

impl RttiType for ExtraSoul {
    const RTTI: VariantID = RTTI_ExtraSoul;
}

impl ExtraDataTyped for ExtraSoul {
    const EXTRADATATYPE: ExtraDataType = ExtraDataType::Soul;
}

inherit!(ExtraSoul : BSExtraData);

impl ExtraSoul {
    pub const RTTI: VariantID = RTTI_ExtraSoul;
    pub const VTABLE: &'static [VariantID] = &VTABLE_ExtraSoul;
    pub const EXTRADATATYPE: ExtraDataType = ExtraDataType::Soul;

    // override (BSExtraData)
    // ExtraDataType GetType() const override;  // 01 - { return kSoul; }
    // bool IsNotEqual(const BSExtraData* a_rhs) const override;  // 02

    #[inline(always)]
    pub fn new(level: SOUL_LEVEL) -> Self {
        Self {
            base: BSExtraData {
                vtable: Self::VTABLE[0].address() as *const usize,
                next: core::ptr::null_mut(),
            },
            soul: EnumSet::from_underlying(level as u8),
            pad11: 0,
            pad12: 0,
            pad14: 0,
        }
    }

    #[inline(always)]
    pub fn is_not_equal_impl(&self, rhs: *const BSExtraData) -> bool {
        let rhs = rhs.cast::<Self>();
        rhs.is_null() || self.soul != unsafe { (*rhs).soul }
    }

    #[inline(always)]
    pub fn get_contained_soul(&self) -> SOUL_LEVEL {
        self.soul.get().unwrap_or(SOUL_LEVEL::None)
    }
}
