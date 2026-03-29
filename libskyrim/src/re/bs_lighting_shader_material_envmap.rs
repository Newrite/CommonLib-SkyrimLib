use core_util::inherit;

use crate::offsets::offsets_rtti::RTTI_BSLightingShaderMaterialEnvmap;
use crate::offsets::offsets_vtable::VTABLE_BSLightingShaderMaterialEnvmap;
use crate::re::{
    BSLightingShaderMaterialBase, BSLightingShaderMaterialKind, BSShaderMaterialFeature, NiPointer,
    NiSourceTexture,
};
use crate::relocation::{RelocationID, RttiType, VariantID};

/// C++ `RE::BSLightingShaderMaterialEnvmap`
#[repr(C)]
pub struct BSLightingShaderMaterialEnvmap {
    pub base: BSLightingShaderMaterialBase,           // 00
    pub env_texture: NiPointer<NiSourceTexture>,      // A0
    pub env_mask_texture: NiPointer<NiSourceTexture>, // A8
    pub env_map_scale: f32,                           // B0
    pub pad_b4: u32,                                  // B4
}

const _: () = assert!(core::mem::size_of::<BSLightingShaderMaterialEnvmap>() == 0xB8);
const _: () = assert!(core::mem::offset_of!(BSLightingShaderMaterialEnvmap, env_texture) == 0xA0);

impl RttiType for BSLightingShaderMaterialEnvmap {
    const RTTI: VariantID = RTTI_BSLightingShaderMaterialEnvmap;
}

inherit!(BSLightingShaderMaterialEnvmap : BSLightingShaderMaterialBase, base);

impl BSLightingShaderMaterialEnvmap {
    pub const RTTI: VariantID = RTTI_BSLightingShaderMaterialEnvmap;
    pub const VTABLE: &'static [VariantID] = &VTABLE_BSLightingShaderMaterialEnvmap;
    pub const FEATURE: BSShaderMaterialFeature = BSShaderMaterialFeature::EnvironmentMap;

    crate::relocation_func! {
        pub(crate) fn ctor(&mut self) -> *mut Self => RelocationID::new(100021, 106728)
    }
}

impl BSLightingShaderMaterialKind for BSLightingShaderMaterialEnvmap {
    const FEATURE: BSShaderMaterialFeature = Self::FEATURE;
}
