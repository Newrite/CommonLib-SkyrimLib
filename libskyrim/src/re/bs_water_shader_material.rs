use core_util::inherit;

use crate::offsets::offsets_rtti::RTTI_BSWaterShaderMaterial;
use crate::offsets::offsets_vtable::VTABLE_BSWaterShaderMaterial;
use crate::re::{
    BSShaderMaterial, DepthProperties, NiColor, NiColorA, NiPlane, NiPointer, NiSourceTexture,
};
use crate::relocation::{RttiType, VariantID};

/// C++ `RE::BSWaterShaderMaterial`
#[repr(C)]
pub struct BSWaterShaderMaterial {
    pub base: BSShaderMaterial,                                // 000
    pub static_reflection_texture: NiPointer<NiSourceTexture>, // 038
    pub normal_texture1: NiPointer<NiSourceTexture>,           // 040
    pub normal_texture2: NiPointer<NiSourceTexture>,           // 048
    pub normal_texture3: NiPointer<NiSourceTexture>,           // 050
    pub normal_texture4: NiPointer<NiSourceTexture>,           // 058
    pub shallow_water_color: NiColor,                          // 060
    pub sun_sparkle_power: f32,                                // 06C
    pub deep_water_color: NiColorA,                            // 070
    pub reflection_color: NiColorA,                            // 080
    pub sun_specular_power: f32,                               // 090
    pub reflection_amount: f32,                                // 094
    pub alpha: f32,                                            // 098
    pub refraction_magnitude: f32,                             // 09C
    pub unk0a0: u64,                                           // 0A0
    pub unk0a8: f32,                                           // 0A8
    pub pad0ac: u32,                                           // 0AC
    pub unk0b0: u64,                                           // 0B0
    pub unk0b8: u64,                                           // 0B8
    pub unk0c0: u64,                                           // 0C0
    pub specular_power: f32,                                   // 0C8
    pub pad0cc: u32,                                           // 0CC
    pub unk0d0: u64,                                           // 0D0
    pub unk0d8: u64,                                           // 0D8
    pub noise_falloff: f32,                                    // 0E0
    pub reflection_magnitude: f32,                             // 0E4
    pub sun_sparkle_magnitude: f32,                            // 0E8
    pub unk0ec: f32,                                           // 0EC
    pub depth_properties: DepthProperties,                     // 0F0
    pub unk100: u64,                                           // 100
    pub unk108: u64,                                           // 108
    pub unk110: u64,                                           // 110
    pub unk118: u64,                                           // 118
    pub uv_scale_a: [f32; 3],                                  // 120
    pub unk12c: u32,                                           // 12C
    pub amplitude_a: [f32; 3],                                 // 130
    pub displacement_dampener: f32,                            // 13C
    pub plane: NiPlane,                                        // 140
    pub unk150: u32,                                           // 150
    pub flowmap_scale: f32,                                    // 154
    pub above_water_fog_dist_far: f32,                         // 158
    pub unk15c: f32,                                           // 15C
    pub unk160: f32,                                           // 160
    pub underwater_fog_dist_far: f32,                          // 164
    pub unk168: f32,                                           // 168
    pub underwater_fog_amount: f32,                            // 16C
    pub fresnel_amount: f32,                                   // 170
    pub unk174: u32,                                           // 174
    pub unk178: u32,                                           // 178
    pub unk17c: u8,                                            // 17C
    pub unk17d: u8,                                            // 17D
    pub unk17e: u8,                                            // 17E
    pub unk17f: u8,                                            // 17F
}

const _: () = assert!(core::mem::size_of::<BSWaterShaderMaterial>() == 0x180);
const _: () =
    assert!(core::mem::offset_of!(BSWaterShaderMaterial, static_reflection_texture) == 0x38);
const _: () = assert!(core::mem::offset_of!(BSWaterShaderMaterial, depth_properties) == 0xF0);
const _: () = assert!(core::mem::offset_of!(BSWaterShaderMaterial, plane) == 0x140);

impl RttiType for BSWaterShaderMaterial {
    const RTTI: VariantID = RTTI_BSWaterShaderMaterial;
}

inherit!(BSWaterShaderMaterial : BSShaderMaterial, base);

impl BSWaterShaderMaterial {
    pub const RTTI: VariantID = RTTI_BSWaterShaderMaterial;
    pub const VTABLE: &'static [VariantID] = &VTABLE_BSWaterShaderMaterial;
}
