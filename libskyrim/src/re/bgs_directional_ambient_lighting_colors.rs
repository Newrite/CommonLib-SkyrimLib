use crate::re::Color;

/// C++ `RE::BGSDirectionalAmbientLightingColors::Directional::MaxMin<T>`
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct BGSDirectionalAmbientLightingColorsMaxMin<T> {
    pub max: T, // 00
    pub min: T, // 04
}

const _: () =
    assert!(core::mem::size_of::<BGSDirectionalAmbientLightingColorsMaxMin<Color>>() == 0x8);

/// C++ `RE::BGSDirectionalAmbientLightingColors::Directional`
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct BGSDirectionalAmbientLightingColorsDirectional {
    pub x: BGSDirectionalAmbientLightingColorsMaxMin<Color>, // 00
    pub y: BGSDirectionalAmbientLightingColorsMaxMin<Color>, // 08
    pub z: BGSDirectionalAmbientLightingColorsMaxMin<Color>, // 10
}

const _: () =
    assert!(core::mem::size_of::<BGSDirectionalAmbientLightingColorsDirectional>() == 0x18);

/// C++ `RE::BGSDirectionalAmbientLightingColors`
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct BGSDirectionalAmbientLightingColors {
    pub directional: BGSDirectionalAmbientLightingColorsDirectional, // 00
    pub specular: Color,                                             // 18
    pub fresnel_power: f32,                                          // 1C
}

const _: () = assert!(core::mem::size_of::<BGSDirectionalAmbientLightingColors>() == 0x20);
const _: () =
    assert!(core::mem::offset_of!(BGSDirectionalAmbientLightingColors, directional) == 0x00);
const _: () = assert!(core::mem::offset_of!(BGSDirectionalAmbientLightingColors, specular) == 0x18);
const _: () =
    assert!(core::mem::offset_of!(BGSDirectionalAmbientLightingColors, fresnel_power) == 0x1C);
