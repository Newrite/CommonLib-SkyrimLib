use crate::offsets::offsets_rtti::RTTI_FavoritesMenu;
use crate::offsets::offsets_vtable::VTABLE_FavoritesMenu;
use crate::relocation::{RttiType, VariantID};

// C++ `RE::FavoritesMenu` minimal menu marker.
//
// TODO: SOURCE - replace this opaque menu marker with the real
// `IMenu`-derived layout from `FavoritesMenu.h/.cpp` when fields, bases, or
// methods are needed; the current stub only preserves RTTI/VTABLE/MENU_NAME
// for menu lookup APIs.
core_util::abstract_type! { pub type FavoritesMenu; }

impl RttiType for FavoritesMenu {
    const RTTI: VariantID = RTTI_FavoritesMenu;
}

impl FavoritesMenu {
    pub const RTTI: VariantID = RTTI_FavoritesMenu;
    pub const VTABLE: &'static [VariantID] = &VTABLE_FavoritesMenu;
    pub const MENU_NAME: &'static str = "FavoritesMenu";
}
