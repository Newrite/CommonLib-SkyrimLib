use crate::relocation::RelocationID;

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

impl bhkCollisionObject {
    crate::relocation_func! {
        pub fn get_rigid_body(&self) -> *mut crate::re::bhkRigidBody => RelocationID::new(12784, 20014)
    }
}
