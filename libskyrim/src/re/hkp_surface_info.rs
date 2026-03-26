#![allow(non_camel_case_types)]

use core_util::EnumSet;

use crate::re::hkVector4;

/// C++ `RE::hkpSurfaceInfo::SupportedState`
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum hkpSurfaceInfoSupportedState {
    Unsupported = 0,
    Sliding = 1,
    Supported = 2,
}

core_util::impl_enumset_type!(hkpSurfaceInfoSupportedState => u32);

/// C++ `RE::hkpSurfaceInfo`
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct hkpSurfaceInfo {
    pub supported_state: EnumSet<hkpSurfaceInfoSupportedState, u32>, // 00
    pub pad04: u32,                                                  // 04
    pub pad08: u64,                                                  // 08
    pub surface_normal: hkVector4,                                   // 10
    pub surface_velocity: hkVector4,                                 // 20
    pub surface_distance_excess: f32,                                // 30
    pub surface_is_dynamic: bool,                                    // 34
    pub pad35: u8,                                                   // 35
    pub pad36: u16,                                                  // 36
    pub pad38: u64,                                                  // 38
}

const _: () = assert!(core::mem::size_of::<hkpSurfaceInfo>() == 0x40);
const _: () = assert!(core::mem::offset_of!(hkpSurfaceInfo, supported_state) == 0x00);
const _: () = assert!(core::mem::offset_of!(hkpSurfaceInfo, surface_normal) == 0x10);
const _: () = assert!(core::mem::offset_of!(hkpSurfaceInfo, surface_velocity) == 0x20);
const _: () = assert!(core::mem::offset_of!(hkpSurfaceInfo, surface_distance_excess) == 0x30);
const _: () = assert!(core::mem::offset_of!(hkpSurfaceInfo, surface_is_dynamic) == 0x34);
