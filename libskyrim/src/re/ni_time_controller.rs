crate::core_util::abstract_type! { pub type NiTimeController; }

impl crate::re::NiRef for NiTimeController {
    #[inline(always)]
    fn inc_ref(&self) {
        let ptr = self as *const _ as *const crate::re::NiRefObject;
        unsafe { (*ptr).inc_ref() }
    }

    #[inline(always)]
    fn dec_ref(&self) {
        let ptr = self as *const _ as *const crate::re::NiRefObject;
        unsafe { (*ptr).dec_ref() }
    }
}
