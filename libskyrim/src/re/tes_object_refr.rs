use crate::offsets::offsets_rtti::RTTI_TESObjectREFR;
use crate::re::BSHandleRefObject;
use crate::re::NiRef;
use crate::relocation::{RttiType, VariantID};

crate::core_util::abstract_type! { pub type TESObjectREFR; }

impl RttiType for TESObjectREFR {
    const RTTI: VariantID = RTTI_TESObjectREFR;
}

impl NiRef for TESObjectREFR {
    #[inline(always)]
    fn inc_ref(&self) {
        unsafe { (&*(self as *const Self as *const BSHandleRefObject)).inc_ref_count() };
    }

    #[inline(always)]
    fn dec_ref(&self) {
        unsafe { (&*(self as *const Self as *const BSHandleRefObject)).dec_ref_count() };
    }
}
