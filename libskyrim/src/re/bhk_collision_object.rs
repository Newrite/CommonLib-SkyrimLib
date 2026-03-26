core_util::abstract_type! { pub type bhkCollisionObject; }

impl crate::re::NiRef for bhkCollisionObject {
    #[inline(always)]
    fn inc_ref(&self) {
        unsafe {
            let this = self as *const Self as *mut crate::re::NiCollisionObject;
            (*this).inc_ref();
        }
    }

    #[inline(always)]
    fn dec_ref(&self) {
        unsafe {
            let this = self as *const Self as *mut crate::re::NiCollisionObject;
            (*this).dec_ref();
        }
    }
}
