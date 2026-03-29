use crate::offsets::offsets_rtti::RTTI_KinectMenu;
use crate::offsets::offsets_vtable::VTABLE_KinectMenu;
use crate::relocation::{RttiType, VariantID};

// C++ `RE::KinectMenu` minimal menu marker.
//
// TODO: SOURCE - replace this opaque menu marker with the real `IMenu`-derived
// layout from `KinectMenu.h/.cpp` when fields, bases, or methods are needed;
// the current stub only preserves RTTI/VTABLE/MENU_NAME for menu lookup APIs.
core_util::abstract_type! { pub type KinectMenu; }

impl RttiType for KinectMenu {
    const RTTI: VariantID = RTTI_KinectMenu;
}

impl KinectMenu {
    pub const RTTI: VariantID = RTTI_KinectMenu;
    pub const VTABLE: &'static [VariantID] = &VTABLE_KinectMenu;
    pub const MENU_NAME: &'static str = "Kinect Menu";
}
