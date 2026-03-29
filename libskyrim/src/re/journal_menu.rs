use crate::offsets::offsets_rtti::RTTI_JournalMenu;
use crate::offsets::offsets_vtable::VTABLE_JournalMenu;
use crate::relocation::{RttiType, VariantID};

// C++ `RE::JournalMenu` minimal menu marker.
//
// TODO: SOURCE - replace this opaque menu marker with the real
// `IMenu`-derived layout from `JournalMenu.h/.cpp` when fields, bases, or
// methods are needed; the current stub only preserves RTTI/VTABLE/MENU_NAME
// for menu lookup APIs.
core_util::abstract_type! { pub type JournalMenu; }

impl RttiType for JournalMenu {
    const RTTI: VariantID = RTTI_JournalMenu;
}

impl JournalMenu {
    pub const RTTI: VariantID = RTTI_JournalMenu;
    pub const VTABLE: &'static [VariantID] = &VTABLE_JournalMenu;
    pub const MENU_NAME: &'static str = "Journal Menu";
}
