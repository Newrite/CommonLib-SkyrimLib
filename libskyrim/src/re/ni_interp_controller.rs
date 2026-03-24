use crate::re::NiRef;
use crate::re::NiTimeController;

core_util::abstract_type! { pub type NiInterpController; }

impl NiRef for NiInterpController {
    #[inline(always)]
    fn inc_ref(&self) {
        let ptr = self as *const _ as *const NiTimeController;
        unsafe { (*ptr).inc_ref() }
    }

    #[inline(always)]
    fn dec_ref(&self) {
        let ptr = self as *const _ as *const NiTimeController;
        unsafe { (*ptr).dec_ref() }
    }
}
