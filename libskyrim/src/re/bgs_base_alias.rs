use crate::offsets::offsets_rtti::RTTI_BGSBaseAlias;
use crate::offsets::offsets_vtable::VTABLE_BGSBaseAlias;
use crate::relocation::{RttiType, VariantID};

core_util::abstract_type! { pub type BGSBaseAlias; }

impl RttiType for BGSBaseAlias {
    const RTTI: VariantID = RTTI_BGSBaseAlias;
}

impl BGSBaseAlias {
    pub const RTTI: VariantID = RTTI_BGSBaseAlias;
    pub const VTABLE: &'static [VariantID] = &VTABLE_BGSBaseAlias;
}
