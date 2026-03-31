use crate::offsets::offsets_rtti::RTTI_hkpCollisionFilter;
use crate::offsets::offsets_vtable::VTABLE_hkpCollisionFilter;
use crate::relocation::{RttiType, VariantID};

crate::core_util::abstract_type! {
    pub type hkpCollisionFilter;
}

impl RttiType for hkpCollisionFilter {
    const RTTI: VariantID = RTTI_hkpCollisionFilter;
}

impl hkpCollisionFilter {
    pub const RTTI: VariantID = RTTI_hkpCollisionFilter;
    pub const VTABLE: &'static [VariantID] = &VTABLE_hkpCollisionFilter;
}

// TODO: SOURCE - `hkpWorldCinfo` / `hkpWorld` currently need this only as a
// pointer target. Replace this stand-in with the full multi-base translation
// from `hkpCollisionFilter.h` once callers need its interfaces or fields.
