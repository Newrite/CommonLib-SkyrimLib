use crate::offsets::offsets_rtti::RTTI_BSPathingDoor;
use crate::offsets::offsets_vtable::VTABLE_BSPathingDoor;
use crate::relocation::{RttiType, VariantID};

crate::core_util::abstract_type! {
    pub type BSPathingDoor;
}

impl RttiType for BSPathingDoor {
    const RTTI: VariantID = RTTI_BSPathingDoor;
}

impl BSPathingDoor {
    pub const RTTI: VariantID = RTTI_BSPathingDoor;
    pub const VTABLE: &'static [VariantID] = &VTABLE_BSPathingDoor;
}

// TODO: SOURCE - translate the concrete `BSPathingDoor` layout and intrusive
// smart-pointer contract when a caller needs more than raw pointer / metadata
// use from the pathing headers.
