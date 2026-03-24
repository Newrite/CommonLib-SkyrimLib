use core_util::inherit;

use crate::offsets::offsets_rtti::RTTI_ExtraNorthRotation;
use crate::offsets::offsets_vtable::VTABLE_ExtraNorthRotation;
use crate::re::{BSExtraData, ExtraDataType, ExtraDataTyped};
use crate::relocation::{RttiType, VariantID};

/// C++ `RE::ExtraNorthRotation`
#[repr(C)]
pub struct ExtraNorthRotation {
    pub base: BSExtraData, // 00
    pub north_rot: f32,    // 10
    pub pad14: u32,        // 14
}

const _: () = assert!(core::mem::size_of::<ExtraNorthRotation>() == 0x18);
const _: () = assert!(core::mem::offset_of!(ExtraNorthRotation, base) == 0x00);
const _: () = assert!(core::mem::offset_of!(ExtraNorthRotation, north_rot) == 0x10);
const _: () = assert!(core::mem::offset_of!(ExtraNorthRotation, pad14) == 0x14);

impl RttiType for ExtraNorthRotation {
    const RTTI: VariantID = RTTI_ExtraNorthRotation;
}

impl ExtraDataTyped for ExtraNorthRotation {
    const EXTRADATATYPE: ExtraDataType = ExtraDataType::NorthRotation;
}

inherit!(ExtraNorthRotation : BSExtraData);

impl ExtraNorthRotation {
    pub const RTTI: VariantID = RTTI_ExtraNorthRotation;
    pub const VTABLE: &'static [VariantID] = &VTABLE_ExtraNorthRotation;
    pub const EXTRADATATYPE: ExtraDataType = ExtraDataType::NorthRotation;

    // override (BSExtraData)
    // ExtraDataType GetType() const override;  // 01 - { return kNorthRotation; }

    #[inline(always)]
    pub fn new(north_rot: f32) -> Self {
        Self {
            base: BSExtraData {
                vtable: Self::VTABLE[0].address() as *const usize,
                next: core::ptr::null_mut(),
            },
            north_rot,
            pad14: 0,
        }
    }
}
