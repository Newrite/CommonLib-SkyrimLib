use crate::offsets::offsets_rtti::RTTI_TESTopic;
use crate::offsets::offsets_vtable::VTABLE_TESTopic;
use crate::relocation::{RttiType, VariantID};

core_util::abstract_type! { pub type TESTopic; }

impl RttiType for TESTopic {
    const RTTI: VariantID = RTTI_TESTopic;
}

impl TESTopic {
    pub const RTTI: VariantID = RTTI_TESTopic;
    pub const VTABLE: &'static [VariantID] = &VTABLE_TESTopic;
}
