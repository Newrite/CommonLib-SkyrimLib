use core_util::inherit;

use crate::offsets::offsets_rtti::RTTI_BSLightingShaderMaterialParallaxOcc;
use crate::offsets::offsets_vtable::VTABLE_BSLightingShaderMaterialParallaxOcc;
use crate::re::{
    BSLightingShaderMaterialBase, BSLightingShaderMaterialKind, BSShaderMaterialFeature, NiPointer,
    NiSourceTexture,
};
use crate::relocation::{RelocationID, RttiType, VariantID};

/// C++ `RE::BSLightingShaderMaterialParallaxOcc`
#[repr(C)]
pub struct BSLightingShaderMaterialParallaxOcc {
    pub base: BSLightingShaderMaterialBase,         // 00
    pub height_texture: NiPointer<NiSourceTexture>, // A0
    pub parallax_occ_max_passes: f32,               // A8
    pub parallax_occ_scale: f32,                    // AC
}

const _: () = assert!(core::mem::size_of::<BSLightingShaderMaterialParallaxOcc>() == 0xB0);
const _: () =
    assert!(core::mem::offset_of!(BSLightingShaderMaterialParallaxOcc, height_texture) == 0xA0);

impl RttiType for BSLightingShaderMaterialParallaxOcc {
    const RTTI: VariantID = RTTI_BSLightingShaderMaterialParallaxOcc;
}

inherit!(BSLightingShaderMaterialParallaxOcc : BSLightingShaderMaterialBase, base);

impl BSLightingShaderMaterialParallaxOcc {
    pub const RTTI: VariantID = RTTI_BSLightingShaderMaterialParallaxOcc;
    pub const VTABLE: &'static [VariantID] = &VTABLE_BSLightingShaderMaterialParallaxOcc;
    pub const FEATURE: BSShaderMaterialFeature = BSShaderMaterialFeature::ParallaxOcc;

    crate::relocation_func! {
        pub(crate) fn ctor(&mut self) -> *mut Self => RelocationID::new(100065, 106772)
    }
}

impl BSLightingShaderMaterialKind for BSLightingShaderMaterialParallaxOcc {
    const FEATURE: BSShaderMaterialFeature = Self::FEATURE;
}
