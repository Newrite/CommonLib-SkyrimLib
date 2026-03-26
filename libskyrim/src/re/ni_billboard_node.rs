use crate::offsets::offsets_nirtti::NiRTTI_NiBillboardNode;
use crate::offsets::offsets_rtti::RTTI_NiBillboardNode;
use crate::offsets::offsets_vtable::VTABLE_NiBillboardNode;
use crate::re::{NiNode, NiRef};
use crate::relocation::{RttiType, VariantID};

core_util::abstract_type! { pub type NiBillboardNode; }

impl NiRef for NiBillboardNode {
    #[inline(always)]
    fn inc_ref(&self) {
        unsafe { (*(self as *const _ as *const NiNode)).inc_ref() }
    }

    #[inline(always)]
    fn dec_ref(&self) {
        unsafe { (*(self as *const _ as *const NiNode)).dec_ref() }
    }
}

impl RttiType for NiBillboardNode {
    const RTTI: VariantID = RTTI_NiBillboardNode;
}

impl NiBillboardNode {
    pub const RTTI: VariantID = RTTI_NiBillboardNode;
    pub const NI_RTTI: VariantID = NiRTTI_NiBillboardNode;
    pub const VTABLE: &'static [VariantID] = &VTABLE_NiBillboardNode;
}
