use crate::offsets::offsets_rtti::RTTI_RaceSexMenu;
use crate::offsets::offsets_vtable::VTABLE_RaceSexMenu;
use crate::relocation::{RttiType, VariantID};

// C++ `RE::RaceSexMenu` minimal menu marker.
//
// TODO: SOURCE - replace this opaque menu marker with the real
// `IMenu`-derived layout from `RaceSexMenu.h/.cpp` when fields, bases, or
// methods are needed; the current stub only preserves RTTI/VTABLE/MENU_NAME
// for menu lookup APIs.
core_util::abstract_type! { pub type RaceSexMenu; }

impl RttiType for RaceSexMenu {
    const RTTI: VariantID = RTTI_RaceSexMenu;
}

impl RaceSexMenu {
    pub const RTTI: VariantID = RTTI_RaceSexMenu;
    pub const VTABLE: &'static [VariantID] = &VTABLE_RaceSexMenu;
    pub const MENU_NAME: &'static str = "RaceSex Menu";
}
