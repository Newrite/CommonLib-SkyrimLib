use core_util::EnumSet;
use core_util::inherit;

use crate::offsets::offsets_rtti::RTTI_BSEffectShaderMaterial;
use crate::offsets::offsets_vtable::VTABLE_BSEffectShaderMaterial;
use crate::re::{
    BSFixedString, BSGraphicsTextureAddressMode, BSShaderMaterial, NiColorA, NiPointer,
    NiSourceTexture,
};
use crate::relocation::{RttiType, VariantID};

/// C++ `RE::BSEffectShaderMaterial`
#[repr(C)]
pub struct BSEffectShaderMaterial {
    pub base: BSShaderMaterial,                                       // 00
    pub falloff_start_angle: f32,                                     // 38
    pub falloff_stop_angle: f32,                                      // 3C
    pub falloff_start_opacity: f32,                                   // 40
    pub falloff_stop_opacity: f32,                                    // 44
    pub base_color: NiColorA,                                         // 48
    pub source_texture: NiPointer<NiSourceTexture>,                   // 58
    pub greyscale_texture: NiPointer<NiSourceTexture>,                // 60
    pub soft_falloff_depth: f32,                                      // 68
    pub base_color_scale: f32,                                        // 6C
    pub source_texture_path: BSFixedString,                           // 70
    pub greyscale_texture_path: BSFixedString,                        // 78
    pub effect_clamp_mode: EnumSet<BSGraphicsTextureAddressMode, u8>, // 80
    pub unk81: u8,                                                    // 81
}

const _: () = assert!(core::mem::size_of::<BSEffectShaderMaterial>() == 0x88);
const _: () = assert!(core::mem::offset_of!(BSEffectShaderMaterial, base_color) == 0x48);
const _: () = assert!(core::mem::offset_of!(BSEffectShaderMaterial, effect_clamp_mode) == 0x80);

impl RttiType for BSEffectShaderMaterial {
    const RTTI: VariantID = RTTI_BSEffectShaderMaterial;
}

inherit!(BSEffectShaderMaterial : BSShaderMaterial, base);

impl BSEffectShaderMaterial {
    pub const RTTI: VariantID = RTTI_BSEffectShaderMaterial;
    pub const VTABLE: &'static [VariantID] = &VTABLE_BSEffectShaderMaterial;
}
