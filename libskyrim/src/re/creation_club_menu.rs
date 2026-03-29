use crate::offsets::offsets_rtti::RTTI_CreationClubMenu;
use crate::offsets::offsets_vtable::VTABLE_CreationClubMenu;
use crate::relocation::{RttiType, VariantID};

// C++ `RE::CreationClubMenu` minimal menu marker.
//
// TODO: SOURCE - replace this opaque menu marker with the real
// `IMenu`-derived layout from `CreationClubMenu.h/.cpp` when fields, bases,
// or methods are needed; the current stub only preserves RTTI/VTABLE/MENU_NAME
// for menu lookup APIs.
core_util::abstract_type! { pub type CreationClubMenu; }

impl RttiType for CreationClubMenu {
    const RTTI: VariantID = RTTI_CreationClubMenu;
}

impl CreationClubMenu {
    pub const RTTI: VariantID = RTTI_CreationClubMenu;
    pub const VTABLE: &'static [VariantID] = &VTABLE_CreationClubMenu;
    pub const MENU_NAME: &'static str = "Creation Club Menu";
}
