use bitflags::bitflags;
use core_util::{EnumSet, inherit};

use crate::offsets::offsets_rtti::RTTI_BGSReferenceEffect;
use crate::offsets::offsets_vtable::VTABLE_BGSReferenceEffect;
use crate::re::BGSArtObject;
use crate::re::FormCastable;
use crate::re::FormType;
use crate::re::TESEffectShader;
use crate::re::TESForm;
use crate::relocation::{RttiType, VariantID};

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BGSReferenceEffectFlag {
    None = 0,
    FaceTarget = 1 << 0,
    AttachToCamera = 1 << 1,
    InheritRotation = 1 << 2,
}

core_util::impl_enumset_type!(BGSReferenceEffectFlag => u32);

bitflags! {
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct ReferenceEffectRecordFlags: u32 {
        const NONE = 0;
        const DELETED = 1 << 5;
        const IGNORED = 1 << 12;
    }
}

unsafe impl bytemuck::Zeroable for ReferenceEffectRecordFlags {}

#[repr(C)]
pub struct BGSReferenceEffectData {
    pub art_object: *mut BGSArtObject,               // 0x00
    pub effect_shader: *mut TESEffectShader,         // 0x08
    pub flags: EnumSet<BGSReferenceEffectFlag, u32>, // 0x10
    pub pad14: u32,                                  // 0x14
}

const _: () = assert!(core::mem::size_of::<BGSReferenceEffectData>() == 0x18);
const _: () = assert!(core::mem::offset_of!(BGSReferenceEffectData, art_object) == 0x00);
const _: () = assert!(core::mem::offset_of!(BGSReferenceEffectData, effect_shader) == 0x08);
const _: () = assert!(core::mem::offset_of!(BGSReferenceEffectData, flags) == 0x10);
const _: () = assert!(core::mem::offset_of!(BGSReferenceEffectData, pad14) == 0x14);

#[repr(C)]
pub struct BGSReferenceEffect {
    pub base: TESForm,                // 0x00
    pub data: BGSReferenceEffectData, // 0x20
}

const _: () = assert!(core::mem::size_of::<BGSReferenceEffect>() == 0x38);
const _: () = assert!(core::mem::offset_of!(BGSReferenceEffect, data) == 0x20);

impl RttiType for BGSReferenceEffect {
    const RTTI: VariantID = RTTI_BGSReferenceEffect;
}

impl FormCastable for BGSReferenceEffect {
    const TARGET_FORM_TYPE: FormType = FormType::ReferenceEffect;
}

inherit!(BGSReferenceEffect : TESForm);

impl BGSReferenceEffect {
    pub const RTTI: VariantID = RTTI_BGSReferenceEffect;
    pub const VTABLE: &'static [VariantID] = &VTABLE_BGSReferenceEffect;
    pub const FORMTYPE: FormType = FormType::ReferenceEffect;

    // override (TESForm)
    // void InitializeData() override;      // 04
    // void ClearData() override;           // 05
    // bool Load(TESFile* a_mod) override;  // 06
    // void InitItemImpl() override;        // 13
}
