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

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DecalData {
    pub data: DecalDataData, // 0x00
}

const _: () = assert!(core::mem::size_of::<DecalData>() == 0x24);
