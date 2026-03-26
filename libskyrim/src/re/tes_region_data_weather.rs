use crate::offsets::offsets_rtti::RTTI_TESRegionDataWeather;
use crate::offsets::offsets_vtable::VTABLE_TESRegionDataWeather;
use crate::relocation::{RttiType, VariantID};

crate::core_util::abstract_type! { pub type TESRegionDataWeather; }

impl RttiType for TESRegionDataWeather {
    const RTTI: VariantID = RTTI_TESRegionDataWeather;
}

impl TESRegionDataWeather {
    pub const RTTI: VariantID = RTTI_TESRegionDataWeather;
    pub const VTABLE: &'static [VariantID] = &VTABLE_TESRegionDataWeather;
}
