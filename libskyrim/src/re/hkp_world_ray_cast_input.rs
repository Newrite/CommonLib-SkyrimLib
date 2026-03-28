#![allow(non_camel_case_types)]

use crate::re::{CFilter, hkVector4};

/// C++ `RE::hkpWorldRayCastInput`
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct hkpWorldRayCastInput {
    pub from: hkVector4,                      // 00
    pub to: hkVector4,                        // 10
    pub enable_shape_collection_filter: bool, // 20
    pub pad21: [u8; 3],                       // 21
    pub filter_info: CFilter,                 // 24
    pub pad28: [u8; 8],                       // 28
}

const _: () = assert!(core::mem::size_of::<hkpWorldRayCastInput>() == 0x30);
const _: () = assert!(core::mem::offset_of!(hkpWorldRayCastInput, from) == 0x00);
const _: () = assert!(core::mem::offset_of!(hkpWorldRayCastInput, to) == 0x10);
const _: () =
    assert!(core::mem::offset_of!(hkpWorldRayCastInput, enable_shape_collection_filter) == 0x20);
const _: () = assert!(core::mem::offset_of!(hkpWorldRayCastInput, filter_info) == 0x24);

impl Default for hkpWorldRayCastInput {
    #[inline(always)]
    fn default() -> Self {
        Self {
            from: hkVector4::zero(),
            to: hkVector4::zero(),
            enable_shape_collection_filter: false,
            pad21: [0; 3],
            filter_info: CFilter::default(),
            pad28: [0; 8],
        }
    }
}
