use core_util::inherit;

use crate::offsets::offsets_rtti::RTTI_BSLightingShaderMaterialHairTint;
use crate::offsets::offsets_vtable::VTABLE_BSLightingShaderMaterialHairTint;
use crate::re::{
    BSLightingShaderMaterialBase, BSLightingShaderMaterialKind, BSShaderMaterialFeature, NiColor,
};
use crate::relocation::{RelocationID, RttiType, VariantID};

/// C++ `RE::BSLightingShaderMaterialHairTint`
#[repr(C)]
pub struct BSLightingShaderMaterialHairTint {
    pub base: BSLightingShaderMaterialBase, // 00
    pub tint_color: NiColor,                // A0
    pub padac: u32,                         // AC
}

const _: () = assert!(core::mem::size_of::<BSLightingShaderMaterialHairTint>() == 0xB0);
const _: () = assert!(core::mem::offset_of!(BSLightingShaderMaterialHairTint, tint_color) == 0xA0);

impl RttiType for BSLightingShaderMaterialHairTint {
    const RTTI: VariantID = RTTI_BSLightingShaderMaterialHairTint;
}

inherit!(BSLightingShaderMaterialHairTint : BSLightingShaderMaterialBase, base);

impl BSLightingShaderMaterialHairTint {
    pub const RTTI: VariantID = RTTI_BSLightingShaderMaterialHairTint;
    pub const VTABLE: &'static [VariantID] = &VTABLE_BSLightingShaderMaterialHairTint;
    pub const FEATURE: BSShaderMaterialFeature = BSShaderMaterialFeature::HairTint;

    crate::relocation_func! {
        pub(crate) fn ctor(&mut self) -> *mut Self => RelocationID::new(100095, 106802)
    }
}

impl BSLightingShaderMaterialKind for BSLightingShaderMaterialHairTint {
    const FEATURE: BSShaderMaterialFeature = Self::FEATURE;
}
