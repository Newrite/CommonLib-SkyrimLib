use core_util::inherit;

use crate::offsets::offsets_rtti::RTTI_BSLightingShaderMaterialFacegenTint;
use crate::offsets::offsets_vtable::VTABLE_BSLightingShaderMaterialFacegenTint;
use crate::re::{
    BSLightingShaderMaterialBase, BSLightingShaderMaterialKind, BSShaderMaterialFeature, NiColor,
};
use crate::relocation::{RelocationID, RttiType, VariantID};

/// C++ `RE::BSLightingShaderMaterialFacegenTint`
#[repr(C)]
pub struct BSLightingShaderMaterialFacegenTint {
    pub base: BSLightingShaderMaterialBase, // 00
    pub tint_color: NiColor,                // A0
    pub padac: u32,                         // AC
}

const _: () = assert!(core::mem::size_of::<BSLightingShaderMaterialFacegenTint>() == 0xB0);
const _: () =
    assert!(core::mem::offset_of!(BSLightingShaderMaterialFacegenTint, tint_color) == 0xA0);

impl RttiType for BSLightingShaderMaterialFacegenTint {
    const RTTI: VariantID = RTTI_BSLightingShaderMaterialFacegenTint;
}

inherit!(BSLightingShaderMaterialFacegenTint : BSLightingShaderMaterialBase, base);

impl BSLightingShaderMaterialFacegenTint {
    pub const RTTI: VariantID = RTTI_BSLightingShaderMaterialFacegenTint;
    pub const VTABLE: &'static [VariantID] = &VTABLE_BSLightingShaderMaterialFacegenTint;
    pub const FEATURE: BSShaderMaterialFeature = BSShaderMaterialFeature::FaceGenRGBTint;

    crate::relocation_func! {
        pub(crate) fn ctor(&mut self) -> *mut Self => RelocationID::new(100087, 106794)
    }
}

impl BSLightingShaderMaterialKind for BSLightingShaderMaterialFacegenTint {
    const FEATURE: BSShaderMaterialFeature = Self::FEATURE;
}
