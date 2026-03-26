use crate::offsets::offsets_rtti::RTTI_hkpSphereShape;
use crate::offsets::offsets_vtable::VTABLE_hkpSphereShape;
use crate::re::hkReferencedObject;
use crate::relocation::{RttiType, VariantID};

crate::core_util::abstract_type! {
    pub type hkpSphereShape;
}

impl AsRef<hkReferencedObject> for hkpSphereShape {
    #[inline(always)]
    fn as_ref(&self) -> &hkReferencedObject {
        unsafe { &*(self as *const Self as *const hkReferencedObject) }
    }
}

impl AsMut<hkReferencedObject> for hkpSphereShape {
    #[inline(always)]
    fn as_mut(&mut self) -> &mut hkReferencedObject {
        unsafe { &mut *(self as *mut Self as *mut hkReferencedObject) }
    }
}

impl RttiType for hkpSphereShape {
    const RTTI: VariantID = RTTI_hkpSphereShape;
}

impl hkpSphereShape {
    pub const RTTI: VariantID = RTTI_hkpSphereShape;
    pub const VTABLE: &'static [VariantID] = &VTABLE_hkpSphereShape;
}
