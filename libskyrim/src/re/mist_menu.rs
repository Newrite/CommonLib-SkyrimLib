use crate::offsets::offsets_rtti::RTTI_MistMenu;
use crate::offsets::offsets_vtable::VTABLE_MistMenu;
use crate::relocation::{RttiType, VariantID};

// C++ `RE::MistMenu` minimal menu marker.
//
// TODO: SOURCE - replace this opaque menu marker with the real `IMenu`-derived
// layout from `MistMenu.h/.cpp` when fields, bases, or methods are needed; the
// current stub only preserves RTTI/VTABLE/MENU_NAME for menu lookup APIs.
core_util::abstract_type! { pub type MistMenu; }

impl RttiType for MistMenu {
    const RTTI: VariantID = RTTI_MistMenu;
}

impl MistMenu {
    pub const RTTI: VariantID = RTTI_MistMenu;
    pub const VTABLE: &'static [VariantID] = &VTABLE_MistMenu;
    pub const MENU_NAME: &'static str = "Mist Menu";
}
