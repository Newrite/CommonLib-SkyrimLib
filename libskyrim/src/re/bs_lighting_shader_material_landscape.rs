use core_util::inherit;

use crate::offsets::offsets_rtti::RTTI_BSLightingShaderMaterialLandscape;
use crate::offsets::offsets_vtable::VTABLE_BSLightingShaderMaterialLandscape;
use crate::re::{
    BSLightingShaderMaterialBase, BSLightingShaderMaterialKind, BSShaderMaterialFeature, NiColorA,
    NiPointer, NiSourceTexture,
};
use crate::relocation::{RelocationID, RttiType, VariantID};

/// C++ `RE::BSLightingShaderMaterialLandscape`
#[repr(C)]
pub struct BSLightingShaderMaterialLandscape {
    pub base: BSLightingShaderMaterialBase, // 00
    pub num_landscape_textures: u32,        // A0
    pub pad_a4: u32,                        // A4
    pub landscape_diffuse_texture: [NiPointer<NiSourceTexture>; 5], // A8
    pub landscape_normal_texture: [NiPointer<NiSourceTexture>; 5], // D0
    pub terrain_overlay_texture: NiPointer<NiSourceTexture>, // F8
    pub terrain_noise_texture: NiPointer<NiSourceTexture>, // 100
    pub land_blend_params: NiColorA,        // 108
    pub texture_is_snow: [f32; 6],          // 118
    pub texture_is_spec_power: [f32; 6],    // 130
    pub terrain_tex_offset_x: f32,          // 148
    pub terrain_tex_offset_y: f32,          // 14C
    pub terrain_tex_fade: f32,              // 150
    pub pad154: u32,                        // 154
}

const _: () = assert!(core::mem::size_of::<BSLightingShaderMaterialLandscape>() == 0x158);
const _: () = assert!(
    core::mem::offset_of!(BSLightingShaderMaterialLandscape, num_landscape_textures) == 0xA0
);
const _: () = assert!(
    core::mem::offset_of!(BSLightingShaderMaterialLandscape, landscape_diffuse_texture) == 0xA8
);
const _: () = assert!(
    core::mem::offset_of!(BSLightingShaderMaterialLandscape, terrain_tex_offset_x) == 0x148
);

impl RttiType for BSLightingShaderMaterialLandscape {
    const RTTI: VariantID = RTTI_BSLightingShaderMaterialLandscape;
}

inherit!(BSLightingShaderMaterialLandscape : BSLightingShaderMaterialBase, base);

impl BSLightingShaderMaterialLandscape {
    pub const RTTI: VariantID = RTTI_BSLightingShaderMaterialLandscape;
    pub const VTABLE: &'static [VariantID] = &VTABLE_BSLightingShaderMaterialLandscape;
    pub const FEATURE: BSShaderMaterialFeature = BSShaderMaterialFeature::MultiTexLandLodBlend;

    crate::relocation_func! {
        pub(crate) fn ctor(&mut self) -> *mut Self => RelocationID::new(100102, 106809)
    }
}

impl BSLightingShaderMaterialKind for BSLightingShaderMaterialLandscape {
    const FEATURE: BSShaderMaterialFeature = Self::FEATURE;
}
