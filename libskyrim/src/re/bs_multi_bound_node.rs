use crate::re::ni_node::NiNode;
use crate::re::ni_ref_object::NiRef;

crate::core_util::abstract_type! { pub type BSMultiBoundNode; }

impl NiRef for BSMultiBoundNode {
    #[inline(always)]
    fn inc_ref(&self) {
        unsafe { (&*(self as *const Self as *const NiNode)).inc_ref() };
    }

    #[inline(always)]
    fn dec_ref(&self) {
        unsafe { (&*(self as *const Self as *const NiNode)).dec_ref() };
    }
}
