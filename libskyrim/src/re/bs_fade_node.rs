use crate::offsets::offsets_nirtti::NiRTTI_BSFadeNode;
use crate::offsets::offsets_rtti::RTTI_BSFadeNode;
use crate::offsets::offsets_vtable::VTABLE_BSFadeNode;
use crate::re::{NiNode, NiRef};
use crate::relocation::{RttiType, VariantID};

crate::core_util::abstract_type! { pub type BSFadeNode; }

impl RttiType for BSFadeNode {
    const RTTI: VariantID = RTTI_BSFadeNode;
}

impl BSFadeNode {
    pub const RTTI: VariantID = RTTI_BSFadeNode;
    pub const NI_RTTI: VariantID = NiRTTI_BSFadeNode;
    pub const VTABLE: &'static [VariantID] = &VTABLE_BSFadeNode;
}

impl NiRef for BSFadeNode {
    #[inline(always)]
    fn inc_ref(&self) {
        unsafe { (*(self as *const Self as *const NiNode)).inc_ref() }
    }

    #[inline(always)]
    fn dec_ref(&self) {
        unsafe { (*(self as *const Self as *const NiNode)).dec_ref() }
    }
}

// TODO: This is currently a pointer-compatible partial translation of `RE::BSFadeNode` backed
// by `BSFadeNode.h` inheritance (`BSFadeNode : NiNode`). Replace this opaque stand-in with an
// honest runtime-layout translation when code needs `BSFadeNode` fields or virtual surface.
