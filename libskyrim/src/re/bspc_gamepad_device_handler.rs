use crate::offsets::offsets_rtti::RTTI_BSPCGamepadDeviceHandler;
use crate::offsets::offsets_vtable::VTABLE_BSPCGamepadDeviceHandler;
use crate::re::{BSIInputDevice, BSPCGamepadDeviceDelegate};
use crate::relocation::{RelocationID, RttiType, VariantID, VariantOffset};

/// C++ `RE::BSPCGamepadDeviceHandler::RUNTIME_DATA`
#[repr(C)]
pub struct BSPCGamepadDeviceHandlerRuntimeData {
    pub current_pc_game_pad_delegate: *mut BSPCGamepadDeviceDelegate, // 00
}

const _: () = assert!(core::mem::size_of::<BSPCGamepadDeviceHandlerRuntimeData>() == 0x8);

/// Honest common prefix of C++ `RE::BSPCGamepadDeviceHandler`.
#[repr(C)]
pub struct BSPCGamepadDeviceHandler {
    pub base: BSIInputDevice, // 00
}

const _: () = assert!(core::mem::size_of::<BSPCGamepadDeviceHandler>() == 0x8);
const _: () = assert!(core::mem::offset_of!(BSPCGamepadDeviceHandler, base) == 0x00);

impl RttiType for BSPCGamepadDeviceHandler {
    const RTTI: VariantID = RTTI_BSPCGamepadDeviceHandler;
}

core_util::inherit!(BSPCGamepadDeviceHandler : BSIInputDevice, base);

impl BSPCGamepadDeviceHandler {
    pub const RTTI: VariantID = RTTI_BSPCGamepadDeviceHandler;
    pub const VTABLE: &'static [VariantID] = &VTABLE_BSPCGamepadDeviceHandler;
    pub const RUNTIME_DATA_OFFSET: VariantOffset = VariantOffset::new(0x8, 0x8, 0x10);

    crate::runtime_data_accessor! {
        pub fn runtime_data() -> BSPCGamepadDeviceHandlerRuntimeData {
            offset: Self::RUNTIME_DATA_OFFSET
        }
    }

    crate::runtime_data_mut_accessor! {
        pub fn runtime_data_mut() -> BSPCGamepadDeviceHandlerRuntimeData {
            offset: Self::RUNTIME_DATA_OFFSET
        }
    }

    crate::relocation_func! {
        pub fn initialize_delegate(&mut self) => RelocationID::new(67457, 68762)
    }
}
