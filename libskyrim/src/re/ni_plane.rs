use crate::re::NiPoint3;

/// C++ `RE::NiPlane`
#[repr(C)]
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct NiPlane {
    pub normal: NiPoint3, // 00
    pub constant: f32,    // 0C
}

const _: () = assert!(core::mem::size_of::<NiPlane>() == 0x10);
const _: () = assert!(core::mem::offset_of!(NiPlane, normal) == 0x00);
const _: () = assert!(core::mem::offset_of!(NiPlane, constant) == 0x0C);
