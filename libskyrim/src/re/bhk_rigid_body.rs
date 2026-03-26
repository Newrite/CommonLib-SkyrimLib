use crate::re::{NiRef, bhkRefObject};

crate::core_util::abstract_type! { pub type bhkRigidBody; }

impl NiRef for bhkRigidBody {
    #[inline(always)]
    fn inc_ref(&self) {
        // Source-backed inheritance chain:
        // bhkRigidBody -> bhkEntity -> bhkWorldObject -> bhkSerializable -> bhkRefObject.
        unsafe { (*(self as *const _ as *mut bhkRefObject)).adjust_ref_count(true) }
    }

    #[inline(always)]
    fn dec_ref(&self) {
        unsafe { (*(self as *const _ as *mut bhkRefObject)).adjust_ref_count(false) }
    }
}
