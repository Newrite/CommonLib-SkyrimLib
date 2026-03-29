use core_util::inherit;

use crate::offsets::offsets_rtti::RTTI_BSShaderMaterial;
use crate::offsets::offsets_vtable::VTABLE_BSShaderMaterial;
use crate::re::{BSIntrusiveRefCounted, NiPoint2};
use crate::relocation::{RttiType, VariantID};

/// C++ `RE::BSShaderMaterial::Feature`
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BSShaderMaterialFeature {
    None = -1,
    Default = 0,
    EnvironmentMap = 1,
    GlowMap = 2,
    Parallax = 3,
    FaceGen = 4,
    FaceGenRGBTint = 5,
    HairTint = 6,
    ParallaxOcc = 7,
    MultiTexLand = 8,
    LodLand = 9,
    Unknown = 10,
    MultilayerParallax = 11,
    TreeAnim = 12,
    MultiIndexTriShapeSnow = 14,
    LodObjectsHd = 15,
    Eye = 16,
    Cloud = 17,
    LodLandNoise = 18,
    MultiTexLandLodBlend = 19,
}

/// C++ `RE::BSShaderMaterial::Type`
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BSShaderMaterialType {
    Base = 0,
    Effect = 1,
    Lighting = 2,
    Water = 3,
}

/// C++ `RE::BSShaderMaterial`
#[repr(C)]
pub struct BSShaderMaterial {
    pub vtable: *const usize,            // 00
    pub base: BSIntrusiveRefCounted,     // 08
    pub tex_coord_offset: [NiPoint2; 2], // 0C
    pub tex_coord_scale: [NiPoint2; 2],  // 1C
    pub hash_key: u32,                   // 2C
    pub unk30: u32,                      // 30
    pub unk34: u32,                      // 34
}

const _: () = assert!(core::mem::size_of::<BSShaderMaterial>() == 0x38);
const _: () = assert!(core::mem::offset_of!(BSShaderMaterial, base) == 0x08);
const _: () = assert!(core::mem::offset_of!(BSShaderMaterial, tex_coord_offset) == 0x0C);

impl RttiType for BSShaderMaterial {
    const RTTI: VariantID = RTTI_BSShaderMaterial;
}

inherit!(BSShaderMaterial : BSIntrusiveRefCounted, base);

impl BSShaderMaterial {
    pub const RTTI: VariantID = RTTI_BSShaderMaterial;
    pub const VTABLE: &'static [VariantID] = &VTABLE_BSShaderMaterial;

    crate::virtual_method! {
        pub const VFUNC_DTOR: usize = 0x00;
        pub fn dtor(&mut self)
    }

    crate::virtual_method! {
        pub const VFUNC_CREATE: usize = 0x01;
        pub fn create(&self) -> *mut BSShaderMaterial
    }

    crate::virtual_method! {
        pub const VFUNC_COPY_MEMBERS: usize = 0x02;
        pub fn copy_members(&mut self, that: *mut BSShaderMaterial)
    }

    crate::virtual_method! {
        pub const VFUNC_DO_IS_COPY: usize = 0x03;
        pub fn do_is_copy(&self, that: *mut BSShaderMaterial) -> bool
    }

    crate::virtual_method! {
        pub const VFUNC_COMPUTE_CRC32: usize = 0x04;
        pub fn compute_crc32(&mut self, src_hash: u32) -> u32
    }

    crate::virtual_method! {
        pub const VFUNC_GET_DEFAULT: usize = 0x05;
        pub fn get_default(&self) -> *mut BSShaderMaterial
    }

    crate::virtual_method! {
        pub const VFUNC_GET_FEATURE: usize = 0x06;
        pub fn get_feature(&self) -> BSShaderMaterialFeature
    }

    crate::virtual_method! {
        pub const VFUNC_GET_TYPE: usize = 0x07;
        pub fn get_type(&self) -> BSShaderMaterialType
    }
}

pub trait BSShaderMaterialExt {
    fn dtor(&mut self);
    fn create(&self) -> *mut BSShaderMaterial;
    fn copy_members(&mut self, that: *mut BSShaderMaterial);
    fn do_is_copy(&self, that: *mut BSShaderMaterial) -> bool;
    fn compute_crc32(&mut self, src_hash: u32) -> u32;
    fn get_default(&self) -> *mut BSShaderMaterial;
    fn get_feature(&self) -> BSShaderMaterialFeature;
    fn get_type(&self) -> BSShaderMaterialType;
}

impl<T: AsRef<BSShaderMaterial> + AsMut<BSShaderMaterial>> BSShaderMaterialExt for T {
    #[inline(always)]
    fn dtor(&mut self) {
        self.as_mut().dtor()
    }

    #[inline(always)]
    fn create(&self) -> *mut BSShaderMaterial {
        self.as_ref().create()
    }

    #[inline(always)]
    fn copy_members(&mut self, that: *mut BSShaderMaterial) {
        self.as_mut().copy_members(that)
    }

    #[inline(always)]
    fn do_is_copy(&self, that: *mut BSShaderMaterial) -> bool {
        self.as_ref().do_is_copy(that)
    }

    #[inline(always)]
    fn compute_crc32(&mut self, src_hash: u32) -> u32 {
        self.as_mut().compute_crc32(src_hash)
    }

    #[inline(always)]
    fn get_default(&self) -> *mut BSShaderMaterial {
        self.as_ref().get_default()
    }

    #[inline(always)]
    fn get_feature(&self) -> BSShaderMaterialFeature {
        self.as_ref().get_feature()
    }

    #[inline(always)]
    fn get_type(&self) -> BSShaderMaterialType {
        self.as_ref().get_type()
    }
}
