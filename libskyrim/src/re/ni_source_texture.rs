use crate::offsets::offsets_nirtti::NiRTTI_NiSourceTexture;
use crate::offsets::offsets_rtti::RTTI_NiSourceTexture;
use crate::offsets::offsets_vtable::VTABLE_NiSourceTexture;
use crate::re::NiRef;
use crate::re::NiRefObject;
use crate::relocation::{RttiType, VariantID};

core_util::abstract_type! {
    pub type NiSourceTexture;
}

impl RttiType for NiSourceTexture {
    const RTTI: VariantID = RTTI_NiSourceTexture;
}

impl NiRef for NiSourceTexture {
    #[inline(always)]
    fn inc_ref(&self) {
        unsafe { (*(self as *const _ as *const NiRefObject)).inc_ref() }
    }

    #[inline(always)]
    fn dec_ref(&self) {
        unsafe { (*(self as *const _ as *const NiRefObject)).dec_ref() }
    }
}

impl NiSourceTexture {
    pub const RTTI: VariantID = RTTI_NiSourceTexture;
    pub const NI_RTTI: VariantID = NiRTTI_NiSourceTexture;
    pub const VTABLE: &'static [VariantID] = &VTABLE_NiSourceTexture;
}
