use crate::offsets::offsets_rtti::RTTI_TrainingMenu;
use crate::offsets::offsets_vtable::VTABLE_TrainingMenu;
use crate::relocation::{RttiType, VariantID};

// C++ `RE::TrainingMenu` minimal menu marker.
//
// TODO: SOURCE - replace this opaque menu marker with the real
// `IMenu`-derived layout from `TrainingMenu.h/.cpp` when fields, bases, or
// methods are needed; the current stub only preserves RTTI/VTABLE/MENU_NAME
// for menu lookup APIs.
core_util::abstract_type! { pub type TrainingMenu; }

impl RttiType for TrainingMenu {
    const RTTI: VariantID = RTTI_TrainingMenu;
}

impl TrainingMenu {
    pub const RTTI: VariantID = RTTI_TrainingMenu;
    pub const VTABLE: &'static [VariantID] = &VTABLE_TrainingMenu;
    pub const MENU_NAME: &'static str = "Training Menu";
}
