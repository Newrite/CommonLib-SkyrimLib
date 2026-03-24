use bitflags::bitflags;
use core_util::inherit;

use crate::offsets::offsets_rtti::RTTI_BGSLocation;
use crate::offsets::offsets_vtable::VTABLE_BGSLocation;
use crate::re::actor::Actor;
use crate::re::bgs_keyword::BGSKeyword;
use crate::re::bgs_keyword_form::BGSKeywordForm;
use crate::re::bgs_location_ref_type::BGSLocationRefType;
use crate::re::bgs_music_type::BGSMusicType;
use crate::re::bs_pointer_handle::ObjectRefHandle;
use crate::re::bst_array::BSTArray;
use crate::re::bst_hash_map::BSTSet;
use crate::re::bst_tuple::BSTTuple;
use crate::re::form_traits::FormCastable;
use crate::re::form_type::FormType;
use crate::re::queued_promote_location_references_task::QueuedPromoteLocationReferencesTask;
use crate::re::tes_faction::TESFaction;
use crate::re::tes_form::{FormID, TESForm};
use crate::re::tes_full_name::TESFullName;
use crate::relocation::{RttiType, VariantID};

bitflags! {
    /// C++ `RE::BGSLocation::ChangeFlags`
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct BGSLocationChangeFlags: u32 {
        const KEYWORD_DATA = 1 << 30;
        const CLEARED = 1 << 31;
    }
}

bitflags! {
    /// C++ `RE::BGSLocation::RecordFlags`
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct BGSLocationRecordFlags: u32 {
        const DELETED = 1 << 5;
        const IGNORED = 1 << 12;
        const CLEARED = 1 << 31;
    }
}

/// C++ `RE::UnloadedRefData::CellKey`
#[repr(C)]
#[derive(Clone, Copy)]
pub union CellKey {
    pub xy: BSTTuple<i16, i16>,
    pub raw: u32,
}

const _: () = assert!(core::mem::size_of::<CellKey>() == 0x4);

/// C++ `RE::UnloadedRefData`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct UnloadedRefData {
    pub ref_id: FormID,          // 00
    pub parent_space_id: FormID, // 04
    pub cell_key: CellKey,       // 08
}

const _: () = assert!(core::mem::size_of::<UnloadedRefData>() == 0xC);

/// C++ `RE::SpecialRefData`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SpecialRefData {
    pub location_ref_type: *mut BGSLocationRefType, // 00
    pub ref_data: UnloadedRefData,                  // 08
    pub pad14: u32,                                 // 14
}

const _: () = assert!(core::mem::size_of::<SpecialRefData>() == 0x18);

/// C++ `RE::UniqueNPCData`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct UniqueNPCData {
    pub actor: *mut Actor,            // 00
    pub ref_id: FormID,               // 08
    pub pad0c: u32,                   // 0C
    pub editor_loc: *mut BGSLocation, // 10
}

const _: () = assert!(core::mem::size_of::<UniqueNPCData>() == 0x18);

/// C++ `RE::OverrideData`
#[repr(C)]
pub struct OverrideData {
    pub added_data: BSTArray<UnloadedRefData>, // 00
    pub removed_data: BSTSet<FormID>,          // 18
}

const _: () = assert!(core::mem::size_of::<OverrideData>() == 0x48);

/// C++ `RE::BGSLocation::KEYWORD_DATA`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct KEYWORD_DATA {
    pub keyword: *mut BGSKeyword, // 00
    pub data: f32,                // 08
    pub pad0c: u32,               // 0C
}

const _: () = assert!(core::mem::size_of::<KEYWORD_DATA>() == 0x10);

/// C++ `RE::BGSLocation`
#[repr(C)]
pub struct BGSLocation {
    pub base: TESForm,                                               // 00
    pub full_name: TESFullName,                                      // 20
    pub keyword_form: BGSKeywordForm,                                // 30
    pub parent_loc: *mut BGSLocation,                                // 48 - PNAM
    pub unreported_crime_faction: *mut TESFaction,                   // 50 - FNAM
    pub music_type: *mut BGSMusicType,                               // 58 - NAM1
    pub world_loc_marker: ObjectRefHandle,                           // 60 - MNAM
    pub world_loc_radius: f32,                                       // 64 - RNAM
    pub horse_loc_marker: ObjectRefHandle,                           // 68 - NAM0
    pub pad6c: u32,                                                  // 6C
    pub special_refs: BSTArray<SpecialRefData>,                      // 70 - LCSR
    pub unique_npcs: BSTArray<UniqueNPCData>,                        // 88 - LCUN
    pub override_data: *mut OverrideData,                            // A0
    pub promote_refs_task: *mut QueuedPromoteLocationReferencesTask, // A8 - NiPointer<...>
    pub promoted_refs: BSTArray<ObjectRefHandle>,                    // B0
    pub loaded_count: i32,                                           // C8
    pub file_offset: u32,                                            // CC
    pub keyword_data: BSTArray<KEYWORD_DATA>,                        // D0
    pub last_checked: u32,                                           // E8
    pub cleared: bool,                                               // EC
    pub ever_cleared: bool,                                          // ED
    pub pad_ee: u16,                                                 // EE
}

const _: () = assert!(core::mem::size_of::<BGSLocation>() == 0xF0);
const _: () = assert!(core::mem::offset_of!(BGSLocation, full_name) == 0x20);
const _: () = assert!(core::mem::offset_of!(BGSLocation, keyword_form) == 0x30);
const _: () = assert!(core::mem::offset_of!(BGSLocation, special_refs) == 0x70);
const _: () = assert!(core::mem::offset_of!(BGSLocation, keyword_data) == 0xD0);
const _: () = assert!(core::mem::offset_of!(BGSLocation, cleared) == 0xEC);

impl RttiType for BGSLocation {
    const RTTI: VariantID = RTTI_BGSLocation;
}

impl FormCastable for BGSLocation {
    const TARGET_FORM_TYPE: FormType = FormType::Location;
}

inherit!(BGSLocation : TESForm);
inherit!(BGSLocation => TESFullName, full_name);
inherit!(BGSLocation => BGSKeywordForm, keyword_form);

impl BGSLocation {
    pub const RTTI: VariantID = RTTI_BGSLocation;
    pub const VTABLE: &'static [VariantID] = &VTABLE_BGSLocation;
    pub const FORMTYPE: FormType = FormType::Location;

    // override (TESForm)
    // void InitializeData() override;                    // 04
    // bool Load(TESFile* a_mod) override;                // 06
    // bool FindInFileFast(TESFile* a_mod) override;      // 0C
    // void SaveGame(BGSSaveFormBuffer* a_buf) override;  // 0E
    // void LoadGame(BGSLoadFormBuffer* a_buf) override;  // 0F
    // void Revert(BGSLoadFormBuffer* a_buf) override;    // 12
    // void InitItemImpl() override;                      // 13

    #[inline(always)]
    pub const fn is_cleared(&self) -> bool {
        self.cleared
    }

    pub fn is_child(&self, possible_child: *const BGSLocation) -> bool {
        if possible_child.is_null() {
            return false;
        }

        let mut it = unsafe { (*possible_child).parent_loc };
        while !it.is_null() {
            if core::ptr::eq(self, unsafe { &*it }) {
                return true;
            }
            it = unsafe { (*it).parent_loc };
        }

        false
    }

    pub fn is_parent(&self, possible_parent: *const BGSLocation) -> bool {
        if possible_parent.is_null() {
            return false;
        }

        let mut it = self.parent_loc;
        while !it.is_null() {
            if core::ptr::eq(it as *const BGSLocation, possible_parent) {
                return true;
            }
            it = unsafe { (*it).parent_loc };
        }

        false
    }

    #[inline(always)]
    pub const fn is_loaded(&self) -> bool {
        self.loaded_count > 0
    }
}
