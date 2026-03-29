use crate::offsets::offsets_rtti::RTTI_CreditsMenu;
use crate::offsets::offsets_vtable::VTABLE_CreditsMenu;
use crate::relocation::{RttiType, VariantID};

// C++ `RE::CreditsMenu` minimal menu marker.
//
// TODO: SOURCE - replace this opaque menu marker with the real `IMenu`-derived
// layout from `CreditsMenu.h/.cpp` when fields, bases, or methods are needed;
// the current stub only preserves RTTI/VTABLE/MENU_NAME for menu lookup APIs.
core_util::abstract_type! { pub type CreditsMenu; }

impl RttiType for CreditsMenu {
    const RTTI: VariantID = RTTI_CreditsMenu;
}

impl CreditsMenu {
    pub const RTTI: VariantID = RTTI_CreditsMenu;
    pub const VTABLE: &'static [VariantID] = &VTABLE_CreditsMenu;
    pub const MENU_NAME: &'static str = "Credits Menu";
}
