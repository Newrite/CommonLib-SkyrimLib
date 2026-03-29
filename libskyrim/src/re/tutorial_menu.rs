use crate::offsets::offsets_rtti::RTTI_TutorialMenu;
use crate::offsets::offsets_vtable::VTABLE_TutorialMenu;
use crate::relocation::{RttiType, VariantID};

// C++ `RE::TutorialMenu` minimal menu marker.
//
// TODO: SOURCE - replace this opaque menu marker with the real
// `IMenu`-derived layout from `TutorialMenu.h/.cpp` when fields, bases, or
// methods are needed; the current stub only preserves RTTI/VTABLE/MENU_NAME
// for menu lookup APIs.
core_util::abstract_type! { pub type TutorialMenu; }

impl RttiType for TutorialMenu {
    const RTTI: VariantID = RTTI_TutorialMenu;
}

impl TutorialMenu {
    pub const RTTI: VariantID = RTTI_TutorialMenu;
    pub const VTABLE: &'static [VariantID] = &VTABLE_TutorialMenu;
    pub const MENU_NAME: &'static str = "Tutorial Menu";
}
