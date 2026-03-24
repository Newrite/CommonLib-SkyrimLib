use crate::re::NiRef;
use crate::re::NiRefObject;

core_util::abstract_type! { pub type NiStringPalette; }

impl NiRef for NiStringPalette {
    #[inline(always)]
    fn inc_ref(&self) {
        let ptr = self as *const _ as *const NiRefObject;
        unsafe { (*ptr).inc_ref() }
    }

    #[inline(always)]
    fn dec_ref(&self) {
        let ptr = self as *const _ as *const NiRefObject;
        unsafe { (*ptr).dec_ref() }
    }
}
