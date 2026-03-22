#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, bytemuck::Zeroable)]
pub struct NiBound {
    pub pad: [u8; 0x10],
}

const _: () = assert!(core::mem::size_of::<NiBound>() == 0x10);
