#![allow(non_camel_case_types)]

use crate::offsets::offsets_nirtti::NiRTTI_bhkMouseSpringAction;
use crate::offsets::offsets_rtti::RTTI_bhkMouseSpringAction;
use crate::offsets::offsets_vtable::VTABLE_bhkMouseSpringAction;
use crate::re::{HkRef, bhkRefObject};
use crate::relocation::{RttiType, VariantID};

crate::core_util::abstract_type! { pub type bhkMouseSpringAction; }

impl RttiType for bhkMouseSpringAction {
    const RTTI: VariantID = RTTI_bhkMouseSpringAction;
}

impl bhkMouseSpringAction {
    pub const RTTI: VariantID = RTTI_bhkMouseSpringAction;
    pub const NI_RTTI: VariantID = NiRTTI_bhkMouseSpringAction;
    pub const VTABLE: &'static [VariantID] = &VTABLE_bhkMouseSpringAction;
}

impl HkRef for bhkMouseSpringAction {
    #[inline(always)]
    fn add_reference(&self) {
        let base = self as *const Self as *mut bhkRefObject;
        unsafe { (*base).adjust_ref_count(true) }
    }

    #[inline(always)]
    fn remove_reference(&self) {
        let base = self as *const Self as *mut bhkRefObject;
        unsafe { (*base).adjust_ref_count(false) }
    }
}
