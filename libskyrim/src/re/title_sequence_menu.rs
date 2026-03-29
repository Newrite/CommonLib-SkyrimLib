use crate::offsets::offsets_rtti::RTTI_TitleSequenceMenu;
use crate::offsets::offsets_vtable::VTABLE_TitleSequenceMenu;
use crate::relocation::{RttiType, VariantID};

// C++ `RE::TitleSequenceMenu` minimal menu marker.
//
// TODO: SOURCE - replace this opaque menu marker with the real
// `IMenu`-derived layout from `TitleSequenceMenu.h/.cpp` when fields, bases,
// or methods are needed; the current stub only preserves RTTI/VTABLE/MENU_NAME
// for menu lookup APIs.
core_util::abstract_type! { pub type TitleSequenceMenu; }

impl RttiType for TitleSequenceMenu {
    const RTTI: VariantID = RTTI_TitleSequenceMenu;
}

impl TitleSequenceMenu {
    pub const RTTI: VariantID = RTTI_TitleSequenceMenu;
    pub const VTABLE: &'static [VariantID] = &VTABLE_TitleSequenceMenu;
    pub const MENU_NAME: &'static str = "TitleSequence Menu";
}
