use crate::offsets::offsets_rtti::RTTI_LockpickingMenu;
use crate::offsets::offsets_vtable::VTABLE_LockpickingMenu;
use crate::relocation::{RttiType, VariantID};

// C++ `RE::LockpickingMenu` minimal menu marker.
//
// TODO: SOURCE - replace this opaque menu marker with the real
// `IMenu`-derived layout from `LockpickingMenu.h/.cpp` when fields, bases, or
// methods are needed; the current stub only preserves RTTI/VTABLE/MENU_NAME
// for menu lookup APIs.
core_util::abstract_type! { pub type LockpickingMenu; }

impl RttiType for LockpickingMenu {
    const RTTI: VariantID = RTTI_LockpickingMenu;
}

impl LockpickingMenu {
    pub const RTTI: VariantID = RTTI_LockpickingMenu;
    pub const VTABLE: &'static [VariantID] = &VTABLE_LockpickingMenu;
    pub const MENU_NAME: &'static str = "Lockpicking Menu";
}
