use crate::offsets::offsets_rtti::RTTI_ConsoleNativeUIMenu;
use crate::offsets::offsets_vtable::VTABLE_ConsoleNativeUIMenu;
use crate::relocation::{RttiType, VariantID};

// C++ `RE::ConsoleNativeUIMenu` minimal menu marker.
//
// TODO: SOURCE - replace this opaque menu marker with the real
// `IMenu`-derived layout from `ConsoleNativeUIMenu.h/.cpp` when fields,
// bases, or methods are needed; the current stub only preserves
// RTTI/VTABLE/MENU_NAME for menu lookup APIs.
core_util::abstract_type! { pub type ConsoleNativeUIMenu; }

impl RttiType for ConsoleNativeUIMenu {
    const RTTI: VariantID = RTTI_ConsoleNativeUIMenu;
}

impl ConsoleNativeUIMenu {
    pub const RTTI: VariantID = RTTI_ConsoleNativeUIMenu;
    pub const VTABLE: &'static [VariantID] = &VTABLE_ConsoleNativeUIMenu;
    pub const MENU_NAME: &'static str = "Console Native UI Menu";
}
