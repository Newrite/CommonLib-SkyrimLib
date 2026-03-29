use core_util::inherit;

use crate::offsets::offsets_rtti::RTTI_TESLandTexture;
use crate::offsets::offsets_vtable::VTABLE_TESLandTexture;
use crate::re::{
    BGSMaterialType, BGSTextureSet, BSSimpleList, FormCastable, FormType, TESForm, TESGrass,
};
use crate::relocation::{RttiType, VariantID};

bitflags::bitflags! {
    /// C++ `RE::TESLandTexture::RecordFlags`
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct TESLandTextureRecordFlags: u32 {
        const DELETED = 1 << 5;
        const IGNORED = 1 << 12;
    }
}

/// C++ `RE::TEXTURE_HAVOK_DATA` (`HNAM`)
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TEXTURE_HAVOK_DATA {
    pub friction: i32,    // 00
    pub restitution: i32, // 04
}

const _: () = assert!(core::mem::size_of::<TEXTURE_HAVOK_DATA>() == 0x8);
const _: () = assert!(core::mem::offset_of!(TEXTURE_HAVOK_DATA, friction) == 0x00);
const _: () = assert!(core::mem::offset_of!(TEXTURE_HAVOK_DATA, restitution) == 0x04);

/// C++ `RE::TESLandTexture`
#[repr(C)]
pub struct TESLandTexture {
    pub base: TESForm,                                   // 00
    pub texture_set: *mut BGSTextureSet,                 // 20 - TNAM
    pub havok_data: TEXTURE_HAVOK_DATA,                  // 28 - HNAM
    pub material_type: *mut BGSMaterialType,             // 30 - MNAM
    pub specular_exponent: i8,                           // 38 - SNAM
    pub pad39: u8,                                       // 39
    pub pad3a: u16,                                      // 3A
    pub shader_texture_index: i32,                       // 3C - INAM
    pub texture_grass_list: BSSimpleList<*mut TESGrass>, // 40 - GNAM
}

const _: () = assert!(core::mem::size_of::<TESLandTexture>() == 0x50);
const _: () = assert!(core::mem::offset_of!(TESLandTexture, texture_set) == 0x20);
const _: () = assert!(core::mem::offset_of!(TESLandTexture, havok_data) == 0x28);
const _: () = assert!(core::mem::offset_of!(TESLandTexture, material_type) == 0x30);
const _: () = assert!(core::mem::offset_of!(TESLandTexture, specular_exponent) == 0x38);
const _: () = assert!(core::mem::offset_of!(TESLandTexture, shader_texture_index) == 0x3C);
const _: () = assert!(core::mem::offset_of!(TESLandTexture, texture_grass_list) == 0x40);

impl RttiType for TESLandTexture {
    const RTTI: VariantID = RTTI_TESLandTexture;
}

impl FormCastable for TESLandTexture {
    const TARGET_FORM_TYPE: FormType = FormType::LandTexture;
}

inherit!(TESLandTexture : TESForm);

impl TESLandTexture {
    pub const RTTI: VariantID = RTTI_TESLandTexture;
    pub const VTABLE: &'static [VariantID] = &VTABLE_TESLandTexture;
    pub const FORMTYPE: FormType = FormType::LandTexture;

    // override (TESForm)
    // void InitializeData() override;  // 04
    // void ClearData() override;       // 05
    // bool Load(TESFile* a_mod) override;  // 06
    // void InitItemImpl() override;    // 13
}
