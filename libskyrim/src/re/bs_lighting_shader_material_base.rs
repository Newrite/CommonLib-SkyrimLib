use core::ptr;

use core_util::inherit;

use crate::offsets::offsets_rtti::RTTI_BSLightingShaderMaterialBase;
use crate::offsets::offsets_vtable::VTABLE_BSLightingShaderMaterialBase;
use crate::re::{
    BSLightingShaderMaterial, BSLightingShaderMaterialEnvmap, BSLightingShaderMaterialEye,
    BSLightingShaderMaterialFacegen, BSLightingShaderMaterialFacegenTint,
    BSLightingShaderMaterialGlowmap, BSLightingShaderMaterialHairTint,
    BSLightingShaderMaterialLODLandscape, BSLightingShaderMaterialLandscape,
    BSLightingShaderMaterialMultiLayerParallax, BSLightingShaderMaterialParallax,
    BSLightingShaderMaterialParallaxOcc, BSLightingShaderMaterialSnow, BSShaderMaterial,
    BSShaderMaterialFeature, BSTextureSet, MemoryManager, NiColor, NiPointer, NiSourceTexture,
    NiStream, ScrapHeap,
};
use crate::relocation::{RelocationID, RttiType, VariantID};

/// C++ `RE::BSLightingShaderMaterialBase`
#[repr(C)]
pub struct BSLightingShaderMaterialBase {
    pub base: BSShaderMaterial,                                     // 00
    pub specular_color: NiColor,                                    // 38
    pub pad44: u32,                                                 // 44
    pub diffuse_texture: NiPointer<NiSourceTexture>,                // 48
    pub diffuse_render_target_source_index: i32,                    // 50
    pub pad54: u32,                                                 // 54
    pub normal_texture: NiPointer<NiSourceTexture>,                 // 58
    pub rim_soft_lighting_texture: NiPointer<NiSourceTexture>,      // 60
    pub specular_back_lighting_texture: NiPointer<NiSourceTexture>, // 68
    pub texture_clamp_mode: i32,                                    // 70
    pub pad74: u32,                                                 // 74
    pub texture_set: NiPointer<BSTextureSet>,                       // 78
    pub material_alpha: f32,                                        // 80
    pub refraction_power: f32,                                      // 84
    pub specular_power: f32,                                        // 88
    pub specular_color_scale: f32,                                  // 8C
    pub sub_surface_light_rolloff: f32,                             // 90
    pub rim_light_power: f32,                                       // 94
    pub unk98: u32,                                                 // 98
}

const _: () = assert!(core::mem::size_of::<BSLightingShaderMaterialBase>() == 0xA0);
const _: () = assert!(core::mem::offset_of!(BSLightingShaderMaterialBase, specular_color) == 0x38);
const _: () = assert!(core::mem::offset_of!(BSLightingShaderMaterialBase, material_alpha) == 0x80);

impl RttiType for BSLightingShaderMaterialBase {
    const RTTI: VariantID = RTTI_BSLightingShaderMaterialBase;
}

inherit!(BSLightingShaderMaterialBase : BSShaderMaterial, base);

pub trait BSLightingShaderMaterialKind {
    const FEATURE: BSShaderMaterialFeature;
}

impl BSLightingShaderMaterialBase {
    pub const RTTI: VariantID = RTTI_BSLightingShaderMaterialBase;
    pub const VTABLE: &'static [VariantID] = &VTABLE_BSLightingShaderMaterialBase;

    crate::virtual_method! {
        pub const VFUNC_ON_LOAD_TEXTURE_SET: usize = 0x08;
        pub fn on_load_texture_set(&mut self, arg1: u64, in_texture_set: *mut BSTextureSet)
    }

    crate::virtual_method! {
        pub const VFUNC_CLEAR_TEXTURES: usize = 0x09;
        pub fn clear_textures(&mut self)
    }

    crate::virtual_method! {
        pub const VFUNC_RECEIVE_VALUES_FROM_ROOT_MATERIAL: usize = 0x0A;
        pub fn receive_values_from_root_material(&mut self, skinned: bool, rim_lighting: bool, soft_lighting: bool, back_lighting: bool, msn: bool)
    }

    crate::virtual_method! {
        pub const VFUNC_GET_TEXTURES: usize = 0x0B;
        pub fn get_textures(&mut self, textures: *mut *mut NiSourceTexture) -> u32
    }

    crate::virtual_method! {
        pub const VFUNC_SAVE_BINARY: usize = 0x0C;
        pub fn save_binary(&mut self, stream: &mut NiStream)
    }

    crate::virtual_method! {
        pub const VFUNC_LOAD_BINARY: usize = 0x0D;
        pub fn load_binary(&mut self, stream: &mut NiStream)
    }

    crate::relocation_func! {
        pub(crate) fn constructor(material: *mut Self) -> *mut Self => RelocationID::new(100004, 106711)
    }

    #[inline(always)]
    fn thread_scrap_heap() -> *mut ScrapHeap {
        let manager = MemoryManager::get_singleton();
        if manager.is_null() {
            ptr::null_mut()
        } else {
            unsafe { (*manager).get_thread_scrap_heap() }
        }
    }

    #[inline(always)]
    unsafe fn allocate_from_thread_scrap_heap<T>(zeroed: bool) -> *mut T {
        let heap = Self::thread_scrap_heap();
        if heap.is_null() {
            return ptr::null_mut();
        }

        let material = unsafe { (*heap).allocate(core::mem::size_of::<T>(), 8) }.cast::<T>();
        if material.is_null() {
            return ptr::null_mut();
        }

        if zeroed {
            unsafe {
                ptr::write_bytes(material.cast::<u8>(), 0, core::mem::size_of::<T>());
            }
        }

        material
    }

    #[inline(always)]
    pub fn create_material(feature: BSShaderMaterialFeature) -> *mut Self {
        match feature {
            BSShaderMaterialFeature::Default => {
                let material = unsafe {
                    Self::allocate_from_thread_scrap_heap::<BSLightingShaderMaterial>(true)
                };
                if material.is_null() {
                    return ptr::null_mut();
                }

                unsafe {
                    (*material).ctor();
                    (*material).base.base.vtable =
                        BSLightingShaderMaterial::VTABLE[0].address() as *const usize;
                }

                material.cast()
            }
            BSShaderMaterialFeature::EnvironmentMap => {
                let material = unsafe {
                    Self::allocate_from_thread_scrap_heap::<BSLightingShaderMaterialEnvmap>(false)
                };
                if !material.is_null() {
                    unsafe {
                        (*material).ctor();
                    }
                }
                material.cast()
            }
            BSShaderMaterialFeature::GlowMap => {
                let material = unsafe {
                    Self::allocate_from_thread_scrap_heap::<BSLightingShaderMaterialGlowmap>(false)
                };
                if !material.is_null() {
                    unsafe {
                        (*material).ctor();
                    }
                }
                material.cast()
            }
            BSShaderMaterialFeature::Parallax => {
                let material = unsafe {
                    Self::allocate_from_thread_scrap_heap::<BSLightingShaderMaterialParallax>(false)
                };
                if !material.is_null() {
                    unsafe {
                        (*material).ctor();
                    }
                }
                material.cast()
            }
            BSShaderMaterialFeature::FaceGen => {
                let material = unsafe {
                    Self::allocate_from_thread_scrap_heap::<BSLightingShaderMaterialFacegen>(false)
                };
                if !material.is_null() {
                    unsafe {
                        (*material).ctor();
                    }
                }
                material.cast()
            }
            BSShaderMaterialFeature::FaceGenRGBTint => {
                let material = unsafe {
                    Self::allocate_from_thread_scrap_heap::<BSLightingShaderMaterialFacegenTint>(
                        false,
                    )
                };
                if !material.is_null() {
                    unsafe {
                        (*material).ctor();
                    }
                }
                material.cast()
            }
            BSShaderMaterialFeature::HairTint => {
                let material = unsafe {
                    Self::allocate_from_thread_scrap_heap::<BSLightingShaderMaterialHairTint>(false)
                };
                if !material.is_null() {
                    unsafe {
                        (*material).ctor();
                    }
                }
                material.cast()
            }
            BSShaderMaterialFeature::ParallaxOcc => {
                let material = unsafe {
                    Self::allocate_from_thread_scrap_heap::<BSLightingShaderMaterialParallaxOcc>(
                        false,
                    )
                };
                if !material.is_null() {
                    unsafe {
                        (*material).ctor();
                    }
                }
                material.cast()
            }
            BSShaderMaterialFeature::MultiTexLand
            | BSShaderMaterialFeature::MultiTexLandLodBlend => {
                let material = unsafe {
                    Self::allocate_from_thread_scrap_heap::<BSLightingShaderMaterialLandscape>(
                        false,
                    )
                };
                if !material.is_null() {
                    unsafe {
                        (*material).ctor();
                    }
                }
                material.cast()
            }
            BSShaderMaterialFeature::LodLand | BSShaderMaterialFeature::LodLandNoise => {
                let material = unsafe {
                    Self::allocate_from_thread_scrap_heap::<BSLightingShaderMaterialLODLandscape>(
                        false,
                    )
                };
                if !material.is_null() {
                    unsafe {
                        (*material).ctor();
                    }
                }
                material.cast()
            }
            BSShaderMaterialFeature::Unknown | BSShaderMaterialFeature::MultiIndexTriShapeSnow => {
                let material = unsafe {
                    Self::allocate_from_thread_scrap_heap::<BSLightingShaderMaterialSnow>(false)
                };
                if !material.is_null() {
                    unsafe {
                        (*material).ctor();
                    }
                }
                material.cast()
            }
            BSShaderMaterialFeature::MultilayerParallax => {
                let material = unsafe {
                    Self::allocate_from_thread_scrap_heap::<
                        BSLightingShaderMaterialMultiLayerParallax,
                    >(false)
                };
                if !material.is_null() {
                    unsafe {
                        (*material).ctor();
                    }
                }
                material.cast()
            }
            BSShaderMaterialFeature::Eye => {
                let material = unsafe {
                    Self::allocate_from_thread_scrap_heap::<BSLightingShaderMaterialEye>(false)
                };
                if !material.is_null() {
                    unsafe {
                        (*material).ctor();
                    }
                }
                material.cast()
            }
            _ => ptr::null_mut(),
        }
    }

    #[inline(always)]
    pub fn create_material_as<T: BSLightingShaderMaterialKind>() -> *mut T {
        Self::create_material(T::FEATURE).cast::<T>()
    }

    #[inline(always)]
    pub fn copy_base_members(&mut self, other: &BSLightingShaderMaterialBase) {
        self.base.tex_coord_offset[0] = other.base.tex_coord_offset[0];
        self.base.tex_coord_offset[1] = other.base.tex_coord_offset[1];
        self.base.tex_coord_scale[0] = other.base.tex_coord_scale[0];
        self.base.tex_coord_scale[1] = other.base.tex_coord_scale[1];

        self.diffuse_texture = other.diffuse_texture.clone();
        self.normal_texture = other.normal_texture.clone();
        self.rim_soft_lighting_texture = other.rim_soft_lighting_texture.clone();
        self.specular_back_lighting_texture = other.specular_back_lighting_texture.clone();
        self.texture_clamp_mode = other.texture_clamp_mode;
        self.texture_set = other.texture_set.clone();

        self.material_alpha = other.material_alpha;
        self.specular_power = other.specular_power;
        self.specular_color = other.specular_color;
        self.specular_color_scale = other.specular_color_scale;
        self.refraction_power = other.refraction_power;
        self.sub_surface_light_rolloff = other.sub_surface_light_rolloff;
        self.rim_light_power = other.rim_light_power;
    }

    #[inline(always)]
    pub fn get_texture_set(&self) -> NiPointer<BSTextureSet> {
        self.texture_set.clone()
    }

    #[inline(always)]
    pub fn set_texture_set(&mut self, texture_set: NiPointer<BSTextureSet>) {
        self.texture_set = texture_set;
    }
}

pub trait BSLightingShaderMaterialBaseExt {
    fn on_load_texture_set(&mut self, arg1: u64, in_texture_set: *mut BSTextureSet);
    fn clear_textures(&mut self);
    fn receive_values_from_root_material(
        &mut self,
        skinned: bool,
        rim_lighting: bool,
        soft_lighting: bool,
        back_lighting: bool,
        msn: bool,
    );
    fn get_textures(&mut self, textures: *mut *mut NiSourceTexture) -> u32;
    fn save_binary(&mut self, stream: &mut NiStream);
    fn load_binary(&mut self, stream: &mut NiStream);
    fn copy_base_members(&mut self, other: &BSLightingShaderMaterialBase);
    fn get_texture_set(&self) -> NiPointer<BSTextureSet>;
    fn set_texture_set(&mut self, texture_set: NiPointer<BSTextureSet>);
}

impl<T: AsRef<BSLightingShaderMaterialBase> + AsMut<BSLightingShaderMaterialBase>>
    BSLightingShaderMaterialBaseExt for T
{
    #[inline(always)]
    fn on_load_texture_set(&mut self, arg1: u64, in_texture_set: *mut BSTextureSet) {
        self.as_mut().on_load_texture_set(arg1, in_texture_set)
    }

    #[inline(always)]
    fn clear_textures(&mut self) {
        self.as_mut().clear_textures()
    }

    #[inline(always)]
    fn receive_values_from_root_material(
        &mut self,
        skinned: bool,
        rim_lighting: bool,
        soft_lighting: bool,
        back_lighting: bool,
        msn: bool,
    ) {
        self.as_mut().receive_values_from_root_material(
            skinned,
            rim_lighting,
            soft_lighting,
            back_lighting,
            msn,
        )
    }

    #[inline(always)]
    fn get_textures(&mut self, textures: *mut *mut NiSourceTexture) -> u32 {
        self.as_mut().get_textures(textures)
    }

    #[inline(always)]
    fn save_binary(&mut self, stream: &mut NiStream) {
        self.as_mut().save_binary(stream)
    }

    #[inline(always)]
    fn load_binary(&mut self, stream: &mut NiStream) {
        self.as_mut().load_binary(stream)
    }

    #[inline(always)]
    fn copy_base_members(&mut self, other: &BSLightingShaderMaterialBase) {
        self.as_mut().copy_base_members(other)
    }

    #[inline(always)]
    fn get_texture_set(&self) -> NiPointer<BSTextureSet> {
        self.as_ref().get_texture_set()
    }

    #[inline(always)]
    fn set_texture_set(&mut self, texture_set: NiPointer<BSTextureSet>) {
        self.as_mut().set_texture_set(texture_set)
    }
}
