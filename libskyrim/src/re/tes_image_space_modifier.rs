use crate::offsets::offsets_rtti::RTTI_TESImageSpaceModifier;
use crate::offsets::offsets_vtable::VTABLE_TESImageSpaceModifier;
use crate::relocation::{RttiType, VariantID};

core_util::abstract_type! {
    pub type TESImageSpaceModifier;
}

impl RttiType for TESImageSpaceModifier {
    const RTTI: VariantID = RTTI_TESImageSpaceModifier;
}

impl TESImageSpaceModifier {
    pub const RTTI: VariantID = RTTI_TESImageSpaceModifier;
    pub const VTABLE: &'static [VariantID] = &VTABLE_TESImageSpaceModifier;
}
