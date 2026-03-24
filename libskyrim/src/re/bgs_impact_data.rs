use bitflags::bitflags;
use core_util::{EnumSet, inherit};

use crate::offsets::offsets_rtti::RTTI_BGSImpactData;
use crate::offsets::offsets_vtable::VTABLE_BGSImpactData;
use crate::re::bgs_hazard::BGSHazard;
use crate::re::bgs_sound_descriptor_form::BGSSoundDescriptorForm;
use crate::re::bgs_texture_set::BGSTextureSet;
use crate::re::decal_data::DecalData;
use crate::re::form_traits::FormCastable;
use crate::re::form_type::FormType;
use crate::re::impact_results::ImpactResult;
use crate::re::sound_levels::SOUND_LEVEL;
use crate::re::tes_form::TESForm;
use crate::re::tes_model::TESModel;
use crate::relocation::{RttiType, VariantID};

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BGSImpactDataOrientation {
    SurfaceNormal = 0,
    ProjVector = 1,
    ProjReflect = 2,
}

core_util::impl_enumset_type!(BGSImpactDataOrientation => u32);

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BGSImpactDataFlag {
    None = 0,
    NoDecalData = 1 << 0,
}

core_util::impl_enumset_type!(BGSImpactDataFlag => u8);

bitflags! {
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct BGSImpactDataRecordFlags: u32 {
        const NONE = 0;
        const DELETED = 1 << 5;
        const IGNORED = 1 << 12;
    }
}

#[repr(C)]
pub struct BGSImpactDataData {
    pub effect_duration: f32,                           // 0x00
    pub orient: EnumSet<BGSImpactDataOrientation, u32>, // 0x04
    pub angle_threshold: f32,                           // 0x08
    pub placement_radius: f32,                          // 0x0C
    pub sound_level: SOUND_LEVEL,                       // 0x10
    pub flags: EnumSet<BGSImpactDataFlag, u8>,          // 0x14
    pub result_override: EnumSet<ImpactResult, u8>,     // 0x15
    pub unk16: u16,                                     // 0x16
}

const _: () = assert!(core::mem::size_of::<BGSImpactDataData>() == 0x18);
const _: () = assert!(core::mem::offset_of!(BGSImpactDataData, effect_duration) == 0x00);
const _: () = assert!(core::mem::offset_of!(BGSImpactDataData, orient) == 0x04);
const _: () = assert!(core::mem::offset_of!(BGSImpactDataData, angle_threshold) == 0x08);
const _: () = assert!(core::mem::offset_of!(BGSImpactDataData, placement_radius) == 0x0C);
const _: () = assert!(core::mem::offset_of!(BGSImpactDataData, sound_level) == 0x10);
const _: () = assert!(core::mem::offset_of!(BGSImpactDataData, flags) == 0x14);
const _: () = assert!(core::mem::offset_of!(BGSImpactDataData, result_override) == 0x15);
const _: () = assert!(core::mem::offset_of!(BGSImpactDataData, unk16) == 0x16);

/// C++ `RE::BGSImpactData`
#[repr(C)]
pub struct BGSImpactData {
    pub base: TESForm,                          // 0x00
    pub model: TESModel,                        // 0x20
    pub data: BGSImpactDataData,                // 0x48 - DATA
    pub decal_texture_set: *mut BGSTextureSet,  // 0x60 - DNAM
    pub decal_texture_set2: *mut BGSTextureSet, // 0x68 - ENAM
    pub sound1: *mut BGSSoundDescriptorForm,    // 0x70 - SNAM
    pub sound2: *mut BGSSoundDescriptorForm,    // 0x78 - NAM1
    pub hazard: *mut BGSHazard,                 // 0x80 - NAM2
    pub decal_data: DecalData,                  // 0x88 - DODT
    pub padac: u32,                             // 0xAC
}

const _: () = assert!(core::mem::size_of::<BGSImpactData>() == 0xB0);
const _: () = assert!(core::mem::offset_of!(BGSImpactData, base) == 0x00);
const _: () = assert!(core::mem::offset_of!(BGSImpactData, model) == 0x20);
const _: () = assert!(core::mem::offset_of!(BGSImpactData, data) == 0x48);
const _: () = assert!(core::mem::offset_of!(BGSImpactData, decal_texture_set) == 0x60);
const _: () = assert!(core::mem::offset_of!(BGSImpactData, decal_texture_set2) == 0x68);
const _: () = assert!(core::mem::offset_of!(BGSImpactData, sound1) == 0x70);
const _: () = assert!(core::mem::offset_of!(BGSImpactData, sound2) == 0x78);
const _: () = assert!(core::mem::offset_of!(BGSImpactData, hazard) == 0x80);
const _: () = assert!(core::mem::offset_of!(BGSImpactData, decal_data) == 0x88);
const _: () = assert!(core::mem::offset_of!(BGSImpactData, padac) == 0xAC);

impl RttiType for BGSImpactData {
    const RTTI: VariantID = RTTI_BGSImpactData;
}

impl FormCastable for BGSImpactData {
    const TARGET_FORM_TYPE: FormType = FormType::Impact;
}

inherit!(BGSImpactData : TESForm);
inherit!(BGSImpactData => TESModel, model);

impl BGSImpactData {
    pub const RTTI: VariantID = RTTI_BGSImpactData;
    pub const VTABLE: &'static [VariantID] = &VTABLE_BGSImpactData;
    pub const FORMTYPE: FormType = FormType::Impact;

    // override (TESForm)
    // void InitializeData() override;      // 04
    // bool Load(TESFile* a_mod) override;  // 06
    // void InitItemImpl() override;        // 13
}
