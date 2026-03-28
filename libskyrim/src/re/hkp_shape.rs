#![allow(non_camel_case_types)]

use core::ffi::c_void;

use crate::core_util::inherit;
use crate::offsets::offsets_rtti::RTTI_hkpShape;
use crate::offsets::offsets_vtable::VTABLE_hkpShape;
use crate::re::{
    CFilter, bhkShape, hkAabb, hkReferencedObject, hkSphere, hkTransform, hkVector4,
    hkVector4Comparison, hkpCdBody, hkpCdVertex, hkpConvexShape, hkpRayHitCollector,
    hkpShapeBuffer, hkpShapeContainer, hkpShapeRayBundleCastInput, hkpShapeRayBundleCastOutput,
    hkpShapeRayCastInput, hkpShapeRayCastOutput, hkpShapeType,
};
use crate::relocation::{RttiType, VariantID};
use crate::virtual_method;

pub type hkpVertexID = u16;
pub const HK_INVALID_VERTEX_ID: hkpVertexID = u16::MAX;

pub type hkpShapeKey = u32;
pub const HK_INVALID_SHAPE_KEY: hkpShapeKey = u32::MAX;

pub type hkpShapeGetSupportingVertexFunc =
    unsafe extern "C" fn(*const c_void, &hkVector4, &mut hkpCdVertex);
pub type hkpShapeConvertVertexIdsToVerticesFunc =
    unsafe extern "C" fn(*const c_void, *const hkpVertexID, i32, *mut hkpCdVertex);
pub type hkpShapeWeldContactPointFunc = unsafe extern "C" fn(
    *const c_void,
    *mut hkpVertexID,
    &mut u8,
    &mut hkVector4,
    *const hkTransform,
    *const hkpConvexShape,
    *const hkTransform,
    &mut hkVector4,
) -> i32;
pub type hkpShapeGetCentreFunc = unsafe extern "C" fn(*const c_void, &mut hkVector4);
pub type hkpShapeGetNumCollisionSpheresFunc = unsafe extern "C" fn(*const c_void) -> i32;
pub type hkpShapeGetCollisionSpheresFunc =
    unsafe extern "C" fn(*const c_void, *mut hkSphere) -> *const hkSphere;
pub type hkpShapeGetAabbFunc = unsafe extern "C" fn(*const c_void, &hkTransform, f32, &mut hkAabb);
pub type hkpShapeCastRayFunc =
    unsafe extern "C" fn(*const c_void, &hkpShapeRayCastInput, &mut hkpShapeRayCastOutput) -> bool;
pub type hkpShapeCastRayWithCollectorFunc =
    unsafe extern "C" fn(*const c_void, &hkpShapeRayCastInput, &hkpCdBody, &mut hkpRayHitCollector);
pub type hkpShapeCastRayBundleFunc = unsafe extern "C" fn(
    *const c_void,
    &hkpShapeRayBundleCastInput,
    &mut hkpShapeRayBundleCastOutput,
    &hkVector4Comparison,
) -> hkVector4Comparison;
pub type hkpShapeGetChildShapeFunc =
    unsafe extern "C" fn(*const c_void, hkpShapeKey, &mut hkpShapeBuffer) -> *const hkpShape;
pub type hkpShapeGetCollisionFilterInfoFunc =
    unsafe extern "C" fn(*const c_void, hkpShapeKey) -> CFilter;
pub type hkpShapeRegsiterFunc = unsafe extern "C" fn(&mut hkpShapeShapeFuncs);

/// C++ `RE::hkpShape::CalcSizeForSpuInput`
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct hkpShapeCalcSizeForSpuInput {
    pub midphase_agent3_registered: bool, // 00
    pub is_fixed_or_keyframed: bool,      // 01
    pub has_dynamic_motion_saved: bool,   // 02
}

const _: () = assert!(core::mem::size_of::<hkpShapeCalcSizeForSpuInput>() == 0x3);

/// C++ `RE::hkpShape::ShapeFuncs`
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct hkpShapeShapeFuncs {
    pub get_supporting_vertex_func: Option<hkpShapeGetSupportingVertexFunc>, // 00
    pub convert_vertex_ids_to_vertices: Option<hkpShapeConvertVertexIdsToVerticesFunc>, // 08
    pub weld_contact_point_func: Option<hkpShapeWeldContactPointFunc>,       // 10
    pub get_centre_func: Option<hkpShapeGetCentreFunc>,                      // 18
    pub get_num_collision_spheres_func: Option<hkpShapeGetNumCollisionSpheresFunc>, // 20
    pub get_collision_spheres_func: Option<hkpShapeGetCollisionSpheresFunc>, // 28
    pub get_aabb_func: Option<hkpShapeGetAabbFunc>,                          // 30
    pub cast_ray: Option<hkpShapeCastRayFunc>,                               // 38
    pub cast_ray_with_collector: Option<hkpShapeCastRayWithCollectorFunc>,   // 40
    pub cast_ray_bundle: Option<hkpShapeCastRayBundleFunc>,                  // 48
    pub get_child_shape_func: Option<hkpShapeGetChildShapeFunc>,             // 50
    pub get_collision_filter_info_func: Option<hkpShapeGetCollisionFilterInfoFunc>, // 58
}

const _: () = assert!(core::mem::size_of::<hkpShapeShapeFuncs>() == 0x60);

/// C++ `RE::hkpShape::ShapeFuncs2`
#[repr(C, align(64))]
#[derive(Clone, Copy, Default)]
pub struct hkpShapeShapeFuncs2 {
    pub get_supporting_vertex_func: Option<hkpShapeGetSupportingVertexFunc>, // 00
    pub convert_vertex_ids_to_vertices: Option<hkpShapeConvertVertexIdsToVerticesFunc>, // 08
    pub weld_contact_point_func: Option<hkpShapeWeldContactPointFunc>,       // 10
    pub get_centre_func: Option<hkpShapeGetCentreFunc>,                      // 18
    pub get_num_collision_spheres_func: Option<hkpShapeGetNumCollisionSpheresFunc>, // 20
    pub get_collision_spheres_func: Option<hkpShapeGetCollisionSpheresFunc>, // 28
    pub get_aabb_func: Option<hkpShapeGetAabbFunc>,                          // 30
    pub cast_ray: Option<hkpShapeCastRayFunc>,                               // 38
    pub cast_ray_with_collector: Option<hkpShapeCastRayWithCollectorFunc>,   // 40
    pub cast_ray_bundle: Option<hkpShapeCastRayBundleFunc>,                  // 48
    pub get_child_shape_func: Option<hkpShapeGetChildShapeFunc>,             // 50
    pub get_collision_filter_info_func: Option<hkpShapeGetCollisionFilterInfoFunc>, // 58
    pub pad60: u64,                                                          // 60
    pub pad68: u64,                                                          // 68
    pub pad70: u64,                                                          // 70
    pub pad78: u64,                                                          // 78
}

const _: () = assert!(core::mem::size_of::<hkpShapeShapeFuncs2>() == 0x80);
const _: () = assert!(core::mem::align_of::<hkpShapeShapeFuncs2>() == 0x40);

/// C++ `RE::hkpShape`
#[repr(C)]
pub struct hkpShape {
    pub base: hkReferencedObject, // 00
    pub user_data: *mut bhkShape, // 10
    pub type_: hkpShapeType,      // 18
    pub pad1c: u32,               // 1C
}

const _: () = assert!(core::mem::size_of::<hkpShape>() == 0x20);
const _: () = assert!(core::mem::offset_of!(hkpShape, base) == 0x00);
const _: () = assert!(core::mem::offset_of!(hkpShape, user_data) == 0x10);
const _: () = assert!(core::mem::offset_of!(hkpShape, type_) == 0x18);
const _: () = assert!(core::mem::offset_of!(hkpShape, pad1c) == 0x1C);

impl RttiType for hkpShape {
    const RTTI: VariantID = RTTI_hkpShape;
}

inherit!(hkpShape : hkReferencedObject, base);

impl AsRef<hkpShape> for hkpShape {
    #[inline(always)]
    fn as_ref(&self) -> &Self {
        self
    }
}

impl AsMut<hkpShape> for hkpShape {
    #[inline(always)]
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl hkpShape {
    pub const RTTI: VariantID = RTTI_hkpShape;
    pub const VTABLE: &'static [VariantID] = &VTABLE_hkpShape;

    // override (hkReferencedObject)
    // ~hkpShape() override;  // 00

    virtual_method! {
        pub const VFUNC_GET_MAXIMUM_PROJECTION: usize = 0x03;
        pub fn get_maximum_projection(&self, direction: &hkVector4) -> f32
    }

    virtual_method! {
        pub const VFUNC_GET_CONTAINER: usize = 0x04;
        pub fn get_container(&self) -> *const hkpShapeContainer
    }

    virtual_method! {
        pub const VFUNC_IS_CONVEX: usize = 0x05;
        pub fn is_convex(&self) -> bool
    }

    virtual_method! {
        pub const VFUNC_CALC_SIZE_FOR_SPU: usize = 0x06;
        pub fn calc_size_for_spu(&self, input: &hkpShapeCalcSizeForSpuInput, spu_buffer_size_left: i32) -> i32
    }

    virtual_method! {
        pub const VFUNC_GET_AABB_IMPL: usize = 0x07;
        pub fn get_aabb_impl(&self, local_to_world: &hkTransform, tolerance: f32, out: &mut hkAabb)
    }

    virtual_method! {
        pub const VFUNC_CAST_RAY_IMPL: usize = 0x08;
        pub fn cast_ray_impl(&self, input: &hkpShapeRayCastInput, output: &mut hkpShapeRayCastOutput) -> bool
    }

    virtual_method! {
        pub const VFUNC_CAST_RAY_WITH_COLLECTOR_IMPL: usize = 0x09;
        pub fn cast_ray_with_collector_impl(
            &self,
            input: &hkpShapeRayCastInput,
            cd_body: &hkpCdBody,
            collector: &mut hkpRayHitCollector
        )
    }

    virtual_method! {
        pub const VFUNC_CAST_RAY_BUNDLE_IMPL: usize = 0x0A;
        pub fn cast_ray_bundle_impl(
            &self,
            input: &hkpShapeRayBundleCastInput,
            output: &mut hkpShapeRayBundleCastOutput,
            mask: &hkVector4Comparison
        ) -> hkVector4Comparison
    }
}

pub trait hkpShapeExt {
    fn get_maximum_projection(&self, direction: &hkVector4) -> f32;
    fn get_container(&self) -> *const hkpShapeContainer;
    fn is_convex(&self) -> bool;
    fn calc_size_for_spu(
        &self,
        input: &hkpShapeCalcSizeForSpuInput,
        spu_buffer_size_left: i32,
    ) -> i32;
    fn get_aabb_impl(&self, local_to_world: &hkTransform, tolerance: f32, out: &mut hkAabb);
    fn cast_ray_impl(
        &self,
        input: &hkpShapeRayCastInput,
        output: &mut hkpShapeRayCastOutput,
    ) -> bool;
    fn cast_ray_with_collector_impl(
        &self,
        input: &hkpShapeRayCastInput,
        cd_body: &hkpCdBody,
        collector: &mut hkpRayHitCollector,
    );
    fn cast_ray_bundle_impl(
        &self,
        input: &hkpShapeRayBundleCastInput,
        output: &mut hkpShapeRayBundleCastOutput,
        mask: &hkVector4Comparison,
    ) -> hkVector4Comparison;
}

impl<T: AsRef<hkpShape>> hkpShapeExt for T {
    #[inline(always)]
    fn get_maximum_projection(&self, direction: &hkVector4) -> f32 {
        hkpShape::get_maximum_projection(self.as_ref(), direction)
    }

    #[inline(always)]
    fn get_container(&self) -> *const hkpShapeContainer {
        hkpShape::get_container(self.as_ref())
    }

    #[inline(always)]
    fn is_convex(&self) -> bool {
        hkpShape::is_convex(self.as_ref())
    }

    #[inline(always)]
    fn calc_size_for_spu(
        &self,
        input: &hkpShapeCalcSizeForSpuInput,
        spu_buffer_size_left: i32,
    ) -> i32 {
        hkpShape::calc_size_for_spu(self.as_ref(), input, spu_buffer_size_left)
    }

    #[inline(always)]
    fn get_aabb_impl(&self, local_to_world: &hkTransform, tolerance: f32, out: &mut hkAabb) {
        hkpShape::get_aabb_impl(self.as_ref(), local_to_world, tolerance, out)
    }

    #[inline(always)]
    fn cast_ray_impl(
        &self,
        input: &hkpShapeRayCastInput,
        output: &mut hkpShapeRayCastOutput,
    ) -> bool {
        hkpShape::cast_ray_impl(self.as_ref(), input, output)
    }

    #[inline(always)]
    fn cast_ray_with_collector_impl(
        &self,
        input: &hkpShapeRayCastInput,
        cd_body: &hkpCdBody,
        collector: &mut hkpRayHitCollector,
    ) {
        hkpShape::cast_ray_with_collector_impl(self.as_ref(), input, cd_body, collector)
    }

    #[inline(always)]
    fn cast_ray_bundle_impl(
        &self,
        input: &hkpShapeRayBundleCastInput,
        output: &mut hkpShapeRayBundleCastOutput,
        mask: &hkVector4Comparison,
    ) -> hkVector4Comparison {
        hkpShape::cast_ray_bundle_impl(self.as_ref(), input, output, mask)
    }
}
