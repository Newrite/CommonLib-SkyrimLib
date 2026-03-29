use crate::offsets::offsets_rtti::RTTI_HUDMenu;
use crate::offsets::offsets_vtable::VTABLE_HUDMenu;
use crate::relocation::{RttiType, VariantID};

// C++ `RE::HUDMenu` minimal menu marker.
//
// TODO: SOURCE - replace this opaque menu marker with the real `IMenu`-derived
// layout from `HUDMenu.h/.cpp` when fields, bases, or methods are needed; the
// current stub only preserves RTTI/VTABLE/MENU_NAME for menu lookup APIs.
core_util::abstract_type! { pub type HUDMenu; }

impl RttiType for HUDMenu {
    const RTTI: VariantID = RTTI_HUDMenu;
}

impl HUDMenu {
    pub const RTTI: VariantID = RTTI_HUDMenu;
    pub const VTABLE: &'static [VariantID] = &VTABLE_HUDMenu;
    pub const MENU_NAME: &'static str = "HUD Menu";
}
