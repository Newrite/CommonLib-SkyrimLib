use core_util::inherit;

use crate::offsets::offsets_rtti::RTTI_BSLightingShaderMaterialEye;
use crate::offsets::offsets_vtable::VTABLE_BSLightingShaderMaterialEye;
use crate::re::{
    BSLightingShaderMaterialBase, BSLightingShaderMaterialKind, BSShaderMaterialFeature, NiPoint3,
    NiPointer, NiSourceTexture,
};
use crate::relocation::{RelocationID, RttiType, VariantID};

/// C++ `RE::BSLightingShaderMaterialEye`
#[repr(C)]
pub struct BSLightingShaderMaterialEye {
    pub base: BSLightingShaderMaterialBase,           // 00
    pub env_texture: NiPointer<NiSourceTexture>,      // A0
    pub env_mask_texture: NiPointer<NiSourceTexture>, // A8
    pub env_map_scale: f32,                           // B0
    pub eye_center: [NiPoint3; 2],                    // B4
    pub pad_cc: u32,                                  // CC
}

const _: () = assert!(core::mem::size_of::<BSLightingShaderMaterialEye>() == 0xD0);
const _: () = assert!(core::mem::offset_of!(BSLightingShaderMaterialEye, env_texture) == 0xA0);
const _: () = assert!(core::mem::offset_of!(BSLightingShaderMaterialEye, eye_center) == 0xB4);

impl RttiType for BSLightingShaderMaterialEye {
    const RTTI: VariantID = RTTI_BSLightingShaderMaterialEye;
}

inherit!(BSLightingShaderMaterialEye : BSLightingShaderMaterialBase, base);

impl BSLightingShaderMaterialEye {
    pub const RTTI: VariantID = RTTI_BSLightingShaderMaterialEye;
    pub const VTABLE: &'static [VariantID] = &VTABLE_BSLightingShaderMaterialEye;
    pub const FEATURE: BSShaderMaterialFeature = BSShaderMaterialFeature::Eye;

    crate::relocation_func! {
        pub(crate) fn ctor(&mut self) -> *mut Self => RelocationID::new(100033, 106740)
    }
}

impl BSLightingShaderMaterialKind for BSLightingShaderMaterialEye {
    const FEATURE: BSShaderMaterialFeature = Self::FEATURE;
}
