use crate::offsets::offsets_rtti::RTTI_MessageBoxMenu;
use crate::offsets::offsets_vtable::VTABLE_MessageBoxMenu;
use crate::relocation::{RttiType, VariantID};

// C++ `RE::MessageBoxMenu` minimal menu marker.
//
// TODO: SOURCE - replace this opaque menu marker with the real
// `IMenu`-derived layout from `MessageBoxMenu.h/.cpp` when fields, bases, or
// methods are needed; the current stub only preserves RTTI/VTABLE/MENU_NAME
// for menu lookup APIs.
core_util::abstract_type! { pub type MessageBoxMenu; }

impl RttiType for MessageBoxMenu {
    const RTTI: VariantID = RTTI_MessageBoxMenu;
}

impl MessageBoxMenu {
    pub const RTTI: VariantID = RTTI_MessageBoxMenu;
    pub const VTABLE: &'static [VariantID] = &VTABLE_MessageBoxMenu;
    pub const MENU_NAME: &'static str = "MessageBoxMenu";
}
