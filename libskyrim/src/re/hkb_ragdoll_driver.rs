use crate::offsets::offsets_rtti::RTTI_hkbRagdollDriver;
use crate::offsets::offsets_vtable::VTABLE_hkbRagdollDriver;
use crate::re::hkReferencedObject;
use crate::relocation::{RttiType, VariantID};

crate::core_util::abstract_type! {
    pub type hkbRagdollDriver;
}

impl AsRef<hkReferencedObject> for hkbRagdollDriver {
    #[inline(always)]
    fn as_ref(&self) -> &hkReferencedObject {
        unsafe { &*(self as *const Self as *const hkReferencedObject) }
    }
}

impl AsMut<hkReferencedObject> for hkbRagdollDriver {
    #[inline(always)]
    fn as_mut(&mut self) -> &mut hkReferencedObject {
        unsafe { &mut *(self as *mut Self as *mut hkReferencedObject) }
    }
}

impl RttiType for hkbRagdollDriver {
    const RTTI: VariantID = RTTI_hkbRagdollDriver;
}

impl hkbRagdollDriver {
    pub const RTTI: VariantID = RTTI_hkbRagdollDriver;
    pub const VTABLE: &'static [VariantID] = &VTABLE_hkbRagdollDriver;
}
