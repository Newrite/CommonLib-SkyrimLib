#![allow(non_camel_case_types)]

use core::ffi::c_char;

/// C++ `RE::hkStringPtr`
#[repr(C)]
pub struct hkStringPtr {
    pub data: *const c_char, // 00
}

const _: () = assert!(core::mem::size_of::<hkStringPtr>() == 0x8);
