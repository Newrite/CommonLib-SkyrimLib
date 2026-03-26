use crate::offsets::offsets_rtti::RTTI_TESRegionDataGrass;
use crate::offsets::offsets_vtable::VTABLE_TESRegionDataGrass;
use crate::relocation::{RttiType, VariantID};

crate::core_util::abstract_type! { pub type TESRegionDataGrass; }

impl RttiType for TESRegionDataGrass {
    const RTTI: VariantID = RTTI_TESRegionDataGrass;
}

impl TESRegionDataGrass {
    pub const RTTI: VariantID = RTTI_TESRegionDataGrass;
    pub const VTABLE: &'static [VariantID] = &VTABLE_TESRegionDataGrass;
}
