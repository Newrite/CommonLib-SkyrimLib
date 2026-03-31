use crate::offsets::offsets_rtti::RTTI_BSPathingNumericIDVisitor;
use crate::offsets::offsets_vtable::VTABLE_BSPathingNumericIDVisitor;
use crate::relocation::{RttiType, VariantID};

crate::core_util::abstract_type! {
    pub type BSPathingNumericIDVisitor;
}

impl RttiType for BSPathingNumericIDVisitor {
    const RTTI: VariantID = RTTI_BSPathingNumericIDVisitor;
}

impl BSPathingNumericIDVisitor {
    pub const RTTI: VariantID = RTTI_BSPathingNumericIDVisitor;
    pub const VTABLE: &'static [VariantID] = &VTABLE_BSPathingNumericIDVisitor;
}

// TODO: SOURCE - keep this as an opaque visitor until a translated caller
// needs the callback ABI from the pathing serialization surface.
