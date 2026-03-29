use core_util::inherit;

use crate::offsets::offsets_rtti::RTTI_BSLightingShaderMaterialLODLandscape;
use crate::offsets::offsets_vtable::VTABLE_BSLightingShaderMaterialLODLandscape;
use crate::re::{
    BSLightingShaderMaterialBase, BSLightingShaderMaterialKind, BSShaderMaterialFeature, NiPointer,
    NiSourceTexture,
};
use crate::relocation::{RelocationID, RttiType, VariantID};

/// C++ `RE::BSLightingShaderMaterialLODLandscape`
#[repr(C)]
pub struct BSLightingShaderMaterialLODLandscape {
    pub base: BSLightingShaderMaterialBase,                  // 00
    pub parent_diffuse_texture: NiPointer<NiSourceTexture>,  // A0
    pub parent_normal_texture: NiPointer<NiSourceTexture>,   // A8
    pub landscape_noise_texture: NiPointer<NiSourceTexture>, // B0
    pub terrain_tex_offset_x: f32,                           // B8
    pub terrain_tex_offset_y: f32,                           // BC
    pub terrain_tex_fade: f32,                               // C0
}

const _: () = assert!(core::mem::size_of::<BSLightingShaderMaterialLODLandscape>() == 0xC8);
const _: () = assert!(
    core::mem::offset_of!(BSLightingShaderMaterialLODLandscape, parent_diffuse_texture) == 0xA0
);
const _: () = assert!(
    core::mem::offset_of!(BSLightingShaderMaterialLODLandscape, terrain_tex_offset_x) == 0xB8
);

impl RttiType for BSLightingShaderMaterialLODLandscape {
    const RTTI: VariantID = RTTI_BSLightingShaderMaterialLODLandscape;
}

inherit!(BSLightingShaderMaterialLODLandscape : BSLightingShaderMaterialBase, base);

impl BSLightingShaderMaterialLODLandscape {
    pub const RTTI: VariantID = RTTI_BSLightingShaderMaterialLODLandscape;
    pub const VTABLE: &'static [VariantID] = &VTABLE_BSLightingShaderMaterialLODLandscape;
    pub const FEATURE: BSShaderMaterialFeature = BSShaderMaterialFeature::LodLandNoise;

    crate::relocation_func! {
        pub(crate) fn ctor(&mut self) -> *mut Self => RelocationID::new(100110, 106817)
    }
}

impl BSLightingShaderMaterialKind for BSLightingShaderMaterialLODLandscape {
    const FEATURE: BSShaderMaterialFeature = Self::FEATURE;
}
