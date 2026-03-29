use crate::offsets::offsets_rtti::RTTI_GiftMenu;
use crate::offsets::offsets_vtable::VTABLE_GiftMenu;
use crate::relocation::{RttiType, VariantID};

// C++ `RE::GiftMenu` minimal menu marker.
//
// TODO: SOURCE - replace this opaque menu marker with the real `IMenu`-derived
// layout from `GiftMenu.h/.cpp` when fields, bases, or methods are needed;
// the current stub only preserves RTTI/VTABLE/MENU_NAME for menu lookup APIs.
core_util::abstract_type! { pub type GiftMenu; }

impl RttiType for GiftMenu {
    const RTTI: VariantID = RTTI_GiftMenu;
}

impl GiftMenu {
    pub const RTTI: VariantID = RTTI_GiftMenu;
    pub const VTABLE: &'static [VariantID] = &VTABLE_GiftMenu;
    pub const MENU_NAME: &'static str = "GiftMenu";
}
