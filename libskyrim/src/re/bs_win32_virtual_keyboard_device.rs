use crate::offsets::offsets_rtti::RTTI_BSWin32VirtualKeyboardDevice;
use crate::offsets::offsets_vtable::VTABLE_BSWin32VirtualKeyboardDevice;
use crate::re::BSVirtualKeyboardDevice;
use crate::relocation::{RttiType, VariantID};

/// C++ `RE::BSWin32VirtualKeyboardDevice`
#[repr(C)]
pub struct BSWin32VirtualKeyboardDevice {
    pub base: BSVirtualKeyboardDevice, // 00
}

const _: () = assert!(core::mem::size_of::<BSWin32VirtualKeyboardDevice>() == 0x70);
const _: () = assert!(core::mem::offset_of!(BSWin32VirtualKeyboardDevice, base) == 0x00);

impl RttiType for BSWin32VirtualKeyboardDevice {
    const RTTI: VariantID = RTTI_BSWin32VirtualKeyboardDevice;
}

core_util::inherit!(BSWin32VirtualKeyboardDevice : BSVirtualKeyboardDevice, base);

impl BSWin32VirtualKeyboardDevice {
    pub const RTTI: VariantID = RTTI_BSWin32VirtualKeyboardDevice;
    pub const VTABLE: &'static [VariantID] = &VTABLE_BSWin32VirtualKeyboardDevice;
}
