use crate::offsets::offsets_rtti::RTTI_Console;
use crate::offsets::offsets_vtable::VTABLE_Console;
use crate::relocation::{RttiType, VariantID};

// C++ `RE::Console` minimal menu marker.
//
// TODO: SOURCE - replace this opaque menu marker with the real `IMenu`-derived
// layout from `Console.h/.cpp` when fields, bases, or methods are needed; the
// current stub only preserves RTTI/VTABLE/MENU_NAME for menu lookup APIs.
core_util::abstract_type! { pub type Console; }

impl RttiType for Console {
    const RTTI: VariantID = RTTI_Console;
}

impl Console {
    pub const RTTI: VariantID = RTTI_Console;
    pub const VTABLE: &'static [VariantID] = &VTABLE_Console;
    pub const MENU_NAME: &'static str = "Console";
}
