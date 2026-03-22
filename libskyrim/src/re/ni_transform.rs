#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct NiTransform {
    pub pad: [u8; 0x34],
}

unsafe impl bytemuck::Zeroable for NiTransform {}

const _: () = assert!(core::mem::size_of::<NiTransform>() == 0x34);
