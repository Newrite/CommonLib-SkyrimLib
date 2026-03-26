use bitflags::bitflags;
use core_util::{EnumSet, inherit};

use crate::offsets::offsets_rtti::RTTI_TESClass;
use crate::offsets::offsets_vtable::VTABLE_TESClass;
use crate::re::form_traits::FormCastable;
use crate::re::form_type::FormType;
use crate::re::tes_description::TESDescription;
use crate::re::tes_file::TESFile;
use crate::re::tes_form::TESForm;
use crate::re::tes_full_name::TESFullName;
use crate::re::tes_texture::TESTexture;
use crate::relocation::{RttiType, VariantID};
use crate::virtual_method;

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TESClassSkill {
    OneHanded = 0,
    TwoHanded = 1,
    Archery = 2,
    Block = 3,
    Smithing = 4,
    HeavyArmor = 5,
    LightArmor = 6,
    Pickpocket = 7,
    Lockpicking = 8,
    Sneak = 9,
    Alchemy = 10,
    Speech = 11,
    Alteration = 12,
    Conjuration = 13,
    Destruction = 14,
    Illusion = 15,
    Restoration = 16,
    Enchanting = 17,
}

core_util::impl_enumset_type!(TESClassSkill => u8);

#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct TESClassSkillWeights {
    pub one_handed: u8,
    pub two_handed: u8,
    pub archery: u8,
    pub block: u8,
    pub smithing: u8,
    pub heavy_armor: u8,
    pub light_armor: u8,
    pub pickpocket: u8,
    pub lockpicking: u8,
    pub sneak: u8,
    pub alchemy: u8,
    pub speech: u8,
    pub alteration: u8,
    pub conjuration: u8,
    pub destruction: u8,
    pub illusion: u8,
    pub restoration: u8,
    pub enchanting: u8,
}

const _: () = assert!(core::mem::size_of::<TESClassSkillWeights>() == 0x12);

#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct TESClassAttributeWeights {
    pub health: u8,
    pub magicka: u8,
    pub stamina: u8,
}

const _: () = assert!(core::mem::size_of::<TESClassAttributeWeights>() == 0x3);

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct TESClassData {
    pub unk00: u32,
    pub teaches: EnumSet<TESClassSkill, u8>,
    pub maximum_training_level: u8,
    pub skill_weights: TESClassSkillWeights,
    pub bleedout_default: f32,
    pub voice_points: u32,
    pub attribute_weights: TESClassAttributeWeights,
    pub pad23: u8,
}

const _: () = assert!(core::mem::size_of::<TESClassData>() == 0x24);
const _: () = assert!(core::mem::offset_of!(TESClassData, unk00) == 0x00);
const _: () = assert!(core::mem::offset_of!(TESClassData, teaches) == 0x04);
const _: () = assert!(core::mem::offset_of!(TESClassData, maximum_training_level) == 0x05);
const _: () = assert!(core::mem::offset_of!(TESClassData, skill_weights) == 0x06);
const _: () = assert!(core::mem::offset_of!(TESClassData, bleedout_default) == 0x18);
const _: () = assert!(core::mem::offset_of!(TESClassData, voice_points) == 0x1C);
const _: () = assert!(core::mem::offset_of!(TESClassData, attribute_weights) == 0x20);
const _: () = assert!(core::mem::offset_of!(TESClassData, pad23) == 0x23);

bitflags! {
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct TESClassChangeFlags: u32 {
        const TAG_SKILLS = 1 << 1;
    }
}

bitflags! {
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct TESClassRecordFlags: u32 {
        const DELETED = 1 << 5;
        const IGNORED = 1 << 12;
    }
}

#[repr(C)]
pub struct TESClass {
    pub base: TESForm,               // 00
    pub full_name: TESFullName,      // 20
    pub description: TESDescription, // 30
    pub texture: TESTexture,         // 40
    pub data: TESClassData,          // 50
    pub pad74: u32,                  // 74
}

const _: () = assert!(core::mem::size_of::<TESClass>() == 0x78);
const _: () = assert!(core::mem::offset_of!(TESClass, full_name) == 0x20);
const _: () = assert!(core::mem::offset_of!(TESClass, description) == 0x30);
const _: () = assert!(core::mem::offset_of!(TESClass, texture) == 0x40);
const _: () = assert!(core::mem::offset_of!(TESClass, data) == 0x50);
const _: () = assert!(core::mem::offset_of!(TESClass, pad74) == 0x74);

impl RttiType for TESClass {
    const RTTI: VariantID = RTTI_TESClass;
}

impl FormCastable for TESClass {
    const TARGET_FORM_TYPE: FormType = FormType::Class;
}

inherit!(TESClass : TESForm);
inherit!(TESClass => TESFullName, full_name);
inherit!(TESClass => TESDescription, description);
inherit!(TESClass => TESTexture, texture);

impl TESClass {
    pub const RTTI: VariantID = RTTI_TESClass;
    pub const VTABLE: &'static [VariantID] = &VTABLE_TESClass;
    pub const FORMTYPE: FormType = FormType::Class;

    // override (TESForm)
    virtual_method! {
        pub const VFUNC_DTOR: usize = 0x00;
        pub fn dtor()
    }

    virtual_method! {
        pub const VFUNC_INITIALIZE_DATA: usize = 0x04;
        pub fn initialize_data()
    }

    virtual_method! {
        pub const VFUNC_LOAD: usize = 0x06;
        pub fn load(mod_file: *mut TESFile) -> bool
    }
}
