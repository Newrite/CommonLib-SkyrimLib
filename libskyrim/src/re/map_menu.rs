use crate::offsets::offsets_rtti::RTTI_MapMenu;
use crate::offsets::offsets_vtable::VTABLE_MapMenu;
use crate::relocation::{RttiType, VariantID};

// C++ `RE::MapMenu` minimal menu marker.
//
// TODO: SOURCE - replace this opaque menu marker with the real `IMenu`-derived
// layout from `MapMenu.h/.cpp` when fields, bases, or methods are needed; the
// current stub only preserves RTTI/VTABLE/MENU_NAME for menu lookup APIs.
core_util::abstract_type! { pub type MapMenu; }

impl RttiType for MapMenu {
    const RTTI: VariantID = RTTI_MapMenu;
}

impl MapMenu {
    pub const RTTI: VariantID = RTTI_MapMenu;
    pub const VTABLE: &'static [VariantID] = &VTABLE_MapMenu;
    pub const MENU_NAME: &'static str = "MapMenu";
}
