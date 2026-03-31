use crate::offsets::offsets_rtti::RTTI_BSPathingSpace;
use crate::offsets::offsets_vtable::VTABLE_BSPathingSpace;
use crate::relocation::{RttiType, VariantID};

crate::core_util::abstract_type! {
    pub type BSPathingSpace;
}

impl RttiType for BSPathingSpace {
    const RTTI: VariantID = RTTI_BSPathingSpace;
}

impl BSPathingSpace {
    pub const RTTI: VariantID = RTTI_BSPathingSpace;
    pub const VTABLE: &'static [VariantID] = &VTABLE_BSPathingSpace;
}

// TODO: SOURCE - `BSPathingCell` currently needs `BSPathingSpace` only as an
// out-param pointee. Replace this opaque stand-in with the concrete
// translation once pathing-space fields or methods are needed.
