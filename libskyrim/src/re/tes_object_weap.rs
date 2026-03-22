use bitflags::bitflags;
use core_util::inherit;
use crate::virtual_method;

use crate::offsets::offsets_rtti::RTTI_TESObjectWEAP;
use crate::offsets::offsets_vtable::VTABLE_TESObjectWEAP;
use crate::re::bs_fixed_string::BSFixedString;
use crate::re::form_type::FormType;
use crate::re::form_traits::FormCastable;
use crate::relocation::{VariantID, RttiType};

use crate::re::tes_bound_object::TESBoundObject;
use crate::re::tes_full_name::TESFullName;
use crate::re::tes_model_texture_swap::TESModelTextureSwap;
use crate::re::tes_value_form::TESValueForm;
use crate::re::tes_weight_form::TESWeightForm;
use crate::re::bgs_keyword_form::BGSKeywordForm;

use crate::re::tes_icon::TESIcon;
use crate::re::tes_enchantable_form::TESEnchantableForm;
use crate::re::tes_attack_damage_form::TESAttackDamageForm;
use crate::re::bgs_destructible_object_form::BGSDestructibleObjectForm;
use crate::re::bgs_equip_type::BGSEquipType;
use crate::re::bgs_preloadable::BGSPreloadable;
use crate::re::bgs_message_icon::BGSMessageIcon;
use crate::re::bgs_pickup_putdown_sounds::BGSPickupPutdownSounds;
use crate::re::bgs_block_bash_data::BGSBlockBashData;
use crate::re::tes_description::TESDescription;

// Opaque types for pointers
use crate::re::spell_item::SpellItem;
use crate::re::tes_model::TESModel;
use crate::re::bgs_sound_descriptor_form::BGSSoundDescriptorForm;
use crate::re::bgs_impact_data_set::BGSImpactDataSet;
use crate::re::tes_object_stat::TESObjectSTAT;
use crate::re::actor_values::ActorValue;

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum WeaponHitBehavior {
    Normal = 0,
    DismemberOnly = 1,
    ExplodeOnly = 2,
    NoDismemberOrExplode = 3,
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum WeaponRumblePattern {
    Constant = 0,
    PeriodicSquare = 1,
    PeriodicTriangle = 2,
    PeriodicSawtooth = 3,
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum WeaponType {
    HandToHandMelee = 0,
    OneHandSword = 1,
    OneHandDagger = 2,
    OneHandAxe = 3,
    OneHandMace = 4,
    TwoHandSword = 5,
    TwoHandAxe = 6,
    Bow = 7,
    Staff = 8,
    Crossbow = 9,
    Total = 10,
}

#[repr(C)]
pub struct RangedData {
    pub sight_fov: f32,                          // 00
    pub fire_rate: f32,                          // 04
    pub firing_rumble_left_motor_strength: f32,  // 08
    pub firing_rumble_right_motor_strength: f32, // 0C
    pub firing_rumble_duration: f32,             // 10
    pub rumble_pattern: u32,                     // 14
    pub num_projectiles: i8,                     // 18
    pub pad19: u8,                               // 19
    pub pad1a: u16,                              // 1A
}

const _: () = assert!(core::mem::size_of::<RangedData>() == 0x1C);

bitflags! {
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct WeapFlags2: u16 {
        const NONE = 0;
        const PLAYER_ONLY = 1 << 0;
        const NPCS_USE_AMMO = 1 << 1;
        const NO_JAM_AFTER_RELOAD = 1 << 2;
        const MINOR_CRIME = 1 << 4;
        const RANGE_FIXED = 1 << 5;
        const NOT_USED_IN_NORMAL_COMBAT = 1 << 6;
        const OVERRIDES_CONDITION_DAMAGE = 1 << 7;
        const DONT_USE_3RD_PERSON_IS_ANIM = 1 << 8;
        const BURST_SHOT = 1 << 9;
        const RUMBLE_ALTERNATE = 1 << 10;
        const LONG_BURSTS = 1 << 11;
        const NON_HOSTILE = 1 << 12;
        const BOUND_WEAPON = 1 << 13;
    }
}

bitflags! {
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct WeapFlags1: u8 {
        const NONE = 0;
        const IGNORES_NORMAL_WEAPON_RESISTANCE = 1 << 0;
        const AUTOMATIC = 1 << 1;
        const HAS_SCOPE = 1 << 2;
        const CANT_DROP = 1 << 3;
        const HIDE_BACKPACK = 1 << 4;
        const EMBEDDED_WEAPON = 1 << 5;
        const DONT_USE_FIRST_PERSON_IS_ANIM = 1 << 6;
        const NON_PLAYABLE = 1 << 7;
    }
}

#[repr(C)]
pub struct WeaponData {
    pub ranged_data: *mut RangedData,            // 00
    pub speed: f32,                              // 08
    pub reach: f32,                              // 0C
    pub min_range: f32,                          // 10
    pub max_range: f32,                          // 14
    pub animation_attack_mult: f32,              // 18
    pub damage_to_weapon_mult: f32,              // 1C
    pub stagger_value: f32,                      // 20
    pub hit_behavior: u32,                       // 24 (EnumSet WEAPONHITBEHAVIOR, u32)
    pub skill: ActorValue,                       // 28 (EnumSet ActorValue)
    pub resistance: ActorValue,                  // 2C (EnumSet ActorValue)
    pub flags2: WeapFlags2,                      // 30
    pub base_vats_to_hit_chance: u8,             // 32
    pub attack_animation: u8,                    // 33 (EnumSet AttackAnimation, u8)
    pub embedded_weapon_av: u8,                  // 34 (EnumSet ActorValue, u8)
    pub animation_type: u8,                      // 35 (EnumSet WEAPON_TYPE, u8)
    pub flags: WeapFlags1,                       // 36
    pub unk37: u8,                               // 37
}

const _: () = assert!(core::mem::size_of::<WeaponData>() == 0x38);

bitflags! {
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct CritFlags: u8 {
        const NONE = 0;
        const ON_DEATH = 1 << 0;
    }
}

#[repr(C)]
pub struct CriticalData {
    pub prcnt_mult: f32,                         // 00
    pub pad04: u32,                              // 04
    pub effect: *mut SpellItem,                  // 08
    pub damage: u16,                             // 10
    pub flags: CritFlags,                        // 12
    pub pad13: u8,                               // 13
    pub pad14: u32,                              // 14
}

const _: () = assert!(core::mem::size_of::<CriticalData>() == 0x18);

#[repr(C)]
pub struct ScopeArt {
    pub unk00: TESModel, // 00 
    pub unk28: *mut core::ffi::c_void, // 28 - TESEffectShader* EIDT
}

const _: () = assert!(core::mem::size_of::<ScopeArt>() == 0x30);


#[repr(C)]
pub struct TESObjectWEAP {
    pub base: TESBoundObject,                     // 000 (Size: 0)
    pub _pad_000: [u8; 0x30],                     // 000 Padding for TESBoundObject
    pub tes_full_name: TESFullName,               // 030 (Size: 0x10)
    pub tes_model_texture_swap: TESModelTextureSwap, // 040 (Size: 0x38)
    
    pub tes_icon: TESIcon,                        // 078 (Size: 0)
    pub _pad_078: [u8; 0x10],                     // 078
    
    pub tes_enchantable_form: TESEnchantableForm, // 088 (Size: 0)
    pub _pad_088: [u8; 0x18],                     // 088
    
    pub tes_value_form: TESValueForm,             // 0A0 (Size: 0x10)
    pub tes_weight_form: TESWeightForm,           // 0B0 (Size: 0x10)
    
    pub tes_attack_damage_form: TESAttackDamageForm, // 0C0 (Size: 0)
    pub _pad_0c0: [u8; 0x10],                     // 0C0
    
    pub bgs_destructible_object_form: BGSDestructibleObjectForm, // 0D0 (Size: 0)
    pub _pad_0d0: [u8; 0x10],                     // 0D0
    
    pub bgs_equip_type: BGSEquipType,             // 0E0 (Size: 0)
    pub _pad_0e0: [u8; 0x10],                     // 0E0
    
    pub bgs_preloadable: BGSPreloadable,          // 0F0 (Size: 0)
    pub _pad_0f0: [u8; 0x08],                     // 0F0
    
    pub bgs_message_icon: BGSMessageIcon,         // 0F8 (Size: 0)
    pub _pad_0f8: [u8; 0x18],                     // 0F8
    
    pub bgs_pickup_putdown_sounds: BGSPickupPutdownSounds, // 110 (Size: 0)
    pub _pad_110: [u8; 0x18],                     // 110
    
    pub bgs_block_bash_data: BGSBlockBashData,    // 128 (Size: 0)
    pub _pad_128: [u8; 0x18],                     // 128
    
    pub bgs_keyword_form: BGSKeywordForm,         // 140 (Size: 0x18)
    
    pub tes_description: TESDescription,          // 158 (Size: 0)
    pub _pad_158: [u8; 0x10],                     // 158

    pub weapon_data: WeaponData,                  // 168 - DNAM
    pub critical_data: CriticalData,              // 1A0 - CRDT
    pub scope_art: *mut ScopeArt,                 // 1B8
    pub attack_sound: *mut BGSSoundDescriptorForm,// 1C0 - SNAM
    pub attack_sound_2d: *mut BGSSoundDescriptorForm, // 1C8 - XNAM
    pub attack_loop_sound: *mut BGSSoundDescriptorForm, // 1D0 - NAM7
    pub attack_fail_sound: *mut BGSSoundDescriptorForm, // 1D8 - TNAM
    pub idle_sound: *mut BGSSoundDescriptorForm,  // 1E0 - UNAM
    pub equip_sound: *mut BGSSoundDescriptorForm, // 1E8 - NAM9
    pub unequip_sound: *mut BGSSoundDescriptorForm, // 1F0 - NAM8
    pub impact_data_set: *mut BGSImpactDataSet,   // 1F8
    pub first_person_model_object: *mut TESObjectSTAT, // 200 - WNAM
    pub template_weapon: *mut TESObjectWEAP,      // 208 - CNAM
    pub embedded_node: BSFixedString,             // 210
    pub sound_level: u32,                         // 218 - VNAM
    pub pad21c: u32,                              // 21C
}

const _: () = assert!(core::mem::size_of::<TESObjectWEAP>() == 0x220);

impl RttiType for TESObjectWEAP {
    const RTTI: VariantID = RTTI_TESObjectWEAP;
}

impl FormCastable for TESObjectWEAP {
    const TARGET_FORM_TYPE: FormType = FormType::Weapon;
}

inherit!(TESObjectWEAP : TESBoundObject);

impl TESObjectWEAP {
    pub const RTTI: VariantID = RTTI_TESObjectWEAP;
    pub const VTABLE: &'static [VariantID] = &VTABLE_TESObjectWEAP;

    pub fn get_speed(&self) -> f32 { self.weapon_data.speed }
    pub fn get_reach(&self) -> f32 { self.weapon_data.reach }
    pub fn get_stagger(&self) -> f32 { self.weapon_data.stagger_value }
    pub fn get_min_range(&self) -> f32 { self.weapon_data.min_range }
    pub fn get_max_range(&self) -> f32 { self.weapon_data.max_range }
    pub fn get_crit_damage(&self) -> u16 { self.critical_data.damage }
    
    crate::relocation_func! {
        pub fn get_fire_node(this: &TESObjectWEAP, root: *mut core::ffi::c_void) -> *mut core::ffi::c_void => VariantID::new(17689, 18098, 0)
    }

    pub fn get_weapon_type(&self) -> WeaponType {
        // animation_type is u8 inside EnumSet
        unsafe { core::mem::transmute::<u32, WeaponType>(self.weapon_data.animation_type as u32) }
    }

    pub fn is_bound(&self) -> bool {
        self.weapon_data.flags2.contains(WeapFlags2::BOUND_WEAPON)
    }

    pub fn is_melee(&self) -> bool {
        let ty = self.get_weapon_type();
        matches!(ty, WeaponType::HandToHandMelee | WeaponType::OneHandSword | WeaponType::OneHandDagger | WeaponType::OneHandAxe | WeaponType::OneHandMace | WeaponType::TwoHandSword | WeaponType::TwoHandAxe)
    }

    pub fn is_ranged(&self) -> bool {
        let ty = self.get_weapon_type();
        matches!(ty, WeaponType::Bow | WeaponType::Staff | WeaponType::Crossbow)
    }
}
