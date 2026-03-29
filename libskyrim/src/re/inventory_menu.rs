use crate::offsets::offsets_rtti::RTTI_InventoryMenu;
use crate::offsets::offsets_vtable::VTABLE_InventoryMenu;
use crate::relocation::{RttiType, VariantID};

// C++ `RE::InventoryMenu` minimal menu marker.
//
// TODO: SOURCE - replace this opaque menu marker with the real
// `IMenu`-derived layout from `InventoryMenu.h/.cpp` when fields, bases, or
// methods are needed; the current stub only preserves RTTI/VTABLE/MENU_NAME
// for menu lookup APIs.
core_util::abstract_type! { pub type InventoryMenu; }

impl RttiType for InventoryMenu {
    const RTTI: VariantID = RTTI_InventoryMenu;
}

impl InventoryMenu {
    pub const RTTI: VariantID = RTTI_InventoryMenu;
    pub const VTABLE: &'static [VariantID] = &VTABLE_InventoryMenu;
    pub const MENU_NAME: &'static str = "InventoryMenu";
}
