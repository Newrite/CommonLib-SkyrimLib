use crate::offsets::offsets_rtti::RTTI_ContainerMenu;
use crate::offsets::offsets_vtable::VTABLE_ContainerMenu;
use crate::relocation::{RttiType, VariantID};

// C++ `RE::ContainerMenu` minimal menu marker.
//
// TODO: SOURCE - replace this opaque menu marker with the real
// `IMenu`-derived layout from `ContainerMenu.h/.cpp` when fields, bases, or
// methods are needed; the current stub only preserves RTTI/VTABLE/MENU_NAME
// for menu lookup APIs.
core_util::abstract_type! { pub type ContainerMenu; }

impl RttiType for ContainerMenu {
    const RTTI: VariantID = RTTI_ContainerMenu;
}

impl ContainerMenu {
    pub const RTTI: VariantID = RTTI_ContainerMenu;
    pub const VTABLE: &'static [VariantID] = &VTABLE_ContainerMenu;
    pub const MENU_NAME: &'static str = "ContainerMenu";
}
