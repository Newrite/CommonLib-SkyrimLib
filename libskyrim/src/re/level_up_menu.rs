use crate::offsets::offsets_rtti::RTTI_LevelUpMenu;
use crate::offsets::offsets_vtable::VTABLE_LevelUpMenu;
use crate::relocation::{RttiType, VariantID};

// C++ `RE::LevelUpMenu` minimal menu marker.
//
// TODO: SOURCE - replace this opaque menu marker with the real
// `IMenu`-derived layout from `LevelUpMenu.h/.cpp` when fields, bases, or
// methods are needed; the current stub only preserves RTTI/VTABLE/MENU_NAME
// for menu lookup APIs.
core_util::abstract_type! { pub type LevelUpMenu; }

impl RttiType for LevelUpMenu {
    const RTTI: VariantID = RTTI_LevelUpMenu;
}

impl LevelUpMenu {
    pub const RTTI: VariantID = RTTI_LevelUpMenu;
    pub const VTABLE: &'static [VariantID] = &VTABLE_LevelUpMenu;
    pub const MENU_NAME: &'static str = "LevelUp Menu";
}
