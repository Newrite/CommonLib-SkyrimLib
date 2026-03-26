use crate::offsets::offsets_rtti::RTTI_hkbBehaviorGraph;
use crate::offsets::offsets_vtable::VTABLE_hkbBehaviorGraph;
use crate::re::hkReferencedObject;
use crate::relocation::{RttiType, VariantID};

crate::core_util::abstract_type! {
    pub type hkbBehaviorGraph;
}

impl AsRef<hkReferencedObject> for hkbBehaviorGraph {
    #[inline(always)]
    fn as_ref(&self) -> &hkReferencedObject {
        unsafe { &*(self as *const Self as *const hkReferencedObject) }
    }
}

impl AsMut<hkReferencedObject> for hkbBehaviorGraph {
    #[inline(always)]
    fn as_mut(&mut self) -> &mut hkReferencedObject {
        unsafe { &mut *(self as *mut Self as *mut hkReferencedObject) }
    }
}

impl RttiType for hkbBehaviorGraph {
    const RTTI: VariantID = RTTI_hkbBehaviorGraph;
}

impl hkbBehaviorGraph {
    pub const RTTI: VariantID = RTTI_hkbBehaviorGraph;
    pub const VTABLE: &'static [VariantID] = &VTABLE_hkbBehaviorGraph;
}
