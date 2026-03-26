//! Minimal source-backed subset of the external OpenVR SDK used by CommonLib VR
//! headers through `#include "openvr.h"`.
//!
//! This is intentionally a narrow ABI surface for translated RE types such as
//! `BSVRInterface`, `BSOpenVR`, and tracked-controller device layouts, not a
//! wholesale mirror of the OpenVR SDK.

#[allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
pub mod vr {
    use core::ffi::c_void;

    core_util::abstract_type! {
        pub type IVRSystem;
        pub type IVRChaperone;
        pub type IVRChaperoneSetup;
        pub type IVRCompositor;
        pub type IVROverlay;
        pub type IVRResources;
        pub type IVRRenderModels;
        pub type IVRExtendedDisplay;
        pub type IVRSettings;
        pub type IVRApplications;
        pub type IVRTrackedCamera;
        pub type IVRScreenshots;
        pub type IVRDriverManager;
    }

    pub type TrackedDeviceIndex_t = u32;
    const _: () = assert!(core::mem::size_of::<TrackedDeviceIndex_t>() == 0x4);

    pub type VROverlayHandle_t = u64;
    const _: () = assert!(core::mem::size_of::<VROverlayHandle_t>() == 0x8);

    pub const k_unTrackedDeviceIndex_Hmd: TrackedDeviceIndex_t = 0;

    /// Minimal source-backed subset of OpenVR's compositor error enum.
    #[repr(i32)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub enum EVRCompositorError {
        VRCompositorError_None = 0,
    }

    const _: () = assert!(core::mem::size_of::<EVRCompositorError>() == 0x4);

    /// Minimal source-backed subset of OpenVR's init error enum.
    #[repr(i32)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub enum EVRInitError {
        VRInitError_None = 0,
    }

    const _: () = assert!(core::mem::size_of::<EVRInitError>() == 0x4);

    #[repr(i32)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub enum EVREye {
        Eye_Left = 0,
        Eye_Right = 1,
    }

    const _: () = assert!(core::mem::size_of::<EVREye>() == 0x4);

    #[repr(u32)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub enum EVRSubmitFlags {
        Submit_Default = 0,
    }

    const _: () = assert!(core::mem::size_of::<EVRSubmitFlags>() == 0x4);

    #[allow(non_camel_case_types)]
    pub type ETextureType = i32;
    const _: () = assert!(core::mem::size_of::<ETextureType>() == 0x4);

    #[allow(non_camel_case_types)]
    pub type EColorSpace = i32;
    const _: () = assert!(core::mem::size_of::<EColorSpace>() == 0x4);

    #[repr(C)]
    #[derive(Debug, Clone, Copy)]
    pub struct Texture_t {
        pub handle: *mut c_void,
        pub e_type: ETextureType,
        pub e_color_space: EColorSpace,
    }

    const _: () = assert!(core::mem::size_of::<Texture_t>() == 0x10);

    #[repr(C)]
    #[derive(Debug, Clone, Copy, Default)]
    pub struct VRTextureBounds_t {
        pub u_min: f32,
        pub v_min: f32,
        pub u_max: f32,
        pub v_max: f32,
    }

    const _: () = assert!(core::mem::size_of::<VRTextureBounds_t>() == 0x10);

    // TODO: CommonLib expects `vr::IVROverlay_Version` from the external
    // OpenVR SDK header, but that vendored header is not present in this tree.
    // Add the real C-string constant here once the exact SDK version string is
    // source-backed locally; until then, keep future overlay factory helpers on
    // the C++ side or behind a type-local TODO instead of guessing it.
}

pub use vr::*;
