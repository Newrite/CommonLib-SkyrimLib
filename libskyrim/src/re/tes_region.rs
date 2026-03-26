use crate::offsets::offsets_rtti::RTTI_TESRegion;
use crate::offsets::offsets_vtable::VTABLE_TESRegion;
use crate::re::{FormCastable, FormType};
use crate::relocation::{RttiType, VariantID};

crate::core_util::abstract_type! { pub type TESRegion; }

impl RttiType for TESRegion {
    const RTTI: VariantID = RTTI_TESRegion;
}

impl FormCastable for TESRegion {
    const TARGET_FORM_TYPE: FormType = FormType::Region;
}

impl TESRegion {
    pub const RTTI: VariantID = RTTI_TESRegion;
    pub const VTABLE: &'static [VariantID] = &VTABLE_TESRegion;
    pub const FORMTYPE: FormType = FormType::Region;
}

// TODO: The requested partial patch only needs pointer-compatible `TESRegion` references for
// `TESRegionDataManager` and `TESRegionList`. Replace this opaque stand-in with a real layout
// translation when a caller needs `TESRegion` fields or helper methods.
