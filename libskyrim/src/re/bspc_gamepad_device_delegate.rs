use crate::offsets::offsets_rtti::RTTI_BSPCGamepadDeviceDelegate;
use crate::offsets::offsets_vtable::VTABLE_BSPCGamepadDeviceDelegate;
use crate::re::{BSGamepadDevice, BSPCGamepadDeviceHandler};
use crate::relocation::{RttiType, VariantID};

/// C++ `RE::BSPCGamepadDeviceDelegate`
#[repr(C)]
pub struct BSPCGamepadDeviceDelegate {
    pub base: BSGamepadDevice,                                 // 00
    pub gamepad_device_handler: *mut BSPCGamepadDeviceHandler, // D0
}

const _: () = assert!(core::mem::size_of::<BSPCGamepadDeviceDelegate>() == 0xD8);
const _: () =
    assert!(core::mem::offset_of!(BSPCGamepadDeviceDelegate, gamepad_device_handler) == 0xD0);

impl RttiType for BSPCGamepadDeviceDelegate {
    const RTTI: VariantID = RTTI_BSPCGamepadDeviceDelegate;
}

core_util::inherit!(BSPCGamepadDeviceDelegate : BSGamepadDevice, base);

impl BSPCGamepadDeviceDelegate {
    pub const RTTI: VariantID = RTTI_BSPCGamepadDeviceDelegate;
    pub const VTABLE: &'static [VariantID] = &VTABLE_BSPCGamepadDeviceDelegate;
}
