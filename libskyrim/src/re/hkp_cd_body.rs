#![allow(non_camel_case_types)]

use crate::re::hkpShape;

/// C++ `RE::hkpCdBody`
#[repr(C)]
pub struct hkpCdBody {
    pub shape: *const hkpShape,           // 00
    pub shape_key: u32,                   // 08
    pub pad0c: u32,                       // 0C
    pub motion: *const core::ffi::c_void, // 10
    pub parent: *const hkpCdBody,         // 18
}

const _: () = assert!(core::mem::size_of::<hkpCdBody>() == 0x20);
