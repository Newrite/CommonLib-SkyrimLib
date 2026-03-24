use bitflags::bitflags;
use core_util::{EnumSet, inherit};

use crate::offsets::offsets_rtti::RTTI_BGSEncounterZone;
use crate::offsets::offsets_vtable::VTABLE_BGSEncounterZone;
use crate::re::bgs_location::BGSLocation;
use crate::re::form_traits::FormCastable;
use crate::re::form_type::FormType;
use crate::re::tes_faction::TESFaction;
use crate::re::tes_form::TESForm;
use crate::relocation::{RttiType, VariantID};

/// C++ `RE::ENCOUNTER_ZONE_DATA::Flag`
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EncounterZoneFlag {
    None = 0,
    NeverResets = 1 << 0,
    MatchPCBelowMinimumLevel = 1 << 1,
    DisableCombatBoundary = 1 << 2,
}

core_util::impl_enumset_type!(EncounterZoneFlag => u8);

bitflags! {
    /// C++ `RE::BGSEncounterZone::ChangeFlags`
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct BGSEncounterZoneChangeFlags: u32 {
        const ZONE_FLAGS = 1 << 1;
        const GAME_DATA = 1 << 31;
    }
}

bitflags! {
    /// C++ `RE::BGSEncounterZone::RecordFlags`
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct BGSEncounterZoneRecordFlags: u32 {
        const DELETED = 1 << 5;
        const IGNORED = 1 << 12;
    }
}

/// C++ `RE::ENCOUNTER_ZONE_DATA`
#[repr(C)]
pub struct ENCOUNTER_ZONE_DATA {
    pub zone_owner: *mut TESFaction,           // 00
    pub location: *mut BGSLocation,            // 08
    pub owner_rank: i8,                        // 10
    pub min_level: i8,                         // 11
    pub flags: EnumSet<EncounterZoneFlag, u8>, // 12
    pub max_level: i8,                         // 13
    pub pad14: u32,                            // 14
}

const _: () = assert!(core::mem::size_of::<ENCOUNTER_ZONE_DATA>() == 0x18);

/// C++ `RE::ENCOUNTER_ZONE_GAME_DATA`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct ENCOUNTER_ZONE_GAME_DATA {
    pub detach_time: u32, // 00
    pub attach_time: u32, // 04
    pub reset_time: u32,  // 08
    pub zone_level: u16,  // 0C
    pub pad0e: u16,       // 0E
}

const _: () = assert!(core::mem::size_of::<ENCOUNTER_ZONE_GAME_DATA>() == 0x10);

/// C++ `RE::BGSEncounterZone`
#[repr(C)]
pub struct BGSEncounterZone {
    pub base: TESForm,                       // 00
    pub data: ENCOUNTER_ZONE_DATA,           // 20
    pub game_data: ENCOUNTER_ZONE_GAME_DATA, // 38
}

const _: () = assert!(core::mem::size_of::<BGSEncounterZone>() == 0x48);
const _: () = assert!(core::mem::offset_of!(BGSEncounterZone, data) == 0x20);
const _: () = assert!(core::mem::offset_of!(BGSEncounterZone, game_data) == 0x38);

impl RttiType for BGSEncounterZone {
    const RTTI: VariantID = RTTI_BGSEncounterZone;
}

impl FormCastable for BGSEncounterZone {
    const TARGET_FORM_TYPE: FormType = FormType::EncounterZone;
}

inherit!(BGSEncounterZone : TESForm);

impl BGSEncounterZone {
    pub const RTTI: VariantID = RTTI_BGSEncounterZone;
    pub const VTABLE: &'static [VariantID] = &VTABLE_BGSEncounterZone;
    pub const FORMTYPE: FormType = FormType::EncounterZone;

    // override (TESForm)
    // void InitializeData() override;                    // 04
    // bool Load(TESFile* a_mod) override;                // 06
    // void SaveGame(BGSSaveFormBuffer* a_buf) override;  // 0E
    // void LoadGame(BGSLoadFormBuffer* a_buf) override;  // 0F
    // void Revert(BGSLoadFormBuffer* a_buf) override;    // 12
    // void InitItemImpl() override;                      // 13

    #[inline(always)]
    pub fn never_resets(&self) -> bool {
        self.data.flags.any(EncounterZoneFlag::NeverResets)
    }
}
