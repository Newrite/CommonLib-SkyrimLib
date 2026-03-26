/// C++ `RE::VRDeviceConnectionChange`
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct VRDeviceConnectionChange {
    pub unk00: bool,    // 00
    pub pad01: [u8; 3], // 01
    pub unk04: u32,     // 04
}

const _: () = assert!(core::mem::size_of::<VRDeviceConnectionChange>() == 0x8);
const _: () = assert!(core::mem::offset_of!(VRDeviceConnectionChange, unk00) == 0x0);
const _: () = assert!(core::mem::offset_of!(VRDeviceConnectionChange, unk04) == 0x4);
