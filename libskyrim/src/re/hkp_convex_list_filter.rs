use crate::offsets::offsets_rtti::RTTI_hkpConvexListFilter;
use crate::offsets::offsets_vtable::VTABLE_hkpConvexListFilter;
use crate::relocation::{RttiType, VariantID};

crate::core_util::abstract_type! {
    pub type hkpConvexListFilter;
}

impl RttiType for hkpConvexListFilter {
    const RTTI: VariantID = RTTI_hkpConvexListFilter;
}

impl hkpConvexListFilter {
    pub const RTTI: VariantID = RTTI_hkpConvexListFilter;
    pub const VTABLE: &'static [VariantID] = &VTABLE_hkpConvexListFilter;
}

// TODO: SOURCE - `hkpWorldCinfo` currently needs this only as a pointer target.
// Replace this opaque stand-in with the full interface from
// `hkpConvexListFilter.h` when convex-list callbacks are needed.
