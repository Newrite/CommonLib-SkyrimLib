use crate::offsets::offsets_rtti::RTTI_StatsMenu;
use crate::offsets::offsets_vtable::VTABLE_StatsMenu;
use crate::relocation::{RttiType, VariantID};

// C++ `RE::StatsMenu` minimal menu marker.
//
// TODO: SOURCE - replace this opaque menu marker with the real `IMenu`-derived
// layout from `StatsMenu.h/.cpp` when fields, bases, or methods are needed;
// the current stub only preserves RTTI/VTABLE/MENU_NAME for menu lookup APIs.
core_util::abstract_type! { pub type StatsMenu; }

impl RttiType for StatsMenu {
    const RTTI: VariantID = RTTI_StatsMenu;
}

impl StatsMenu {
    pub const RTTI: VariantID = RTTI_StatsMenu;
    pub const VTABLE: &'static [VariantID] = &VTABLE_StatsMenu;
    pub const MENU_NAME: &'static str = "StatsMenu";
}
