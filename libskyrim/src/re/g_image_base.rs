#![allow(non_camel_case_types)]

use core_util::EnumSet;

use crate::re::{GArray, GColor};

/// C++ `RE::GImageBase::ImageFormat`
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GImageBaseImageFormat {
    kNone = 0,
    kARGB_8888 = 1,
    kRGB_888 = 2,
    kL_8 = 8,
    kA_8 = 9,
    kDXT1 = 10,
    kDXT3 = 11,
    kDXT5 = 12,
    kP_8 = 100,
    kYUV_822 = 200,
    kYUVA_8228 = 201,
}

core_util::impl_enumset_type!(GImageBaseImageFormat => u32);

/// C++ `RE::GImageBase`
#[repr(C)]
pub struct GImageBase {
    pub format: EnumSet<GImageBaseImageFormat, u32>, // 00
    pub width: u32,                                  // 04
    pub height: u32,                                 // 08
    pub pitch: u32,                                  // 0C
    pub data: *mut u8,                               // 10
    pub data_size: u32,                              // 18
    pub mip_map_count: u32,                          // 1C
    pub color_map: GArray<GColor>,                   // 20
}

const _: () = assert!(core::mem::size_of::<GImageBase>() == 0x38);
const _: () = assert!(core::mem::offset_of!(GImageBase, format) == 0x0);
const _: () = assert!(core::mem::offset_of!(GImageBase, width) == 0x4);
const _: () = assert!(core::mem::offset_of!(GImageBase, height) == 0x8);
const _: () = assert!(core::mem::offset_of!(GImageBase, pitch) == 0xC);
const _: () = assert!(core::mem::offset_of!(GImageBase, data) == 0x10);
const _: () = assert!(core::mem::offset_of!(GImageBase, data_size) == 0x18);
const _: () = assert!(core::mem::offset_of!(GImageBase, mip_map_count) == 0x1C);
const _: () = assert!(core::mem::offset_of!(GImageBase, color_map) == 0x20);
