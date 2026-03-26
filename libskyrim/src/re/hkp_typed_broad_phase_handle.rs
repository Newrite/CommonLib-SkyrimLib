#![allow(non_camel_case_types)]

use crate::re::{CFilter, hkpBroadPhaseHandle};

/// C++ `RE::hkpTypedBroadPhaseHandle`
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct hkpTypedBroadPhaseHandle {
    pub base: hkpBroadPhaseHandle,      // 00
    pub r#type: i8,                     // 04
    pub owner_offset: i8,               // 05
    pub object_quality_type: i8,        // 06
    pub pad7: i8,                       // 07
    pub collision_filter_info: CFilter, // 08
}

const _: () = assert!(core::mem::size_of::<hkpTypedBroadPhaseHandle>() == 0x0C);
