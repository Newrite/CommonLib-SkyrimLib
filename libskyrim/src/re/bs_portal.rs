use crate::offsets::offsets_rtti::RTTI_BSPortal;
use crate::offsets::offsets_vtable::VTABLE_BSPortal;
use crate::re::ni_object::NiObject;
use crate::re::ni_ref_object::NiRef;
use crate::relocation::{RttiType, VariantID};

crate::core_util::abstract_type! { pub type BSPortal; }

impl RttiType for BSPortal {
    const RTTI: VariantID = RTTI_BSPortal;
}

impl NiRef for BSPortal {
    #[inline(always)]
    fn inc_ref(&self) {
        unsafe { (&*(self as *const Self as *const NiObject)).inc_ref() };
    }

    #[inline(always)]
    fn dec_ref(&self) {
        unsafe { (&*(self as *const Self as *const NiObject)).dec_ref() };
    }
}

impl BSPortal {
    pub const RTTI: VariantID = RTTI_BSPortal;
    pub const VTABLE: &'static [VariantID] = &VTABLE_BSPortal;
}
