use core_util::inherit;

use crate::offsets::offsets_rtti::RTTI_ExtraLeveledCreature;
use crate::offsets::offsets_vtable::VTABLE_ExtraLeveledCreature;
use crate::re::{BSExtraData, ExtraDataType, ExtraDataTyped, TESActorBase};
use crate::relocation::{RttiType, VariantID};

/// C++ `RE::ExtraLeveledCreature`
#[repr(C)]
pub struct ExtraLeveledCreature {
    pub base: BSExtraData,                // 00
    pub original_base: *mut TESActorBase, // 10
    pub template_base: *mut TESActorBase, // 18
}

const _: () = assert!(core::mem::size_of::<ExtraLeveledCreature>() == 0x20);
const _: () = assert!(core::mem::offset_of!(ExtraLeveledCreature, original_base) == 0x10);
const _: () = assert!(core::mem::offset_of!(ExtraLeveledCreature, template_base) == 0x18);

impl RttiType for ExtraLeveledCreature {
    const RTTI: VariantID = RTTI_ExtraLeveledCreature;
}

impl ExtraDataTyped for ExtraLeveledCreature {
    const EXTRADATATYPE: ExtraDataType = ExtraDataType::LeveledCreature;
}

inherit!(ExtraLeveledCreature : BSExtraData);

impl ExtraLeveledCreature {
    pub const RTTI: VariantID = RTTI_ExtraLeveledCreature;
    pub const VTABLE: &'static [VariantID] = &VTABLE_ExtraLeveledCreature;
    pub const EXTRADATATYPE: ExtraDataType = ExtraDataType::LeveledCreature;
}
