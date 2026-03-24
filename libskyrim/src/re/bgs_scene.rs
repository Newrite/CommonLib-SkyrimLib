use crate::offsets::offsets_rtti::RTTI_BGSScene;
use crate::offsets::offsets_vtable::VTABLE_BGSScene;
use crate::relocation::{RttiType, VariantID};

core_util::abstract_type! { pub type BGSScene; }

impl RttiType for BGSScene {
    const RTTI: VariantID = RTTI_BGSScene;
}

impl BGSScene {
    pub const RTTI: VariantID = RTTI_BGSScene;
    pub const VTABLE: &'static [VariantID] = &VTABLE_BGSScene;
}
