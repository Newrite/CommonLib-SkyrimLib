use crate::offsets::offsets_nirtti::NiRTTI_BSLightingShaderProperty;
use crate::offsets::offsets_rtti::RTTI_BSLightingShaderProperty;
use crate::offsets::offsets_vtable::VTABLE_BSLightingShaderProperty;
use crate::re::{NiObject, NiRef};
use crate::relocation::{RttiType, VariantID};

crate::core_util::abstract_type! {
    pub type BSLightingShaderProperty;
}

impl NiRef for BSLightingShaderProperty {
    #[inline(always)]
    fn inc_ref(&self) {
        unsafe { (*(self as *const _ as *const NiObject)).inc_ref() }
    }

    #[inline(always)]
    fn dec_ref(&self) {
        unsafe { (*(self as *const _ as *const NiObject)).dec_ref() }
    }
}

impl RttiType for BSLightingShaderProperty {
    const RTTI: VariantID = RTTI_BSLightingShaderProperty;
}

impl BSLightingShaderProperty {
    pub const RTTI: VariantID = RTTI_BSLightingShaderProperty;
    pub const NI_RTTI: VariantID = NiRTTI_BSLightingShaderProperty;
    pub const VTABLE: &'static [VariantID] = &VTABLE_BSLightingShaderProperty;
}
