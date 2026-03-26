#![allow(non_camel_case_types)]

use core_util::{EnumSet, inherit};

use crate::offsets::offsets_rtti::RTTI_DeviceConnectEvent;
use crate::offsets::offsets_vtable::VTABLE_DeviceConnectEvent;
use crate::re::{INPUT_DEVICE, InputEvent};
use crate::relocation::{RttiType, VariantID};

/// C++ `RE::DeviceConnectEvent`
#[repr(C)]
pub struct DeviceConnectEvent {
    pub base: InputEvent, // 00
    pub connected: bool,  // 18
    pub pad19: u8,        // 19
    pub pad1a: u16,       // 1A
    pub pad1c: u32,       // 1C
}

const _: () = assert!(core::mem::size_of::<DeviceConnectEvent>() == 0x20);
const _: () = assert!(core::mem::offset_of!(DeviceConnectEvent, base) == 0x00);
const _: () = assert!(core::mem::offset_of!(DeviceConnectEvent, connected) == 0x18);

impl RttiType for DeviceConnectEvent {
    const RTTI: VariantID = RTTI_DeviceConnectEvent;
}

inherit!(DeviceConnectEvent : InputEvent, base);

impl DeviceConnectEvent {
    pub const RTTI: VariantID = RTTI_DeviceConnectEvent;
    pub const VTABLE: &'static [VariantID] = &VTABLE_DeviceConnectEvent;

    #[inline(always)]
    pub fn init(&mut self, device: INPUT_DEVICE, connected: bool) {
        self.base.device = EnumSet::from_underlying(device as u32);
        self.connected = connected;
    }
}
