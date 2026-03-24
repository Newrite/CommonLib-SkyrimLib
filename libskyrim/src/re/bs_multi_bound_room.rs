use crate::offsets::offsets_rtti::RTTI_BSMultiBoundRoom;
use crate::offsets::offsets_vtable::VTABLE_BSMultiBoundRoom;
use crate::re::bs_multi_bound_node::BSMultiBoundNode;
use crate::re::ni_ref_object::NiRef;
use crate::relocation::{RttiType, VariantID};

crate::core_util::abstract_type! { pub type BSMultiBoundRoom; }

impl RttiType for BSMultiBoundRoom {
    const RTTI: VariantID = RTTI_BSMultiBoundRoom;
}

impl NiRef for BSMultiBoundRoom {
    #[inline(always)]
    fn inc_ref(&self) {
        unsafe { (&*(self as *const Self as *const BSMultiBoundNode)).inc_ref() };
    }

    #[inline(always)]
    fn dec_ref(&self) {
        unsafe { (&*(self as *const Self as *const BSMultiBoundNode)).dec_ref() };
    }
}

impl BSMultiBoundRoom {
    pub const RTTI: VariantID = RTTI_BSMultiBoundRoom;
    pub const VTABLE: &'static [VariantID] = &VTABLE_BSMultiBoundRoom;
}
