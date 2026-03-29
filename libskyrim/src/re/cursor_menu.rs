use crate::offsets::offsets_rtti::RTTI_CursorMenu;
use crate::offsets::offsets_vtable::VTABLE_CursorMenu;
use crate::relocation::{RttiType, VariantID};

// C++ `RE::CursorMenu` minimal menu marker.
//
// TODO: SOURCE - replace this opaque menu marker with the real `IMenu`-derived
// layout from `CursorMenu.h/.cpp` when fields, bases, or methods are needed;
// the current stub only preserves RTTI/VTABLE/MENU_NAME for menu lookup APIs.
core_util::abstract_type! { pub type CursorMenu; }

impl RttiType for CursorMenu {
    const RTTI: VariantID = RTTI_CursorMenu;
}

impl CursorMenu {
    pub const RTTI: VariantID = RTTI_CursorMenu;
    pub const VTABLE: &'static [VariantID] = &VTABLE_CursorMenu;
    pub const MENU_NAME: &'static str = "Cursor Menu";
}
