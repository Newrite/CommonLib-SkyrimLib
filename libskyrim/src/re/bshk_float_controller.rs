use crate::offsets::offsets_rtti::RTTI_BShkFloatController;
use crate::offsets::offsets_vtable::VTABLE_BShkFloatController;
use crate::relocation::{RttiType, VariantID};

crate::core_util::abstract_type! {
    pub type BShkFloatController;
}

impl RttiType for BShkFloatController {
    const RTTI: VariantID = RTTI_BShkFloatController;
}

impl BShkFloatController {
    pub const RTTI: VariantID = RTTI_BShkFloatController;
    pub const VTABLE: &'static [VariantID] = &VTABLE_BShkFloatController;
}
