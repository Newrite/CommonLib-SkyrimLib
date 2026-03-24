use crate::offsets::offsets_rtti::RTTI_NiTexture;
use crate::offsets::offsets_vtable::VTABLE_NiTexture;
use crate::re::ni_object::NiObject;
use crate::re::ni_ref_object::NiRef;
use crate::relocation::{RttiType, VariantID};

crate::core_util::abstract_type! { pub type NiTexture; }

impl RttiType for NiTexture {
    const RTTI: VariantID = RTTI_NiTexture;
}

impl NiRef for NiTexture {
    #[inline(always)]
    fn inc_ref(&self) {
        unsafe { (&*(self as *const Self as *const NiObject)).inc_ref() };
    }

    #[inline(always)]
    fn dec_ref(&self) {
        unsafe { (&*(self as *const Self as *const NiObject)).dec_ref() };
    }
}

impl NiTexture {
    pub const RTTI: VariantID = RTTI_NiTexture;
    pub const VTABLE: &'static [VariantID] = &VTABLE_NiTexture;
}
