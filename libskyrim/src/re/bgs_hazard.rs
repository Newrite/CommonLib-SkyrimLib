use bitflags::bitflags;
use core_util::{EnumSet, inherit};

use crate::offsets::offsets_rtti::RTTI_BGSHazard;
use crate::offsets::offsets_vtable::VTABLE_BGSHazard;
use crate::re::bgs_impact_data_set::BGSImpactDataSet;
use crate::re::bgs_preloadable::BGSPreloadable;
use crate::re::bgs_sound_descriptor_form::BGSSoundDescriptorForm;
use crate::re::form_traits::FormCastable;
use crate::re::form_type::FormType;
use crate::re::spell_item::SpellItem;
use crate::re::tes_bound_object::TESBoundObject;
use crate::re::tes_full_name::TESFullName;
use crate::re::tes_image_space_modifiable_form::TESImageSpaceModifiableForm;
use crate::re::tes_model::TESModel;
use crate::re::tes_object_ligh::TESObjectLIGH;
use crate::relocation::{RttiType, VariantID};

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BGSHazardFlag {
    None = 0,
    PCOnly = 1 << 0,
    InheritDuration = 1 << 1,
    AlignToNormal = 1 << 2,
    InheritRadius = 1 << 3,
    DropToGround = 1 << 4,
}

core_util::impl_enumset_type!(BGSHazardFlag => u32);

bitflags! {
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct BGSHazardRecordFlags: u32 {
        const NONE = 0;
        const DELETED = 1 << 5;
        const IGNORED = 1 << 12;
    }
}

#[repr(C)]
pub struct BGSHazardData {
    pub limit: u32,                             // 0x00
    pub radius: f32,                            // 0x04
    pub lifetime: f32,                          // 0x08
    pub image_space_radius: f32,                // 0x0C
    pub target_interval: f32,                   // 0x10
    pub flags: EnumSet<BGSHazardFlag, u32>,     // 0x14
    pub spell: *mut SpellItem,                  // 0x18
    pub light: *mut TESObjectLIGH,              // 0x20
    pub impact_data_set: *mut BGSImpactDataSet, // 0x28
    pub sound: *mut BGSSoundDescriptorForm,     // 0x30
}

const _: () = assert!(core::mem::size_of::<BGSHazardData>() == 0x38);
const _: () = assert!(core::mem::offset_of!(BGSHazardData, limit) == 0x00);
const _: () = assert!(core::mem::offset_of!(BGSHazardData, radius) == 0x04);
const _: () = assert!(core::mem::offset_of!(BGSHazardData, lifetime) == 0x08);
const _: () = assert!(core::mem::offset_of!(BGSHazardData, image_space_radius) == 0x0C);
const _: () = assert!(core::mem::offset_of!(BGSHazardData, target_interval) == 0x10);
const _: () = assert!(core::mem::offset_of!(BGSHazardData, flags) == 0x14);
const _: () = assert!(core::mem::offset_of!(BGSHazardData, spell) == 0x18);
const _: () = assert!(core::mem::offset_of!(BGSHazardData, light) == 0x20);
const _: () = assert!(core::mem::offset_of!(BGSHazardData, impact_data_set) == 0x28);
const _: () = assert!(core::mem::offset_of!(BGSHazardData, sound) == 0x30);

/// C++ `RE::BGSHazard`
#[repr(C)]
pub struct BGSHazard {
    pub base: TESBoundObject,                                     // 0x00
    pub full_name: TESFullName,                                   // 0x30
    pub model: TESModel,                                          // 0x40
    pub preloadable: BGSPreloadable,                              // 0x68
    pub image_space_modifiable_form: TESImageSpaceModifiableForm, // 0x70
    pub data: BGSHazardData,                                      // 0x80 - DATA
}

const _: () = assert!(core::mem::size_of::<BGSHazard>() == 0xB8);
const _: () = assert!(core::mem::offset_of!(BGSHazard, base) == 0x00);
const _: () = assert!(core::mem::offset_of!(BGSHazard, full_name) == 0x30);
const _: () = assert!(core::mem::offset_of!(BGSHazard, model) == 0x40);
const _: () = assert!(core::mem::offset_of!(BGSHazard, preloadable) == 0x68);
const _: () = assert!(core::mem::offset_of!(BGSHazard, image_space_modifiable_form) == 0x70);
const _: () = assert!(core::mem::offset_of!(BGSHazard, data) == 0x80);

impl RttiType for BGSHazard {
    const RTTI: VariantID = RTTI_BGSHazard;
}

impl FormCastable for BGSHazard {
    const TARGET_FORM_TYPE: FormType = FormType::Hazard;
}

inherit!(BGSHazard : TESBoundObject);
inherit!(BGSHazard => TESFullName, full_name);
inherit!(BGSHazard => TESModel, model);
inherit!(BGSHazard => BGSPreloadable, preloadable);
inherit!(BGSHazard => TESImageSpaceModifiableForm, image_space_modifiable_form);

impl BGSHazard {
    pub const RTTI: VariantID = RTTI_BGSHazard;
    pub const VTABLE: &'static [VariantID] = &VTABLE_BGSHazard;
    pub const FORMTYPE: FormType = FormType::Hazard;

    // override (TESBoundObject)
    // void InitializeData() override;      // 04
    // bool Load(TESFile* a_mod) override;  // 06
    // void InitItemImpl() override;        // 13
}
