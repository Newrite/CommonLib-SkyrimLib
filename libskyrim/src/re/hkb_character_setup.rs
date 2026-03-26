use crate::offsets::offsets_rtti::RTTI_hkbCharacterSetup;
use crate::offsets::offsets_vtable::VTABLE_hkbCharacterSetup;
use crate::re::hkReferencedObject;
use crate::relocation::{RttiType, VariantID};

crate::core_util::abstract_type! {
    pub type hkbCharacterSetup;
}

impl AsRef<hkReferencedObject> for hkbCharacterSetup {
    #[inline(always)]
    fn as_ref(&self) -> &hkReferencedObject {
        unsafe { &*(self as *const Self as *const hkReferencedObject) }
    }
}

impl AsMut<hkReferencedObject> for hkbCharacterSetup {
    #[inline(always)]
    fn as_mut(&mut self) -> &mut hkReferencedObject {
        unsafe { &mut *(self as *mut Self as *mut hkReferencedObject) }
    }
}

impl RttiType for hkbCharacterSetup {
    const RTTI: VariantID = RTTI_hkbCharacterSetup;
}

impl hkbCharacterSetup {
    pub const RTTI: VariantID = RTTI_hkbCharacterSetup;
    pub const VTABLE: &'static [VariantID] = &VTABLE_hkbCharacterSetup;
}
