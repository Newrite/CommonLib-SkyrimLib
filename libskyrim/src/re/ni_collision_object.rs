crate::core_util::abstract_type! { pub type NiCollisionObject; }

impl crate::re::NiRef for NiCollisionObject {
    fn inc_ref(&self) {
        unsafe {
            let this = self as *const Self as *mut crate::re::NiRefObject;
            (*this).inc_ref();
        }
    }
    fn dec_ref(&self) {
        unsafe {
            let this = self as *const Self as *mut crate::re::NiRefObject;
            (*this).dec_ref();
        }
    }
}
