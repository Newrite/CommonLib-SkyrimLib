#![allow(non_camel_case_types)]

use crate::re::hkTime;

/// C++ `RE::hkStepInfo`
#[repr(C, align(16))]
#[derive(Debug, Clone, Copy, Default)]
pub struct hkStepInfo {
    pub start_time: hkTime,  // 00
    pub end_time: hkTime,    // 04
    pub delta_time: f32,     // 08
    pub inv_delta_time: f32, // 0C
}

const _: () = assert!(core::mem::size_of::<hkStepInfo>() == 0x10);
const _: () = assert!(core::mem::offset_of!(hkStepInfo, start_time) == 0x00);
const _: () = assert!(core::mem::offset_of!(hkStepInfo, end_time) == 0x04);
const _: () = assert!(core::mem::offset_of!(hkStepInfo, delta_time) == 0x08);
const _: () = assert!(core::mem::offset_of!(hkStepInfo, inv_delta_time) == 0x0C);
