use crate::offsets::offsets_rtti::RTTI_SleepWaitMenu;
use crate::offsets::offsets_vtable::VTABLE_SleepWaitMenu;
use crate::relocation::{RttiType, VariantID};

// C++ `RE::SleepWaitMenu` minimal menu marker.
//
// TODO: SOURCE - replace this opaque menu marker with the real
// `IMenu`-derived layout from `SleepWaitMenu.h/.cpp` when fields, bases, or
// methods are needed; the current stub only preserves RTTI/VTABLE/MENU_NAME
// for menu lookup APIs.
core_util::abstract_type! { pub type SleepWaitMenu; }

impl RttiType for SleepWaitMenu {
    const RTTI: VariantID = RTTI_SleepWaitMenu;
}

impl SleepWaitMenu {
    pub const RTTI: VariantID = RTTI_SleepWaitMenu;
    pub const VTABLE: &'static [VariantID] = &VTABLE_SleepWaitMenu;
    pub const MENU_NAME: &'static str = "Sleep/Wait Menu";
}
