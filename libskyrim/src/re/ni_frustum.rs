/// C++ `RE::NiFrustum`
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct NiFrustum {
    pub f_left: f32,    // 00
    pub f_right: f32,   // 04
    pub f_top: f32,     // 08
    pub f_bottom: f32,  // 0C
    pub f_near: f32,    // 10
    pub f_far: f32,     // 14
    pub b_ortho: bool,  // 18
    pub pad19: [u8; 3], // 19
}

const _: () = assert!(core::mem::size_of::<NiFrustum>() == 0x1C);
const _: () = assert!(core::mem::offset_of!(NiFrustum, f_left) == 0x00);
const _: () = assert!(core::mem::offset_of!(NiFrustum, f_right) == 0x04);
const _: () = assert!(core::mem::offset_of!(NiFrustum, f_top) == 0x08);
const _: () = assert!(core::mem::offset_of!(NiFrustum, f_bottom) == 0x0C);
const _: () = assert!(core::mem::offset_of!(NiFrustum, f_near) == 0x10);
const _: () = assert!(core::mem::offset_of!(NiFrustum, f_far) == 0x14);
const _: () = assert!(core::mem::offset_of!(NiFrustum, b_ortho) == 0x18);
