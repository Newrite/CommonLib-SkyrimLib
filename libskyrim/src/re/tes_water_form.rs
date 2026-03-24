use bitflags::bitflags;
use core::ffi::c_void;
use core_util::{EnumSet, inherit};

use crate::offsets::offsets_rtti::RTTI_TESWaterForm;
use crate::offsets::offsets_vtable::VTABLE_TESWaterForm;
use crate::re::bgs_material_type::BGSMaterialType;
use crate::re::bgs_sound_descriptor_form::BGSSoundDescriptorForm;
use crate::re::color::Color;
use crate::re::form_traits::FormCastable;
use crate::re::form_type::FormType;
use crate::re::ni_color_a::NiColorA;
use crate::re::ni_point3::NiPoint3;
use crate::re::ni_smart_pointer::NiPointer;
use crate::re::ni_texture::NiTexture;
use crate::re::spell_item::SpellItem;
use crate::re::tes_attack_damage_form::TESAttackDamageForm;
use crate::re::tes_form::TESForm;
use crate::re::tes_full_name::TESFullName;
use crate::re::tes_image_space::TESImageSpace;
use crate::re::tes_object_acti::TESObjectACTI;
use crate::re::tes_texture::TESTexture;
use crate::relocation::{RttiType, VariantID};

/// C++ `RE::TESWaterForm::Flag`
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TESWaterFormFlag {
    None = 0,
    CauseDamage = 1 << 0,
    EnableFlowmap = 1 << 3,
    BlendNormals = 1 << 4,
}

core_util::impl_enumset_type!(TESWaterFormFlag => u8);

bitflags! {
    /// C++ `RE::TESWaterForm::RecordFlags`
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct TESWaterFormRecordFlags: u32 {
        const DELETED = 1 << 5;
        const IGNORED = 1 << 12;
    }
}

/// C++ `RE::WaterShaderData::DepthProperties`
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct DepthProperties {
    pub reflections: f32,       // D0
    pub refraction: f32,        // D4
    pub normals: f32,           // D8
    pub specular_lighting: f32, // DC
}

const _: () = assert!(core::mem::size_of::<DepthProperties>() == 0x10);

/// C++ `RE::WaterShaderData`
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct WaterShaderData {
    pub unk00: f32,                        // 00
    pub unk04: f32,                        // 04
    pub unk08: f32,                        // 08
    pub unk0c: f32,                        // 0C
    pub sun_specular_power: f32,           // 10
    pub reflection_amount: f32,            // 14
    pub fresnel_amount: f32,               // 18
    pub unk1c: u32,                        // 1C
    pub above_water_fog_dist_near: f32,    // 20
    pub above_water_fog_dist_far: f32,     // 24
    pub shallow_water_color: Color,        // 28
    pub deep_water_color: Color,           // 2C
    pub reflection_water_color: Color,     // 30
    pub unk34: u32,                        // 34
    pub unk38: f32,                        // 38
    pub unk3c: f32,                        // 3C
    pub unk40: f32,                        // 40
    pub unk44: f32,                        // 44
    pub displacement_size: f32,            // 48
    pub displacement_force: f32,           // 4C
    pub displacement_velocity: f32,        // 50
    pub displacement_falloff: f32,         // 54
    pub displacement_dampener: f32,        // 58
    pub unk5c: f32,                        // 5C
    pub noise_falloff: f32,                // 60
    pub noise_wind_direction_a: [f32; 3],  // 64
    pub noise_wind_speed_a: [f32; 3],      // 70
    pub unk7c: f32,                        // 7C
    pub unk80: f32,                        // 80
    pub above_water_fog_amount: f32,       // 84
    pub unk88: f32,                        // 88
    pub underwater_fog_amount: f32,        // 8C
    pub underwater_fog_dist_near: f32,     // 90
    pub underwater_fog_dist_far: f32,      // 94
    pub refraction_magnitude: f32,         // 98
    pub specular_power: f32,               // 9C
    pub unk_a0: f32,                       // A0
    pub specular_radius: f32,              // A4
    pub specular_brightness: f32,          // A8
    pub uv_scale_a: [f32; 3],              // AC
    pub amplitude_a: [f32; 3],             // B8
    pub reflection_magnitude: f32,         // C4
    pub sun_sparkle_magnitude: f32,        // C8
    pub sun_specular_magnitude: f32,       // CC
    pub depth_properties: DepthProperties, // D0
    pub sun_sparkle_power: f32,            // E0
    pub flowmap_scale: f32,                // E4
}

const _: () = assert!(core::mem::size_of::<WaterShaderData>() == 0xE8);

/// C++ `RE::TESWaterForm`
#[repr(C)]
pub struct TESWaterForm {
    pub base: TESForm,                                 // 000
    pub full_name: TESFullName,                        // 020
    pub attack_damage_form: TESAttackDamageForm,       // 030
    pub need_update: bool,                             // 040
    pub pad41: u8,                                     // 041
    pub pad42: u16,                                    // 042
    pub tex_scroll: [NiColorA; 3],                     // 044
    pub pad074: u32,                                   // 074
    pub noise_textures: [TESTexture; 4],               // 078
    pub alpha: i8,                                     // 0B8
    pub flags: EnumSet<TESWaterFormFlag, u8>,          // 0B9
    pub pad0ba: u16,                                   // 0BA
    pub pad0bc: u32,                                   // 0BC
    pub material_type: *mut BGSMaterialType,           // 0C0
    pub water_sound: *mut BGSSoundDescriptorForm,      // 0C8
    pub data: WaterShaderData,                         // 0D0
    pub water_weather_control: [*mut TESWaterForm; 3], // 1B8
    pub current_texture_select: [i32; 2],              // 1D0
    pub frequency_x: u32,                              // 1D8
    pub frequency_y: u32,                              // 1DC
    pub octaves: i32,                                  // 1E0
    pub amplitude: f32,                                // 1E4
    pub lacunarity: f32,                               // 1E8
    pub bias: f32,                                     // 1EC
    pub gain: f32,                                     // 1F0
    pub pad1f4: u32,                                   // 1F4
    pub contact_spell: *mut SpellItem,                 // 1F8
    pub noise_texture_data: [NiPointer<NiTexture>; 4], // 200
    pub placeable_auto_water: *mut TESObjectACTI,      // 220
    pub placeable_lod_water: *mut TESObjectACTI,       // 228
    pub water_shader_material: *mut c_void,            // 230 - BSWaterShaderMaterial*
    pub reset_noise_textures: bool,                    // 238
    pub pad239: u8,                                    // 239
    pub pad23a: u16,                                   // 23A
    pub pad23c: u32,                                   // 23C
    pub image_space: *mut TESImageSpace,               // 240 - INAM
    pub linear_velocity: NiPoint3,                     // 248
    pub angular_velocity: NiPoint3,                    // 254
}

const _: () = assert!(core::mem::size_of::<TESWaterForm>() == 0x260);
const _: () = assert!(core::mem::offset_of!(TESWaterForm, full_name) == 0x20);
const _: () = assert!(core::mem::offset_of!(TESWaterForm, attack_damage_form) == 0x30);
const _: () = assert!(core::mem::offset_of!(TESWaterForm, noise_textures) == 0x78);
const _: () = assert!(core::mem::offset_of!(TESWaterForm, data) == 0xD0);
const _: () = assert!(core::mem::offset_of!(TESWaterForm, linear_velocity) == 0x248);

impl RttiType for TESWaterForm {
    const RTTI: VariantID = RTTI_TESWaterForm;
}

impl FormCastable for TESWaterForm {
    const TARGET_FORM_TYPE: FormType = FormType::Water;
}

inherit!(TESWaterForm : TESForm);
inherit!(TESWaterForm => TESFullName, full_name);
inherit!(TESWaterForm => TESAttackDamageForm, attack_damage_form);

impl TESWaterForm {
    pub const RTTI: VariantID = RTTI_TESWaterForm;
    pub const VTABLE: &'static [VariantID] = &VTABLE_TESWaterForm;
    pub const FORMTYPE: FormType = FormType::Water;

    // override (TESForm)
    // void InitializeData() override;                   // 04
    // bool Load(TESFile* a_mod) override;               // 06
    // void InitItemImpl() override;                     // 13
    // bool GetDangerous() const override;               // 1B
    // bool Activate(...) override;                      // 37

    #[inline(always)]
    pub fn is_dangerous(&self) -> bool {
        self.flags.any(TESWaterFormFlag::CauseDamage)
    }
}
