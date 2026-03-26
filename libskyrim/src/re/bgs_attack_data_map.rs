use crate::offsets::offsets_rtti::RTTI_BGSAttackDataMap;
use crate::offsets::offsets_vtable::VTABLE_BGSAttackDataMap;
use crate::re::{NiRef, NiRefObject};
use crate::relocation::{RttiType, VariantID};

// TODO: Translate the full `BGSAttackDataMap` layout when field access beyond
// pointer-compatible `NiPointer<BGSAttackDataMap>` ownership is needed.
core_util::abstract_type! { pub type BGSAttackDataMap; }

impl NiRef for BGSAttackDataMap {
    #[inline(always)]
    fn inc_ref(&self) {
        unsafe { (*(self as *const _ as *const NiRefObject)).inc_ref() }
    }

    #[inline(always)]
    fn dec_ref(&self) {
        unsafe { (*(self as *const _ as *const NiRefObject)).dec_ref() }
    }
}

impl RttiType for BGSAttackDataMap {
    const RTTI: VariantID = RTTI_BGSAttackDataMap;
}

impl BGSAttackDataMap {
    pub const RTTI: VariantID = RTTI_BGSAttackDataMap;
    pub const VTABLE: &'static [VariantID] = &VTABLE_BGSAttackDataMap;
}
