use crate::re::{hkRotation, hkVector4};

/// C++ `RE::hkTransform`
#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct hkTransform {
    pub rotation: hkRotation,   // 00
    pub translation: hkVector4, // 30
}

const _: () = assert!(core::mem::size_of::<hkTransform>() == 0x40);
const _: () = assert!(core::mem::offset_of!(hkTransform, rotation) == 0x00);
const _: () = assert!(core::mem::offset_of!(hkTransform, translation) == 0x30);
