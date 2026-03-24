use crate::re::BSHandleRefObject;
use crate::re::NiRef;

crate::core_util::abstract_type! { pub type TESObjectREFR; }

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
