use core_util::inherit;

use crate::offsets::offsets_rtti::RTTI_ExtraWorn;
use crate::offsets::offsets_vtable::VTABLE_ExtraWorn;
use crate::re::{BSExtraData, ExtraDataType, ExtraDataTyped};
use crate::relocation::{RttiType, VariantID};

/// C++ `RE::ExtraWorn`
#[repr(C)]
pub struct ExtraWorn {
    pub base: BSExtraData, // 00
}

const _: () = assert!(core::mem::size_of::<ExtraWorn>() == 0x10);
const _: () = assert!(core::mem::offset_of!(ExtraWorn, base) == 0x00);

impl RttiType for ExtraWorn {
    const RTTI: VariantID = RTTI_ExtraWorn;
}

impl ExtraDataTyped for ExtraWorn {
    const EXTRADATATYPE: ExtraDataType = ExtraDataType::Worn;
}

inherit!(ExtraWorn : BSExtraData);

impl ExtraWorn {
    pub const RTTI: VariantID = RTTI_ExtraWorn;
    pub const VTABLE: &'static [VariantID] = &VTABLE_ExtraWorn;
    pub const EXTRADATATYPE: ExtraDataType = ExtraDataType::Worn;

    // override (BSExtraData)
    // ~ExtraWorn() override;  // 00
    // ExtraDataType GetType() const override;  // 01 - { return kWorn; }
}
