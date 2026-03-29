use core_util::inherit;

use crate::offsets::offsets_rtti::RTTI_BSLightingShaderMaterialParallax;
use crate::offsets::offsets_vtable::VTABLE_BSLightingShaderMaterialParallax;
use crate::re::{
    BSLightingShaderMaterialBase, BSLightingShaderMaterialKind, BSShaderMaterialFeature, NiPointer,
    NiSourceTexture,
};
use crate::relocation::{RelocationID, RttiType, VariantID};

/// C++ `RE::BSLightingShaderMaterialParallax`
#[repr(C)]
pub struct BSLightingShaderMaterialParallax {
    pub base: BSLightingShaderMaterialBase,         // 00
    pub height_texture: NiPointer<NiSourceTexture>, // A0
}

const _: () = assert!(core::mem::size_of::<BSLightingShaderMaterialParallax>() == 0xA8);
const _: () =
    assert!(core::mem::offset_of!(BSLightingShaderMaterialParallax, height_texture) == 0xA0);

impl RttiType for BSLightingShaderMaterialParallax {
    const RTTI: VariantID = RTTI_BSLightingShaderMaterialParallax;
}

inherit!(BSLightingShaderMaterialParallax : BSLightingShaderMaterialBase, base);

impl BSLightingShaderMaterialParallax {
    pub const RTTI: VariantID = RTTI_BSLightingShaderMaterialParallax;
    pub const VTABLE: &'static [VariantID] = &VTABLE_BSLightingShaderMaterialParallax;
    pub const FEATURE: BSShaderMaterialFeature = BSShaderMaterialFeature::Parallax;

    crate::relocation_func! {
        pub(crate) fn ctor(&mut self) -> *mut Self => RelocationID::new(100055, 106762)
    }
}

impl BSLightingShaderMaterialKind for BSLightingShaderMaterialParallax {
    const FEATURE: BSShaderMaterialFeature = Self::FEATURE;
}
