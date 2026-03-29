#![allow(non_camel_case_types)]

use crate::re::GASGlobalContext;

/// C++ `RE::GASStringContext`
#[repr(C)]
pub struct GASStringContext {
    pub global_context: *mut GASGlobalContext, // 00
    pub version: u8,                           // 08
    pub pad09: [u8; 7],                        // 09
    pub unk10: u64,                            // 10
    pub unk18: u64,                            // 18
    pub unk20: u64,                            // 20
    pub unk28: u64,                            // 28
}

const _: () = assert!(core::mem::size_of::<GASStringContext>() == 0x30);
const _: () = assert!(core::mem::offset_of!(GASStringContext, global_context) == 0x0);
const _: () = assert!(core::mem::offset_of!(GASStringContext, version) == 0x8);
