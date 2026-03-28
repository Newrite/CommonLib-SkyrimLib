use bitflags::bitflags;
use core::fmt::Write;
use core_util::{Enum, StringBuffer, inherit};

use crate::offsets::offsets_rtti::RTTI_TESObjectWEAP;
use crate::offsets::offsets_vtable::VTABLE_TESObjectWEAP;
use crate::re::bs_fixed_string::BSFixedString;
use crate::re::form_traits::FormCastable;
use crate::re::form_type::FormType;
use crate::relocation::{RelocationID, RttiType, VariantID};

use crate::re::bgs_keyword_form::BGSKeywordForm;
use crate::re::tes_bound_object::TESBoundObject;
use crate::re::tes_full_name::TESFullName;
use crate::re::tes_model_texture_swap::TESModelTextureSwap;
use crate::re::tes_value_form::TESValueForm;
use crate::re::tes_weight_form::TESWeightForm;

use crate::re::bgs_block_bash_data::BGSBlockBashData;
use crate::re::bgs_destructible_object_form::BGSDestructibleObjectForm;
use crate::re::bgs_equip_type::BGSEquipType;
use crate::re::bgs_message_icon::BGSMessageIcon;
use crate::re::bgs_pickup_putdown_sounds::BGSPickupPutdownSounds;
use crate::re::bgs_preloadable::BGSPreloadable;
use crate::re::tes_attack_damage_form::TESAttackDamageForm;
use crate::re::tes_description::TESDescription;
use crate::re::tes_enchantable_form::TESEnchantableForm;
use crate::re::tes_icon::TESIcon;

// Opaque types for pointers
use crate::re::actor_values::ActorValue;
use crate::re::bgs_impact_data_set::BGSImpactDataSet;
use crate::re::bgs_sound_descriptor_form::BGSSoundDescriptorForm;
use crate::re::sound_levels::SoundLevel;
use crate::re::spell_item::SpellItem;
use crate::re::tes_effect_shader::TESEffectShader;
use crate::re::tes_model::TESModel;
use crate::re::tes_object_stat::TESObjectSTAT;

use crate::re::ni_av_object::NiAVObject;

// use crate::re::bgs_keyword::BGSKeyword;

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum WeaponHitBehavior {
    Normal = 0,
    DismemberOnly = 1,
    ExplodeOnly = 2,
    NoDismemberOrExplode = 3,
}

core_util::impl_enumset_type!(WeaponHitBehavior => u32);

impl TryFrom<u32> for WeaponHitBehavior {
    type Error = ();

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        if value <= Self::NoDismemberOrExplode as u32 {
            Ok(unsafe { core::mem::transmute::<u32, Self>(value) })
        } else {
            Err(())
        }
    }
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum WeaponRumblePattern {
    Constant = 0,
    PeriodicSquare = 1,
    PeriodicTriangle = 2,
    PeriodicSawtooth = 3,
}

core_util::impl_enumset_type!(WeaponRumblePattern => u32);

impl TryFrom<u32> for WeaponRumblePattern {
    type Error = ();

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        if value <= Self::PeriodicSawtooth as u32 {
            Ok(unsafe { core::mem::transmute::<u32, Self>(value) })
        } else {
            Err(())
        }
    }
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

core_util::impl_enumset_type!(WeaponType => u8);
core_util::impl_enumset_type!(WeaponType => u32);

impl TryFrom<u8> for WeaponType {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Self::try_from(value as u32)
    }
}

impl TryFrom<u32> for WeaponType {
    type Error = ();

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        if value <= Self::Total as u32 {
            Ok(unsafe { core::mem::transmute::<u32, Self>(value) })
        } else {
            Err(())
        }
    }
}

#[repr(C)]
#[derive(bytemuck::Zeroable)]
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

impl RangedData {
    #[inline(always)]
    pub const fn rumble_pattern_storage(&self) -> Enum<WeaponRumblePattern, u32> {
        Enum::from_underlying(self.rumble_pattern)
    }

    #[inline(always)]
    pub fn try_get_rumble_pattern(&self) -> Option<WeaponRumblePattern> {
        self.rumble_pattern_storage().get()
    }
}

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

unsafe impl bytemuck::Zeroable for WeapFlags2 {}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum AttackAnimation {
    AttackLeft = 26,
    AttackRight = 32,
    Attack3 = 38,
    Attack4 = 44,
    Attack5 = 50,
    Attack7 = 62,
    Attack8 = 68,
    AttackLoop = 74,
    AttackSpin = 80,
    AttackSpin2 = 86,
    PlaceMine = 97,
    PlaceMine2 = 103,
    AttackThrow = 109,
    AttackThrow2 = 115,
    AttackThrow3 = 121,
    AttackThrow4 = 127,
    AttackThrow5 = 133,
    Default = 255,
}

core_util::impl_enumset_type!(AttackAnimation => u8);

impl TryFrom<u8> for AttackAnimation {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            26 => Ok(Self::AttackLeft),
            32 => Ok(Self::AttackRight),
            38 => Ok(Self::Attack3),
            44 => Ok(Self::Attack4),
            50 => Ok(Self::Attack5),
            62 => Ok(Self::Attack7),
            68 => Ok(Self::Attack8),
            74 => Ok(Self::AttackLoop),
            80 => Ok(Self::AttackSpin),
            86 => Ok(Self::AttackSpin2),
            97 => Ok(Self::PlaceMine),
            103 => Ok(Self::PlaceMine2),
            109 => Ok(Self::AttackThrow),
            115 => Ok(Self::AttackThrow2),
            121 => Ok(Self::AttackThrow3),
            127 => Ok(Self::AttackThrow4),
            133 => Ok(Self::AttackThrow5),
            255 => Ok(Self::Default),
            _ => Err(()),
        }
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

unsafe impl bytemuck::Zeroable for WeapFlags1 {}

#[repr(C)]
#[derive(bytemuck::Zeroable)]
pub struct WeaponData {
    pub ranged_data: *mut RangedData, // 00
    pub speed: f32,                   // 08
    pub reach: f32,                   // 0C
    pub min_range: f32,               // 10
    pub max_range: f32,               // 14
    pub animation_attack_mult: f32,   // 18
    pub damage_to_weapon_mult: f32,   // 1C
    pub stagger_value: f32,           // 20
    pub hit_behavior: u32,            // 24 (EnumSet WEAPONHITBEHAVIOR, u32)
    pub skill: u32,                   // 28 (EnumSet ActorValue)
    pub resistance: u32,              // 2C (EnumSet ActorValue)
    pub flags2: WeapFlags2,           // 30
    pub base_vats_to_hit_chance: u8,  // 32
    pub attack_animation: u8,         // 33 (EnumSet AttackAnimation, u8)
    pub embedded_weapon_av: u8,       // 34 (EnumSet ActorValue, u8)
    pub animation_type: u8,           // 35 (EnumSet WEAPON_TYPE, u8)
    pub flags: WeapFlags1,            // 36
    pub unk37: u8,                    // 37
}

const _: () = assert!(core::mem::size_of::<WeaponData>() == 0x38);

impl WeaponData {
    #[inline(always)]
    pub const fn hit_behavior_storage(&self) -> Enum<WeaponHitBehavior, u32> {
        Enum::from_underlying(self.hit_behavior)
    }

    #[inline(always)]
    pub fn try_get_hit_behavior(&self) -> Option<WeaponHitBehavior> {
        self.hit_behavior_storage().get()
    }

    #[inline(always)]
    pub const fn skill_storage(&self) -> Enum<ActorValue, u32> {
        Enum::from_underlying(self.skill)
    }

    #[inline(always)]
    pub fn try_get_skill(&self) -> Option<ActorValue> {
        self.skill_storage().get()
    }

    #[inline(always)]
    pub const fn resistance_storage(&self) -> Enum<ActorValue, u32> {
        Enum::from_underlying(self.resistance)
    }

    #[inline(always)]
    pub fn try_get_resistance(&self) -> Option<ActorValue> {
        self.resistance_storage().get()
    }

    #[inline(always)]
    pub const fn attack_animation_storage(&self) -> Enum<AttackAnimation, u8> {
        Enum::from_underlying(self.attack_animation)
    }

    #[inline(always)]
    pub fn try_get_attack_animation(&self) -> Option<AttackAnimation> {
        self.attack_animation_storage().get()
    }

    #[inline(always)]
    pub const fn embedded_weapon_av_storage(&self) -> Enum<ActorValue, u8> {
        Enum::from_underlying(self.embedded_weapon_av)
    }

    #[inline(always)]
    pub fn try_get_embedded_weapon_av(&self) -> Option<ActorValue> {
        self.embedded_weapon_av_storage().get()
    }

    #[inline(always)]
    pub const fn animation_type_storage(&self) -> Enum<WeaponType, u8> {
        Enum::from_underlying(self.animation_type)
    }

    #[inline(always)]
    pub fn try_get_animation_type(&self) -> Option<WeaponType> {
        self.animation_type_storage().get()
    }
}

bitflags! {
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct CritFlags: u8 {
        const NONE = 0;
        const ON_DEATH = 1 << 0;
    }
}

unsafe impl bytemuck::Zeroable for CritFlags {}

#[repr(C)]
#[derive(bytemuck::Zeroable)]
pub struct CriticalData {
    pub prcnt_mult: f32,        // 00
    pub pad04: u32,             // 04
    pub effect: *mut SpellItem, // 08
    pub damage: u16,            // 10
    pub flags: CritFlags,       // 12
    pub pad13: u8,              // 13
    pub pad14: u32,             // 14
}

const _: () = assert!(core::mem::size_of::<CriticalData>() == 0x18);

#[repr(C)]
pub struct ScopeArt {
    pub unk00: TESModel,             // 00
    pub unk28: *mut TESEffectShader, // 28 - EIDT
}

const _: () = assert!(core::mem::size_of::<ScopeArt>() == 0x30);

bitflags! {
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct RecordFlags: u32 {
        const NON_PLAYABLE = 1 << 2;
        const HAS_INHERITED_FROM_TEMPLATE = 1 << 3;
        const DELETED = 1 << 5;
        const IGNORED = 1 << 12;
    }
}

unsafe impl bytemuck::Zeroable for RecordFlags {}

#[repr(C)]
pub struct TESObjectWEAP {
    pub base: TESBoundObject,                                    // 000
    pub tes_full_name: TESFullName,                              // 030
    pub tes_model_texture_swap: TESModelTextureSwap,             // 040
    pub tes_icon: TESIcon,                                       // 078
    pub tes_enchantable_form: TESEnchantableForm,                // 088
    pub tes_value_form: TESValueForm,                            // 0A0
    pub tes_weight_form: TESWeightForm,                          // 0B0
    pub tes_attack_damage_form: TESAttackDamageForm,             // 0C0
    pub bgs_destructible_object_form: BGSDestructibleObjectForm, // 0D0
    pub bgs_equip_type: BGSEquipType,                            // 0E0
    pub bgs_preloadable: BGSPreloadable,                         // 0F0
    pub bgs_message_icon: BGSMessageIcon,                        // 0F8
    pub bgs_pickup_putdown_sounds: BGSPickupPutdownSounds,       // 110
    pub bgs_block_bash_data: BGSBlockBashData,                   // 128
    pub bgs_keyword_form: BGSKeywordForm,                        // 140
    pub tes_description: TESDescription,                         // 158

    pub weapon_data: WeaponData,                        // 168 - DNAM
    pub critical_data: CriticalData,                    // 1A0 - CRDT
    pub scope_art: *mut ScopeArt,                       // 1B8
    pub attack_sound: *mut BGSSoundDescriptorForm,      // 1C0 - SNAM
    pub attack_sound_2d: *mut BGSSoundDescriptorForm,   // 1C8 - XNAM
    pub attack_loop_sound: *mut BGSSoundDescriptorForm, // 1D0 - NAM7
    pub attack_fail_sound: *mut BGSSoundDescriptorForm, // 1D8 - TNAM
    pub idle_sound: *mut BGSSoundDescriptorForm,        // 1E0 - UNAM
    pub equip_sound: *mut BGSSoundDescriptorForm,       // 1E8 - NAM9
    pub unequip_sound: *mut BGSSoundDescriptorForm,     // 1F0 - NAM8
    pub impact_data_set: *mut BGSImpactDataSet,         // 1F8
    pub first_person_model_object: *mut TESObjectSTAT,  // 200 - WNAM
    pub template_weapon: *mut TESObjectWEAP,            // 208 - CNAM
    pub embedded_node: BSFixedString,                   // 210
    pub sound_level: u32,                               // 218 - VNAM
    pub pad21c: u32,                                    // 21C
}

const _: () = assert!(core::mem::size_of::<TESObjectWEAP>() == 0x220);

// Assertions for mixin offsets
const _: () = assert!(core::mem::offset_of!(TESObjectWEAP, tes_full_name) == 0x30);
const _: () = assert!(core::mem::offset_of!(TESObjectWEAP, tes_model_texture_swap) == 0x40);
const _: () = assert!(core::mem::offset_of!(TESObjectWEAP, tes_icon) == 0x78);
const _: () = assert!(core::mem::offset_of!(TESObjectWEAP, tes_enchantable_form) == 0x88);
const _: () = assert!(core::mem::offset_of!(TESObjectWEAP, tes_value_form) == 0xA0);
const _: () = assert!(core::mem::offset_of!(TESObjectWEAP, tes_weight_form) == 0xB0);
const _: () = assert!(core::mem::offset_of!(TESObjectWEAP, tes_attack_damage_form) == 0xC0);
const _: () = assert!(core::mem::offset_of!(TESObjectWEAP, bgs_destructible_object_form) == 0xD0);
const _: () = assert!(core::mem::offset_of!(TESObjectWEAP, bgs_equip_type) == 0xE0);
const _: () = assert!(core::mem::offset_of!(TESObjectWEAP, bgs_preloadable) == 0xF0);
const _: () = assert!(core::mem::offset_of!(TESObjectWEAP, bgs_message_icon) == 0xF8);
const _: () = assert!(core::mem::offset_of!(TESObjectWEAP, bgs_pickup_putdown_sounds) == 0x110);
const _: () = assert!(core::mem::offset_of!(TESObjectWEAP, bgs_block_bash_data) == 0x128);
const _: () = assert!(core::mem::offset_of!(TESObjectWEAP, bgs_keyword_form) == 0x140);
const _: () = assert!(core::mem::offset_of!(TESObjectWEAP, tes_description) == 0x158);

impl RttiType for TESObjectWEAP {
    const RTTI: VariantID = RTTI_TESObjectWEAP;
}

impl FormCastable for TESObjectWEAP {
    const TARGET_FORM_TYPE: FormType = FormType::Weapon;
}

inherit!(TESObjectWEAP : TESBoundObject);
inherit!(TESObjectWEAP => TESFullName, tes_full_name);
inherit!(TESObjectWEAP => TESModelTextureSwap, tes_model_texture_swap);
inherit!(TESObjectWEAP => TESIcon, tes_icon);
inherit!(TESObjectWEAP => TESEnchantableForm, tes_enchantable_form);
inherit!(TESObjectWEAP => TESValueForm, tes_value_form);
inherit!(TESObjectWEAP => TESWeightForm, tes_weight_form);
inherit!(TESObjectWEAP => TESAttackDamageForm, tes_attack_damage_form);
inherit!(TESObjectWEAP => BGSDestructibleObjectForm, bgs_destructible_object_form);
inherit!(TESObjectWEAP => BGSEquipType, bgs_equip_type);
inherit!(TESObjectWEAP => BGSPreloadable, bgs_preloadable);
inherit!(TESObjectWEAP => BGSMessageIcon, bgs_message_icon);
inherit!(TESObjectWEAP => BGSPickupPutdownSounds, bgs_pickup_putdown_sounds);
inherit!(TESObjectWEAP => BGSBlockBashData, bgs_block_bash_data);
inherit!(TESObjectWEAP => BGSKeywordForm, bgs_keyword_form);
inherit!(TESObjectWEAP => TESDescription, tes_description);

impl TESObjectWEAP {
    pub const RTTI: VariantID = RTTI_TESObjectWEAP;
    pub const VTABLE: &'static [VariantID] = &VTABLE_TESObjectWEAP;
    pub const FORMTYPE: FormType = FormType::Weapon;

    crate::virtual_method! {
        pub const VFUNC_DTOR: usize = 0x00;
        pub fn dtor()
    }

    // ~TESObjectWEAP() override;                     // 00
    // override (TESBoundObject)
    // void InitializeData() override;                    // 04
    // void ClearData() override;                         // 05
    // bool Load(TESFile* a_mod) override;                // 06
    // void SaveGame(BGSSaveFormBuffer* a_buf) override;  // 0E
    // void LoadGame(BGSLoadFormBuffer* a_buf) override;  // 0F
    // void InitItemImpl() override;                      // 13
    // TESFile* GetDescriptionOwnerFile() const override; // 14
    // bool GetPlayable() const override;                 // 19
    // const char* GetObjectTypeName() const override;    // 39

    // ADDED: gap-fill
    // override (BGSKeywordForm)
    // BGSKeyword* GetDefaultKeyword() const override;    // 05

    pub fn get_speed(&self) -> f32 {
        self.weapon_data.speed
    }
    pub fn get_reach(&self) -> f32 {
        self.weapon_data.reach
    }
    pub fn get_stagger(&self) -> f32 {
        self.weapon_data.stagger_value
    }
    pub fn get_min_range(&self) -> f32 {
        self.weapon_data.min_range
    }
    pub fn get_max_range(&self) -> f32 {
        self.weapon_data.max_range
    }
    pub fn get_crit_damage(&self) -> u16 {
        self.critical_data.damage
    }

    // ADDED: gap-fill
    // RELOCATION_ID SE: 17689, AE: 18098
    crate::relocation_func! {
        pub fn get_fire_node(this: &TESObjectWEAP, root: *mut NiAVObject) -> *mut NiAVObject => RelocationID::new(17689, 18098)
    }

    pub fn get_node_name(&self, _dst_buff: *mut core::ffi::c_char) {
        if _dst_buff.is_null() {
            return;
        }

        let mut buffer = StringBuffer::<19>::new();
        let _ = write!(&mut buffer, "Weapon  ({:08X})", self.form_id);
        let bytes = buffer.as_c_str().to_bytes_with_nul();
        unsafe {
            core::ptr::copy_nonoverlapping(bytes.as_ptr().cast(), _dst_buff, bytes.len());
        }
    }

    #[inline(always)]
    pub fn try_get_weapon_type(&self) -> Option<WeaponType> {
        self.weapon_data.try_get_animation_type()
    }

    pub fn get_weapon_type(&self) -> WeaponType {
        self.try_get_weapon_type().unwrap_or(WeaponType::Total)
    }

    pub fn is_bound(&self) -> bool {
        self.weapon_data.flags2.contains(WeapFlags2::BOUND_WEAPON)
    }

    pub fn is_melee(&self) -> bool {
        matches!(
            self.try_get_weapon_type(),
            Some(
                WeaponType::HandToHandMelee
                    | WeaponType::OneHandSword
                    | WeaponType::OneHandDagger
                    | WeaponType::OneHandAxe
                    | WeaponType::OneHandMace
                    | WeaponType::TwoHandSword
                    | WeaponType::TwoHandAxe
            )
        )
    }

    pub fn is_ranged(&self) -> bool {
        matches!(
            self.try_get_weapon_type(),
            Some(WeaponType::Bow | WeaponType::Staff | WeaponType::Crossbow)
        )
    }

    pub fn is_hand_to_hand_melee(&self) -> bool {
        self.weapon_data.animation_type_storage() == WeaponType::HandToHandMelee
    }

    pub fn is_one_handed_sword(&self) -> bool {
        self.weapon_data.animation_type_storage() == WeaponType::OneHandSword
    }

    pub fn is_one_handed_dagger(&self) -> bool {
        self.weapon_data.animation_type_storage() == WeaponType::OneHandDagger
    }

    pub fn is_one_handed_axe(&self) -> bool {
        self.weapon_data.animation_type_storage() == WeaponType::OneHandAxe
    }

    pub fn is_one_handed_mace(&self) -> bool {
        self.weapon_data.animation_type_storage() == WeaponType::OneHandMace
    }

    pub fn is_two_handed_sword(&self) -> bool {
        self.weapon_data.animation_type_storage() == WeaponType::TwoHandSword
    }

    pub fn is_two_handed_axe(&self) -> bool {
        self.weapon_data.animation_type_storage() == WeaponType::TwoHandAxe
    }

    pub fn is_bow(&self) -> bool {
        self.weapon_data.animation_type_storage() == WeaponType::Bow
    }

    pub fn is_staff(&self) -> bool {
        self.weapon_data.animation_type_storage() == WeaponType::Staff
    }

    pub fn is_crossbow(&self) -> bool {
        self.weapon_data.animation_type_storage() == WeaponType::Crossbow
    }

    #[inline(always)]
    pub const fn sound_level_storage(&self) -> Enum<SoundLevel, u32> {
        Enum::from_underlying(self.sound_level)
    }

    #[inline(always)]
    pub fn try_get_sound_level(&self) -> Option<SoundLevel> {
        self.sound_level_storage().get()
    }

    #[inline]
    pub fn get_fire_node_ptr(&self, root: *mut NiAVObject) -> *mut NiAVObject {
        Self::get_fire_node(self, root)
    }

    /// SAFETY: root and returned pointer must be valid or null
    #[inline]
    pub fn get_fire_node_ref(&self, root: *mut NiAVObject) -> Option<&NiAVObject> {
        unsafe { Self::get_fire_node(self, root).as_ref() }
    }
}

pub trait TESObjectWEAPExt {
    fn get_speed(&self) -> f32;
    fn get_reach(&self) -> f32;
    fn get_stagger(&self) -> f32;
    fn get_min_range(&self) -> f32;
    fn get_max_range(&self) -> f32;
    fn get_crit_damage(&self) -> u16;
    fn get_fire_node_ptr(&self, root: *mut NiAVObject) -> *mut NiAVObject;
    fn get_fire_node_ref(&self, root: *mut NiAVObject) -> Option<&NiAVObject>;
    fn get_node_name(&self, dst_buff: *mut core::ffi::c_char);
    fn try_get_sound_level(&self) -> Option<SoundLevel>;
    fn try_get_weapon_type(&self) -> Option<WeaponType>;
    fn get_weapon_type(&self) -> WeaponType;
    fn is_bound(&self) -> bool;
    fn is_melee(&self) -> bool;
    fn is_ranged(&self) -> bool;
    fn is_hand_to_hand_melee(&self) -> bool;
    fn is_one_handed_sword(&self) -> bool;
    fn is_one_handed_dagger(&self) -> bool;
    fn is_one_handed_axe(&self) -> bool;
    fn is_one_handed_mace(&self) -> bool;
    fn is_two_handed_sword(&self) -> bool;
    fn is_two_handed_axe(&self) -> bool;
    fn is_bow(&self) -> bool;
    fn is_staff(&self) -> bool;
    fn is_crossbow(&self) -> bool;
}

impl<T: AsRef<TESObjectWEAP>> TESObjectWEAPExt for T {
    fn get_speed(&self) -> f32 {
        self.as_ref().get_speed()
    }
    fn get_reach(&self) -> f32 {
        self.as_ref().get_reach()
    }
    fn get_stagger(&self) -> f32 {
        self.as_ref().get_stagger()
    }
    fn get_min_range(&self) -> f32 {
        self.as_ref().get_min_range()
    }
    fn get_max_range(&self) -> f32 {
        self.as_ref().get_max_range()
    }
    fn get_crit_damage(&self) -> u16 {
        self.as_ref().get_crit_damage()
    }
    fn get_fire_node_ptr(&self, root: *mut NiAVObject) -> *mut NiAVObject {
        self.as_ref().get_fire_node_ptr(root)
    }
    fn get_fire_node_ref(&self, root: *mut NiAVObject) -> Option<&NiAVObject> {
        self.as_ref().get_fire_node_ref(root)
    }
    fn get_node_name(&self, dst_buff: *mut core::ffi::c_char) {
        self.as_ref().get_node_name(dst_buff)
    }
    fn try_get_sound_level(&self) -> Option<SoundLevel> {
        self.as_ref().try_get_sound_level()
    }
    fn try_get_weapon_type(&self) -> Option<WeaponType> {
        self.as_ref().try_get_weapon_type()
    }
    fn get_weapon_type(&self) -> WeaponType {
        self.as_ref().get_weapon_type()
    }
    fn is_bound(&self) -> bool {
        self.as_ref().is_bound()
    }
    fn is_melee(&self) -> bool {
        self.as_ref().is_melee()
    }
    fn is_ranged(&self) -> bool {
        self.as_ref().is_ranged()
    }
    fn is_hand_to_hand_melee(&self) -> bool {
        self.as_ref().is_hand_to_hand_melee()
    }
    fn is_one_handed_sword(&self) -> bool {
        self.as_ref().is_one_handed_sword()
    }
    fn is_one_handed_dagger(&self) -> bool {
        self.as_ref().is_one_handed_dagger()
    }
    fn is_one_handed_axe(&self) -> bool {
        self.as_ref().is_one_handed_axe()
    }
    fn is_one_handed_mace(&self) -> bool {
        self.as_ref().is_one_handed_mace()
    }
    fn is_two_handed_sword(&self) -> bool {
        self.as_ref().is_two_handed_sword()
    }
    fn is_two_handed_axe(&self) -> bool {
        self.as_ref().is_two_handed_axe()
    }
    fn is_bow(&self) -> bool {
        self.as_ref().is_bow()
    }
    fn is_staff(&self) -> bool {
        self.as_ref().is_staff()
    }
    fn is_crossbow(&self) -> bool {
        self.as_ref().is_crossbow()
    }
}
