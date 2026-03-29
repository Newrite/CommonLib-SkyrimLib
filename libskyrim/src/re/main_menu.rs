use crate::offsets::offsets_rtti::RTTI_MainMenu;
use crate::offsets::offsets_vtable::VTABLE_MainMenu;
use crate::relocation::{RttiType, VariantID};

// C++ `RE::MainMenu` minimal menu marker.
//
// TODO: SOURCE - replace this opaque menu marker with the real `IMenu`-derived
// layout from `MainMenu.h/.cpp` when fields, bases, or methods are needed; the
// current stub only preserves RTTI/VTABLE/MENU_NAME for menu lookup APIs.
core_util::abstract_type! { pub type MainMenu; }

impl RttiType for MainMenu {
    const RTTI: VariantID = RTTI_MainMenu;
}

impl MainMenu {
    pub const RTTI: VariantID = RTTI_MainMenu;
    pub const VTABLE: &'static [VariantID] = &VTABLE_MainMenu;
    pub const MENU_NAME: &'static str = "Main Menu";
}
