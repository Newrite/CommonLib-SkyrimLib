use bitflags::bitflags;
use core_util::{Enum, EnumSet, inherit};

use crate::offsets::offsets_rtti::RTTI_TESRace;
use crate::offsets::offsets_vtable::VTABLE_TESRace;
use crate::re::attack_animation_array_map::AttackAnimationArrayMap;
#[allow(unused_imports)]
use crate::re::actor_values::ActorValue;
use crate::re::bgs_art_object::BGSArtObject;
use crate::re::bgs_attack_data_form::BGSAttackDataForm;
use crate::re::bgs_behavior_graph_model::BGSBehaviorGraphModel;
use crate::re::bgs_biped_object_form::BGSBipedObjectForm;
use crate::re::bgs_body_part_data::BGSBodyPartData;
use crate::re::bgs_color_form::BGSColorForm;
use crate::re::bgs_equip_slot::BGSEquipSlot;
use crate::re::bgs_head_part::{BGSHeadPart, HeadPartType};
use crate::re::bgs_impact_data_set::BGSImpactDataSet;
use crate::re::bgs_keyword_form::BGSKeywordForm;
use crate::re::bgs_material_type::BGSMaterialType;
use crate::re::bgs_movement_type::BGSMovementType;
use crate::re::bgs_skin_form::BGSSkinForm;
use crate::re::bgs_sound_descriptor_form::BGSSoundDescriptorForm;
use crate::re::bgs_texture_model::BGSTextureModel;
use crate::re::bgs_texture_set::BGSTextureSet;
use crate::re::bgs_voice_type::BGSVoiceType;
use crate::re::biped_objects::BipedObject;
use crate::re::bs_fixed_string::BSFixedString;
use crate::re::bs_resource_id::BSResourceID;
use crate::re::bst_array::BSTArray;
use crate::re::form_traits::FormCastable;
use crate::re::form_type::FormType;
use crate::re::ni_point3::NiPoint3;
use crate::re::sexes::{SEX, SEXES_TOTAL};
use crate::re::tes_description::TESDescription;
use crate::re::tes_form::TESForm;
use crate::re::tes_full_name::TESFullName;
use crate::re::tes_model::TESModel;
use crate::re::tes_npc::TESNPC;
use crate::re::tes_object_armo::TESObjectARMO;
use crate::re::tes_spell_list::TESSpellList;
use crate::re::tes_texture::TESTexture;
use crate::relocation::{RttiType, VariantID};

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum RaceSize {
    Small = 0,
    Medium = 1,
    Large = 2,
    ExtraLarge = 3,
}

core_util::impl_enumset_type!(RaceSize => u32);

impl TryFrom<u32> for RaceSize {
    type Error = ();

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        if value <= Self::ExtraLarge as u32 {
            Ok(unsafe { core::mem::transmute::<u32, Self>(value) })
        } else {
            Err(())
        }
    }
}

bitflags! {
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct RaceDataFlags: u32 {
        const NONE = 0;
        const PLAYABLE = 1 << 0;
        const FACE_GEN_HEAD = 1 << 1;
        const CHILD = 1 << 2;
        const TILT_FRONT_BACK = 1 << 3;
        const TILT_LEFT_RIGHT = 1 << 4;
        const NO_SHADOW = 1 << 5;
        const SWIMS = 1 << 6;
        const FLIES = 1 << 7;
        const WALKS = 1 << 8;
        const IMMOBILE = 1 << 9;
        const NOT_PUSHABLE = 1 << 10;
        const NO_COMBAT_IN_WATER = 1 << 11;
        const NO_ROTATING_TO_HEAD_TRACK = 1 << 12;
        const DONT_SHOW_BLOOD_SPRAY = 1 << 13;
        const DONT_SHOW_BLOOD_DECAL = 1 << 14;
        const USE_HEAD_TRACK_ANIMS = 1 << 15;
        const SPELLS_ALIGN_WITH_MAGIC_NODE = 1 << 16;
        const USE_WORLD_RAYCASTS_FOR_FOOT_IK = 1 << 17;
        const ALLOW_RAGDOLL_COLLISION = 1 << 18;
        const REGEN_HP_IN_COMBAT = 1 << 19;
        const CANT_OPEN_DOORS = 1 << 20;
        const ALLOW_PC_DIALOGUE = 1 << 21;
        const NO_KNOCKDOWNS = 1 << 22;
        const ALLOW_PICKPOCKET = 1 << 23;
        const ALWAYS_USE_PROXY_CONTROLLER = 1 << 24;
        const DONT_SHOW_WEAPON_BLOOD = 1 << 25;
        const OVERLAY_HEAD_PART_LIST = 1 << 26;
        const OVERRIDE_HEAD_PART_LIST = 1 << 27;
        const CAN_PICKUP_ITEMS = 1 << 28;
        const ALLOW_MULTIPLE_MEMBRANE_SHADERS = 1 << 29;
        const CAN_DUAL_WIELD = 1 << 30;
        const AVOIDS_ROADS = 1 << 31;
    }
}

bitflags! {
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct RaceDataFlags2: u32 {
        const NONE = 0;
        const USE_ADVANCED_AVOIDANCE = 1 << 0;
        const NON_HOSTILE = 1 << 1;
        const ALLOW_MOUNTED_COMBAT = 1 << 4;
    }
}

/// C++: `REX::EnumSet<ActorValue, u8>` - use ActorValue variants as values
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct SkillBoost {
    pub skill: u8,  // 00 (ActorValue as u8)
    pub bonus: u8,  // 01
}

const _: () = assert!(core::mem::size_of::<SkillBoost>() == 0x2);

impl SkillBoost {
    #[inline(always)]
    pub const fn skill_storage(&self) -> Enum<ActorValue, u8> {
        Enum::from_underlying(self.skill)
    }

    #[inline(always)]
    pub fn try_get_skill(&self) -> Option<ActorValue> {
        self.skill_storage().get()
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct RaceData {
    pub skill_boosts: [SkillBoost; 7],  // 0x00
    pub pad0e: u16,                     // 0x0E
    pub height: [f32; SEXES_TOTAL],     // 0x10
    pub weight: [f32; SEXES_TOTAL],     // 0x18
    pub flags: RaceDataFlags,           // 0x20
    pub starting_health: f32,           // 0x24
    pub starting_magicka: f32,          // 0x28
    pub starting_stamina: f32,          // 0x2C
    pub base_carry_weight: f32,         // 0x30
    pub base_mass: f32,                 // 0x34
    pub accelerate: f32,                // 0x38
    pub decelerate: f32,                // 0x3C
    pub race_size: RaceSize,            // 0x40 (EnumSet<RACE_SIZE, u32>)
    pub head_object: u32,               // 0x44 (EnumSet<BIPED_OBJECT, u32>)
    pub hair_object: u32,               // 0x48 (EnumSet<BIPED_OBJECT, u32>)
    pub injured_health_percent: f32,    // 0x4C
    pub shield_object: u32,             // 0x50 (EnumSet<BIPED_OBJECT, u32>)
    pub health_regen: f32,              // 0x54
    pub magicka_regen: f32,             // 0x58
    pub stamina_regen: f32,             // 0x5C
    pub unarmed_damage: f32,            // 0x60
    pub unarmed_reach: f32,             // 0x64
    pub body_object: u32,               // 0x68 (EnumSet<BIPED_OBJECT, u32>)
    pub aim_angle_tolerance: f32,       // 0x6C
    pub flight_radius: f32,             // 0x70
    pub angle_accelerate: f32,          // 0x74
    pub angle_tolerance: f32,           // 0x78
    pub flags2: RaceDataFlags2,         // 0x7C
    pub mount_offset: NiPoint3,         // 0x80
    pub dismount_offset: NiPoint3,      // 0x8C
    pub mount_camera_offset: NiPoint3,  // 0x98
}

const _: () = assert!(core::mem::size_of::<RaceData>() == 0xA4);

impl RaceData {
    #[inline(always)]
    pub const fn head_object_set(&self) -> EnumSet<BipedObject, u32> {
        EnumSet::from_underlying(self.head_object)
    }

    #[inline(always)]
    pub const fn hair_object_set(&self) -> EnumSet<BipedObject, u32> {
        EnumSet::from_underlying(self.hair_object)
    }

    #[inline(always)]
    pub const fn shield_object_set(&self) -> EnumSet<BipedObject, u32> {
        EnumSet::from_underlying(self.shield_object)
    }

    #[inline(always)]
    pub const fn body_object_set(&self) -> EnumSet<BipedObject, u32> {
        EnumSet::from_underlying(self.body_object)
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Morph {
    pub morph_flags: u32,  // 0x00
    pub unk04: u32,        // 0x04
    pub unk08: u32,        // 0x08
    pub unk0c: u32,        // 0x0C
    pub unk10: u32,        // 0x10
    pub unk14: u32,        // 0x14
    pub unk18: u32,        // 0x18
    pub unk1c: u32,        // 0x1C
}

const _: () = assert!(core::mem::size_of::<Morph>() == 0x20);

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SkinTone {
    None = 0,
    LipColor = 1,
    CheekColor = 2,
    Eyeliner = 3,
    EyeSocketUpper = 4,
    EyeSocketLower = 5,
    SkinTone = 6,
    Paint = 7,
    LaughLines = 8,
    CheekColorLower = 9,
    Nose = 10,
    Chin = 11,
    Neck = 12,
    Forehead = 13,
    Dirt = 14,
}

#[repr(C)]
pub struct TintLayer {
    pub index: u16,                    // 0x00 - TINI
    pub skin_tone: SkinTone,           // 0x02 - TINP
    pub pad03: u8,                     // 0x03
    pub pad04: u32,                    // 0x04
    pub file: TESTexture,              // 0x08 - TINT
    pub preset_default: *mut BGSColorForm,  // 0x18 - TIND
}

const _: () = assert!(core::mem::size_of::<TintLayer>() == 0x20);

#[repr(C)]
pub struct TintAssetPresets {
    pub colors: BSTArray<*mut BGSColorForm>,  // 0x00 - TINC
    pub default_values: BSTArray<f32>,        // 0x18 - TINV
    pub indices: BSTArray<u16>,               // 0x30 - TIRS
}

const _: () = assert!(core::mem::size_of::<TintAssetPresets>() == 0x48);

#[repr(C)]
pub struct TintAsset {
    pub texture: TintLayer,          // 0x00
    pub presets: TintAssetPresets,   // 0x20
}

const _: () = assert!(core::mem::size_of::<TintAsset>() == 0x68);

#[repr(C)]
pub struct FaceRelatedData {
    pub available_morphs: [Morph; 4],                                 // 0x00
    pub num_flags_set: [u32; 4],                                      // 0x80
    pub tint_masks: *mut BSTArray<*mut TintAsset>,                    // 0x90
    pub face_details_texture_sets: *mut BSTArray<*mut BGSTextureSet>, // 0x98 - FTSM / FTSF
    pub default_face_details_texture_set: *mut BGSTextureSet,         // 0xA0 - DFTM / DFTF
    pub preset_npcs: *mut BSTArray<*mut TESNPC>,                      // 0xA8 - RPRM / RPRF
    pub available_hair_colors: *mut BSTArray<*mut BGSColorForm>,      // 0xB0 - AHCM / AHCF
    pub default_hair_color: *mut BGSColorForm,                        // 0xB8
    pub head_parts: *mut BSTArray<*mut BGSHeadPart>,                  // 0xC0 - HEAD
}

const _: () = assert!(core::mem::size_of::<FaceRelatedData>() == 0xC8);

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct UnkData {
    pub unk_hashes: *mut *mut BSResourceID,  // 0x00 (BSResource::ID**)
    pub num_unk_hashes1: u32,                // 0x08
    pub pad0c: u32,                          // 0x0C
    pub num_unk_hashes2: u32,                // 0x10
    pub pad14: u32,                          // 0x14
}

const _: () = assert!(core::mem::size_of::<UnkData>() == 0x18);

bitflags! {
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct EquipmentFlags: u32 {
        const NONE = 0;
        const HAND_TO_HAND_MELEE = 1 << 0;
        const ONE_HAND_SWORD = 1 << 1;
        const ONE_HAND_DAGGER = 1 << 2;
        const ONE_HAND_AXE = 1 << 3;
        const ONE_HAND_MACE = 1 << 4;
        const TWO_HAND_SWORD = 1 << 5;
        const TWO_HAND_AXE = 1 << 6;
        const BOW = 1 << 7;
        const STAFF = 1 << 8;
        const SPELL = 1 << 9;
        const SHIELD = 1 << 10;
        const TORCH = 1 << 11;
        const CROSSBOW = 1 << 12;
    }
}

pub const MOVEMENT_TYPES_TOTAL: usize = 6;

#[repr(C)]
pub struct TESRace {
    pub base: TESForm,  // 0x000
    pub full_name: TESFullName,  // 0x020
    pub description: TESDescription,  // 0x030
    pub spell_list: TESSpellList,  // 0x040
    pub skin_form: BGSSkinForm,  // 0x050
    pub biped_object_form: BGSBipedObjectForm,  // 0x060
    pub keyword_form: BGSKeywordForm,  // 0x070
    pub attack_data_form: BGSAttackDataForm,  // 0x088
    pub skeleton_models: [TESModel; SEXES_TOTAL],  // 0x098 - ANAM
    pub data: RaceData,  // 0x0E8
    pub clamp_face_geo_value: f32,  // 0x18C - PNAM
    pub clamp_face_geo_value2: f32,  // 0x190 - UNAM (C++ header comment repeats 0x18C)
    pub pad194: u32,  // 0x194
    pub body_texture_models: [BGSTextureModel; SEXES_TOTAL],  // 0x198
    pub behavior_graphs: [BGSBehaviorGraphModel; SEXES_TOTAL],  // 0x1E8
    pub root_behavior_graph_names: [BSFixedString; SEXES_TOTAL],  // 0x238
    pub behavior_graph_project_names: [BSFixedString; SEXES_TOTAL],  // 0x248
    pub default_voice_types: [*mut BGSVoiceType; SEXES_TOTAL],  // 0x258 - VTCK
    pub body_part_data: *mut BGSBodyPartData,  // 0x268 - GNAM
    pub decapitate_armors: [*mut TESObjectARMO; SEXES_TOTAL],  // 0x270 - DNAM
    pub unk280: UnkData,  // 0x280
    pub unk298: UnkData,  // 0x298
    pub unk2b0: u64,  // 0x2B0
    pub unk2b8: u64,  // 0x2B8
    pub unk2c0: u64,  // 0x2C0
    pub unk2c8: u64,  // 0x2C8
    pub attack_animation_array_map: [*mut AttackAnimationArrayMap; SEXES_TOTAL],  // 0x2D0
    pub form_editor_id: BSFixedString,  // 0x2E0 - EDID
    pub blood_impact_material: *mut BGSMaterialType,  // 0x2E8 - NAM4
    pub impact_data_set: *mut BGSImpactDataSet,  // 0x2F0 - NAM5
    pub dismember_blood: *mut BGSArtObject,  // 0x2F8 - NAM7
    pub corpse_open_sound: *mut BGSSoundDescriptorForm,  // 0x300 - ONAM
    pub corpse_close_sound: *mut BGSSoundDescriptorForm,  // 0x308 - LNAM
    pub biped_object_name_a: [BSFixedString; BipedObject::EDITOR_TOTAL],  // 0x310 - NAME
    pub equip_slots: BSTArray<*mut BGSEquipSlot>,  // 0x410 - QNAM
    pub valid_equip_types: EquipmentFlags,  // 0x428 - VNAM
    pub unk42c: u32,  // 0x42C
    pub unarmed_equip_slot: *mut BGSEquipSlot,  // 0x430 - UNES
    pub morph_race: *mut TESRace,  // 0x438 - NAM8
    pub armor_parent_race: *mut TESRace,  // 0x440 - RNAM
    pub unk448: UnkData,  // 0x448
    pub phoneme_targets: BSTArray<BSFixedString>,  // 0x460 - PHTN
    pub base_move_types: [*mut BGSMovementType; MOVEMENT_TYPES_TOTAL],  // 0x478
    pub face_related_data: [*mut FaceRelatedData; SEXES_TOTAL],  // 0x4A8
}

const _: () = assert!(core::mem::size_of::<TESRace>() == 0x4B8);

const _: () = assert!(core::mem::offset_of!(TESRace, full_name) == 0x20);
const _: () = assert!(core::mem::offset_of!(TESRace, description) == 0x30);
const _: () = assert!(core::mem::offset_of!(TESRace, spell_list) == 0x40);
const _: () = assert!(core::mem::offset_of!(TESRace, skin_form) == 0x50);
const _: () = assert!(core::mem::offset_of!(TESRace, biped_object_form) == 0x60);
const _: () = assert!(core::mem::offset_of!(TESRace, keyword_form) == 0x70);
const _: () = assert!(core::mem::offset_of!(TESRace, attack_data_form) == 0x88);

impl RttiType for TESRace {
    const RTTI: VariantID = RTTI_TESRace;
}

impl FormCastable for TESRace {
    const TARGET_FORM_TYPE: FormType = FormType::Race;
}

inherit!(TESRace : TESForm);
inherit!(TESRace => TESFullName, full_name);
inherit!(TESRace => TESDescription, description);
inherit!(TESRace => TESSpellList, spell_list);
inherit!(TESRace => BGSSkinForm, skin_form);
inherit!(TESRace => BGSBipedObjectForm, biped_object_form);
inherit!(TESRace => BGSKeywordForm, keyword_form);
inherit!(TESRace => BGSAttackDataForm, attack_data_form);

impl TESRace {
    pub const RTTI: VariantID = RTTI_TESRace;
    pub const VTABLE: &'static [VariantID] = &VTABLE_TESRace;
    pub const FORMTYPE: FormType = FormType::Race;

    // override (TESForm)
    // void InitializeData() override;                    // 04
    // void ClearData() override;                         // 05
    // bool Load(TESFile* a_mod) override;                // 06
    // void InitItemImpl() override;                      // 13
    // bool GetPlayable() const override;                 // 19
    // const char* GetFormEditorID() const override;      // 32
    // bool SetFormEditorID(const char* a_str) override;  // 33

    #[inline]
    pub fn allows_pc_dialogue(&self) -> bool {
        self.data.flags.contains(RaceDataFlags::ALLOW_PC_DIALOGUE)
    }

    #[inline]
    pub fn allows_pickpocket(&self) -> bool {
        self.data.flags.contains(RaceDataFlags::ALLOW_PICKPOCKET)
    }

    pub fn get_head_part_by_type(&self, head_part_type: HeadPartType, sex: SEX) -> *mut BGSHeadPart {
        let sex_index = sex as u32;
        if sex_index >= SEXES_TOTAL as u32 {
            return core::ptr::null_mut();
        }

        let face_data_ptr = self.face_related_data[sex_index as usize];
        let Some(face_data) = (unsafe { face_data_ptr.as_ref() }) else {
            return core::ptr::null_mut();
        };

        if face_data.head_parts.is_null() {
            return core::ptr::null_mut();
        }

        let head_parts = unsafe { (*face_data.head_parts).as_slice() };
        for &head_part in head_parts {
            if head_part.is_null() {
                continue;
            }

            if unsafe { (*head_part).part_type } == head_part_type {
                return head_part;
            }
        }

        core::ptr::null_mut()
    }

    #[inline]
    pub fn is_child_race(&self) -> bool {
        self.data.flags.contains(RaceDataFlags::CHILD)
    }
}

pub trait TESRaceExt {
    fn allows_pc_dialogue(&self) -> bool;
    fn allows_pickpocket(&self) -> bool;
    fn get_head_part_by_type(&self, head_part_type: HeadPartType, sex: SEX) -> *mut BGSHeadPart;
    fn is_child_race(&self) -> bool;
}

impl<T: AsRef<TESRace>> TESRaceExt for T {
    fn allows_pc_dialogue(&self) -> bool {
        self.as_ref().allows_pc_dialogue()
    }

    fn allows_pickpocket(&self) -> bool {
        self.as_ref().allows_pickpocket()
    }

    fn get_head_part_by_type(&self, head_part_type: HeadPartType, sex: SEX) -> *mut BGSHeadPart {
        self.as_ref().get_head_part_by_type(head_part_type, sex)
    }

    fn is_child_race(&self) -> bool {
        self.as_ref().is_child_race()
    }
}
