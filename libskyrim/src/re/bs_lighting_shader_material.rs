use core_util::inherit;

use crate::offsets::offsets_rtti::RTTI_BSLightingShaderMaterial;
use crate::offsets::offsets_vtable::VTABLE_BSLightingShaderMaterial;
use crate::re::{
    BSLightingShaderMaterialBase, BSLightingShaderMaterialKind, BSShaderMaterialFeature,
};
use crate::relocation::{RelocationID, RttiType, VariantID};

/// C++ `RE::BSLightingShaderMaterial`
#[repr(C)]
pub struct BSLightingShaderMaterial {
    pub base: BSLightingShaderMaterialBase, // 00
}

const _: () = assert!(core::mem::size_of::<BSLightingShaderMaterial>() == 0xA0);

impl RttiType for BSLightingShaderMaterial {
    const RTTI: VariantID = RTTI_BSLightingShaderMaterial;
}

inherit!(BSLightingShaderMaterial : BSLightingShaderMaterialBase, base);

impl BSLightingShaderMaterial {
    pub const RTTI: VariantID = RTTI_BSLightingShaderMaterial;
    pub const VTABLE: &'static [VariantID] = &VTABLE_BSLightingShaderMaterial;
    pub const FEATURE: BSShaderMaterialFeature = BSShaderMaterialFeature::Default;

    crate::relocation_func! {
        pub(crate) fn ctor(&mut self) -> *mut Self => RelocationID::new(100004, 106711)
    }
}

impl BSLightingShaderMaterialKind for BSLightingShaderMaterial {
    const FEATURE: BSShaderMaterialFeature = Self::FEATURE;
}
