use crate::offsets::offsets_rtti::RTTI_ModManagerMenu;
use crate::offsets::offsets_vtable::VTABLE_ModManagerMenu;
use crate::relocation::{RttiType, VariantID};

// C++ `RE::ModManagerMenu` minimal menu marker.
//
// TODO: SOURCE - replace this opaque menu marker with the real
// `IMenu`-derived layout from `ModManagerMenu.h/.cpp` when fields, bases, or
// methods are needed; the current stub only preserves RTTI/VTABLE/MENU_NAME
// for menu lookup APIs.
core_util::abstract_type! { pub type ModManagerMenu; }

impl RttiType for ModManagerMenu {
    const RTTI: VariantID = RTTI_ModManagerMenu;
}

impl ModManagerMenu {
    pub const RTTI: VariantID = RTTI_ModManagerMenu;
    pub const VTABLE: &'static [VariantID] = &VTABLE_ModManagerMenu;
    pub const MENU_NAME: &'static str = "Mod Manager Menu";
}
