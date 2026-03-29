use crate::offsets::offsets_rtti::RTTI_MagicMenu;
use crate::offsets::offsets_vtable::VTABLE_MagicMenu;
use crate::relocation::{RttiType, VariantID};

// C++ `RE::MagicMenu` minimal menu marker.
//
// TODO: SOURCE - replace this opaque menu marker with the real `IMenu`-derived
// layout from `MagicMenu.h/.cpp` when fields, bases, or methods are needed;
// the current stub only preserves RTTI/VTABLE/MENU_NAME for menu lookup APIs.
core_util::abstract_type! { pub type MagicMenu; }

impl RttiType for MagicMenu {
    const RTTI: VariantID = RTTI_MagicMenu;
}

impl MagicMenu {
    pub const RTTI: VariantID = RTTI_MagicMenu;
    pub const VTABLE: &'static [VariantID] = &VTABLE_MagicMenu;
    pub const MENU_NAME: &'static str = "MagicMenu";
}
