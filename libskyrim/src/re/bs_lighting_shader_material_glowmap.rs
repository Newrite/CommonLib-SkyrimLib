use core_util::inherit;

use crate::offsets::offsets_rtti::RTTI_BSLightingShaderMaterialGlowmap;
use crate::offsets::offsets_vtable::VTABLE_BSLightingShaderMaterialGlowmap;
use crate::re::{
    BSLightingShaderMaterialBase, BSLightingShaderMaterialKind, BSShaderMaterialFeature, NiPointer,
    NiSourceTexture,
};
use crate::relocation::{RelocationID, RttiType, VariantID};

/// C++ `RE::BSLightingShaderMaterialGlowmap`
#[repr(C)]
pub struct BSLightingShaderMaterialGlowmap {
    pub base: BSLightingShaderMaterialBase,       // 00
    pub glow_texture: NiPointer<NiSourceTexture>, // A0
}

const _: () = assert!(core::mem::size_of::<BSLightingShaderMaterialGlowmap>() == 0xA8);
const _: () = assert!(core::mem::offset_of!(BSLightingShaderMaterialGlowmap, glow_texture) == 0xA0);

impl RttiType for BSLightingShaderMaterialGlowmap {
    const RTTI: VariantID = RTTI_BSLightingShaderMaterialGlowmap;
}

inherit!(BSLightingShaderMaterialGlowmap : BSLightingShaderMaterialBase, base);

impl BSLightingShaderMaterialGlowmap {
    pub const RTTI: VariantID = RTTI_BSLightingShaderMaterialGlowmap;
    pub const VTABLE: &'static [VariantID] = &VTABLE_BSLightingShaderMaterialGlowmap;
    pub const FEATURE: BSShaderMaterialFeature = BSShaderMaterialFeature::GlowMap;

    crate::relocation_func! {
        pub(crate) fn ctor(&mut self) -> *mut Self => RelocationID::new(100045, 106752)
    }
}

impl BSLightingShaderMaterialKind for BSLightingShaderMaterialGlowmap {
    const FEATURE: BSShaderMaterialFeature = Self::FEATURE;
}
