use crate::core_util::inherit;
use crate::offsets::offsets_rtti::RTTI_BGSExplosion;
use crate::offsets::offsets_vtable::VTABLE_BGSExplosion;
use crate::re::BGSImpactDataSet;
use crate::re::BGSPreloadable;
use crate::re::BGSProjectile;
use crate::re::BGSSoundDescriptorForm;
use crate::re::FormCastable;
use crate::re::FormType;
use crate::re::SOUND_LEVEL;
use crate::re::TESBoundObject;
use crate::re::TESEnchantableForm;
use crate::re::TESFullName;
use crate::re::TESImageSpaceModifiableForm;
use crate::re::TESModel;
use crate::re::TESObjectLIGH;
use crate::re::TESObjectREFR;
use crate::relocation::{RttiType, VariantID};
use core_util::EnumSet;

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BGSExplosionFlag {
    None = 0,
    AlwaysUsesWorldOrientation = 1 << 1,
    KnockDownAlways = 1 << 2,
    KnockDownByFormula = 1 << 3,
    IgnoreLOSCheck = 1 << 4,
    PushExplosionSourceRefOnly = 1 << 5,
    IgnoreImageSpaceSwap = 1 << 6,
    Chain = 1 << 7,
    NoControllerVibration = 1 << 8,
}

core_util::impl_enumset_type!(BGSExplosionFlag => u32);

bitflags::bitflags! {
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct ExplosionRecordFlags: u32 {
        const NONE = 0;
        const DELETED = 1 << 5;
        const IGNORED = 1 << 12;
    }
}

#[repr(C)]
pub struct BGSExplosionData {
    pub light: *mut TESObjectLIGH,                // 0x00
    pub sound1: *mut BGSSoundDescriptorForm,      // 0x08
    pub sound2: *mut BGSSoundDescriptorForm,      // 0x10
    pub impact_data_set: *mut BGSImpactDataSet,   // 0x18
    pub impact_placed_object: *mut TESObjectREFR, // 0x20
    pub spawn_projectile: *mut BGSProjectile,     // 0x28
    pub force: f32,                               // 0x30
    pub damage: f32,                              // 0x34
    pub radius: f32,                              // 0x38
    pub image_space_radius: f32,                  // 0x3C
    pub vertical_offset_mult: f32,                // 0x40
    pub flags: EnumSet<BGSExplosionFlag, u32>,    // 0x44
    pub sound_level: EnumSet<SOUND_LEVEL, u32>,   // 0x48
    pub pad4c: u32,                               // 0x4C
}

const _: () = assert!(core::mem::size_of::<BGSExplosionData>() == 0x50);

#[repr(C)]
pub struct BGSExplosion {
    pub base: TESBoundObject,                                     // 0x00
    pub full_name: TESFullName,                                   // 0x30
    pub model: TESModel,                                          // 0x40
    pub enchantable_form: TESEnchantableForm,                     // 0x68
    pub preloadable: BGSPreloadable,                              // 0x80
    pub image_space_modifiable_form: TESImageSpaceModifiableForm, // 0x88
    pub data: BGSExplosionData,                                   // 0x98
}

const _: () = assert!(core::mem::size_of::<BGSExplosion>() == 0xE8);
const _: () = assert!(core::mem::offset_of!(BGSExplosion, full_name) == 0x30);
const _: () = assert!(core::mem::offset_of!(BGSExplosion, model) == 0x40);
const _: () = assert!(core::mem::offset_of!(BGSExplosion, enchantable_form) == 0x68);
const _: () = assert!(core::mem::offset_of!(BGSExplosion, preloadable) == 0x80);
const _: () = assert!(core::mem::offset_of!(BGSExplosion, image_space_modifiable_form) == 0x88);

impl RttiType for BGSExplosion {
    const RTTI: VariantID = RTTI_BGSExplosion;
}

impl FormCastable for BGSExplosion {
    const TARGET_FORM_TYPE: FormType = FormType::Explosion;
}

inherit!(BGSExplosion : TESBoundObject);
inherit!(BGSExplosion => TESFullName, full_name);
inherit!(BGSExplosion => TESModel, model);
inherit!(BGSExplosion => TESEnchantableForm, enchantable_form);
inherit!(BGSExplosion => BGSPreloadable, preloadable);
inherit!(BGSExplosion => TESImageSpaceModifiableForm, image_space_modifiable_form);

impl BGSExplosion {
    pub const RTTI: VariantID = RTTI_BGSExplosion;
    pub const VTABLE: &'static [VariantID] = &VTABLE_BGSExplosion;
    pub const FORMTYPE: FormType = FormType::Explosion;

    // override (TESBoundObject)
    // void        InitializeData() override;                            // 04
    // bool        Load(TESFile* a_mod) override;                        // 06
    // void        InitItemImpl() override;                              // 13
    // NiAVObject* Clone3D(TESObjectREFR* a_ref, bool a_arg3) override;  // 40
    // void        UnClone3D(TESObjectREFR* a_ref) override;             // 41
}
