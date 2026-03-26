use crate::offsets::offsets_rtti::RTTI_TESRegionDataMap;
use crate::offsets::offsets_vtable::VTABLE_TESRegionDataMap;
use crate::relocation::{RttiType, VariantID};

crate::core_util::abstract_type! { pub type TESRegionDataMap; }

impl RttiType for TESRegionDataMap {
    const RTTI: VariantID = RTTI_TESRegionDataMap;
}

impl TESRegionDataMap {
    pub const RTTI: VariantID = RTTI_TESRegionDataMap;
    pub const VTABLE: &'static [VariantID] = &VTABLE_TESRegionDataMap;
}
