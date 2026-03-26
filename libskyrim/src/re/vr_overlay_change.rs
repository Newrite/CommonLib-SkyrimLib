/// C++ `RE::VROverlayChange`
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct VROverlayChange {
    pub unk00: u8, // 00
}

const _: () = assert!(core::mem::size_of::<VROverlayChange>() == 0x1);
