use core_util::inherit;

use crate::offsets::offsets_rtti::RTTI_BSLightingShaderMaterialMultiLayerParallax;
use crate::offsets::offsets_vtable::VTABLE_BSLightingShaderMaterialMultiLayerParallax;
use crate::re::{
    BSLightingShaderMaterialBase, BSLightingShaderMaterialKind, BSShaderMaterialFeature, NiPointer,
    NiSourceTexture,
};
use crate::relocation::{RelocationID, RttiType, VariantID};

/// C++ `RE::BSLightingShaderMaterialMultiLayerParallax`
#[repr(C)]
pub struct BSLightingShaderMaterialMultiLayerParallax {
    pub base: BSLightingShaderMaterialBase,           // 00
    pub layer_texture: NiPointer<NiSourceTexture>,    // A0
    pub env_texture: NiPointer<NiSourceTexture>,      // A8
    pub env_mask_texture: NiPointer<NiSourceTexture>, // B0
    pub parallax_layer_thickness: f32,                // B8
    pub parallax_refraction_scale: f32,               // BC
    pub parallax_inner_layer_u_scale: f32,            // C0
    pub parallax_inner_layer_v_scale: f32,            // C4
    pub envmap_scale: f32,                            // C8
    pub pad_cc: u32,                                  // CC
}

const _: () = assert!(core::mem::size_of::<BSLightingShaderMaterialMultiLayerParallax>() == 0xD0);
const _: () = assert!(
    core::mem::offset_of!(BSLightingShaderMaterialMultiLayerParallax, layer_texture) == 0xA0
);
const _: () = assert!(
    core::mem::offset_of!(
        BSLightingShaderMaterialMultiLayerParallax,
        parallax_layer_thickness
    ) == 0xB8
);

impl RttiType for BSLightingShaderMaterialMultiLayerParallax {
    const RTTI: VariantID = RTTI_BSLightingShaderMaterialMultiLayerParallax;
}

inherit!(BSLightingShaderMaterialMultiLayerParallax : BSLightingShaderMaterialBase, base);

impl BSLightingShaderMaterialMultiLayerParallax {
    pub const RTTI: VariantID = RTTI_BSLightingShaderMaterialMultiLayerParallax;
    pub const VTABLE: &'static [VariantID] = &VTABLE_BSLightingShaderMaterialMultiLayerParallax;
    pub const FEATURE: BSShaderMaterialFeature = BSShaderMaterialFeature::MultilayerParallax;

    crate::relocation_func! {
        pub(crate) fn ctor(&mut self) -> *mut Self => RelocationID::new(100125, 106832)
    }
}

impl BSLightingShaderMaterialKind for BSLightingShaderMaterialMultiLayerParallax {
    const FEATURE: BSShaderMaterialFeature = Self::FEATURE;
}
