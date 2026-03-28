use alloc::boxed::Box;
use core::ffi::c_void;
use core_util::{EnumSet, inherit};

use crate::offsets::offsets_rtti::RTTI_TESNPC;
use crate::offsets::offsets_vtable::VTABLE_TESNPC;
use crate::re::Actor;
use crate::re::BGSColorForm;
use crate::re::BGSHeadPart;
use crate::re::BGSKeywordForm;
use crate::re::BGSKeywordFormExt;
use crate::re::BGSListForm;
use crate::re::BGSOutfit;
use crate::re::BGSOverridePackCollection;
use crate::re::BGSPerk;
use crate::re::BGSRelationship;
use crate::re::BGSTextureSet;
use crate::re::BSEventNotifyControl;
use crate::re::BSFaceGenNiNode;
use crate::re::BSFixedString;
use crate::re::BSTArray;
use crate::re::BSTEventSink;
use crate::re::BSTEventSource;
use crate::re::Color;
use crate::re::FormCastable;
use crate::re::FormType;
use crate::re::HeadPartType;
use crate::re::MenuOpenCloseEvent;
use crate::re::NiColorA;
use crate::re::PerkRankData;
use crate::re::SEX;
use crate::re::SEXES_TOTAL;
use crate::re::SOUND_LEVEL;
use crate::re::TESActorBase;
use crate::re::TESClass;
use crate::re::TESCombatStyle;
use crate::re::TESFaction;
use crate::re::TESRace;
use crate::re::TESRaceForm;
use crate::re::TintMask;
use crate::relocation::{RelocationID, RttiType, VariantID};

bitflags::bitflags! {
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct TESNPCChangeFlags: u32 {
        const BASE_DATA = 1 << 1;
        const ATTRIBUTES = 1 << 2;
        const AI_DATA = 1 << 3;
        const SPELL_LIST = 1 << 4;
        const FULL_NAME = 1 << 5;
        const FACTIONS = 1 << 6;
        const NPC_SKILLS = 1 << 9;
        const CLASS = 1 << 10;
        const FACE = 1 << 11;
        const DEFAULT_OUTFIT = 1 << 12;
        const SLEEP_OUTFIT = 1 << 13;
        const GENDER = 1 << 24;
        const RACE = 1 << 25;
    }
}

bitflags::bitflags! {
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct TESNPCRecordFlags: u32 {
        const DELETED = 1 << 5;
        const IGNORED = 1 << 12;
        const COMPRESSED = 1 << 18;
        const BLEEDOUT_OVERRIDE = 1 << 29;
    }
}

pub const TES_NPC_SKILL_TOTAL: usize = Skills::TOTAL;
pub const TES_NPC_FACE_MORPH_TOTAL: usize = FaceDataMorphs::TOTAL;
pub const TES_NPC_FACE_PART_TOTAL: usize = FaceDataParts::TOTAL;
pub const TES_NPC_OVERLAY_PART_COUNT: usize = 8;

#[repr(C)]
pub struct CreatureSounds {
    pub unk00: [*mut BSTArray<*mut c_void>; TES_NPC_OVERLAY_PART_COUNT], // 00
}

const _: () = assert!(core::mem::size_of::<CreatureSounds>() == 0x40);

#[repr(C)]
#[derive(Clone, Copy)]
pub struct Skills {
    pub values: [u8; TES_NPC_SKILL_TOTAL],  // 00
    pub offsets: [u8; TES_NPC_SKILL_TOTAL], // 12
    pub health: u16,                        // 24
    pub magicka: u16,                       // 26
    pub stamina: u16,                       // 28
    pub pad2a: u16,                         // 2A
    pub faraway_model_distance: f32,        // 2C
}

const _: () = assert!(core::mem::size_of::<Skills>() == 0x30);

impl Skills {
    pub const ONE_HANDED: usize = 0;
    pub const TWO_HANDED: usize = 1;
    pub const MARKSMAN: usize = 2;
    pub const BLOCK: usize = 3;
    pub const SMITHING: usize = 4;
    pub const HEAVY_ARMOR: usize = 5;
    pub const LIGHT_ARMOR: usize = 6;
    pub const PICKPOCKET: usize = 7;
    pub const LOCKPICKING: usize = 8;
    pub const SNEAK: usize = 9;
    pub const ALCHEMY: usize = 10;
    pub const SPEECHCRAFT: usize = 11;
    pub const ALTERATION: usize = 12;
    pub const CONJURATION: usize = 13;
    pub const DESTRUCTION: usize = 14;
    pub const ILLUSION: usize = 15;
    pub const RESTORATION: usize = 16;
    pub const ENCHANTING: usize = 17;
    pub const TOTAL: usize = 18;
}

#[repr(C)]
pub struct HeadRelatedData {
    pub hair_color: *mut BGSColorForm,    // 00
    pub face_details: *mut BGSTextureSet, // 08
}

const _: () = assert!(core::mem::size_of::<HeadRelatedData>() == 0x10);

impl Default for HeadRelatedData {
    fn default() -> Self {
        Self::new()
    }
}

impl HeadRelatedData {
    #[inline]
    pub const fn new() -> Self {
        Self {
            hair_color: core::ptr::null_mut(),
            face_details: core::ptr::null_mut(),
        }
    }
}

#[repr(C)]
pub union Sounds {
    pub sound_creature: *mut TESNPC,
    pub creature_sounds: *mut CreatureSounds,
}

const _: () = assert!(core::mem::size_of::<Sounds>() == 0x8);

#[repr(C)]
pub struct FaceData {
    pub morphs: [f32; TES_NPC_FACE_MORPH_TOTAL], // 00
    pub parts: [i32; TES_NPC_FACE_PART_TOTAL],   // 4C
}

const _: () = assert!(core::mem::size_of::<FaceData>() == 0x5C);

pub struct FaceDataMorphs;

impl FaceDataMorphs {
    pub const NOSE_LONG_SHORT: usize = 0;
    pub const NOSE_UP_DOWN: usize = 1;
    pub const JAW_UP_DOWN: usize = 2;
    pub const JAW_NARROW_WIDE: usize = 3;
    pub const JAW_FORWARD_BACK: usize = 4;
    pub const CHEEKS_UP_DOWN: usize = 5;
    pub const CHEEKS_FORWARD_BACK: usize = 6;
    pub const EYES_UP_DOWN: usize = 7;
    pub const EYES_IN_OUT: usize = 8;
    pub const BROWS_UP_DOWN: usize = 9;
    pub const BROWS_IN_OUT: usize = 10;
    pub const BROWS_FORWARD_BACK: usize = 11;
    pub const LIPS_UP_DOWN: usize = 12;
    pub const LIPS_IN_OUT: usize = 13;
    pub const CHIN_NARROW_WIDE: usize = 14;
    pub const CHIN_UP_DOWN: usize = 15;
    pub const CHIN_UNDERBITE_OVERBITE: usize = 16;
    pub const EYES_FORWARD_BACK: usize = 17;
    pub const UNK: usize = 18;
    pub const TOTAL: usize = 19;
}

pub struct FaceDataParts;

impl FaceDataParts {
    pub const NOSE: usize = 0;
    pub const UNKNOWN: usize = 1;
    pub const EYES: usize = 2;
    pub const MOUTH: usize = 3;
    pub const TOTAL: usize = 4;
}

impl FaceData {
    pub const DEFAULT: i32 = 0x7F7FFFFF;
    pub const TOTAL_PRESETS: usize = 4;
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct Layer {
    pub tint_color: Color,        // 00
    pub tint_index: u16,          // 04
    pub preset: u16,              // 06
    pub interpolation_value: u16, // 08
    pub pad0a: u16,               // 0A
    pub pad0c: u32,               // 0C
}

const _: () = assert!(core::mem::size_of::<Layer>() == 0x10);

impl Layer {
    #[inline]
    pub fn get_interpolation_value(&self) -> f32 {
        self.interpolation_value as f32 / 100.0
    }
}

#[repr(C)]
pub struct TESNPC {
    pub actor_base: TESActorBase,                            // 000
    pub race_form: TESRaceForm,                              // 150
    pub override_pack_collection: BGSOverridePackCollection, // 160
    pub event_sink: BSTEventSink<MenuOpenCloseEvent>,        // 188
    pub player_skills: Skills,                               // 190
    pub npc_class: *mut TESClass,                            // 1C0
    pub head_related_data: *mut HeadRelatedData,             // 1C8
    pub gift_filter: *mut BGSListForm,                       // 1D0
    pub combat_style: *mut TESCombatStyle,                   // 1D8
    pub file_offset: u32,                                    // 1E0
    pub pad1e4: u32,                                         // 1E4
    pub original_race: *mut TESRace,                         // 1E8
    pub face_npc: *mut TESNPC,                               // 1F0
    pub height: f32,                                         // 1F8
    pub weight: f32,                                         // 1FC
    pub sounds: Sounds,                                      // 200
    pub short_name: BSFixedString,                           // 208
    pub far_skin: *mut crate::re::TESObjectARMO,             // 210
    pub default_outfit: *mut BGSOutfit,                      // 218
    pub sleep_outfit: *mut BGSOutfit,                        // 220
    pub default_pack_list: *mut BGSListForm,                 // 228
    pub crime_faction: *mut TESFaction,                      // 230
    pub head_parts: *mut *mut BGSHeadPart,                   // 238
    pub num_head_parts: i8,                                  // 240
    pub unk241: u8,                                          // 241
    pub unk242: u8,                                          // 242
    pub unk243: u8,                                          // 243
    pub unk244: u8,                                          // 244
    pub sound_level: EnumSet<SOUND_LEVEL, u8>,               // 245
    pub body_tint_color: Color,                              // 246
    pub pad24a: u16,                                         // 24A
    pub pad24c: u32,                                         // 24C
    pub relationships: *mut BSTArray<*mut BGSRelationship>,  // 250
    pub face_data: *mut FaceData,                            // 258
    pub tint_layers: *mut BSTArray<*mut Layer>,              // 260
}

const _: () = assert!(core::mem::size_of::<TESNPC>() == 0x268);
const _: () = assert!(core::mem::offset_of!(TESNPC, race_form) == 0x150);
const _: () = assert!(core::mem::offset_of!(TESNPC, override_pack_collection) == 0x160);
const _: () = assert!(core::mem::offset_of!(TESNPC, event_sink) == 0x188);
const _: () = assert!(core::mem::offset_of!(TESNPC, player_skills) == 0x190);
const _: () = assert!(core::mem::offset_of!(TESNPC, npc_class) == 0x1C0);
const _: () = assert!(core::mem::offset_of!(TESNPC, combat_style) == 0x1D8);
const _: () = assert!(core::mem::offset_of!(TESNPC, original_race) == 0x1E8);
const _: () = assert!(core::mem::offset_of!(TESNPC, face_npc) == 0x1F0);
const _: () = assert!(core::mem::offset_of!(TESNPC, short_name) == 0x208);
const _: () = assert!(core::mem::offset_of!(TESNPC, default_outfit) == 0x218);
const _: () = assert!(core::mem::offset_of!(TESNPC, sleep_outfit) == 0x220);
const _: () = assert!(core::mem::offset_of!(TESNPC, head_parts) == 0x238);
const _: () = assert!(core::mem::offset_of!(TESNPC, sound_level) == 0x245);
const _: () = assert!(core::mem::offset_of!(TESNPC, relationships) == 0x250);
const _: () = assert!(core::mem::offset_of!(TESNPC, face_data) == 0x258);
const _: () = assert!(core::mem::offset_of!(TESNPC, tint_layers) == 0x260);

impl RttiType for TESNPC {
    const RTTI: VariantID = RTTI_TESNPC;
}

impl FormCastable for TESNPC {
    const TARGET_FORM_TYPE: FormType = FormType::NPC;
}

impl AsRef<BGSKeywordForm> for TESNPC {
    #[inline(always)]
    fn as_ref(&self) -> &BGSKeywordForm {
        &self.actor_base.keyword_form
    }
}

impl AsMut<BGSKeywordForm> for TESNPC {
    #[inline(always)]
    fn as_mut(&mut self) -> &mut BGSKeywordForm {
        &mut self.actor_base.keyword_form
    }
}

inherit!(TESNPC : TESActorBase, actor_base);
inherit!(TESNPC => TESRaceForm, race_form);
inherit!(TESNPC => BGSOverridePackCollection, override_pack_collection);
inherit!(TESNPC => BSTEventSink<MenuOpenCloseEvent>, event_sink);

impl TESNPC {
    pub const RTTI: VariantID = RTTI_TESNPC;
    pub const VTABLE: &'static [VariantID] = &VTABLE_TESNPC;
    pub const FORMTYPE: FormType = FormType::NPC;

    crate::virtual_method! {
        pub const VFUNC_DTOR: usize = 0x00;
        pub fn dtor()
    }

    // ~TESNPC() override;                                      // 00
    // override (TESActorBase)
    // void          InitializeData() override;                     // 04
    // void          ClearData() override;                          // 05
    // bool          Load(TESFile* a_mod) override;                 // 06
    // bool          FindInFileFast(TESFile* a_mod) override;       // 0C
    // void          SaveGame(BGSSaveFormBuffer* a_buf) override;   // 0E
    // void          LoadGame(BGSLoadFormBuffer* a_buf) override;   // 0F
    // void          Revert(BGSLoadFormBuffer* a_buf) override;     // 12
    // void          InitItemImpl() override;                       // 13
    // const char*   GetTextForParsedSubTag(...) const override;    // 2E
    // void          Copy(TESForm* a_srcForm) override;             // 2F
    // bool          Activate(...) override;                        // 37
    // void          UnClone3D(TESObjectREFR* a_ref) override;      // 41
    // void          SetObjectVoiceType(BGSVoiceType*) override;    // 48
    // BGSVoiceType* GetObjectVoiceType() const override;           // 49
    // NiAVObject*   Clone3D(TESObjectREFR* a_ref) override;        // 4A
    // bool          GetActivateText(...) override;                 // 4C
    // bool          CalculateDoFavor(...) override;                // 4D
    // TESCombatStyle* GetCombatStyle() override;                   // 54
    // void            SetCombatStyle(TESCombatStyle*) override;    // 55

    // override (TESActorBaseData)
    // void CopyFromTemplateForms(TESActorBase** a_templateForms) override; // 04

    // override (ActorValueOwner)
    // float GetActorValue(...) const override;                     // 01
    // void  SetActorValue(...) override;                           // 07

    // override (BSTEventSink<MenuOpenCloseEvent>)
    // BSEventNotifyControl ProcessEvent(...) override;             // 01

    crate::relocation_func! {
        pub fn change_head_part(&mut self, target: *mut BGSHeadPart) => RelocationID::new(24246, 24750)
    }

    crate::relocation_func! {
        pub fn get_base_overlays(&self) -> *mut *mut BGSHeadPart => RelocationID::new(24275, 24791)
    }

    crate::relocation_func! {
        pub fn get_unique_actor(&mut self) -> *mut Actor => RelocationID::new(24180, 24684)
    }

    crate::relocation_func! {
        pub fn get_num_base_overlays(&self) -> u32 => RelocationID::new(24276, 24792)
    }

    crate::relocation_func! {
        pub fn has_overlays(&mut self) -> bool => RelocationID::new(24274, 24790)
    }

    crate::relocation_func! {
        pub fn set_skin_from_tint(
            &mut self,
            result: *mut NiColorA,
            tint_mask: *mut TintMask,
            from_tint: bool
        ) => RelocationID::new(24206, 24710)
    }

    crate::relocation_func! {
        pub fn update_neck(&mut self, face_node: *mut BSFaceGenNiNode) => RelocationID::new(24207, 24711)
    }

    #[inline(always)]
    pub fn process_menu_open_close_event(
        &mut self,
        event: *const MenuOpenCloseEvent,
        event_source: *mut BSTEventSource<MenuOpenCloseEvent>,
    ) -> BSEventNotifyControl {
        unsafe { self.event_sink.process_event(event, event_source) }
    }

    fn copy_perk_rank_array(&mut self, copied_data: &[PerkRankData]) {
        let old_data = self.actor_base.perk_rank_array.perks;
        let new_size = copied_data.len();
        let new_data = if new_size == 0 {
            core::ptr::null_mut()
        } else {
            unsafe {
                crate::ffi::commonlib_calloc(new_size, core::mem::size_of::<PerkRankData>())
                    as *mut PerkRankData
            }
        };

        if new_size != 0 {
            assert!(
                !new_data.is_null(),
                "TESNPC::copy_perk_rank_array allocation failed"
            );
            unsafe {
                core::ptr::copy_nonoverlapping(copied_data.as_ptr(), new_data, new_size);
            }
        }

        self.actor_base.perk_rank_array.perk_count = new_size as u32;
        self.actor_base.perk_rank_array.perks = new_data;

        if !old_data.is_null() {
            unsafe { crate::ffi::commonlib_free(old_data.cast::<c_void>()) };
        }
    }

    pub fn add_perk(&mut self, perk: *mut BGSPerk, rank: i8) -> bool {
        if self.get_perk_index(perk).is_some() {
            return false;
        }

        let mut copied_data = self.actor_base.perk_rank_array.perks_slice().to_vec();
        copied_data.push(PerkRankData::with_rank(perk, rank));
        self.copy_perk_rank_array(&copied_data);
        true
    }

    pub fn add_perks(&mut self, perks: &[*mut BGSPerk], rank: i8) -> bool {
        let mut copied_data = self.actor_base.perk_rank_array.perks_slice().to_vec();
        for &perk in perks {
            if self.get_perk_index(perk).is_none() {
                copied_data.push(PerkRankData::with_rank(perk, rank));
            }
        }
        self.copy_perk_rank_array(&copied_data);
        true
    }

    pub fn contains_keyword(&self, editor_id: &str) -> bool {
        if self.contains_keyword_string(editor_id) {
            return true;
        }

        let race = self.get_race();
        if race.is_null() {
            return false;
        }

        unsafe { (*race).contains_keyword_string(editor_id) }
    }

    pub fn get_base_scale(&self) -> f32 {
        let race = self.get_race();
        if race.is_null() {
            return 1.0;
        }

        let sex_index = self.get_sex() as usize;
        if sex_index >= SEXES_TOTAL {
            1.0
        } else {
            unsafe { (*race).data.height[sex_index] }
        }
    }

    pub fn get_current_head_part_by_type(&mut self, part_type: HeadPartType) -> *mut BGSHeadPart {
        if self.has_overlays() {
            self.get_head_part_overlay_by_type(part_type)
        } else {
            self.get_head_part_by_type(part_type)
        }
    }

    pub fn get_head_part_by_type(&self, part_type: HeadPartType) -> *mut BGSHeadPart {
        if self.head_parts.is_null() {
            return core::ptr::null_mut();
        }

        let len = self.num_head_parts.max(0) as usize;
        for i in 0..len {
            let part = unsafe { *self.head_parts.add(i) };
            if !part.is_null() && unsafe { (*part).part_type } == part_type {
                return part;
            }
        }

        core::ptr::null_mut()
    }

    pub fn get_head_part_overlay_by_type(&self, part_type: HeadPartType) -> *mut BGSHeadPart {
        let count = self.get_num_base_overlays() as usize;
        let overlays = self.get_base_overlays();
        if overlays.is_null() {
            return core::ptr::null_mut();
        }

        for i in 0..count {
            let part = unsafe { *overlays.add(i) };
            if !part.is_null() && unsafe { (*part).part_type } == part_type {
                return part;
            }
        }

        core::ptr::null_mut()
    }

    pub fn get_height(&self) -> f32 {
        let race = self.get_race();
        if race.is_null() {
            return self.height;
        }

        let sex_index = self.get_sex() as usize;
        if sex_index >= SEXES_TOTAL {
            0.0
        } else {
            unsafe { (*race).data.height[sex_index] * self.height }
        }
    }

    pub fn get_perk_index(&self, perk: *mut BGSPerk) -> Option<u32> {
        self.actor_base
            .perk_rank_array
            .perks_slice()
            .iter()
            .position(|entry| entry.perk == perk)
            .map(|index| index as u32)
    }

    pub fn get_spell_list(&mut self) -> *mut crate::re::SpellData {
        if self.actor_base.spell_list.actor_effects.is_null() {
            self.actor_base.spell_list.actor_effects =
                Box::into_raw(Box::new(crate::re::SpellData::new()));
        }
        self.actor_base.spell_list.actor_effects
    }

    #[inline]
    pub fn get_race(&self) -> *mut TESRace {
        self.race_form.race
    }

    pub fn get_root_face_npc(&mut self) -> *mut TESNPC {
        let mut iter = self as *mut TESNPC;
        while !iter.is_null() {
            let next = unsafe { (*iter).face_npc };
            if next.is_null() {
                break;
            }
            iter = next;
        }
        iter
    }

    pub fn get_root_face_npc_const(&self) -> *const TESNPC {
        let mut iter = self as *const TESNPC;
        while !iter.is_null() {
            let next = unsafe { (*iter).face_npc } as *const TESNPC;
            if next.is_null() {
                break;
            }
            iter = next;
        }
        iter
    }

    #[inline]
    pub const fn get_sex(&self) -> SEX {
        if self.actor_base.actor_base_data.is_female() {
            SEX::Female
        } else {
            SEX::Male
        }
    }

    pub fn has_applicable_keyword_string(&self, editor_id: &str) -> bool {
        if self.has_keyword_string(editor_id) {
            return true;
        }

        let race = self.get_race();
        if race.is_null() {
            return false;
        }

        unsafe { (*race).has_keyword_string(editor_id) }
    }

    #[inline]
    pub fn is_in_class(&self, class: *mut TESClass) -> bool {
        !self.npc_class.is_null() && self.npc_class == class
    }

    #[inline]
    pub fn is_in_faction(&self, faction: *mut TESFaction) -> bool {
        unsafe { self.actor_base.actor_base_data.factions.as_slice() }
            .iter()
            .any(|entry| entry.faction == faction && entry.rank > -1)
    }

    pub fn remove_perk(&mut self, perk: *mut BGSPerk) -> bool {
        let Some(index) = self.get_perk_index(perk) else {
            return false;
        };

        let mut copied_data = self.actor_base.perk_rank_array.perks_slice().to_vec();
        copied_data.remove(index as usize);
        self.copy_perk_rank_array(&copied_data);
        true
    }

    pub fn remove_perks(&mut self, perks: &[*mut BGSPerk]) -> bool {
        let mut copied_data = self.actor_base.perk_rank_array.perks_slice().to_vec();
        let old_len = copied_data.len();
        copied_data.retain(|entry| !perks.contains(&entry.perk));
        if copied_data.len() != old_len {
            self.copy_perk_rank_array(&copied_data);
            true
        } else {
            false
        }
    }

    pub fn set_default_outfit(&mut self, outfit: *mut BGSOutfit) -> bool {
        self.default_outfit = outfit;
        self.actor_base
            .base
            .base
            .base
            .base
            .add_change(TESNPCChangeFlags::DEFAULT_OUTFIT.bits())
    }

    pub fn set_face_texture(&mut self, texture_set: *mut BGSTextureSet) {
        if self.head_related_data.is_null() && !texture_set.is_null() {
            self.head_related_data = Box::into_raw(Box::new(HeadRelatedData::new()));
        }

        if !self.head_related_data.is_null() {
            unsafe { (*self.head_related_data).face_details = texture_set };
        }
    }

    pub fn set_hair_color(&mut self, hair_color: *mut BGSColorForm) {
        if self.head_related_data.is_null() && !hair_color.is_null() {
            self.head_related_data = Box::into_raw(Box::new(HeadRelatedData::new()));
        }

        if !self.head_related_data.is_null() {
            unsafe { (*self.head_related_data).hair_color = hair_color };
        }
    }

    pub fn set_sleep_outfit(&mut self, outfit: *mut BGSOutfit) -> bool {
        self.sleep_outfit = outfit;
        self.actor_base
            .base
            .base
            .base
            .base
            .add_change(TESNPCChangeFlags::SLEEP_OUTFIT.bits())
    }
}
