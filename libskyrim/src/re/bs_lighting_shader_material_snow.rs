use core_util::inherit;

use crate::offsets::offsets_rtti::RTTI_BSLightingShaderMaterialSnow;
use crate::offsets::offsets_vtable::VTABLE_BSLightingShaderMaterialSnow;
use crate::re::{
    BSLightingShaderMaterialBase, BSLightingShaderMaterialKind, BSShaderMaterialFeature, NiColorA,
};
use crate::relocation::{RelocationID, RttiType, VariantID};

/// C++ `RE::BSLightingShaderMaterialSnow`
#[repr(C)]
pub struct BSLightingShaderMaterialSnow {
    pub base: BSLightingShaderMaterialBase, // 00
    pub sparkle_params: NiColorA,           // A0
}

const _: () = assert!(core::mem::size_of::<BSLightingShaderMaterialSnow>() == 0xB0);
const _: () = assert!(core::mem::offset_of!(BSLightingShaderMaterialSnow, sparkle_params) == 0xA0);

impl RttiType for BSLightingShaderMaterialSnow {
    const RTTI: VariantID = RTTI_BSLightingShaderMaterialSnow;
}

inherit!(BSLightingShaderMaterialSnow : BSLightingShaderMaterialBase, base);

impl BSLightingShaderMaterialSnow {
    pub const RTTI: VariantID = RTTI_BSLightingShaderMaterialSnow;
    pub const VTABLE: &'static [VariantID] = &VTABLE_BSLightingShaderMaterialSnow;
    pub const FEATURE: BSShaderMaterialFeature = BSShaderMaterialFeature::MultiIndexTriShapeSnow;

    crate::relocation_func! {
        pub(crate) fn ctor(&mut self) -> *mut Self => RelocationID::new(100118, 106825)
    }
}

impl BSLightingShaderMaterialKind for BSLightingShaderMaterialSnow {
    const FEATURE: BSShaderMaterialFeature = Self::FEATURE;
}
