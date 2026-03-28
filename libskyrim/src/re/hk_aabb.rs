#![allow(non_camel_case_types)]

use crate::re::hkVector4;

/// C++ `RE::hkAabb`
#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct hkAabb {
    pub min: hkVector4, // 00
    pub max: hkVector4, // 10
}

const _: () = assert!(core::mem::size_of::<hkAabb>() == 0x20);
const _: () = assert!(core::mem::offset_of!(hkAabb, min) == 0x00);
const _: () = assert!(core::mem::offset_of!(hkAabb, max) == 0x10);
