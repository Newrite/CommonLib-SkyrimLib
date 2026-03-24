use bitflags::bitflags;
use core::ffi::c_void;

use core_util::{EnumSet, inherit};

use crate::offsets::offsets_rtti::RTTI_EffectSetting;
use crate::offsets::offsets_vtable::VTABLE_EffectSetting;
use crate::re::BGSProjectile;
use crate::re::actor_values::ActorValue;
use crate::re::bgs_art_object::BGSArtObject;
use crate::re::bgs_dual_cast_data::BGSDualCastData;
use crate::re::bgs_explosion::BGSExplosion;
use crate::re::bgs_impact_data_set::BGSImpactDataSet;
use crate::re::bgs_keyword::BGSKeyword;
use crate::re::bgs_keyword_form::BGSKeywordForm;
use crate::re::bgs_menu_display_object::BGSMenuDisplayObject;
use crate::re::bgs_perk::BGSPerk;
use crate::re::bgs_reference_effect::BGSReferenceEffect;
use crate::re::bgs_sound_descriptor_form::BGSSoundDescriptorForm;
use crate::re::bs_fixed_string::BSFixedString;
use crate::re::bssimple_list::BSSimpleList;
use crate::re::bst_array::BSTArray;
use crate::re::effect_archetypes::EffectArchetypeId;
use crate::re::form_traits::FormCastable;
use crate::re::form_type::FormType;
use crate::re::magic_system::CastingType;
use crate::re::magic_system::Delivery;
use crate::re::magic_system::SoundID;
use crate::re::sound_levels::SOUND_LEVEL;
use crate::re::spell_item::SpellItem;
use crate::re::tes_condition::TESCondition;
use crate::re::tes_effect_shader::TESEffectShader;
use crate::re::tes_form::TESForm;
use crate::re::tes_full_name::TESFullName;
use crate::re::tes_image_space_modifier::TESImageSpaceModifier;
use crate::re::tes_object_ligh::TESObjectLIGH;
use crate::relocation::{RttiType, VariantID};

pub type EffectSettingArchetype = EffectArchetypeId;
pub type EffectSettingFilterValidation =
    unsafe extern "C" fn(*mut EffectSetting, *mut c_void) -> bool;

bitflags! {
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct EffectSettingRecordFlags: u32 {
        const NONE = 0;
        const DELETED = 1 << 5;
        const IGNORED = 1 << 12;
    }
}

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EffectSettingDataFlag {
    None = 0,
    Hostile = 1 << 0,
    Recover = 1 << 1,
    Detrimental = 1 << 2,
    SnapToNavMesh = 1 << 3,
    NoHitEvent = 1 << 4,
    DispelWithKeywords = 1 << 8,
    NoDuration = 1 << 9,
    NoMagnitude = 1 << 10,
    NoArea = 1 << 11,
    FXPersist = 1 << 12,
    GoryVisuals = 1 << 14,
    HideInUI = 1 << 15,
    NoRecast = 1 << 17,
    PowerAffectsMagnitude = 1 << 21,
    PowerAffectsDuration = 1 << 22,
    Painless = 1 << 26,
    NoHitEffect = 1 << 27,
    NoDeathDispel = 1 << 28,
}

core_util::impl_enumset_type!(EffectSettingDataFlag => u32);

#[repr(C)]
pub struct EffectSettingData {
    pub flags: EnumSet<EffectSettingDataFlag, u32>, // 0x00
    pub base_cost: f32,                             // 0x04
    pub associated_form: *mut TESForm,              // 0x08
    pub associated_skill: ActorValue,               // 0x10
    pub resist_variable: ActorValue,                // 0x14
    pub num_counter_effects: i16,                   // 0x18
    pub pad1a: u16,                                 // 0x1A
    pub pad1c: u32,                                 // 0x1C
    pub light: *mut TESObjectLIGH,                  // 0x20
    pub taper_weight: f32,                          // 0x28
    pub pad2c: u32,                                 // 0x2C
    pub effect_shader: *mut TESEffectShader,        // 0x30
    pub enchant_shader: *mut TESEffectShader,       // 0x38
    pub minimum_skill: i32,                         // 0x40
    pub spellmaking_area: i32,                      // 0x44
    pub spellmaking_charge_time: f32,               // 0x48
    pub taper_curve: f32,                           // 0x4C
    pub taper_duration: f32,                        // 0x50
    pub second_av_weight: f32,                      // 0x54
    pub archetype: EffectSettingArchetype,          // 0x58
    pub primary_av: ActorValue,                     // 0x5C
    pub projectile_base: *mut BGSProjectile,        // 0x60
    pub explosion: *mut BGSExplosion,               // 0x68
    pub casting_type: CastingType,                  // 0x70
    pub delivery: Delivery,                         // 0x74
    pub secondary_av: ActorValue,                   // 0x78
    pub casting_art: *mut BGSArtObject,             // 0x80
    pub hit_effect_art: *mut BGSArtObject,          // 0x88
    pub impact_data_set: *mut BGSImpactDataSet,     // 0x90
    pub skill_usage_mult: f32,                      // 0x98
    pub pad9c: u32,                                 // 0x9C
    pub dual_cast_data: *mut BGSDualCastData,       // 0xA0
    pub dual_cast_scale: f32,                       // 0xA8
    pub padac: u32,                                 // 0xAC
    pub enchant_effect_art: *mut BGSArtObject,      // 0xB0
    pub hit_visuals: *mut BGSReferenceEffect,       // 0xB8
    pub enchant_visuals: *mut BGSReferenceEffect,   // 0xC0
    pub equip_ability: *mut SpellItem,              // 0xC8
    pub image_space_mod: *mut TESImageSpaceModifier, // 0xD0
    pub perk: *mut BGSPerk,                         // 0xD8
    pub casting_sound_level: SOUND_LEVEL,           // 0xE0
    pub ai_score: f32,                              // 0xE4
    pub ai_delay_timer: f32,                        // 0xE8
    pub padec: u32,                                 // 0xEC
}

const _: () = assert!(core::mem::size_of::<EffectSettingData>() == 0xF0);
const _: () = assert!(core::mem::offset_of!(EffectSettingData, flags) == 0x00);
const _: () = assert!(core::mem::offset_of!(EffectSettingData, base_cost) == 0x04);
const _: () = assert!(core::mem::offset_of!(EffectSettingData, associated_form) == 0x08);
const _: () = assert!(core::mem::offset_of!(EffectSettingData, associated_skill) == 0x10);
const _: () = assert!(core::mem::offset_of!(EffectSettingData, resist_variable) == 0x14);
const _: () = assert!(core::mem::offset_of!(EffectSettingData, num_counter_effects) == 0x18);
const _: () = assert!(core::mem::offset_of!(EffectSettingData, pad1a) == 0x1A);
const _: () = assert!(core::mem::offset_of!(EffectSettingData, pad1c) == 0x1C);
const _: () = assert!(core::mem::offset_of!(EffectSettingData, light) == 0x20);
const _: () = assert!(core::mem::offset_of!(EffectSettingData, taper_weight) == 0x28);
const _: () = assert!(core::mem::offset_of!(EffectSettingData, pad2c) == 0x2C);
const _: () = assert!(core::mem::offset_of!(EffectSettingData, effect_shader) == 0x30);
const _: () = assert!(core::mem::offset_of!(EffectSettingData, enchant_shader) == 0x38);
const _: () = assert!(core::mem::offset_of!(EffectSettingData, minimum_skill) == 0x40);
const _: () = assert!(core::mem::offset_of!(EffectSettingData, spellmaking_area) == 0x44);
const _: () = assert!(core::mem::offset_of!(EffectSettingData, spellmaking_charge_time) == 0x48);
const _: () = assert!(core::mem::offset_of!(EffectSettingData, taper_curve) == 0x4C);
const _: () = assert!(core::mem::offset_of!(EffectSettingData, taper_duration) == 0x50);
const _: () = assert!(core::mem::offset_of!(EffectSettingData, second_av_weight) == 0x54);
const _: () = assert!(core::mem::offset_of!(EffectSettingData, archetype) == 0x58);
const _: () = assert!(core::mem::offset_of!(EffectSettingData, primary_av) == 0x5C);
const _: () = assert!(core::mem::offset_of!(EffectSettingData, projectile_base) == 0x60);
const _: () = assert!(core::mem::offset_of!(EffectSettingData, explosion) == 0x68);
const _: () = assert!(core::mem::offset_of!(EffectSettingData, casting_type) == 0x70);
const _: () = assert!(core::mem::offset_of!(EffectSettingData, delivery) == 0x74);
const _: () = assert!(core::mem::offset_of!(EffectSettingData, secondary_av) == 0x78);
const _: () = assert!(core::mem::offset_of!(EffectSettingData, casting_art) == 0x80);
const _: () = assert!(core::mem::offset_of!(EffectSettingData, hit_effect_art) == 0x88);
const _: () = assert!(core::mem::offset_of!(EffectSettingData, impact_data_set) == 0x90);
const _: () = assert!(core::mem::offset_of!(EffectSettingData, skill_usage_mult) == 0x98);
const _: () = assert!(core::mem::offset_of!(EffectSettingData, pad9c) == 0x9C);
const _: () = assert!(core::mem::offset_of!(EffectSettingData, dual_cast_data) == 0xA0);
const _: () = assert!(core::mem::offset_of!(EffectSettingData, dual_cast_scale) == 0xA8);
const _: () = assert!(core::mem::offset_of!(EffectSettingData, padac) == 0xAC);
const _: () = assert!(core::mem::offset_of!(EffectSettingData, enchant_effect_art) == 0xB0);
const _: () = assert!(core::mem::offset_of!(EffectSettingData, hit_visuals) == 0xB8);
const _: () = assert!(core::mem::offset_of!(EffectSettingData, enchant_visuals) == 0xC0);
const _: () = assert!(core::mem::offset_of!(EffectSettingData, equip_ability) == 0xC8);
const _: () = assert!(core::mem::offset_of!(EffectSettingData, image_space_mod) == 0xD0);
const _: () = assert!(core::mem::offset_of!(EffectSettingData, perk) == 0xD8);
const _: () = assert!(core::mem::offset_of!(EffectSettingData, casting_sound_level) == 0xE0);
const _: () = assert!(core::mem::offset_of!(EffectSettingData, ai_score) == 0xE4);
const _: () = assert!(core::mem::offset_of!(EffectSettingData, ai_delay_timer) == 0xE8);
const _: () = assert!(core::mem::offset_of!(EffectSettingData, padec) == 0xEC);

#[repr(C)]
pub struct EffectSettingSoundPair {
    pub id: SoundID,                        // 0x00
    pub pad04: u32,                         // 0x04
    pub sound: *mut BGSSoundDescriptorForm, // 0x08
}

const _: () = assert!(core::mem::size_of::<EffectSettingSoundPair>() == 0x10);
const _: () = assert!(core::mem::offset_of!(EffectSettingSoundPair, id) == 0x00);
const _: () = assert!(core::mem::offset_of!(EffectSettingSoundPair, pad04) == 0x04);
const _: () = assert!(core::mem::offset_of!(EffectSettingSoundPair, sound) == 0x08);

/// C++ `RE::EffectSetting`
#[repr(C)]
pub struct EffectSetting {
    pub base: TESForm,                                                     // 0x000
    pub full_name: TESFullName,                                            // 0x020
    pub menu_display_object: BGSMenuDisplayObject,                         // 0x030
    pub keyword_form: BGSKeywordForm,                                      // 0x040
    pub filter_validation_function: Option<EffectSettingFilterValidation>, // 0x058
    pub filter_validation_item: *mut c_void,                               // 0x060
    pub data: EffectSettingData,                                           // 0x068
    pub counter_effects: BSSimpleList<*mut EffectSetting>,                 // 0x158
    pub effect_sounds: BSTArray<EffectSettingSoundPair>,                   // 0x168
    pub magic_item_description: BSFixedString,                             // 0x180
    pub effect_loaded_count: i32,                                          // 0x188
    pub associated_item_loaded_count: i32,                                 // 0x18C
    pub conditions: TESCondition,                                          // 0x190
}

const _: () = assert!(core::mem::size_of::<EffectSetting>() == 0x198);
const _: () = assert!(core::mem::offset_of!(EffectSetting, base) == 0x000);
const _: () = assert!(core::mem::offset_of!(EffectSetting, full_name) == 0x020);
const _: () = assert!(core::mem::offset_of!(EffectSetting, menu_display_object) == 0x030);
const _: () = assert!(core::mem::offset_of!(EffectSetting, keyword_form) == 0x040);
const _: () = assert!(core::mem::offset_of!(EffectSetting, filter_validation_function) == 0x058);
const _: () = assert!(core::mem::offset_of!(EffectSetting, filter_validation_item) == 0x060);
const _: () = assert!(core::mem::offset_of!(EffectSetting, data) == 0x068);
const _: () = assert!(core::mem::offset_of!(EffectSetting, counter_effects) == 0x158);
const _: () = assert!(core::mem::offset_of!(EffectSetting, effect_sounds) == 0x168);
const _: () = assert!(core::mem::offset_of!(EffectSetting, magic_item_description) == 0x180);
const _: () = assert!(core::mem::offset_of!(EffectSetting, effect_loaded_count) == 0x188);
const _: () = assert!(core::mem::offset_of!(EffectSetting, associated_item_loaded_count) == 0x18C);
const _: () = assert!(core::mem::offset_of!(EffectSetting, conditions) == 0x190);

impl RttiType for EffectSetting {
    const RTTI: VariantID = RTTI_EffectSetting;
}

impl FormCastable for EffectSetting {
    const TARGET_FORM_TYPE: FormType = FormType::MagicEffect;
}

inherit!(EffectSetting : TESForm);
inherit!(EffectSetting => TESFullName, full_name);
inherit!(EffectSetting => BGSMenuDisplayObject, menu_display_object);
inherit!(EffectSetting => BGSKeywordForm, keyword_form);

impl EffectSetting {
    pub const RTTI: VariantID = RTTI_EffectSetting;
    pub const VTABLE: &'static [VariantID] = &VTABLE_EffectSetting;
    pub const FORMTYPE: FormType = FormType::MagicEffect;

    // override (TESForm)
    // void ClearData() override;                // 05
    // bool Load(TESFile* a_mod) override;       // 06
    // void InitItemImpl() override;             // 13
    // void Copy(TESForm* a_srcForm) override;   // 2F
    // const char* GetObjectTypeName() const;    // 39

    // override (BGSKeywordForm)
    // bool HasKeyword(const BGSKeyword* a_keyword) const override;  // 04

    #[inline]
    pub const fn get_archetype(&self) -> EffectSettingArchetype {
        self.data.archetype
    }

    #[inline]
    pub const fn get_magick_skill(&self) -> ActorValue {
        self.data.associated_skill
    }

    #[inline]
    pub const fn get_minimum_skill_level(&self) -> i32 {
        self.data.minimum_skill
    }

    pub fn get_archetype_as_string(&self) -> BSFixedString {
        let name = match self.data.archetype {
            EffectSettingArchetype::ValueModifier => "ValueMod",
            EffectSettingArchetype::Script => "Script",
            EffectSettingArchetype::Dispel => "Dispel",
            EffectSettingArchetype::CureDisease => "CureDisease",
            EffectSettingArchetype::Absorb => "Absorb",
            EffectSettingArchetype::DualValueModifier => "DualValueMod",
            EffectSettingArchetype::Calm => "Calm",
            EffectSettingArchetype::Demoralize => "Demoralize",
            EffectSettingArchetype::Frenzy => "Frenzy",
            EffectSettingArchetype::Disarm => "Disarm",
            EffectSettingArchetype::CommandSummoned => "CommandSummoned",
            EffectSettingArchetype::Invisibility => "Invisibility",
            EffectSettingArchetype::Light => "Light",
            EffectSettingArchetype::Lock => "Lock",
            EffectSettingArchetype::Open => "Open",
            EffectSettingArchetype::BoundWeapon => "BoundWeapon",
            EffectSettingArchetype::SummonCreature => "SummonCreature",
            EffectSettingArchetype::DetectLife => "DetectLife",
            EffectSettingArchetype::Telekinesis => "Telekinesis",
            EffectSettingArchetype::Paralysis => "Paralysis",
            EffectSettingArchetype::Reanimate => "Reanimate",
            EffectSettingArchetype::SoulTrap => "SoulTrap",
            EffectSettingArchetype::TurnUndead => "TurnUndead",
            EffectSettingArchetype::Guide => "Guide",
            EffectSettingArchetype::WerewolfFeed => "WerewolfFeed",
            EffectSettingArchetype::CureParalysis => "CureParalysis",
            EffectSettingArchetype::CureAddiction => "CureAddiction",
            EffectSettingArchetype::CurePoison => "CurePoison",
            EffectSettingArchetype::Concussion => "Concussion",
            EffectSettingArchetype::ValueAndParts => "ValueAndParts",
            EffectSettingArchetype::AccumulateMagnitude => "AccumulateMagnitude",
            EffectSettingArchetype::Stagger => "Stagger",
            EffectSettingArchetype::PeakValueModifier => "PeakValueMod",
            EffectSettingArchetype::Cloak => "Cloak",
            EffectSettingArchetype::Werewolf => "Werewolf",
            EffectSettingArchetype::SlowTime => "SlowTime",
            EffectSettingArchetype::Rally => "Rally",
            EffectSettingArchetype::EnhanceWeapon => "EnhanceWeapon",
            EffectSettingArchetype::SpawnHazard => "SpawnHazard",
            EffectSettingArchetype::Etherealize => "Etherealize",
            EffectSettingArchetype::Banish => "Banish",
            EffectSettingArchetype::Disguise => "Disguise",
            EffectSettingArchetype::GrabActor => "GrabActor",
            EffectSettingArchetype::VampireLord => "VampireLord",
            _ => return BSFixedString::empty(),
        };

        BSFixedString::from_str(name)
    }

    #[inline]
    pub fn has_archetype(&self, archetype: EffectSettingArchetype) -> bool {
        self.data.archetype == archetype
    }

    #[inline]
    pub fn has_keyword_string(&self, editor_id: &str) -> bool {
        self.keyword_form.has_keyword_string(editor_id)
    }

    #[inline]
    pub fn has_keyword(&self, keyword: *const BGSKeyword) -> bool {
        BGSKeywordForm::has_keyword(&self.keyword_form, keyword)
    }

    #[inline]
    pub fn is_detrimental(&self) -> bool {
        self.data.flags.all(EffectSettingDataFlag::Detrimental)
    }

    #[inline]
    pub fn is_hostile(&self) -> bool {
        self.data.flags.all(EffectSettingDataFlag::Hostile)
    }
}
