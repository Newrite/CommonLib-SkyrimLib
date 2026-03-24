use crate::re::hkVector4;

/// C++ `RE::hkMatrix3`
#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct hkMatrix3 {
    pub col0: hkVector4, // 00
    pub col1: hkVector4, // 10
    pub col2: hkVector4, // 20
}

const _: () = assert!(core::mem::size_of::<hkMatrix3>() == 0x30);
const _: () = assert!(core::mem::offset_of!(hkMatrix3, col0) == 0x00);
const _: () = assert!(core::mem::offset_of!(hkMatrix3, col1) == 0x10);
const _: () = assert!(core::mem::offset_of!(hkMatrix3, col2) == 0x20);
