use core_util::EnumSet;

use crate::re::BGSDirectionalAmbientLightingColors;
use crate::re::Color;

/// C++ `RE::INTERIOR_DATA::Inherit`
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum InteriorDataInherit {
    AmbientColor = 1 << 0,
    DirectionalColor = 1 << 1,
    FogColor = 1 << 2,
    FogNear = 1 << 3,
    FogFar = 1 << 4,
    DirectionalRotation = 1 << 5,
    DirectionalFade = 1 << 6,
    ClipDistance = 1 << 7,
    FogPower = 1 << 8,
    FogMax = 1 << 9,
    LightFadeDistances = 1 << 10,
}

core_util::impl_enumset_type!(InteriorDataInherit => u32);

/// C++ `RE::INTERIOR_DATA`
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct INTERIOR_DATA {
    pub ambient: Color,                                                           // 00
    pub directional: Color,                                                       // 04
    pub fog_color_near: Color,                                                    // 08
    pub fog_near: f32,                                                            // 0C
    pub fog_far: f32,                                                             // 10
    pub directional_xy: u32,                                                      // 14
    pub directional_z: u32,                                                       // 18
    pub directional_fade: f32,                                                    // 1C
    pub clip_dist: f32,                                                           // 20
    pub fog_power: f32,                                                           // 24
    pub directional_ambient_lighting_colors: BGSDirectionalAmbientLightingColors, // 28
    pub fog_color_far: Color,                                                     // 48
    pub fog_clamp: f32,                                                           // 4C
    pub light_fade_start: f32,                                                    // 50
    pub light_fade_end: f32,                                                      // 54
    pub lighting_template_inheritance_flags: EnumSet<InteriorDataInherit, u32>,   // 58
    pub unk5c: u32,                                                               // 5C
}

const _: () = assert!(core::mem::size_of::<INTERIOR_DATA>() == 0x60);
const _: () = assert!(core::mem::offset_of!(INTERIOR_DATA, ambient) == 0x00);
const _: () = assert!(core::mem::offset_of!(INTERIOR_DATA, directional) == 0x04);
const _: () = assert!(core::mem::offset_of!(INTERIOR_DATA, fog_color_near) == 0x08);
const _: () = assert!(core::mem::offset_of!(INTERIOR_DATA, fog_near) == 0x0C);
const _: () = assert!(core::mem::offset_of!(INTERIOR_DATA, fog_far) == 0x10);
const _: () = assert!(core::mem::offset_of!(INTERIOR_DATA, directional_xy) == 0x14);
const _: () = assert!(core::mem::offset_of!(INTERIOR_DATA, directional_z) == 0x18);
const _: () = assert!(core::mem::offset_of!(INTERIOR_DATA, directional_fade) == 0x1C);
const _: () = assert!(core::mem::offset_of!(INTERIOR_DATA, clip_dist) == 0x20);
const _: () = assert!(core::mem::offset_of!(INTERIOR_DATA, fog_power) == 0x24);
const _: () =
    assert!(core::mem::offset_of!(INTERIOR_DATA, directional_ambient_lighting_colors) == 0x28);
const _: () = assert!(core::mem::offset_of!(INTERIOR_DATA, fog_color_far) == 0x48);
const _: () = assert!(core::mem::offset_of!(INTERIOR_DATA, fog_clamp) == 0x4C);
const _: () = assert!(core::mem::offset_of!(INTERIOR_DATA, light_fade_start) == 0x50);
const _: () = assert!(core::mem::offset_of!(INTERIOR_DATA, light_fade_end) == 0x54);
const _: () =
    assert!(core::mem::offset_of!(INTERIOR_DATA, lighting_template_inheritance_flags) == 0x58);
const _: () = assert!(core::mem::offset_of!(INTERIOR_DATA, unk5c) == 0x5C);
