use crate::offsets::offsets_rtti::RTTI_TESRegionDataLandscape;
use crate::offsets::offsets_vtable::VTABLE_TESRegionDataLandscape;
use crate::relocation::{RttiType, VariantID};

crate::core_util::abstract_type! { pub type TESRegionDataLandscape; }

impl RttiType for TESRegionDataLandscape {
    const RTTI: VariantID = RTTI_TESRegionDataLandscape;
}

impl TESRegionDataLandscape {
    pub const RTTI: VariantID = RTTI_TESRegionDataLandscape;
    pub const VTABLE: &'static [VariantID] = &VTABLE_TESRegionDataLandscape;
}
