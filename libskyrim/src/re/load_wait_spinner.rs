use crate::offsets::offsets_rtti::RTTI_LoadWaitSpinner;
use crate::offsets::offsets_vtable::VTABLE_LoadWaitSpinner;
use crate::relocation::{RttiType, VariantID};

// C++ `RE::LoadWaitSpinner` minimal menu marker.
//
// TODO: SOURCE - replace this opaque menu marker with the real
// `IMenu`-derived layout from `LoadWaitSpinner.h/.cpp` when fields, bases, or
// methods are needed; the current stub only preserves RTTI/VTABLE/MENU_NAME
// for menu lookup APIs.
core_util::abstract_type! { pub type LoadWaitSpinner; }

impl RttiType for LoadWaitSpinner {
    const RTTI: VariantID = RTTI_LoadWaitSpinner;
}

impl LoadWaitSpinner {
    pub const RTTI: VariantID = RTTI_LoadWaitSpinner;
    pub const VTABLE: &'static [VariantID] = &VTABLE_LoadWaitSpinner;
    pub const MENU_NAME: &'static str = "LoadWaitSpinner";
}
