use crate::re::Color;
use core_util::EnumSet;

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DecalDataFlag {
    None = 0,
    Parallax = 1 << 0,
    AlphaBlending = 1 << 1,
    AlphaTesting = 1 << 2,
    NoSubtextures = 1 << 3,
}

core_util::impl_enumset_type!(DecalDataFlag => u8);

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DecalDataData {
    pub decal_min_width: f32,              // 0x00
    pub decal_max_width: f32,              // 0x04
    pub decal_min_height: f32,             // 0x08
    pub decal_max_height: f32,             // 0x0C
    pub depth: f32,                        // 0x10
    pub shininess: f32,                    // 0x14
    pub parallax_scale: f32,               // 0x18
    pub parallax_passes: i8,               // 0x1C
    pub flags: EnumSet<DecalDataFlag, u8>, // 0x1D
    pub pad1e: u16,                        // 0x1E
    pub color: Color,                      // 0x20
}

const _: () = assert!(core::mem::size_of::<DecalDataData>() == 0x24);
const _: () = assert!(core::mem::offset_of!(DecalDataData, decal_min_width) == 0x00);
const _: () = assert!(core::mem::offset_of!(DecalDataData, decal_max_width) == 0x04);
const _: () = assert!(core::mem::offset_of!(DecalDataData, decal_min_height) == 0x08);
const _: () = assert!(core::mem::offset_of!(DecalDataData, decal_max_height) == 0x0C);
const _: () = assert!(core::mem::offset_of!(DecalDataData, depth) == 0x10);
const _: () = assert!(core::mem::offset_of!(DecalDataData, shininess) == 0x14);
const _: () = assert!(core::mem::offset_of!(DecalDataData, parallax_scale) == 0x18);
const _: () = assert!(core::mem::offset_of!(DecalDataData, parallax_passes) == 0x1C);
const _: () = assert!(core::mem::offset_of!(DecalDataData, flags) == 0x1D);
const _: () = assert!(core::mem::offset_of!(DecalDataData, pad1e) == 0x1E);
const _: () = assert!(core::mem::offset_of!(DecalDataData, color) == 0x20);

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DecalData {
    pub data: DecalDataData, // 0x00
}

const _: () = assert!(core::mem::size_of::<DecalData>() == 0x24);
const _: () = assert!(core::mem::offset_of!(DecalData, data) == 0x00);
