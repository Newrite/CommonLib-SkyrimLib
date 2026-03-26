use crate::offsets::offsets_nirtti::NiRTTI_NiPointLight;
use crate::offsets::offsets_rtti::RTTI_NiPointLight;
use crate::offsets::offsets_vtable::VTABLE_NiPointLight;
use crate::re::{NiLight, NiRef};
use crate::relocation::{RttiType, VariantID};

core_util::abstract_type! { pub type NiPointLight; }

impl NiRef for NiPointLight {
    #[inline(always)]
    fn inc_ref(&self) {
        unsafe { (*(self as *const _ as *const NiLight)).inc_ref() }
    }

    #[inline(always)]
    fn dec_ref(&self) {
        unsafe { (*(self as *const _ as *const NiLight)).dec_ref() }
    }
}

impl RttiType for NiPointLight {
    const RTTI: VariantID = RTTI_NiPointLight;
}

impl NiPointLight {
    pub const RTTI: VariantID = RTTI_NiPointLight;
    pub const NI_RTTI: VariantID = NiRTTI_NiPointLight;
    pub const VTABLE: &'static [VariantID] = &VTABLE_NiPointLight;
}
