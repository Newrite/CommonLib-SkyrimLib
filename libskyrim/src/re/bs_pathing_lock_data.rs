use crate::offsets::offsets_rtti::RTTI_BSPathingLockData;
use crate::offsets::offsets_vtable::VTABLE_BSPathingLockData;
use crate::relocation::{RttiType, VariantID};

crate::core_util::abstract_type! {
    pub type BSPathingLockData;
}

impl RttiType for BSPathingLockData {
    const RTTI: VariantID = RTTI_BSPathingLockData;
}

impl BSPathingLockData {
    pub const RTTI: VariantID = RTTI_BSPathingLockData;
    pub const VTABLE: &'static [VariantID] = &VTABLE_BSPathingLockData;
}

// TODO: SOURCE - `BSPathingActorAttributes` currently only needs pointer-sized
// storage for `BSPathingLockData`. Replace this opaque stand-in with the real
// layout once lock-data fields or helpers are needed.
