use core_util::inherit;

use crate::offsets::offsets_rtti::RTTI_ExtraFactionChanges;
use crate::offsets::offsets_vtable::VTABLE_ExtraFactionChanges;
use crate::re::{BSExtraData, BSTArray, ExtraDataType, ExtraDataTyped, FACTION_RANK, TESFaction};
use crate::relocation::{RttiType, VariantID};

/// C++ `RE::ExtraFactionChanges`
#[repr(C)]
pub struct ExtraFactionChanges {
    pub base: BSExtraData,                       // 00
    pub faction_changes: BSTArray<FACTION_RANK>, // 10
    pub crime_faction: *mut TESFaction,          // 28
    pub remove_crime_faction: bool,              // 30
    pub pad31: u8,                               // 31
    pub pad32: u16,                              // 32
    pub pad34: u32,                              // 34
}

const _: () = assert!(core::mem::size_of::<ExtraFactionChanges>() == 0x38);
const _: () = assert!(core::mem::offset_of!(ExtraFactionChanges, faction_changes) == 0x10);
const _: () = assert!(core::mem::offset_of!(ExtraFactionChanges, crime_faction) == 0x28);
const _: () = assert!(core::mem::offset_of!(ExtraFactionChanges, remove_crime_faction) == 0x30);

impl RttiType for ExtraFactionChanges {
    const RTTI: VariantID = RTTI_ExtraFactionChanges;
}

impl ExtraDataTyped for ExtraFactionChanges {
    const EXTRADATATYPE: ExtraDataType = ExtraDataType::FactionChanges;
}

inherit!(ExtraFactionChanges : BSExtraData);

impl ExtraFactionChanges {
    pub const RTTI: VariantID = RTTI_ExtraFactionChanges;
    pub const VTABLE: &'static [VariantID] = &VTABLE_ExtraFactionChanges;
    pub const EXTRADATATYPE: ExtraDataType = ExtraDataType::FactionChanges;
}
