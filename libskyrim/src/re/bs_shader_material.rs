use crate::offsets::offsets_rtti::RTTI_BSShaderMaterial;
use crate::offsets::offsets_vtable::VTABLE_BSShaderMaterial;
use crate::re::{BSGraphicsVertexDesc, BSIntrusiveRefCounted, NiPoint2};
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

impl BSShaderMaterial {
    pub const RTTI: VariantID = RTTI_BSShaderMaterial;
    pub const VTABLE: &'static [VariantID] = &VTABLE_BSShaderMaterial;

    crate::virtual_method! {
        pub const VFUNC_CREATE: usize = 0x01;
        pub fn create() -> *mut BSShaderMaterial
    }

    crate::virtual_method! {
        pub const VFUNC_COPY_MEMBERS: usize = 0x02;
        pub fn copy_members(&mut self, that: *mut BSShaderMaterial)
    }

    crate::virtual_method! {
        pub const VFUNC_DO_IS_COPY: usize = 0x03;
        pub fn do_is_copy(&mut self, that: *mut BSShaderMaterial) -> bool
    }

    crate::virtual_method! {
        pub const VFUNC_COMPUTE_CRC32: usize = 0x04;
        pub fn compute_crc32(&mut self, src_hash: u32) -> u32
    }

    crate::virtual_method! {
        pub const VFUNC_GET_DEFAULT: usize = 0x05;
        pub fn get_default() -> *mut BSShaderMaterial
    }

    crate::virtual_method! {
        pub const VFUNC_GET_FEATURE: usize = 0x06;
        pub fn get_feature() -> BSShaderMaterialFeature
    }

    crate::virtual_method! {
        pub const VFUNC_GET_TYPE: usize = 0x07;
        pub fn get_type() -> BSShaderMaterialType
    }
}
