use crate::offsets::offsets_rtti::RTTI_LoadingMenu;
use crate::offsets::offsets_vtable::VTABLE_LoadingMenu;
use crate::relocation::{RttiType, VariantID};

// C++ `RE::LoadingMenu` minimal menu marker.
//
// TODO: SOURCE - replace this opaque menu marker with the real
// `IMenu`-derived layout from `LoadingMenu.h/.cpp` when fields, bases, or
// methods are needed; the current stub only preserves RTTI/VTABLE/MENU_NAME
// for menu lookup APIs.
core_util::abstract_type! { pub type LoadingMenu; }

impl RttiType for LoadingMenu {
    const RTTI: VariantID = RTTI_LoadingMenu;
}

impl LoadingMenu {
    pub const RTTI: VariantID = RTTI_LoadingMenu;
    pub const VTABLE: &'static [VariantID] = &VTABLE_LoadingMenu;
    pub const MENU_NAME: &'static str = "Loading Menu";
}
