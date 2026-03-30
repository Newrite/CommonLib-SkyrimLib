use core_util::EnumSet;

use crate::core_util::inherit;
use crate::offsets::offsets_nirtti::NiRTTI_BSShaderProperty;
use crate::offsets::offsets_rtti::{RTTI_BSShaderProperty, RTTI_BSShaderProperty__ForEachVisitor};
use crate::offsets::offsets_vtable::{
    VTABLE_BSShaderProperty, VTABLE_BSShaderProperty__ForEachVisitor,
};
use crate::re::{
    BSEffectShaderData, BSFadeNode, BSGeometry, BSLight, BSRenderPass, BSShader,
    BSShaderAccumulator, BSShaderMaterial, BSShaderMaterialType, BSShaderPropertyLightData,
    BSTSmartPointer, NiRTTI, NiRef, NiShadeProperty, NiSourceTexture,
};
use crate::relocation::{ID, RelocationID, RttiType, VariantID};

/// C++ `RE::BSShaderProperty::ForEachVisitor`
#[repr(C)]
pub struct BSShaderPropertyForEachVisitor {
    pub vtable: *const usize, // 00
}

const _: () = assert!(core::mem::size_of::<BSShaderPropertyForEachVisitor>() == 0x8);

impl BSShaderPropertyForEachVisitor {
    pub const RTTI: VariantID = RTTI_BSShaderProperty__ForEachVisitor;
    pub const VTABLE: &'static [VariantID] = &VTABLE_BSShaderProperty__ForEachVisitor;

    crate::virtual_method! {
        pub const VFUNC_DTOR: usize = 0x00;
        pub fn dtor(&mut self)
    }

    crate::virtual_method! {
        pub const VFUNC_ACCEPT: usize = 0x01;
        pub fn accept(&mut self, texture: *mut NiSourceTexture) -> u32
    }
}

/// C++ `RE::BSShaderProperty::RenderPassArray`
#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct BSShaderPropertyRenderPassArray {
    pub head: *mut BSRenderPass, // 00
    pub unk08: u64,              // 08
}

const _: () = assert!(core::mem::size_of::<BSShaderPropertyRenderPassArray>() == 0x10);
const _: () = assert!(core::mem::offset_of!(BSShaderPropertyRenderPassArray, head) == 0x00);

impl BSShaderPropertyRenderPassArray {
    #[inline(always)]
    pub fn clear(&mut self) {
        while !self.head.is_null() {
            let next = unsafe { (*self.head).next };
            unsafe { (*self.head).clear_render_pass() };
            self.head = next;
        }
        self.head = core::ptr::null_mut();
    }

    #[inline(always)]
    pub fn emplace_pass(
        &mut self,
        shader: *mut BSShader,
        property: *mut BSShaderProperty,
        geometry: *mut BSGeometry,
        technique: u32,
        num_lights: u8,
        light0: *mut BSLight,
        light1: *mut BSLight,
        light2: *mut BSLight,
        light3: *mut BSLight,
    ) -> *mut BSRenderPass {
        let mut lights = [light0, light1, light2, light3];
        let new_pass = unsafe {
            (*shader).make_render_pass(
                property,
                geometry,
                technique,
                num_lights,
                lights.as_mut_ptr(),
            )
        };
        if !self.head.is_null() {
            let mut last_pass = self.head;
            while unsafe { !(*last_pass).next.is_null() } {
                last_pass = unsafe { (*last_pass).next };
            }
            unsafe {
                (*last_pass).next = new_pass;
            }
        } else {
            self.head = new_pass;
        }
        new_pass
    }
}

/// C++ `RE::BSShaderProperty::EShaderPropertyFlag`
#[repr(u64)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BSShaderPropertyFlag {
    Specular = 1u64 << 0,
    Skinned = 1u64 << 1,
    TempRefraction = 1u64 << 2,
    VertexAlpha = 1u64 << 3,
    GrayscaleToPaletteColor = 1u64 << 4,
    GrayscaleToPaletteAlpha = 1u64 << 5,
    Falloff = 1u64 << 6,
    EnvMap = 1u64 << 7,
    ReceiveShadows = 1u64 << 8,
    CastShadows = 1u64 << 9,
    Face = 1u64 << 10,
    Parallax = 1u64 << 11,
    ModelSpaceNormals = 1u64 << 12,
    NonProjectiveShadows = 1u64 << 13,
    MultiTextureLandscape = 1u64 << 14,
    Refraction = 1u64 << 15,
    RefractionFalloff = 1u64 << 16,
    EyeReflect = 1u64 << 17,
    HairTint = 1u64 << 18,
    ScreendoorAlphaFade = 1u64 << 19,
    LocalMapClear = 1u64 << 20,
    FaceGenRGBTint = 1u64 << 21,
    OwnEmit = 1u64 << 22,
    ProjectedUV = 1u64 << 23,
    MultipleTextures = 1u64 << 24,
    RemappableTextures = 1u64 << 25,
    Decal = 1u64 << 26,
    DynamicDecal = 1u64 << 27,
    ParallaxOcclusion = 1u64 << 28,
    ExternalEmittance = 1u64 << 29,
    SoftEffect = 1u64 << 30,
    ZBufferTest = 1u64 << 31,
    ZBufferWrite = 1u64 << 32,
    LodLandscape = 1u64 << 33,
    LodObjects = 1u64 << 34,
    NoFade = 1u64 << 35,
    TwoSided = 1u64 << 36,
    VertexColors = 1u64 << 37,
    GlowMap = 1u64 << 38,
    AssumeShadowmask = 1u64 << 39,
    CharacterLighting = 1u64 << 40,
    MultiIndexSnow = 1u64 << 41,
    VertexLighting = 1u64 << 42,
    UniformScale = 1u64 << 43,
    FitSlope = 1u64 << 44,
    Billboard = 1u64 << 45,
    NoLodLandBlend = 1u64 << 46,
    EnvmapLightFade = 1u64 << 47,
    Wireframe = 1u64 << 48,
    WeaponBlood = 1u64 << 49,
    HideOnLocalMap = 1u64 << 50,
    PremultAlpha = 1u64 << 51,
    CloudLod = 1u64 << 52,
    AnisotropicLighting = 1u64 << 53,
    NoTransparencyMultiSample = 1u64 << 54,
    MenuScreen = 1u64 << 55,
    MultiLayerParallax = 1u64 << 56,
    SoftLighting = 1u64 << 57,
    RimLighting = 1u64 << 58,
    BackLighting = 1u64 << 59,
    Snow = 1u64 << 60,
    TreeAnim = 1u64 << 61,
    EffectLighting = 1u64 << 62,
    HdLodObjects = 1u64 << 63,
}

core_util::impl_enumset_type!(BSShaderPropertyFlag => u64);

/// C++ `RE::BSShaderProperty::EShaderPropertyFlag8`
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BSShaderPropertyFlag8 {
    Specular = 0,
    Skinned = 1,
    TempRefraction = 2,
    VertexAlpha = 3,
    GrayscaleToPaletteColor = 4,
    GrayscaleToPaletteAlpha = 5,
    Falloff = 6,
    EnvMap = 7,
    ReceiveShadows = 8,
    CastShadows = 9,
    Face = 10,
    Parallax = 11,
    ModelSpaceNormals = 12,
    NonProjectiveShadows = 13,
    MultiTextureLandscape = 14,
    Refraction = 15,
    RefractionFalloff = 16,
    EyeReflect = 17,
    HairTint = 18,
    ScreendoorAlphaFade = 19,
    LocalMapClear = 20,
    FaceGenRGBTint = 21,
    OwnEmit = 22,
    ProjectedUV = 23,
    MultipleTextures = 24,
    RemappableTextures = 25,
    Decal = 26,
    DynamicDecal = 27,
    ParallaxOcclusion = 28,
    ExternalEmittance = 29,
    SoftEffect = 30,
    ZBufferTest = 31,
    ZBufferWrite = 32,
    LodLandscape = 33,
    LodObjects = 34,
    NoFade = 35,
    TwoSided = 36,
    VertexColors = 37,
    GlowMap = 38,
    AssumeShadowmask = 39,
    CharacterLighting = 40,
    MultiIndexSnow = 41,
    VertexLighting = 42,
    UniformScale = 43,
    FitSlope = 44,
    Billboard = 45,
    NoLodLandBlend = 46,
    EnvmapLightFade = 47,
    Wireframe = 48,
    WeaponBlood = 49,
    HideOnLocalMap = 50,
    PremultAlpha = 51,
    CloudLod = 52,
    AnisotropicLighting = 53,
    NoTransparencyMultiSample = 54,
    MenuScreen = 55,
    MultiLayerParallax = 56,
    SoftLighting = 57,
    RimLighting = 58,
    BackLighting = 59,
    Snow = 60,
    TreeAnim = 61,
    EffectLighting = 62,
    HdLodObjects = 63,
}

/// C++ `RE::BSShaderProperty`
#[repr(C)]
pub struct BSShaderProperty {
    pub base: NiShadeProperty,                                   // 00
    pub alpha: f32,                                              // 30
    pub last_render_pass_state: i32,                             // 34
    pub flags: EnumSet<BSShaderPropertyFlag, u64>,               // 38
    pub render_pass_list: BSShaderPropertyRenderPassArray,       // 40
    pub debug_render_pass_list: BSShaderPropertyRenderPassArray, // 50
    pub fade_node: *mut BSFadeNode,                              // 60
    pub effect_data: BSTSmartPointer<BSEffectShaderData>,        // 68
    pub light_data: *mut BSShaderPropertyLightData,              // 70
    pub material: *mut BSShaderMaterial,                         // 78
    pub last_accumulated_frame_count: u32,                       // 80
    pub pad84: u32,                                              // 84
}

const _: () = assert!(core::mem::size_of::<BSShaderProperty>() == 0x88);
const _: () = assert!(core::mem::offset_of!(BSShaderProperty, base) == 0x00);
const _: () = assert!(core::mem::offset_of!(BSShaderProperty, alpha) == 0x30);
const _: () = assert!(core::mem::offset_of!(BSShaderProperty, render_pass_list) == 0x40);
const _: () = assert!(core::mem::offset_of!(BSShaderProperty, effect_data) == 0x68);
const _: () =
    assert!(core::mem::offset_of!(BSShaderProperty, last_accumulated_frame_count) == 0x80);

inherit!(BSShaderProperty : NiShadeProperty);

impl RttiType for BSShaderProperty {
    const RTTI: VariantID = RTTI_BSShaderProperty;
}

impl NiRef for BSShaderProperty {
    #[inline(always)]
    fn inc_ref(&self) {
        self.base.inc_ref();
    }

    #[inline(always)]
    fn dec_ref(&self) {
        self.base.dec_ref();
    }
}

impl BSShaderProperty {
    pub const RTTI: VariantID = RTTI_BSShaderProperty;
    pub const NI_RTTI: VariantID = NiRTTI_BSShaderProperty;
    pub const VTABLE: &'static [VariantID] = &VTABLE_BSShaderProperty;

    // TODO: `BSShaderProperty.h` still has untranslated clone/stream helpers
    // beyond the `LoadBinary(...)` wrapper below. End state: carry over the
    // rest of the same-name header/source pair once the remaining stream-layer
    // dependencies are verified here.

    crate::virtual_method! {
        pub const VFUNC_GET_RTTI: usize = 0x02;
        pub fn get_rtti() -> *const NiRTTI
    }

    #[inline(always)]
    pub fn load_binary(&mut self, stream: *mut crate::re::NiStream) {
        let vtable = Self::VTABLE[0].address() as *const *const ();
        let base_method: extern "C" fn(*mut BSShaderProperty, *mut crate::re::NiStream) =
            unsafe { core::mem::transmute(*vtable.add(0x18)) };
        base_method(self, stream);
    }

    crate::virtual_method! {
        pub const VFUNC_SETUP_GEOMETRY: usize = 0x27;
        pub fn setup_geometry(&mut self, geometry: *mut BSGeometry) -> bool
    }

    crate::virtual_method! {
        pub const VFUNC_SET_LIGHT_STATE: usize = 0x29;
        pub fn set_light_state(&mut self, light_index: i32)
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
        pub const VFUNC_GET_NUMBER_OF_PASSES: usize = 0x2E;
        pub fn get_number_of_passes(&mut self, geometry: *mut BSGeometry) -> i32
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
        pub const VFUNC_CLARIFY_SHADER: usize = 0x36;
        pub fn clarify_shader(&mut self, geometry: *mut BSGeometry, arg2: bool, arg3: bool)
    }

    crate::virtual_method! {
        pub const VFUNC_GET_BASE_TEXTURE: usize = 0x37;
        pub fn get_base_texture(&mut self) -> *mut NiSourceTexture
    }

    crate::virtual_method! {
        pub const VFUNC_GET_WATER_FOG_PASS_LIST: usize = 0x38;
        pub fn get_water_fog_pass_list(&mut self, geometry: *mut BSGeometry)
    }

    crate::virtual_method! {
        pub const VFUNC_ACCEPTS_EFFECT_DATA: usize = 0x39;
        pub fn accepts_effect_data(&self) -> bool
    }

    crate::virtual_method! {
        pub const VFUNC_PRECACHE_TEXTURES: usize = 0x3A;
        pub fn precache_textures(&mut self)
    }

    crate::virtual_method! {
        pub const VFUNC_UNK_3B: usize = 0x3B;
        pub fn unk_3b(&mut self)
    }

    crate::virtual_method! {
        pub const VFUNC_UNK_3C: usize = 0x3C;
        pub fn unk_3c(&mut self)
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
    pub fn get_base_material(&self) -> *mut BSShaderMaterial {
        self.material
    }

    #[inline(always)]
    pub fn invalidate_material(&mut self) -> bool {
        crate::runtime::require_vr("BSShaderProperty::invalidate_material");
        let func: extern "C" fn(*mut BSShaderProperty) -> bool =
            unsafe { core::mem::transmute(ID::new(5370397616).address()) };
        func(self)
    }

    #[inline(always)]
    pub fn set_effect_shader_data(&mut self, data: &BSTSmartPointer<BSEffectShaderData>) {
        self.last_render_pass_state = i32::MAX;
        self.effect_data = data.clone();
    }

    crate::relocation_func! {
        pub fn set_material(&mut self, material: *mut BSShaderMaterial, unique: bool) => RelocationID::new(98897, 105544)
    }

    crate::relocation_func! {
        pub fn set_flags(&mut self, flag: BSShaderPropertyFlag8, set: bool) => RelocationID::new(98893, 105540)
    }
}

impl AsRef<BSShaderProperty> for BSShaderProperty {
    #[inline(always)]
    fn as_ref(&self) -> &Self {
        self
    }
}

impl AsMut<BSShaderProperty> for BSShaderProperty {
    #[inline(always)]
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

pub trait BSShaderPropertyExt {
    fn get_rtti(&self) -> *const NiRTTI;
    fn setup_geometry(&mut self, geometry: *mut BSGeometry) -> bool;
    fn set_light_state(&mut self, light_index: i32);
    fn get_render_passes(
        &mut self,
        geometry: *mut BSGeometry,
        render_mode: u32,
        accumulator: *mut BSShaderAccumulator,
    ) -> *mut BSShaderPropertyRenderPassArray;
    fn get_render_passes_shadow_map_or_mask(
        &mut self,
        geometry: *mut BSGeometry,
        render_mode: u32,
        accumulator: *mut BSShaderAccumulator,
    ) -> *mut BSShaderPropertyRenderPassArray;
    fn get_render_passes_local_map(
        &mut self,
        geometry: *mut BSGeometry,
        render_mode: u32,
        accumulator: *mut BSShaderAccumulator,
    ) -> *mut BSShaderPropertyRenderPassArray;
    fn get_render_passes_occlusion(
        &mut self,
        geometry: *mut BSGeometry,
        render_mode: u32,
        accumulator: *mut BSShaderAccumulator,
    ) -> *mut BSShaderPropertyRenderPassArray;
    fn get_number_of_passes(&mut self, geometry: *mut BSGeometry) -> i32;
    fn get_render_depth_pass(&mut self, geometry: *mut BSGeometry) -> *mut BSRenderPass;
    fn can_merge(&mut self, other: *const BSShaderProperty) -> bool;
    fn set_material_alpha(&mut self, alpha: f32);
    fn q_material_alpha(&mut self) -> f32;
    fn for_each_texture(&mut self, visitor: &mut BSShaderPropertyForEachVisitor) -> i32;
    fn do_clear_render_passes(&mut self);
    fn q_shader(&mut self) -> i32;
    fn clarify_shader(&mut self, geometry: *mut BSGeometry, arg2: bool, arg3: bool);
    fn get_base_texture(&mut self) -> *mut NiSourceTexture;
    fn get_water_fog_pass_list(&mut self, geometry: *mut BSGeometry);
    fn accepts_effect_data(&self) -> bool;
    fn precache_textures(&mut self);
    fn unk_3b(&mut self);
    fn unk_3c(&mut self);
    fn determine_utility_shader_decl(&mut self) -> u32;
    fn get_material_type(&mut self) -> BSShaderMaterialType;
    fn get_base_material(&self) -> *mut BSShaderMaterial;
    fn invalidate_material(&mut self) -> bool;
    fn set_effect_shader_data(&mut self, data: &BSTSmartPointer<BSEffectShaderData>);
    fn set_material(&mut self, material: *mut BSShaderMaterial, unique: bool);
    fn set_flags(&mut self, flag: BSShaderPropertyFlag8, set: bool);
}

impl<T: AsRef<BSShaderProperty> + AsMut<BSShaderProperty>> BSShaderPropertyExt for T {
    #[inline(always)]
    fn get_rtti(&self) -> *const NiRTTI {
        BSShaderProperty::get_rtti(self.as_ref())
    }

    #[inline(always)]
    fn setup_geometry(&mut self, geometry: *mut BSGeometry) -> bool {
        BSShaderProperty::setup_geometry(self.as_mut(), geometry)
    }

    #[inline(always)]
    fn set_light_state(&mut self, light_index: i32) {
        BSShaderProperty::set_light_state(self.as_mut(), light_index)
    }

    #[inline(always)]
    fn get_render_passes(
        &mut self,
        geometry: *mut BSGeometry,
        render_mode: u32,
        accumulator: *mut BSShaderAccumulator,
    ) -> *mut BSShaderPropertyRenderPassArray {
        BSShaderProperty::get_render_passes(self.as_mut(), geometry, render_mode, accumulator)
    }

    #[inline(always)]
    fn get_render_passes_shadow_map_or_mask(
        &mut self,
        geometry: *mut BSGeometry,
        render_mode: u32,
        accumulator: *mut BSShaderAccumulator,
    ) -> *mut BSShaderPropertyRenderPassArray {
        BSShaderProperty::get_render_passes_shadow_map_or_mask(
            self.as_mut(),
            geometry,
            render_mode,
            accumulator,
        )
    }

    #[inline(always)]
    fn get_render_passes_local_map(
        &mut self,
        geometry: *mut BSGeometry,
        render_mode: u32,
        accumulator: *mut BSShaderAccumulator,
    ) -> *mut BSShaderPropertyRenderPassArray {
        BSShaderProperty::get_render_passes_local_map(
            self.as_mut(),
            geometry,
            render_mode,
            accumulator,
        )
    }

    #[inline(always)]
    fn get_render_passes_occlusion(
        &mut self,
        geometry: *mut BSGeometry,
        render_mode: u32,
        accumulator: *mut BSShaderAccumulator,
    ) -> *mut BSShaderPropertyRenderPassArray {
        BSShaderProperty::get_render_passes_occlusion(
            self.as_mut(),
            geometry,
            render_mode,
            accumulator,
        )
    }

    #[inline(always)]
    fn get_number_of_passes(&mut self, geometry: *mut BSGeometry) -> i32 {
        BSShaderProperty::get_number_of_passes(self.as_mut(), geometry)
    }

    #[inline(always)]
    fn get_render_depth_pass(&mut self, geometry: *mut BSGeometry) -> *mut BSRenderPass {
        BSShaderProperty::get_render_depth_pass(self.as_mut(), geometry)
    }

    #[inline(always)]
    fn can_merge(&mut self, other: *const BSShaderProperty) -> bool {
        BSShaderProperty::can_merge(self.as_mut(), other)
    }

    #[inline(always)]
    fn set_material_alpha(&mut self, alpha: f32) {
        BSShaderProperty::set_material_alpha(self.as_mut(), alpha)
    }

    #[inline(always)]
    fn q_material_alpha(&mut self) -> f32 {
        BSShaderProperty::q_material_alpha(self.as_mut())
    }

    #[inline(always)]
    fn for_each_texture(&mut self, visitor: &mut BSShaderPropertyForEachVisitor) -> i32 {
        BSShaderProperty::for_each_texture(self.as_mut(), visitor)
    }

    #[inline(always)]
    fn do_clear_render_passes(&mut self) {
        BSShaderProperty::do_clear_render_passes(self.as_mut())
    }

    #[inline(always)]
    fn q_shader(&mut self) -> i32 {
        BSShaderProperty::q_shader(self.as_mut())
    }

    #[inline(always)]
    fn clarify_shader(&mut self, geometry: *mut BSGeometry, arg2: bool, arg3: bool) {
        BSShaderProperty::clarify_shader(self.as_mut(), geometry, arg2, arg3)
    }

    #[inline(always)]
    fn get_base_texture(&mut self) -> *mut NiSourceTexture {
        BSShaderProperty::get_base_texture(self.as_mut())
    }

    #[inline(always)]
    fn get_water_fog_pass_list(&mut self, geometry: *mut BSGeometry) {
        BSShaderProperty::get_water_fog_pass_list(self.as_mut(), geometry)
    }

    #[inline(always)]
    fn accepts_effect_data(&self) -> bool {
        BSShaderProperty::accepts_effect_data(self.as_ref())
    }

    #[inline(always)]
    fn precache_textures(&mut self) {
        BSShaderProperty::precache_textures(self.as_mut())
    }

    #[inline(always)]
    fn unk_3b(&mut self) {
        BSShaderProperty::unk_3b(self.as_mut())
    }

    #[inline(always)]
    fn unk_3c(&mut self) {
        BSShaderProperty::unk_3c(self.as_mut())
    }

    #[inline(always)]
    fn determine_utility_shader_decl(&mut self) -> u32 {
        BSShaderProperty::determine_utility_shader_decl(self.as_mut())
    }

    #[inline(always)]
    fn get_material_type(&mut self) -> BSShaderMaterialType {
        BSShaderProperty::get_material_type(self.as_mut())
    }

    #[inline(always)]
    fn get_base_material(&self) -> *mut BSShaderMaterial {
        BSShaderProperty::get_base_material(self.as_ref())
    }

    #[inline(always)]
    fn invalidate_material(&mut self) -> bool {
        BSShaderProperty::invalidate_material(self.as_mut())
    }

    #[inline(always)]
    fn set_effect_shader_data(&mut self, data: &BSTSmartPointer<BSEffectShaderData>) {
        BSShaderProperty::set_effect_shader_data(self.as_mut(), data)
    }

    #[inline(always)]
    fn set_material(&mut self, material: *mut BSShaderMaterial, unique: bool) {
        BSShaderProperty::set_material(self.as_mut(), material, unique)
    }

    #[inline(always)]
    fn set_flags(&mut self, flag: BSShaderPropertyFlag8, set: bool) {
        BSShaderProperty::set_flags(self.as_mut(), flag, set)
    }
}
