use crate::offsets::offsets_rtti::RTTI_BarterMenu;
use crate::offsets::offsets_vtable::VTABLE_BarterMenu;
use crate::relocation::{RttiType, VariantID};

// C++ `RE::BarterMenu` minimal menu marker.
//
// TODO: SOURCE - replace this opaque menu marker with the real `IMenu`-derived
// layout from `BarterMenu.h/.cpp` when fields, bases, or methods are needed;
// the current stub only preserves RTTI/VTABLE/MENU_NAME for menu lookup APIs.
core_util::abstract_type! { pub type BarterMenu; }

impl RttiType for BarterMenu {
    const RTTI: VariantID = RTTI_BarterMenu;
}

impl BarterMenu {
    pub const RTTI: VariantID = RTTI_BarterMenu;
    pub const VTABLE: &'static [VariantID] = &VTABLE_BarterMenu;
    pub const MENU_NAME: &'static str = "BarterMenu";
}
