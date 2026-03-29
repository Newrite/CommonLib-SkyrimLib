use crate::offsets::offsets_rtti::RTTI_SafeZoneMenu;
use crate::offsets::offsets_vtable::VTABLE_SafeZoneMenu;
use crate::relocation::{RttiType, VariantID};

// C++ `RE::SafeZoneMenu` minimal menu marker.
//
// TODO: SOURCE - replace this opaque menu marker with the real
// `IMenu`-derived layout from `SafeZoneMenu.h/.cpp` when fields, bases, or
// methods are needed; the current stub only preserves RTTI/VTABLE/MENU_NAME
// for menu lookup APIs.
core_util::abstract_type! { pub type SafeZoneMenu; }

impl RttiType for SafeZoneMenu {
    const RTTI: VariantID = RTTI_SafeZoneMenu;
}

impl SafeZoneMenu {
    pub const RTTI: VariantID = RTTI_SafeZoneMenu;
    pub const VTABLE: &'static [VariantID] = &VTABLE_SafeZoneMenu;
    pub const MENU_NAME: &'static str = "SafeZoneMenu";
}
