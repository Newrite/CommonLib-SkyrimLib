use core_util::inherit;

use crate::offsets::offsets_rtti::RTTI_BSLightingShaderMaterialFacegen;
use crate::offsets::offsets_vtable::VTABLE_BSLightingShaderMaterialFacegen;
use crate::re::{
    BSLightingShaderMaterialBase, BSLightingShaderMaterialKind, BSShaderMaterialFeature, NiPointer,
    NiSourceTexture,
};
use crate::relocation::{RelocationID, RttiType, VariantID};

/// C++ `RE::BSLightingShaderMaterialFacegen`
#[repr(C)]
pub struct BSLightingShaderMaterialFacegen {
    pub base: BSLightingShaderMaterialBase,             // 00
    pub tint_texture: NiPointer<NiSourceTexture>,       // A0
    pub detail_texture: NiPointer<NiSourceTexture>,     // A8
    pub subsurface_texture: NiPointer<NiSourceTexture>, // B0
}

const _: () = assert!(core::mem::size_of::<BSLightingShaderMaterialFacegen>() == 0xB8);
const _: () = assert!(core::mem::offset_of!(BSLightingShaderMaterialFacegen, tint_texture) == 0xA0);

impl RttiType for BSLightingShaderMaterialFacegen {
    const RTTI: VariantID = RTTI_BSLightingShaderMaterialFacegen;
}

inherit!(BSLightingShaderMaterialFacegen : BSLightingShaderMaterialBase, base);

impl BSLightingShaderMaterialFacegen {
    pub const RTTI: VariantID = RTTI_BSLightingShaderMaterialFacegen;
    pub const VTABLE: &'static [VariantID] = &VTABLE_BSLightingShaderMaterialFacegen;
    pub const FEATURE: BSShaderMaterialFeature = BSShaderMaterialFeature::FaceGen;

    crate::relocation_func! {
        pub(crate) fn ctor(&mut self) -> *mut Self => RelocationID::new(100077, 106784)
    }
}

impl BSLightingShaderMaterialKind for BSLightingShaderMaterialFacegen {
    const FEATURE: BSShaderMaterialFeature = Self::FEATURE;
}
