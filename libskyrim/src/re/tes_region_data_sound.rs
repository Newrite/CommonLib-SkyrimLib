use crate::offsets::offsets_rtti::RTTI_TESRegionDataSound;
use crate::offsets::offsets_vtable::VTABLE_TESRegionDataSound;
use crate::relocation::{RttiType, VariantID};

crate::core_util::abstract_type! { pub type TESRegionDataSound; }

impl RttiType for TESRegionDataSound {
    const RTTI: VariantID = RTTI_TESRegionDataSound;
}

impl TESRegionDataSound {
    pub const RTTI: VariantID = RTTI_TESRegionDataSound;
    pub const VTABLE: &'static [VariantID] = &VTABLE_TESRegionDataSound;
}
