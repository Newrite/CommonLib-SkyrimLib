use core_util::inherit;

use crate::offsets::offsets_rtti::RTTI_ExtraWornLeft;
use crate::offsets::offsets_vtable::VTABLE_ExtraWornLeft;
use crate::re::{BSExtraData, ExtraDataType, ExtraDataTyped};
use crate::relocation::{RttiType, VariantID};

/// C++ `RE::ExtraWornLeft`
#[repr(C)]
pub struct ExtraWornLeft {
    pub base: BSExtraData, // 00
}

const _: () = assert!(core::mem::size_of::<ExtraWornLeft>() == 0x10);
const _: () = assert!(core::mem::offset_of!(ExtraWornLeft, base) == 0x00);

impl RttiType for ExtraWornLeft {
    const RTTI: VariantID = RTTI_ExtraWornLeft;
}

impl ExtraDataTyped for ExtraWornLeft {
    const EXTRADATATYPE: ExtraDataType = ExtraDataType::WornLeft;
}

inherit!(ExtraWornLeft : BSExtraData);

impl ExtraWornLeft {
    pub const RTTI: VariantID = RTTI_ExtraWornLeft;
    pub const VTABLE: &'static [VariantID] = &VTABLE_ExtraWornLeft;
    pub const EXTRADATATYPE: ExtraDataType = ExtraDataType::WornLeft;

    // override (BSExtraData)
    // ~ExtraWornLeft() override;  // 00
    // ExtraDataType GetType() const override;  // 01 - { return kWornLeft; }
}
