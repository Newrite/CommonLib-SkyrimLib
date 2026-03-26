use core::ffi::c_void;

use crate::core_util::inherit;
use crate::offsets::offsets_nirtti::NiRTTI_NiCamera;
use crate::offsets::offsets_rtti::RTTI_NiCamera;
use crate::offsets::offsets_vtable::VTABLE_NiCamera;
use crate::re::{BSTArray, NiAVObject, NiBound, NiFrustum, NiMatrix44, NiPoint3, NiRect};
use crate::relocation::{RelocationID, RttiType, VariantID, VariantOffset};

/// C++ `RE::NiCamera::RUNTIME_DATA`
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct NiCameraRuntimeData {
    pub world_to_cam: [[f32; 4]; 4], // 00
}

const _: () = assert!(core::mem::size_of::<NiCameraRuntimeData>() == 0x40);

/// C++ `RE::NiCamera::RUNTIME_DATA_VR`
#[repr(C)]
pub struct NiCameraRuntimeDataVr {
    pub world_to_cam: [[f32; 4]; 4],               // 00
    pub view_frustum_buffer: *mut NiFrustum,       // 40
    pub view_frustum_array: *mut NiFrustum,        // 48
    pub eye_viewport_rects: BSTArray<NiRect<f32>>, // 50
    pub eye_view_matrices: BSTArray<NiMatrix44>,   // 68
    pub eye_culling_procs: [*mut c_void; 2],       // 80
    pub unk1c8: u32,                               // 90
    pub pad94: u32,                                // 94
}

const _: () = assert!(core::mem::size_of::<NiCameraRuntimeDataVr>() == 0x98);
const _: () = assert!(core::mem::offset_of!(NiCameraRuntimeDataVr, view_frustum_buffer) == 0x40);
const _: () = assert!(core::mem::offset_of!(NiCameraRuntimeDataVr, view_frustum_array) == 0x48);
const _: () = assert!(core::mem::offset_of!(NiCameraRuntimeDataVr, eye_viewport_rects) == 0x50);
const _: () = assert!(core::mem::offset_of!(NiCameraRuntimeDataVr, eye_view_matrices) == 0x68);
const _: () = assert!(core::mem::offset_of!(NiCameraRuntimeDataVr, eye_culling_procs) == 0x80);
const _: () = assert!(core::mem::offset_of!(NiCameraRuntimeDataVr, unk1c8) == 0x90);

/// C++ `RE::NiCamera::RUNTIME_DATA2`
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct NiCameraRuntimeData2 {
    pub view_frustum: NiFrustum,  // 00
    pub min_near_plane_dist: f32, // 1C
    pub max_far_near_ratio: f32,  // 20
    pub port: NiRect<f32>,        // 24
    pub lod_adjust: f32,          // 34
}

const _: () = assert!(core::mem::size_of::<NiCameraRuntimeData2>() == 0x38);
const _: () = assert!(core::mem::offset_of!(NiCameraRuntimeData2, view_frustum) == 0x00);
const _: () = assert!(core::mem::offset_of!(NiCameraRuntimeData2, min_near_plane_dist) == 0x1C);
const _: () = assert!(core::mem::offset_of!(NiCameraRuntimeData2, max_far_near_ratio) == 0x20);
const _: () = assert!(core::mem::offset_of!(NiCameraRuntimeData2, port) == 0x24);
const _: () = assert!(core::mem::offset_of!(NiCameraRuntimeData2, lod_adjust) == 0x34);

/// Honest cross-runtime common prefix for `RE::NiCamera`.
#[repr(C)]
pub struct NiCamera {
    pub base: NiAVObject, // 000
}

const _: () = assert!(core::mem::size_of::<NiCamera>() == 0x110);
const _: () = assert!(core::mem::offset_of!(NiCamera, base) == 0x00);

impl RttiType for NiCamera {
    const RTTI: VariantID = RTTI_NiCamera;
}

inherit!(NiCamera : NiAVObject, base);

impl crate::re::ni_ref_object::NiRef for NiCamera {
    #[inline(always)]
    fn inc_ref(&self) {
        self.base.inc_ref();
    }

    #[inline(always)]
    fn dec_ref(&self) {
        self.base.dec_ref();
    }
}

impl NiCamera {
    pub const RTTI: VariantID = RTTI_NiCamera;
    pub const NI_RTTI: VariantID = NiRTTI_NiCamera;
    pub const VTABLE: &'static [VariantID] = &VTABLE_NiCamera;

    // Inference from the header/cpp pair: `worldToCam` is the common leading block
    // in both the flat and VR tails, and `NiCamera.cpp` reads it through
    // `GetRuntimeData().worldToCam` without a VR branch.
    pub const RUNTIME_DATA_OFFSET: VariantOffset = VariantOffset::new(0x110, 0x110, 0x138);
    pub const RUNTIME_DATA2_OFFSET: VariantOffset = VariantOffset::new(0x150, 0x150, 0x1D0);
    pub const VR_RUNTIME_DATA_OFFSET: VariantOffset = VariantOffset::new(0x0, 0x0, 0x138);

    // override (NiAVObject)
    // const NiRTTI* GetRTTI() const override;                           // 02
    // NiObject*     CreateClone(NiCloningProcess& a_cloning) override;  // 17
    // void          LoadBinary(NiStream& a_stream) override;            // 18
    // void          LinkObject(NiStream& a_stream) override;            // 19
    // bool          RegisterStreamables(NiStream& a_stream) override;   // 1A
    // void          SaveBinary(NiStream& a_stream) override;            // 1B
    // bool          IsEqual(NiObject* a_object) override;               // 1C
    // Flat-only overrides past the VR compatibility split:
    // void UpdateWorldBound() override;                                 // 2F
    // void UpdateWorldData(NiUpdateData* a_data) override;              // 30

    crate::runtime_data_accessor! {
        pub fn get_runtime_data() -> NiCameraRuntimeData {
            offset: Self::RUNTIME_DATA_OFFSET
        }
    }

    crate::runtime_data_mut_accessor! {
        pub fn get_runtime_data_mut() -> NiCameraRuntimeData {
            offset: Self::RUNTIME_DATA_OFFSET
        }
    }

    crate::runtime_data_accessor! {
        pub fn get_runtime_data2() -> NiCameraRuntimeData2 {
            offset: Self::RUNTIME_DATA2_OFFSET
        }
    }

    crate::runtime_data_mut_accessor! {
        pub fn get_runtime_data2_mut() -> NiCameraRuntimeData2 {
            offset: Self::RUNTIME_DATA2_OFFSET
        }
    }

    crate::runtime_data_accessor! {
        pub fn get_vr_runtime_data() -> NiCameraRuntimeDataVr {
            offset: Self::VR_RUNTIME_DATA_OFFSET
        }
    }

    crate::runtime_data_mut_accessor! {
        pub fn get_vr_runtime_data_mut() -> NiCameraRuntimeDataVr {
            offset: Self::VR_RUNTIME_DATA_OFFSET
        }
    }

    crate::relocation_func! {
        pub fn bound_in_frustum(a_bound: &NiBound, a_camera: *mut NiCamera) -> bool => RelocationID::new(15671, 15899)
    }

    crate::relocation_func! {
        pub fn window_point_to_ray(&mut self, x: i32, y: i32, origin: &mut NiPoint3, dir: &mut NiPoint3, window_width: f32, window_height: f32) -> bool => RelocationID::new(69263, 70630)
    }

    crate::relocation_func! {
        pub fn world_pt_to_screen_pt3_with_matrix(matrix: &[[f32; 4]; 4], port: &NiRect<f32>, point: &NiPoint3, x_out: &mut f32, y_out: &mut f32, z_out: &mut f32, zero_tolerance: f32) -> bool => RelocationID::new(69270, 70640)
    }

    #[inline(always)]
    pub fn node_in_frustum(&self, node: *mut NiAVObject) -> bool {
        if node.is_null() {
            return false;
        }

        // TODO: `NiAVObjectExt::get_world_bound` does not exist yet; once
        // `ni_av_object.rs` is verified beyond its current blob `NiBound`
        // dependency, switch this to an owner-side helper instead of direct
        // field access through the existing concrete layout.
        Self::bound_in_frustum(
            unsafe { &(*node).world_bound },
            self as *const _ as *mut Self,
        )
    }

    #[inline(always)]
    pub fn point_in_frustum(&self, point: &NiPoint3, radius: f32) -> bool {
        let bound = NiBound::new(*point, radius);
        Self::bound_in_frustum(&bound, self as *const _ as *mut Self)
    }

    #[inline(always)]
    pub fn world_pt_to_screen_pt3(
        &self,
        point: &NiPoint3,
        x_out: &mut f32,
        y_out: &mut f32,
        z_out: &mut f32,
        zero_tolerance: f32,
    ) -> bool {
        Self::world_pt_to_screen_pt3_with_matrix(
            &self.get_runtime_data().world_to_cam,
            &self.get_runtime_data2().port,
            point,
            x_out,
            y_out,
            z_out,
            zero_tolerance,
        )
    }

    #[inline(always)]
    pub fn get_near_plane(&self) -> f32 {
        if crate::runtime::is_vr() {
            unsafe { (*self.get_vr_runtime_data().view_frustum_array).f_near }
        } else {
            self.get_runtime_data2().view_frustum.f_near
        }
    }
}

pub trait NiCameraExt {
    fn get_runtime_data(&self) -> &NiCameraRuntimeData;
    fn get_runtime_data_mut(&mut self) -> &mut NiCameraRuntimeData;
    fn get_runtime_data2(&self) -> &NiCameraRuntimeData2;
    fn get_runtime_data2_mut(&mut self) -> &mut NiCameraRuntimeData2;
    fn get_vr_runtime_data(&self) -> &NiCameraRuntimeDataVr;
    fn get_vr_runtime_data_mut(&mut self) -> &mut NiCameraRuntimeDataVr;
    fn window_point_to_ray(
        &mut self,
        x: i32,
        y: i32,
        origin: &mut NiPoint3,
        dir: &mut NiPoint3,
        window_width: f32,
        window_height: f32,
    ) -> bool;
    fn node_in_frustum(&self, node: *mut NiAVObject) -> bool;
    fn point_in_frustum(&self, point: &NiPoint3, radius: f32) -> bool;
    fn world_pt_to_screen_pt3(
        &self,
        point: &NiPoint3,
        x_out: &mut f32,
        y_out: &mut f32,
        z_out: &mut f32,
        zero_tolerance: f32,
    ) -> bool;
    fn get_near_plane(&self) -> f32;
}

impl<T> NiCameraExt for T
where
    T: AsRef<NiCamera> + AsMut<NiCamera>,
{
    #[inline(always)]
    fn get_runtime_data(&self) -> &NiCameraRuntimeData {
        NiCamera::get_runtime_data(self.as_ref())
    }

    #[inline(always)]
    fn get_runtime_data_mut(&mut self) -> &mut NiCameraRuntimeData {
        NiCamera::get_runtime_data_mut(self.as_mut())
    }

    #[inline(always)]
    fn get_runtime_data2(&self) -> &NiCameraRuntimeData2 {
        NiCamera::get_runtime_data2(self.as_ref())
    }

    #[inline(always)]
    fn get_runtime_data2_mut(&mut self) -> &mut NiCameraRuntimeData2 {
        NiCamera::get_runtime_data2_mut(self.as_mut())
    }

    #[inline(always)]
    fn get_vr_runtime_data(&self) -> &NiCameraRuntimeDataVr {
        NiCamera::get_vr_runtime_data(self.as_ref())
    }

    #[inline(always)]
    fn get_vr_runtime_data_mut(&mut self) -> &mut NiCameraRuntimeDataVr {
        NiCamera::get_vr_runtime_data_mut(self.as_mut())
    }

    #[inline(always)]
    fn window_point_to_ray(
        &mut self,
        x: i32,
        y: i32,
        origin: &mut NiPoint3,
        dir: &mut NiPoint3,
        window_width: f32,
        window_height: f32,
    ) -> bool {
        NiCamera::window_point_to_ray(
            self.as_mut(),
            x,
            y,
            origin,
            dir,
            window_width,
            window_height,
        )
    }

    #[inline(always)]
    fn node_in_frustum(&self, node: *mut NiAVObject) -> bool {
        NiCamera::node_in_frustum(self.as_ref(), node)
    }

    #[inline(always)]
    fn point_in_frustum(&self, point: &NiPoint3, radius: f32) -> bool {
        NiCamera::point_in_frustum(self.as_ref(), point, radius)
    }

    #[inline(always)]
    fn world_pt_to_screen_pt3(
        &self,
        point: &NiPoint3,
        x_out: &mut f32,
        y_out: &mut f32,
        z_out: &mut f32,
        zero_tolerance: f32,
    ) -> bool {
        NiCamera::world_pt_to_screen_pt3(self.as_ref(), point, x_out, y_out, z_out, zero_tolerance)
    }

    #[inline(always)]
    fn get_near_plane(&self) -> f32 {
        NiCamera::get_near_plane(self.as_ref())
    }
}
