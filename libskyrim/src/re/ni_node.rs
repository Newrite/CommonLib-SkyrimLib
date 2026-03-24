use crate::re::NiObject;
use crate::re::NiRef;

crate::core_util::abstract_type! { pub type NiNode; }

impl NiRef for NiNode {
    #[inline(always)]
    fn inc_ref(&self) {
        unsafe { (&*(self as *const Self as *const NiObject)).inc_ref() };
    }

    #[inline(always)]
    fn dec_ref(&self) {
        unsafe { (&*(self as *const Self as *const NiObject)).dec_ref() };
    }
}
