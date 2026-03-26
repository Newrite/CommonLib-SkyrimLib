use core_util::inherit;

use crate::offsets::offsets_rtti::RTTI_ExtraProcessMiddleLow;
use crate::offsets::offsets_vtable::VTABLE_ExtraProcessMiddleLow;
use crate::re::{BSExtraData, ExtraDataType, ExtraDataTyped};
use crate::relocation::{RttiType, VariantID};

/// C++ `RE::ExtraProcessMiddleLow`
#[repr(C)]
pub struct ExtraProcessMiddleLow {
    pub base: BSExtraData, // 00
    pub ref_count: u32,    // 10
    pub pad14: u32,        // 14
}

const _: () = assert!(core::mem::size_of::<ExtraProcessMiddleLow>() == 0x18);
const _: () = assert!(core::mem::offset_of!(ExtraProcessMiddleLow, ref_count) == 0x10);

impl RttiType for ExtraProcessMiddleLow {
    const RTTI: VariantID = RTTI_ExtraProcessMiddleLow;
}

impl ExtraDataTyped for ExtraProcessMiddleLow {
    const EXTRADATATYPE: ExtraDataType = ExtraDataType::ProcessMiddleLow;
}

inherit!(ExtraProcessMiddleLow : BSExtraData);

impl ExtraProcessMiddleLow {
    pub const RTTI: VariantID = RTTI_ExtraProcessMiddleLow;
    pub const VTABLE: &'static [VariantID] = &VTABLE_ExtraProcessMiddleLow;
    pub const EXTRADATATYPE: ExtraDataType = ExtraDataType::ProcessMiddleLow;
}
