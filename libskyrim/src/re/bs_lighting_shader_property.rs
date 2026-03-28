use crate::core_util::inherit;
use crate::offsets::offsets_nirtti::NiRTTI_BSLightingShaderProperty;
use crate::offsets::offsets_rtti::RTTI_BSLightingShaderProperty;
use crate::offsets::offsets_vtable::VTABLE_BSLightingShaderProperty;
use crate::re::{
    BSGeometry, BSRenderPass, BSShaderAccumulator, BSShaderMaterialType, BSShaderProperty,
    BSShaderPropertyFlag, BSShaderPropertyForEachVisitor, BSShaderPropertyLightData,
    BSShaderPropertyRenderPassArray, NiColor, NiColorA, NiRTTI, NiRef, NiSourceTexture,
};
use crate::relocation::{ID, RelocationID, RttiType, VariantID};

/// C++ `RE::BSLightingShaderProperty`
#[repr(C)]
pub struct BSLightingShaderProperty {
    pub base: BSShaderProperty,                                     // 00
    pub array_queue: [BSShaderPropertyRenderPassArray; 3],          // 88
    pub shadow_map_or_mask_passes: BSShaderPropertyRenderPassArray, // B8
    pub occlusion_passes: BSShaderPropertyRenderPassArray,          // C8
    pub volumetric_shadow_utility_passes: BSShaderPropertyRenderPassArray, // D8
    pub depth_pass: *mut BSRenderPass,                              // E8
    pub emissive_color: *mut NiColor,                               // F0
    pub emissive_mult: f32,                                         // F8
    pub forced_darkness: f32,                                       // FC
    pub specular_lod_fade: f32,                                     // 100
    pub envmap_lod_fade: f32,                                       // 104
    pub unk108: u32,                                                // 108
    pub projected_uv_params: NiColorA,                              // 10C
    pub projected_uv_color: NiColorA,                               // 11C
    pub unk12c: i32,                                                // 12C
    pub unk130: u32,                                                // 130
    pub unk134: u32,                                                // 134
    pub lighting_light_data: BSShaderPropertyLightData,             // 138
}

const _: () = assert!(core::mem::size_of::<BSLightingShaderProperty>() == 0x160);
const _: () = assert!(core::mem::offset_of!(BSLightingShaderProperty, array_queue) == 0x88);
const _: () =
    assert!(core::mem::offset_of!(BSLightingShaderProperty, lighting_light_data) == 0x138);

impl NiRef for BSLightingShaderProperty {
    #[inline(always)]
    fn inc_ref(&self) {
        self.base.inc_ref();
    }

    #[inline(always)]
    fn dec_ref(&self) {
        self.base.dec_ref();
    }
}

impl RttiType for BSLightingShaderProperty {
    const RTTI: VariantID = RTTI_BSLightingShaderProperty;
}

inherit!(BSLightingShaderProperty : BSShaderProperty, base);

impl BSLightingShaderProperty {
    pub const RTTI: VariantID = RTTI_BSLightingShaderProperty;
    pub const NI_RTTI: VariantID = NiRTTI_BSLightingShaderProperty;
    pub const VTABLE: &'static [VariantID] = &VTABLE_BSLightingShaderProperty;

    // TODO: `BSLightingShaderProperty.h` still has untranslated clone/binary
    // stream helpers. End state: carry over the rest of the same-name
    // header/source pair once the inherited stream-layer surface is verified
    // here instead of exposing only the currently-used render/material subset.

    crate::virtual_method! {
        pub const VFUNC_GET_RTTI: usize = 0x02;
        pub fn get_rtti() -> *const NiRTTI
    }

    crate::virtual_method! {
        pub const VFUNC_SETUP_GEOMETRY: usize = 0x27;
        pub fn setup_geometry(&mut self, geometry: *mut BSGeometry) -> bool
    }

    crate::virtual_method! {
        pub const VFUNC_FINISH_SETUP_GEOMETRY: usize = 0x28;
        pub fn finish_setup_geometry(&mut self, geometry: *mut BSGeometry) -> bool
    }

    crate::virtual_method! {
        pub const VFUNC_GET_RENDER_PASSES: usize = 0x2A;
        pub fn get_render_passes(&mut self, geometry: *mut BSGeometry, render_mode: u32, accumulator: *mut BSShaderAccumulator) -> *mut BSShaderPropertyRenderPassArray
    }

    crate::virtual_method! {
        pub const VFUNC_GET_RENDER_PASSES_SHADOW_MAP_OR_MASK: usize = 0x2B;
        pub fn get_render_passes_shadow_map_or_mask(&mut self, geometry: *mut BSGeometry, render_mode: u32, accumulator: *mut BSShaderAccumulator) -> *mut BSShaderPropertyRenderPassArray
    }

    crate::virtual_method! {
        pub const VFUNC_GET_RENDER_PASSES_LOCAL_MAP: usize = 0x2C;
        pub fn get_render_passes_local_map(&mut self, geometry: *mut BSGeometry, render_mode: u32, accumulator: *mut BSShaderAccumulator) -> *mut BSShaderPropertyRenderPassArray
    }

    crate::virtual_method! {
        pub const VFUNC_GET_RENDER_PASSES_OCCLUSION: usize = 0x2D;
        pub fn get_render_passes_occlusion(&mut self, geometry: *mut BSGeometry, render_mode: u32, accumulator: *mut BSShaderAccumulator) -> *mut BSShaderPropertyRenderPassArray
    }

    crate::virtual_method! {
        pub const VFUNC_GET_RENDER_DEPTH_PASS: usize = 0x2F;
        pub fn get_render_depth_pass(&mut self, geometry: *mut BSGeometry) -> *mut BSRenderPass
    }

    crate::virtual_method! {
        pub const VFUNC_CAN_MERGE: usize = 0x30;
        pub fn can_merge(&mut self, other: *const BSShaderProperty) -> bool
    }

    crate::virtual_method! {
        pub const VFUNC_SET_MATERIAL_ALPHA: usize = 0x31;
        pub fn set_material_alpha(&mut self, alpha: f32)
    }

    crate::virtual_method! {
        pub const VFUNC_QMATERIAL_ALPHA: usize = 0x32;
        pub fn q_material_alpha(&mut self) -> f32
    }

    crate::virtual_method! {
        pub const VFUNC_FOR_EACH_TEXTURE: usize = 0x33;
        pub fn for_each_texture(&mut self, visitor: &mut BSShaderPropertyForEachVisitor) -> i32
    }

    crate::virtual_method! {
        pub const VFUNC_DO_CLEAR_RENDER_PASSES: usize = 0x34;
        pub fn do_clear_render_passes(&mut self)
    }

    crate::virtual_method! {
        pub const VFUNC_QSHADER: usize = 0x35;
        pub fn q_shader(&mut self) -> i32
    }

    crate::virtual_method! {
        pub const VFUNC_GET_BASE_TEXTURE: usize = 0x37;
        pub fn get_base_texture(&mut self) -> *mut NiSourceTexture
    }

    crate::virtual_method! {
        pub const VFUNC_ACCEPTS_EFFECT_DATA: usize = 0x39;
        pub fn accepts_effect_data(&self) -> bool
    }

    crate::virtual_method! {
        pub const VFUNC_UNK_3B: usize = 0x3B;
        pub fn unk_3b(&mut self)
    }

    crate::virtual_method! {
        pub const VFUNC_DETERMINE_UTILITY_SHADER_DECL: usize = 0x3D;
        pub fn determine_utility_shader_decl(&mut self) -> u32
    }

    crate::virtual_method! {
        pub const VFUNC_GET_MATERIAL_TYPE: usize = 0x3E;
        pub fn get_material_type(&mut self) -> BSShaderMaterialType
    }

    #[inline(always)]
    pub fn copy_members(&mut self, other: *mut BSLightingShaderProperty) {
        self.base.flags = unsafe { (*other).base.flags };
        self.base.alpha = unsafe { (*other).base.alpha };
        self.base.last_render_pass_state = i32::MAX;
        if self.base.flags.all(BSShaderPropertyFlag::OwnEmit)
            && unsafe { !(*other).emissive_color.is_null() }
        {
            if self.emissive_color.is_null() {
                // TODO: `BSLightingShaderProperty.cpp` uses `new NiColor()`
                // here. This Rust port still falls back to `commonlib_malloc`
                // because there is no shared ABI-safe bridge yet for raw
                // `NiColor` allocation that is guaranteed to match the engine's
                // eventual delete path. End state: route this through an exact
                // C++ allocation helper instead of a Rust-side raw allocation.
                unsafe {
                    let memory = crate::ffi::commonlib_malloc(core::mem::size_of::<NiColor>())
                        .cast::<NiColor>();
                    assert!(
                        !memory.is_null(),
                        "BSLightingShaderProperty::copy_members emissive_color allocation failed"
                    );
                    core::ptr::write(memory, NiColor::default());
                    self.emissive_color = memory;
                }
            }
            unsafe {
                (*self.emissive_color) = *(*other).emissive_color;
            }
        }
        self.projected_uv_params = unsafe { (*other).projected_uv_params };
        self.projected_uv_color = unsafe { (*other).projected_uv_color };
        self.emissive_mult = unsafe { (*other).emissive_mult };
    }

    #[inline(always)]
    pub fn invalidate_textures(&mut self, unk1: u32) {
        crate::runtime::require_vr("BSLightingShaderProperty::invalidate_textures");
        let func: extern "C" fn(*mut BSLightingShaderProperty, u32) =
            unsafe { core::mem::transmute(ID::new(5388393136).address()) };
        func(self, unk1);
    }

    crate::relocation_func! {
        pub fn ctor(&mut self) -> *mut BSLightingShaderProperty => RelocationID::new(99854, 106499)
    }
}
