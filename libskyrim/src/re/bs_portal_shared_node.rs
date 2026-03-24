use crate::offsets::offsets_rtti::RTTI_BSPortalSharedNode;
use crate::offsets::offsets_vtable::VTABLE_BSPortalSharedNode;
use crate::re::ni_node::NiNode;
use crate::re::ni_ref_object::NiRef;
use crate::relocation::{RttiType, VariantID};

crate::core_util::abstract_type! { pub type BSPortalSharedNode; }

impl RttiType for BSPortalSharedNode {
    const RTTI: VariantID = RTTI_BSPortalSharedNode;
}

impl NiRef for BSPortalSharedNode {
    #[inline(always)]
    fn inc_ref(&self) {
        unsafe { (&*(self as *const Self as *const NiNode)).inc_ref() };
    }

    #[inline(always)]
    fn dec_ref(&self) {
        unsafe { (&*(self as *const Self as *const NiNode)).dec_ref() };
    }
}

impl BSPortalSharedNode {
    pub const RTTI: VariantID = RTTI_BSPortalSharedNode;
    pub const VTABLE: &'static [VariantID] = &VTABLE_BSPortalSharedNode;
}
