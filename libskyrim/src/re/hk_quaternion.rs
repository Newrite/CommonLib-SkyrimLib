use crate::re::hkVector4;

/// C++ `RE::hkQuaternion`
#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct hkQuaternion {
    pub vec: hkVector4, // 00
}

const _: () = assert!(core::mem::size_of::<hkQuaternion>() == 0x10);
const _: () = assert!(core::mem::offset_of!(hkQuaternion, vec) == 0x00);
