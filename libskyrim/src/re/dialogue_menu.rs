use crate::offsets::offsets_rtti::RTTI_DialogueMenu;
use crate::offsets::offsets_vtable::VTABLE_DialogueMenu;
use crate::relocation::{RttiType, VariantID};

// C++ `RE::DialogueMenu` minimal menu marker.
//
// TODO: SOURCE - replace this opaque menu marker with the real
// `IMenu`-derived layout from `DialogueMenu.h/.cpp` when fields, bases, or
// methods are needed; the current stub only preserves RTTI/VTABLE/MENU_NAME
// for menu lookup APIs.
core_util::abstract_type! { pub type DialogueMenu; }

impl RttiType for DialogueMenu {
    const RTTI: VariantID = RTTI_DialogueMenu;
}

impl DialogueMenu {
    pub const RTTI: VariantID = RTTI_DialogueMenu;
    pub const VTABLE: &'static [VariantID] = &VTABLE_DialogueMenu;
    pub const MENU_NAME: &'static str = "Dialogue Menu";
}
