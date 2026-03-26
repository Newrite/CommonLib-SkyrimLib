use crate::offsets::offsets_rtti::RTTI_TESRegionDataObjects;
use crate::offsets::offsets_vtable::VTABLE_TESRegionDataObjects;
use crate::relocation::{RttiType, VariantID};

crate::core_util::abstract_type! { pub type TESRegionDataObjects; }

impl RttiType for TESRegionDataObjects {
    const RTTI: VariantID = RTTI_TESRegionDataObjects;
}

impl TESRegionDataObjects {
    pub const RTTI: VariantID = RTTI_TESRegionDataObjects;
    pub const VTABLE: &'static [VariantID] = &VTABLE_TESRegionDataObjects;
}
