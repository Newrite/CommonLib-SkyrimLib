use crate::offsets::offsets_rtti::RTTI_TweenMenu;
use crate::offsets::offsets_vtable::VTABLE_TweenMenu;
use crate::relocation::{RttiType, VariantID};

// C++ `RE::TweenMenu` minimal menu marker.
//
// TODO: SOURCE - replace this opaque menu marker with the real `IMenu`-derived
// layout from `TweenMenu.h/.cpp` when fields, bases, or methods are needed;
// the current stub only preserves RTTI/VTABLE/MENU_NAME for menu lookup APIs.
core_util::abstract_type! { pub type TweenMenu; }

impl RttiType for TweenMenu {
    const RTTI: VariantID = RTTI_TweenMenu;
}

impl TweenMenu {
    pub const RTTI: VariantID = RTTI_TweenMenu;
    pub const VTABLE: &'static [VariantID] = &VTABLE_TweenMenu;
    pub const MENU_NAME: &'static str = "TweenMenu";
}
