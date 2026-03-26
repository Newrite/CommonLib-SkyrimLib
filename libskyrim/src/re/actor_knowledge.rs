use crate::offsets::offsets_rtti::RTTI_ActorKnowledge;
use crate::offsets::offsets_vtable::VTABLE_ActorKnowledge;
use crate::re::{NiRef, NiRefObject};
use crate::relocation::{RttiType, VariantID};

core_util::abstract_type! { pub type ActorKnowledge; }

impl NiRef for ActorKnowledge {
    #[inline(always)]
    fn inc_ref(&self) {
        unsafe { (*(self as *const _ as *const NiRefObject)).inc_ref() }
    }

    #[inline(always)]
    fn dec_ref(&self) {
        unsafe { (*(self as *const _ as *const NiRefObject)).dec_ref() }
    }
}

impl RttiType for ActorKnowledge {
    const RTTI: VariantID = RTTI_ActorKnowledge;
}

impl ActorKnowledge {
    pub const RTTI: VariantID = RTTI_ActorKnowledge;
    pub const VTABLE: &'static [VariantID] = &VTABLE_ActorKnowledge;
}
