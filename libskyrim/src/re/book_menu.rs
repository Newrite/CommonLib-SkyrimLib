use crate::offsets::offsets_rtti::RTTI_BookMenu;
use crate::offsets::offsets_vtable::VTABLE_BookMenu;
use crate::relocation::{RttiType, VariantID};

// C++ `RE::BookMenu` minimal menu marker.
//
// TODO: SOURCE - replace this opaque menu marker with the real `IMenu`-derived
// layout from `BookMenu.h/.cpp` when fields, bases, or methods are needed;
// the current stub only preserves RTTI/VTABLE/MENU_NAME for menu lookup APIs.
core_util::abstract_type! { pub type BookMenu; }

impl RttiType for BookMenu {
    const RTTI: VariantID = RTTI_BookMenu;
}

impl BookMenu {
    pub const RTTI: VariantID = RTTI_BookMenu;
    pub const VTABLE: &'static [VariantID] = &VTABLE_BookMenu;
    pub const MENU_NAME: &'static str = "Book Menu";
}
