use core_util::inherit;

use crate::offsets::offsets_rtti::RTTI_ExtraCharge;
use crate::offsets::offsets_vtable::VTABLE_ExtraCharge;
use crate::re::{BSExtraData, ExtraDataType, ExtraDataTyped};
use crate::relocation::{RttiType, VariantID};

/// C++ `RE::ExtraCharge`
#[repr(C)]
pub struct ExtraCharge {
    pub base: BSExtraData, // 00
    pub charge: f32,       // 10
    pub pad14: u32,        // 14
}

const _: () = assert!(core::mem::size_of::<ExtraCharge>() == 0x18);
const _: () = assert!(core::mem::offset_of!(ExtraCharge, base) == 0x00);
const _: () = assert!(core::mem::offset_of!(ExtraCharge, charge) == 0x10);
const _: () = assert!(core::mem::offset_of!(ExtraCharge, pad14) == 0x14);

impl RttiType for ExtraCharge {
    const RTTI: VariantID = RTTI_ExtraCharge;
}

impl ExtraDataTyped for ExtraCharge {
    const EXTRADATATYPE: ExtraDataType = ExtraDataType::Charge;
}

inherit!(ExtraCharge : BSExtraData);

impl ExtraCharge {
    pub const RTTI: VariantID = RTTI_ExtraCharge;
    pub const VTABLE: &'static [VariantID] = &VTABLE_ExtraCharge;
    pub const EXTRADATATYPE: ExtraDataType = ExtraDataType::Charge;

    // override (BSExtraData)
    // ExtraDataType GetType() const override;  // 01 - { return kCharge; }
    // bool IsNotEqual(const BSExtraData* a_rhs) const override;  // 02

    #[inline(always)]
    pub fn new(charge: f32) -> Self {
        Self {
            base: BSExtraData {
                vtable: Self::VTABLE[0].address() as *const usize,
                next: core::ptr::null_mut(),
            },
            charge,
            pad14: 0,
        }
    }

    #[inline(always)]
    pub fn is_not_equal_impl(&self, rhs: *const BSExtraData) -> bool {
        let rhs = rhs.cast::<Self>();
        rhs.is_null() || self.charge != unsafe { (*rhs).charge }
    }
}
