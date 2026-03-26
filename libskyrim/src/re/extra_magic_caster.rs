use crate::offsets::offsets_rtti::RTTI_ExtraMagicCaster;
use crate::offsets::offsets_vtable::VTABLE_ExtraMagicCaster;
use crate::re::{BSExtraData, ExtraDataType, ExtraDataTyped, MagicCaster};
use crate::relocation::{RttiType, VariantID};

/// C++ `RE::ExtraMagicCaster`
#[repr(C)]
pub struct ExtraMagicCaster {
    pub base: BSExtraData,         // 00
    pub magic_caster: MagicCaster, // 10
}

const _: () = assert!(core::mem::size_of::<ExtraMagicCaster>() == 0x58);
const _: () = assert!(core::mem::offset_of!(ExtraMagicCaster, base) == 0x00);
const _: () = assert!(core::mem::offset_of!(ExtraMagicCaster, magic_caster) == 0x10);

impl ExtraDataTyped for ExtraMagicCaster {
    const EXTRADATATYPE: ExtraDataType = ExtraDataType::MagicCaster;
}

impl RttiType for ExtraMagicCaster {
    const RTTI: VariantID = RTTI_ExtraMagicCaster;
}

core_util::inherit!(ExtraMagicCaster : BSExtraData);
core_util::inherit!(ExtraMagicCaster => MagicCaster, magic_caster);

impl ExtraMagicCaster {
    pub const RTTI: VariantID = RTTI_ExtraMagicCaster;
    pub const VTABLE: &'static [VariantID] = &VTABLE_ExtraMagicCaster;
    pub const EXTRADATATYPE: ExtraDataType = ExtraDataType::MagicCaster;

    // override (BSExtraData)
    // 00 ~ExtraMagicCaster
    // 01 GetType
}
