use bitflags::bitflags;
use core_util::{EnumSet, inherit};

use crate::offsets::offsets_rtti::RTTI_TESEffectShader;
use crate::offsets::offsets_vtable::VTABLE_TESEffectShader;
use crate::re::BGSDebris;
use crate::re::BGSSoundDescriptorForm;
use crate::re::Color;
use crate::re::FormCastable;
use crate::re::FormType;
use crate::re::TESForm;
use crate::re::TESTexture;
use crate::relocation::{RttiType, VariantID};
use crate::rex::{D3DBLEND, D3DBLENDOP, D3DCMPFUNC};

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EffectShaderDataFlags {
    None = 0,
    DisableTextureShader = 1 << 0,
    GreyscaleToColor = 1 << 1,
    GreyscaleToAlpha = 1 << 2,
    DisableParticleShader = 1 << 3,
    EdgeColorSubtractive = 1 << 4,
    SkinOnly = 1 << 5,
    IgnoreTexAlpha = 1 << 6,
    FillTexProjectedUv = 1 << 7,
    IgnoreBaseGeomTexAlpha = 1 << 8,
    Lighting = 1 << 9,
    IgnoreWeapons = 1 << 10,
    Alpha = 1 << 11,
    PreferDismemberedLimb = 1 << 12,
    ParticleAnimated = 1 << 15,
    ParticleGreyscaleColor = 1 << 16,
    ParticleGreyscaleAlpha = 1 << 17,
    UseBloodGeometry = 1 << 24,
}

core_util::impl_enumset_type!(EffectShaderDataFlags => u32);

bitflags! {
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct EffectShaderRecordFlags: u32 {
        const NONE = 0;
        const DELETED = 1 << 5;
        const IGNORED = 1 << 12;
    }
}

unsafe impl bytemuck::Zeroable for EffectShaderRecordFlags {}

#[repr(C)]
pub struct EffectShaderData {
    pub unk00: u32, // 0x000

    pub membrane_shader_source_blend_mode: D3DBLEND, // 0x004
    pub membrane_shader_blend_operation: D3DBLENDOP, // 0x008
    pub membrane_shader_z_test_function: D3DCMPFUNC, // 0x00C

    pub fill_texture_effect_color_key1: Color, // 0x010
    pub fill_texture_effect_alpha_fade_in_time: f32, // 0x014
    pub fill_texture_effect_full_alpha_time: f32, // 0x018
    pub fill_texture_effect_alpha_fade_out_time: f32, // 0x01C
    pub fill_texture_effect_persistent_alpha_ratio: f32, // 0x020
    pub fill_texture_effect_alpha_pulse_amplitude: f32, // 0x024
    pub fill_texture_effect_alpha_pulse_frequency: f32, // 0x028
    pub fill_texture_effect_texture_animation_speed_u: f32, // 0x02C
    pub fill_texture_effect_texture_animation_speed_v: f32, // 0x030

    pub edge_effect_fall_off: f32,               // 0x034
    pub edge_effect_color: Color,                // 0x038
    pub edge_effect_alpha_fade_in_time: f32,     // 0x03C
    pub edge_effect_full_alpha_time: f32,        // 0x040
    pub edge_effect_alpha_fade_out_time: f32,    // 0x044
    pub edge_effect_persistent_alpha_ratio: f32, // 0x048
    pub edge_effect_alpha_pulse_amplitude: f32,  // 0x04C
    pub edge_effect_alpha_pulse_frequency: f32,  // 0x050

    pub fill_texture_effect_full_alpha_ratio: f32, // 0x054
    pub edge_effect_full_alpha_ratio: f32,         // 0x058

    pub membrane_shader_dest_blend_mode: D3DBLEND, // 0x05C

    pub particle_shader_source_blend_mode: D3DBLEND, // 0x060
    pub particle_shader_blend_operation: D3DBLENDOP, // 0x064
    pub particle_shader_z_test_operation: D3DCMPFUNC, // 0x068
    pub particle_shader_dest_blend_mode: D3DBLEND,   // 0x06C
    pub particle_shader_particle_birth_ramp_up_time: f32, // 0x070
    pub particle_shader_full_particle_birth_time: f32, // 0x074
    pub particle_shader_particle_birth_ramp_down_time: f32, // 0x078
    pub particle_shader_full_particle_birth_ratio: f32, // 0x07C
    pub particle_shader_persistant_particle_count: f32, // 0x080
    pub particle_shader_particle_lifetime: f32,      // 0x084
    pub particle_shader_particle_lifetime_variance: f32, // 0x088
    pub particle_shader_initial_speed_along_normal: f32, // 0x08C
    pub particle_shader_acceleration_along_normal: f32, // 0x090
    pub particle_shader_initial_velocity1: f32,      // 0x094
    pub particle_shader_initial_velocity2: f32,      // 0x098
    pub particle_shader_initial_velocity3: f32,      // 0x09C
    pub particle_shader_acceleration1: f32,          // 0x0A0
    pub particle_shader_acceleration2: f32,          // 0x0A4
    pub particle_shader_acceleration3: f32,          // 0x0A8
    pub particle_shader_scale_key1: f32,             // 0x0AC
    pub particle_shader_scale_key2: f32,             // 0x0B0
    pub particle_shader_scale_key1_time: f32,        // 0x0B4
    pub particle_shader_scale_key2_time: f32,        // 0x0B8

    pub color_key1: Color,              // 0x0BC
    pub color_key2: Color,              // 0x0C0
    pub color_key3: Color,              // 0x0C4
    pub color_key1_color_alpha: f32,    // 0x0C8
    pub color_key2_color_alpha: f32,    // 0x0CC
    pub color_key3_color_alpha: f32,    // 0x0D0
    pub color_key1_color_key_time: f32, // 0x0D4
    pub color_key2_color_key_time: f32, // 0x0D8
    pub color_key3_color_key_time: f32, // 0x0DC

    pub particle_shader_initial_speed_along_normal_variance: f32, // 0x0E0
    pub particle_shader_initial_rotation: f32,                    // 0x0E4
    pub particle_shader_initial_rotation_variance: f32,           // 0x0E8
    pub particle_shader_rotation_speed: f32,                      // 0x0EC
    pub particle_shader_rotation_speed_variance: f32,             // 0x0F0

    pub pad0f4: u32, // 0x0F4

    pub addon_models: *mut BGSDebris, // 0x0F8

    pub holes_start_time: f32, // 0x100
    pub holes_end_time: f32,   // 0x104
    pub holes_start_val: f32,  // 0x108
    pub holes_end_val: f32,    // 0x10C

    pub edge_width_alpha_units: f32, // 0x110
    pub edge_color: Color,           // 0x114

    pub explosion_wind_speed: f32, // 0x118
    pub texture_count_u: f32,      // 0x11C
    pub texture_count_v: f32,      // 0x120

    pub addon_models_fade_in_time: f32,   // 0x124
    pub addon_models_fade_out_time: f32,  // 0x128
    pub addon_models_scale_start: f32,    // 0x12C
    pub addon_models_scale_end: f32,      // 0x130
    pub addon_models_scale_in_time: f32,  // 0x134
    pub addon_models_scale_out_time: f32, // 0x138

    pub pad13c: u32, // 0x13C

    pub ambient_sound: *mut BGSSoundDescriptorForm, // 0x140

    pub fill_texture_effect_color_key2: Color, // 0x148
    pub fill_texture_effect_color_key3: Color, // 0x14C
    pub fill_texture_effect_color_key_scale_time_color_key1_scale: f32, // 0x150
    pub fill_texture_effect_color_key_scale_time_color_key2_scale: f32, // 0x154
    pub fill_texture_effect_color_key_scale_time_color_key3_scale: f32, // 0x158
    pub fill_texture_effect_color_key_scale_time_color_key1_time: f32, // 0x15C
    pub fill_texture_effect_color_key_scale_time_color_key2_time: f32, // 0x160
    pub fill_texture_effect_color_key_scale_time_color_key3_time: f32, // 0x164

    pub color_scale: f32, // 0x168

    pub birth_position_offset: f32,          // 0x16C
    pub birth_position_offset_variance: f32, // 0x170

    pub particle_shader_animated_start_frame: f32, // 0x174
    pub particle_shader_animated_start_frame_variance: f32, // 0x178
    pub particle_shader_animated_end_frame: f32,   // 0x17C
    pub particle_shader_animated_loop_start_frame: f32, // 0x180
    pub particle_shader_animated_loop_start_variance: f32, // 0x184
    pub particle_shader_animated_frame_count: f32, // 0x188
    pub particle_shader_animated_frame_count_variance: f32, // 0x18C

    pub flags: EnumSet<EffectShaderDataFlags, u32>, // 0x190

    pub fill_texture_effect_texture_scale_u: f32, // 0x194
    pub fill_texture_effect_texture_scale_v: f32, // 0x198

    pub scene_graph_emit_depth_limit: u32, // 0x19C
}

const _: () = assert!(core::mem::size_of::<EffectShaderData>() == 0x1A0);
const _: () = assert!(core::mem::offset_of!(EffectShaderData, unk00) == 0x000);
const _: () =
    assert!(core::mem::offset_of!(EffectShaderData, membrane_shader_source_blend_mode) == 0x004);
const _: () =
    assert!(core::mem::offset_of!(EffectShaderData, membrane_shader_blend_operation) == 0x008);
const _: () =
    assert!(core::mem::offset_of!(EffectShaderData, membrane_shader_z_test_function) == 0x00C);
const _: () =
    assert!(core::mem::offset_of!(EffectShaderData, fill_texture_effect_color_key1) == 0x010);
const _: () = assert!(
    core::mem::offset_of!(EffectShaderData, fill_texture_effect_alpha_fade_in_time) == 0x014
);
const _: () =
    assert!(core::mem::offset_of!(EffectShaderData, fill_texture_effect_full_alpha_time) == 0x018);
const _: () = assert!(
    core::mem::offset_of!(EffectShaderData, fill_texture_effect_alpha_fade_out_time) == 0x01C
);
const _: () = assert!(
    core::mem::offset_of!(EffectShaderData, fill_texture_effect_persistent_alpha_ratio) == 0x020
);
const _: () = assert!(
    core::mem::offset_of!(EffectShaderData, fill_texture_effect_alpha_pulse_amplitude) == 0x024
);
const _: () = assert!(
    core::mem::offset_of!(EffectShaderData, fill_texture_effect_alpha_pulse_frequency) == 0x028
);
const _: () = assert!(
    core::mem::offset_of!(
        EffectShaderData,
        fill_texture_effect_texture_animation_speed_u
    ) == 0x02C
);
const _: () = assert!(
    core::mem::offset_of!(
        EffectShaderData,
        fill_texture_effect_texture_animation_speed_v
    ) == 0x030
);
const _: () = assert!(core::mem::offset_of!(EffectShaderData, edge_effect_fall_off) == 0x034);
const _: () = assert!(core::mem::offset_of!(EffectShaderData, edge_effect_color) == 0x038);
const _: () =
    assert!(core::mem::offset_of!(EffectShaderData, edge_effect_alpha_fade_in_time) == 0x03C);
const _: () =
    assert!(core::mem::offset_of!(EffectShaderData, edge_effect_full_alpha_time) == 0x040);
const _: () =
    assert!(core::mem::offset_of!(EffectShaderData, edge_effect_alpha_fade_out_time) == 0x044);
const _: () =
    assert!(core::mem::offset_of!(EffectShaderData, edge_effect_persistent_alpha_ratio) == 0x048);
const _: () =
    assert!(core::mem::offset_of!(EffectShaderData, edge_effect_alpha_pulse_amplitude) == 0x04C);
const _: () =
    assert!(core::mem::offset_of!(EffectShaderData, edge_effect_alpha_pulse_frequency) == 0x050);
const _: () =
    assert!(core::mem::offset_of!(EffectShaderData, fill_texture_effect_full_alpha_ratio) == 0x054);
const _: () =
    assert!(core::mem::offset_of!(EffectShaderData, edge_effect_full_alpha_ratio) == 0x058);
const _: () =
    assert!(core::mem::offset_of!(EffectShaderData, membrane_shader_dest_blend_mode) == 0x05C);
const _: () =
    assert!(core::mem::offset_of!(EffectShaderData, particle_shader_source_blend_mode) == 0x060);
const _: () =
    assert!(core::mem::offset_of!(EffectShaderData, particle_shader_blend_operation) == 0x064);
const _: () =
    assert!(core::mem::offset_of!(EffectShaderData, particle_shader_z_test_operation) == 0x068);
const _: () =
    assert!(core::mem::offset_of!(EffectShaderData, particle_shader_dest_blend_mode) == 0x06C);
const _: () = assert!(
    core::mem::offset_of!(
        EffectShaderData,
        particle_shader_particle_birth_ramp_up_time
    ) == 0x070
);
const _: () = assert!(
    core::mem::offset_of!(EffectShaderData, particle_shader_full_particle_birth_time) == 0x074
);
const _: () = assert!(
    core::mem::offset_of!(
        EffectShaderData,
        particle_shader_particle_birth_ramp_down_time
    ) == 0x078
);
const _: () = assert!(
    core::mem::offset_of!(EffectShaderData, particle_shader_full_particle_birth_ratio) == 0x07C
);
const _: () = assert!(
    core::mem::offset_of!(EffectShaderData, particle_shader_persistant_particle_count) == 0x080
);
const _: () =
    assert!(core::mem::offset_of!(EffectShaderData, particle_shader_particle_lifetime) == 0x084);
const _: () = assert!(
    core::mem::offset_of!(EffectShaderData, particle_shader_particle_lifetime_variance) == 0x088
);
const _: () = assert!(
    core::mem::offset_of!(EffectShaderData, particle_shader_initial_speed_along_normal) == 0x08C
);
const _: () = assert!(
    core::mem::offset_of!(EffectShaderData, particle_shader_acceleration_along_normal) == 0x090
);
const _: () =
    assert!(core::mem::offset_of!(EffectShaderData, particle_shader_initial_velocity1) == 0x094);
const _: () =
    assert!(core::mem::offset_of!(EffectShaderData, particle_shader_initial_velocity2) == 0x098);
const _: () =
    assert!(core::mem::offset_of!(EffectShaderData, particle_shader_initial_velocity3) == 0x09C);
const _: () =
    assert!(core::mem::offset_of!(EffectShaderData, particle_shader_acceleration1) == 0x0A0);
const _: () =
    assert!(core::mem::offset_of!(EffectShaderData, particle_shader_acceleration2) == 0x0A4);
const _: () =
    assert!(core::mem::offset_of!(EffectShaderData, particle_shader_acceleration3) == 0x0A8);
const _: () = assert!(core::mem::offset_of!(EffectShaderData, particle_shader_scale_key1) == 0x0AC);
const _: () = assert!(core::mem::offset_of!(EffectShaderData, particle_shader_scale_key2) == 0x0B0);
const _: () =
    assert!(core::mem::offset_of!(EffectShaderData, particle_shader_scale_key1_time) == 0x0B4);
const _: () =
    assert!(core::mem::offset_of!(EffectShaderData, particle_shader_scale_key2_time) == 0x0B8);
const _: () = assert!(core::mem::offset_of!(EffectShaderData, color_key1) == 0x0BC);
const _: () = assert!(core::mem::offset_of!(EffectShaderData, color_key2) == 0x0C0);
const _: () = assert!(core::mem::offset_of!(EffectShaderData, color_key3) == 0x0C4);
const _: () = assert!(core::mem::offset_of!(EffectShaderData, color_key1_color_alpha) == 0x0C8);
const _: () = assert!(core::mem::offset_of!(EffectShaderData, color_key2_color_alpha) == 0x0CC);
const _: () = assert!(core::mem::offset_of!(EffectShaderData, color_key3_color_alpha) == 0x0D0);
const _: () = assert!(core::mem::offset_of!(EffectShaderData, color_key1_color_key_time) == 0x0D4);
const _: () = assert!(core::mem::offset_of!(EffectShaderData, color_key2_color_key_time) == 0x0D8);
const _: () = assert!(core::mem::offset_of!(EffectShaderData, color_key3_color_key_time) == 0x0DC);
const _: () = assert!(
    core::mem::offset_of!(
        EffectShaderData,
        particle_shader_initial_speed_along_normal_variance
    ) == 0x0E0
);
const _: () =
    assert!(core::mem::offset_of!(EffectShaderData, particle_shader_initial_rotation) == 0x0E4);
const _: () = assert!(
    core::mem::offset_of!(EffectShaderData, particle_shader_initial_rotation_variance) == 0x0E8
);
const _: () =
    assert!(core::mem::offset_of!(EffectShaderData, particle_shader_rotation_speed) == 0x0EC);
const _: () = assert!(
    core::mem::offset_of!(EffectShaderData, particle_shader_rotation_speed_variance) == 0x0F0
);
const _: () = assert!(core::mem::offset_of!(EffectShaderData, pad0f4) == 0x0F4);
const _: () = assert!(core::mem::offset_of!(EffectShaderData, addon_models) == 0x0F8);
const _: () = assert!(core::mem::offset_of!(EffectShaderData, holes_start_time) == 0x100);
const _: () = assert!(core::mem::offset_of!(EffectShaderData, holes_end_time) == 0x104);
const _: () = assert!(core::mem::offset_of!(EffectShaderData, holes_start_val) == 0x108);
const _: () = assert!(core::mem::offset_of!(EffectShaderData, holes_end_val) == 0x10C);
const _: () = assert!(core::mem::offset_of!(EffectShaderData, edge_width_alpha_units) == 0x110);
const _: () = assert!(core::mem::offset_of!(EffectShaderData, edge_color) == 0x114);
const _: () = assert!(core::mem::offset_of!(EffectShaderData, explosion_wind_speed) == 0x118);
const _: () = assert!(core::mem::offset_of!(EffectShaderData, texture_count_u) == 0x11C);
const _: () = assert!(core::mem::offset_of!(EffectShaderData, texture_count_v) == 0x120);
const _: () = assert!(core::mem::offset_of!(EffectShaderData, addon_models_fade_in_time) == 0x124);
const _: () = assert!(core::mem::offset_of!(EffectShaderData, addon_models_fade_out_time) == 0x128);
const _: () = assert!(core::mem::offset_of!(EffectShaderData, addon_models_scale_start) == 0x12C);
const _: () = assert!(core::mem::offset_of!(EffectShaderData, addon_models_scale_end) == 0x130);
const _: () = assert!(core::mem::offset_of!(EffectShaderData, addon_models_scale_in_time) == 0x134);
const _: () =
    assert!(core::mem::offset_of!(EffectShaderData, addon_models_scale_out_time) == 0x138);
const _: () = assert!(core::mem::offset_of!(EffectShaderData, pad13c) == 0x13C);
const _: () = assert!(core::mem::offset_of!(EffectShaderData, ambient_sound) == 0x140);
const _: () =
    assert!(core::mem::offset_of!(EffectShaderData, fill_texture_effect_color_key2) == 0x148);
const _: () =
    assert!(core::mem::offset_of!(EffectShaderData, fill_texture_effect_color_key3) == 0x14C);
const _: () = assert!(
    core::mem::offset_of!(
        EffectShaderData,
        fill_texture_effect_color_key_scale_time_color_key1_scale
    ) == 0x150
);
const _: () = assert!(
    core::mem::offset_of!(
        EffectShaderData,
        fill_texture_effect_color_key_scale_time_color_key2_scale
    ) == 0x154
);
const _: () = assert!(
    core::mem::offset_of!(
        EffectShaderData,
        fill_texture_effect_color_key_scale_time_color_key3_scale
    ) == 0x158
);
const _: () = assert!(
    core::mem::offset_of!(
        EffectShaderData,
        fill_texture_effect_color_key_scale_time_color_key1_time
    ) == 0x15C
);
const _: () = assert!(
    core::mem::offset_of!(
        EffectShaderData,
        fill_texture_effect_color_key_scale_time_color_key2_time
    ) == 0x160
);
const _: () = assert!(
    core::mem::offset_of!(
        EffectShaderData,
        fill_texture_effect_color_key_scale_time_color_key3_time
    ) == 0x164
);
const _: () = assert!(core::mem::offset_of!(EffectShaderData, color_scale) == 0x168);
const _: () = assert!(core::mem::offset_of!(EffectShaderData, birth_position_offset) == 0x16C);
const _: () =
    assert!(core::mem::offset_of!(EffectShaderData, birth_position_offset_variance) == 0x170);
const _: () =
    assert!(core::mem::offset_of!(EffectShaderData, particle_shader_animated_start_frame) == 0x174);
const _: () = assert!(
    core::mem::offset_of!(
        EffectShaderData,
        particle_shader_animated_start_frame_variance
    ) == 0x178
);
const _: () =
    assert!(core::mem::offset_of!(EffectShaderData, particle_shader_animated_end_frame) == 0x17C);
const _: () = assert!(
    core::mem::offset_of!(EffectShaderData, particle_shader_animated_loop_start_frame) == 0x180
);
const _: () = assert!(
    core::mem::offset_of!(
        EffectShaderData,
        particle_shader_animated_loop_start_variance
    ) == 0x184
);
const _: () =
    assert!(core::mem::offset_of!(EffectShaderData, particle_shader_animated_frame_count) == 0x188);
const _: () = assert!(
    core::mem::offset_of!(
        EffectShaderData,
        particle_shader_animated_frame_count_variance
    ) == 0x18C
);
const _: () = assert!(core::mem::offset_of!(EffectShaderData, flags) == 0x190);
const _: () =
    assert!(core::mem::offset_of!(EffectShaderData, fill_texture_effect_texture_scale_u) == 0x194);
const _: () =
    assert!(core::mem::offset_of!(EffectShaderData, fill_texture_effect_texture_scale_v) == 0x198);
const _: () =
    assert!(core::mem::offset_of!(EffectShaderData, scene_graph_emit_depth_limit) == 0x19C);

#[repr(C)]
pub struct TESEffectShader {
    pub base: TESForm,                        // 0x000
    pub data: EffectShaderData,               // 0x020
    pub fill_texture: TESTexture,             // 0x1C0
    pub particle_shader_texture: TESTexture,  // 0x1D0
    pub holes_texture: TESTexture,            // 0x1E0
    pub membrane_palette_texture: TESTexture, // 0x1F0
    pub particle_palette_texture: TESTexture, // 0x200
    pub unk210: *mut core::ffi::c_void,       // 0x210
    pub unk218: *mut core::ffi::c_void,       // 0x218
}

const _: () = assert!(core::mem::size_of::<TESEffectShader>() == 0x220);
const _: () = assert!(core::mem::offset_of!(TESEffectShader, data) == 0x020);
const _: () = assert!(core::mem::offset_of!(TESEffectShader, fill_texture) == 0x1C0);
const _: () = assert!(core::mem::offset_of!(TESEffectShader, particle_shader_texture) == 0x1D0);
const _: () = assert!(core::mem::offset_of!(TESEffectShader, holes_texture) == 0x1E0);
const _: () = assert!(core::mem::offset_of!(TESEffectShader, membrane_palette_texture) == 0x1F0);
const _: () = assert!(core::mem::offset_of!(TESEffectShader, particle_palette_texture) == 0x200);
const _: () = assert!(core::mem::offset_of!(TESEffectShader, unk210) == 0x210);
const _: () = assert!(core::mem::offset_of!(TESEffectShader, unk218) == 0x218);

impl RttiType for TESEffectShader {
    const RTTI: VariantID = RTTI_TESEffectShader;
}

impl FormCastable for TESEffectShader {
    const TARGET_FORM_TYPE: FormType = FormType::EffectShader;
}

inherit!(TESEffectShader : TESForm);

impl TESEffectShader {
    pub const RTTI: VariantID = RTTI_TESEffectShader;
    pub const VTABLE: &'static [VariantID] = &VTABLE_TESEffectShader;
    pub const FORMTYPE: FormType = FormType::EffectShader;

    // override (TESForm)
    // void InitializeData() override;      // 04
    // bool Load(TESFile* a_mod) override;  // 06
    // void InitItemImpl() override;        // 13
}
