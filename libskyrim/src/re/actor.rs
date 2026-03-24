use crate::offsets::offsets_rtti::RTTI_Actor;
use crate::re::BSHandleRefObject;
use crate::re::NiRef;
use crate::relocation::{RttiType, VariantID};

core_util::abstract_type! { pub type Actor; }

impl RttiType for Actor {
    const RTTI: VariantID = RTTI_Actor;
}

impl NiRef for Actor {
    #[inline(always)]
    fn inc_ref(&self) {
        unsafe { (&*(self as *const Self as *const BSHandleRefObject)).inc_ref_count() };
    }

    #[inline(always)]
    fn dec_ref(&self) {
        unsafe { (&*(self as *const Self as *const BSHandleRefObject)).dec_ref_count() };
    }
}
