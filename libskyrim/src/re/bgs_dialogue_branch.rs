use crate::offsets::offsets_rtti::RTTI_BGSDialogueBranch;
use crate::offsets::offsets_vtable::VTABLE_BGSDialogueBranch;
use crate::relocation::{RttiType, VariantID};

core_util::abstract_type! { pub type BGSDialogueBranch; }

impl RttiType for BGSDialogueBranch {
    const RTTI: VariantID = RTTI_BGSDialogueBranch;
}

impl BGSDialogueBranch {
    pub const RTTI: VariantID = RTTI_BGSDialogueBranch;
    pub const VTABLE: &'static [VariantID] = &VTABLE_BGSDialogueBranch;
}
