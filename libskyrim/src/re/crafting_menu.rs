use crate::offsets::offsets_rtti::RTTI_CraftingMenu;
use crate::offsets::offsets_vtable::VTABLE_CraftingMenu;
use crate::relocation::{RttiType, VariantID};

// C++ `RE::CraftingMenu` minimal menu marker.
//
// TODO: SOURCE - replace this opaque menu marker with the real `IMenu`-derived
// layout from `CraftingMenu.h/.cpp` when fields, bases, or methods are
// needed; the current stub only preserves RTTI/VTABLE/MENU_NAME for menu
// lookup APIs.
core_util::abstract_type! { pub type CraftingMenu; }

impl RttiType for CraftingMenu {
    const RTTI: VariantID = RTTI_CraftingMenu;
}

impl CraftingMenu {
    pub const RTTI: VariantID = RTTI_CraftingMenu;
    pub const VTABLE: &'static [VariantID] = &VTABLE_CraftingMenu;
    pub const MENU_NAME: &'static str = "Crafting Menu";
}
