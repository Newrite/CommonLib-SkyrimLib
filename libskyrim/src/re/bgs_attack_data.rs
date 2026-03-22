use crate::re::NiRef;
use crate::re::NiRefObject;

core_util::abstract_type! {
    pub type BGSAttackData;
}

impl NiRef for BGSAttackData {
    #[inline(always)]
    fn inc_ref(&self) {
        unsafe { (*(self as *const _ as *const NiRefObject)).inc_ref() }
    }

    #[inline(always)]
    fn dec_ref(&self) {
        unsafe { (*(self as *const _ as *const NiRefObject)).dec_ref() }
    }
}
