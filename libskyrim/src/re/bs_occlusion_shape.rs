use crate::offsets::offsets_rtti::RTTI_BSOcclusionShape;
use crate::offsets::offsets_vtable::VTABLE_BSOcclusionShape;
use crate::re::ni_object::NiObject;
use crate::re::ni_ref_object::NiRef;
use crate::relocation::{RttiType, VariantID};

crate::core_util::abstract_type! { pub type BSOcclusionShape; }

impl RttiType for BSOcclusionShape {
    const RTTI: VariantID = RTTI_BSOcclusionShape;
}

impl NiRef for BSOcclusionShape {
    #[inline(always)]
    fn inc_ref(&self) {
        unsafe { (&*(self as *const Self as *const NiObject)).inc_ref() };
    }

    #[inline(always)]
    fn dec_ref(&self) {
        unsafe { (&*(self as *const Self as *const NiObject)).dec_ref() };
    }
}

impl BSOcclusionShape {
    pub const RTTI: VariantID = RTTI_BSOcclusionShape;
    pub const VTABLE: &'static [VariantID] = &VTABLE_BSOcclusionShape;
}
