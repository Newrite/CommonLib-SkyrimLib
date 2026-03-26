use crate::re::{
    BSTEventSink, BSTEventSource, NiNode, NiPointer, NiTransform, VRDeviceConnectionChange,
    VROverlayChange, VRResetHMDHeight,
};
use crate::relocation::{RttiType, VariantID};
use crate::rex::openvr::vr;

/// C++ `RE::BSVRInterface::HMDDeviceType`
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BSVRInterfaceHMDDeviceType {
    kLighthouse = 0,
    kOculus = 1,
    kHolographics = 2,
}

const _: () = assert!(core::mem::size_of::<BSVRInterfaceHMDDeviceType>() == 0x4);

/// C++ `RE::BSVRInterface::Hand`
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BSVRInterfaceHand {
    kLeft = 0,
    kRight = 1,
    kTotal = 2,
}

impl BSVRInterfaceHand {
    pub const TOTAL: usize = Self::kTotal as usize;
}

const _: () = assert!(core::mem::size_of::<BSVRInterfaceHand>() == 0x4);

/// C++ `RE::BSVRInterface::Unk118::PoseTransform::TrackingStatus`
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BSVRInterfaceTrackingStatus {
    kNotRunning = 0,
    kRunningOk = 1,
    kOutOfRange = 2,
}

const _: () = assert!(core::mem::size_of::<BSVRInterfaceTrackingStatus>() == 0x4);

/// C++ `RE::BSVRInterface::Unk118::PoseTransform`
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct BSVRInterfacePoseTransform {
    pub pose_status: BSVRInterfaceTrackingStatus, // 00
    pub render_pose_transform: NiTransform,       // 04
    pub game_pose_transform: NiTransform,         // 38
}

const _: () = assert!(core::mem::size_of::<BSVRInterfacePoseTransform>() == 0x6C);
const _: () = assert!(core::mem::offset_of!(BSVRInterfacePoseTransform, pose_status) == 0x00);
const _: () =
    assert!(core::mem::offset_of!(BSVRInterfacePoseTransform, render_pose_transform) == 0x04);
const _: () =
    assert!(core::mem::offset_of!(BSVRInterfacePoseTransform, game_pose_transform) == 0x38);

/// C++ `RE::BSVRInterface::Unk118`
#[repr(C)]
pub struct BSVRInterfaceUnk118 {
    pub unk00: *mut BSVRInterfaceUnk118,                 // 00
    pub unk08: *mut BSVRInterfaceUnk118,                 // 08
    pub unk10: *mut BSVRInterfaceUnk118,                 // 10
    pub unk18: u8,                                       // 18
    pub unk19: bool,                                     // 19
    pub unk1a: u16,                                      // 1A
    pub pose_index: u32,                                 // 1C
    pub pose_transform: *mut BSVRInterfacePoseTransform, // 20
    pub unk28: u64,                                      // 28
    pub unk30: u64,                                      // 30
    pub unk38: u64,                                      // 38
    pub unk40: u64,                                      // 40
    pub unk48: u64,                                      // 48
    pub unk50: u64,                                      // 50
    pub unk58: NiTransform,                              // 58
}

const _: () = assert!(core::mem::size_of::<BSVRInterfaceUnk118>() == 0x90);
const _: () = assert!(core::mem::offset_of!(BSVRInterfaceUnk118, pose_index) == 0x1C);
const _: () = assert!(core::mem::offset_of!(BSVRInterfaceUnk118, pose_transform) == 0x20);
const _: () = assert!(core::mem::offset_of!(BSVRInterfaceUnk118, unk58) == 0x58);

/// C++ `RE::BSVRInterface::COpenVRContext`
#[repr(C)]
pub struct BSVRInterfaceCOpenVRContext {
    pub vr_system: *mut vr::IVRSystem,                    // 00
    pub vr_chaperone: *mut vr::IVRChaperone,              // 08
    pub vr_chaperone_setup: *mut vr::IVRChaperoneSetup,   // 10
    pub vr_compositor: *mut vr::IVRCompositor,            // 18
    pub vr_overlay: *mut vr::IVROverlay,                  // 20
    pub vr_resources: *mut vr::IVRResources,              // 28
    pub vr_render_models: *mut vr::IVRRenderModels,       // 30
    pub vr_extended_display: *mut vr::IVRExtendedDisplay, // 38
    pub vr_settings: *mut vr::IVRSettings,                // 40
    pub vr_applications: *mut vr::IVRApplications,        // 48
    pub vr_tracked_camera: *mut vr::IVRTrackedCamera,     // 50
    pub vr_screenshots: *mut vr::IVRScreenshots,          // 58
    pub vr_driver_manager: *mut vr::IVRDriverManager,     // 60
}

const _: () = assert!(core::mem::size_of::<BSVRInterfaceCOpenVRContext>() == 0x68);

/// C++ `RE::BSVRInterface`
#[repr(C)]
pub struct BSVRInterface {
    pub vtable: *const usize,                                      // 00
    pub vr_overlay_change_source: BSTEventSource<VROverlayChange>, // 08
    pub vr_device_connection_change_source: BSTEventSource<VRDeviceConnectionChange>, // 60
    pub vr_reset_hmd_height_source: BSTEventSource<VRResetHMDHeight>, // B8
    pub unk110: u8,                                                // 110
    pub pad111: [u8; 7],                                           // 111
    pub unk118: *mut BSVRInterfaceUnk118,                          // 118
    pub unk120: u64,                                               // 120
    pub render_transform: NiTransform,                             // 128
    pub game_transform: NiTransform,                               // 15C
    pub vr_context: BSVRInterfaceCOpenVRContext,                   // 190
    pub input_overlay: vr::VROverlayHandle_t,                      // 1F8
    pub is_headset_tracking_ok: bool,                              // 200
    pub pad201: [u8; 7],                                           // 201
}

const _: () = assert!(core::mem::size_of::<BSVRInterface>() == 0x208);
const _: () = assert!(core::mem::offset_of!(BSVRInterface, vtable) == 0x00);
const _: () = assert!(core::mem::offset_of!(BSVRInterface, vr_overlay_change_source) == 0x08);
const _: () =
    assert!(core::mem::offset_of!(BSVRInterface, vr_device_connection_change_source) == 0x60);
const _: () = assert!(core::mem::offset_of!(BSVRInterface, vr_reset_hmd_height_source) == 0xB8);
const _: () = assert!(core::mem::offset_of!(BSVRInterface, unk118) == 0x118);
const _: () = assert!(core::mem::offset_of!(BSVRInterface, render_transform) == 0x128);
const _: () = assert!(core::mem::offset_of!(BSVRInterface, game_transform) == 0x15C);
const _: () = assert!(core::mem::offset_of!(BSVRInterface, vr_context) == 0x190);
const _: () = assert!(core::mem::offset_of!(BSVRInterface, input_overlay) == 0x1F8);
const _: () = assert!(core::mem::offset_of!(BSVRInterface, is_headset_tracking_ok) == 0x200);

impl RttiType for BSVRInterface {
    const RTTI: VariantID = VariantID::new(0, 0, 0x1F5FB80);
}

core_util::inherit!(BSVRInterface => BSTEventSource<VROverlayChange>, vr_overlay_change_source);
core_util::inherit!(
    BSVRInterface => BSTEventSource<VRDeviceConnectionChange>,
    vr_device_connection_change_source
);
core_util::inherit!(
    BSVRInterface => BSTEventSource<VRResetHMDHeight>,
    vr_reset_hmd_height_source
);

impl BSVRInterface {
    pub const RTTI: VariantID = VariantID::new(0, 0, 0x1F5FB80);
    pub const VTABLE: &'static [VariantID] = &[VariantID::new(0, 0, 0x17E69D8)];

    crate::virtual_method! {
        pub const VFUNC_INITIALIZE_VR: usize = 0x00;
        pub fn initialize_vr(&mut self)
    }

    crate::virtual_method! {
        pub const VFUNC_SHUTDOWN_VR: usize = 0x01;
        pub fn shutdown_vr(&mut self)
    }

    crate::virtual_method! {
        pub const VFUNC_POST_PRESENT_HANDOFF: usize = 0x02;
        pub fn post_present_handoff(&mut self)
    }

    crate::virtual_method! {
        pub const VFUNC_SUBMIT: usize = 0x03;
        pub fn submit(
            &mut self,
            texture: *const vr::Texture_t,
            bounds: *const vr::VRTextureBounds_t,
            submit_flags: vr::EVRSubmitFlags
        ) -> vr::EVRCompositorError
    }

    crate::virtual_method! {
        pub const VFUNC_SUBMIT_FOR_EYE: usize = 0x04;
        pub fn submit_for_eye(
            &mut self,
            eye: vr::EVREye,
            texture: *const vr::Texture_t,
            bounds: *const vr::VRTextureBounds_t,
            submit_flags: vr::EVRSubmitFlags
        ) -> vr::EVRCompositorError
    }

    crate::virtual_method! {
        pub const VFUNC_SET_TRACKING_SPACE_AS_STANDING: usize = 0x05;
        pub fn set_tracking_space_as_standing(&mut self)
    }

    crate::virtual_method! {
        pub const VFUNC_SET_TRACKING_SPACE_AS_SEATED: usize = 0x06;
        pub fn set_tracking_space_as_seated(&mut self)
    }

    crate::virtual_method! {
        pub const VFUNC_UNK_07: usize = 0x07;
        pub fn unk_07(&mut self)
    }

    crate::virtual_method! {
        pub const VFUNC_GET_PROJECTION_RAW: usize = 0x08;
        pub fn get_projection_raw(
            &mut self,
            eye: vr::EVREye,
            left: *mut f32,
            right: *mut f32,
            top: *mut f32,
            bottom: *mut f32
        )
    }

    crate::virtual_method! {
        pub const VFUNC_GET_EYE_TO_HEAD_TRANSFORM: usize = 0x09;
        pub fn get_eye_to_head_transform(
            &mut self,
            out: &mut NiTransform,
            get_right_eye: bool
        ) -> *mut NiTransform
    }

    crate::virtual_method! {
        pub const VFUNC_UNK_0A: usize = 0x0A;
        pub fn unk_0a(
            &mut self,
            out: &mut NiTransform,
            get_right_controller: bool,
            unk1: bool
        ) -> *mut NiTransform
    }

    crate::virtual_method! {
        pub const VFUNC_UNK_0B: usize = 0x0B;
        pub fn unk_0b(&mut self)
    }

    crate::virtual_method! {
        pub const VFUNC_GET_TRACKED_DEVICE_INDEX_FOR_HMD: usize = 0x0C;
        pub fn get_tracked_device_index_for_hmd(&mut self) -> vr::TrackedDeviceIndex_t
    }

    crate::virtual_method! {
        pub const VFUNC_GET_TRACKED_DEVICE_INDEX_FOR_HAND: usize = 0x0D;
        pub fn get_tracked_device_index_for_hand(&mut self, get_right_hand: bool) -> vr::TrackedDeviceIndex_t
    }

    crate::virtual_method! {
        pub const VFUNC_TRIGGER_HAPTIC_PULSE: usize = 0x0E;
        pub fn trigger_haptic_pulse(&mut self, do_right_controller: bool, duration: f32)
    }

    crate::virtual_method! {
        pub const VFUNC_UNK_0F: usize = 0x0F;
        pub fn unk_0f(&mut self)
    }

    crate::virtual_method! {
        pub const VFUNC_UNK_10: usize = 0x10;
        pub fn unk_10(&mut self)
    }

    crate::virtual_method! {
        pub const VFUNC_UNK_11: usize = 0x11;
        pub fn unk_11(&mut self)
    }

    crate::virtual_method! {
        pub const VFUNC_GET_RENDER_TARGET_SIZE: usize = 0x12;
        pub fn get_render_target_size(&mut self, width: *mut u32, height: *mut u32)
    }

    crate::virtual_method! {
        pub const VFUNC_UNK_13: usize = 0x13;
        pub fn unk_13(&mut self)
    }

    crate::virtual_method! {
        pub const VFUNC_GET_CONTROLLER_NODE: usize = 0x14;
        pub fn get_controller_node(
            &mut self,
            out: &mut NiPointer<NiNode>,
            hand: BSVRInterfaceHand
        ) -> *mut NiPointer<NiNode>
    }

    crate::virtual_method! {
        pub const VFUNC_UNK_15: usize = 0x15;
        pub fn unk_15(&mut self)
    }

    crate::virtual_method! {
        pub const VFUNC_GET_HMD_DEVICE_TYPE: usize = 0x16;
        pub fn get_hmd_device_type(&mut self) -> BSVRInterfaceHMDDeviceType
    }

    crate::virtual_method! {
        pub const VFUNC_CREATE_CONTROLLER_NODE: usize = 0x17;
        pub fn create_controller_node(
            &mut self,
            out: &mut NiPointer<NiNode>,
            hand: BSVRInterfaceHand
        ) -> *mut NiPointer<NiNode>
    }

    #[inline(always)]
    pub unsafe fn add_vr_overlay_change_sink(&mut self, sink: *mut BSTEventSink<VROverlayChange>) {
        unsafe { self.vr_overlay_change_source.add_event_sink(sink) }
    }

    #[inline(always)]
    pub unsafe fn remove_vr_overlay_change_sink(
        &mut self,
        sink: *mut BSTEventSink<VROverlayChange>,
    ) {
        unsafe { self.vr_overlay_change_source.remove_event_sink(sink) }
    }

    #[inline(always)]
    pub unsafe fn add_vr_device_connection_change_sink(
        &mut self,
        sink: *mut BSTEventSink<VRDeviceConnectionChange>,
    ) {
        unsafe { self.vr_device_connection_change_source.add_event_sink(sink) }
    }

    #[inline(always)]
    pub unsafe fn remove_vr_device_connection_change_sink(
        &mut self,
        sink: *mut BSTEventSink<VRDeviceConnectionChange>,
    ) {
        unsafe {
            self.vr_device_connection_change_source
                .remove_event_sink(sink)
        }
    }

    #[inline(always)]
    pub unsafe fn add_vr_reset_hmd_height_sink(
        &mut self,
        sink: *mut BSTEventSink<VRResetHMDHeight>,
    ) {
        unsafe { self.vr_reset_hmd_height_source.add_event_sink(sink) }
    }

    #[inline(always)]
    pub unsafe fn remove_vr_reset_hmd_height_sink(
        &mut self,
        sink: *mut BSTEventSink<VRResetHMDHeight>,
    ) {
        unsafe { self.vr_reset_hmd_height_source.remove_event_sink(sink) }
    }
}

pub trait BSVRInterfaceExt {
    fn initialize_vr(&mut self);
    fn shutdown_vr(&mut self);
    fn post_present_handoff(&mut self);
    fn submit(
        &mut self,
        texture: *const vr::Texture_t,
        bounds: *const vr::VRTextureBounds_t,
        submit_flags: vr::EVRSubmitFlags,
    ) -> vr::EVRCompositorError;
    fn submit_for_eye(
        &mut self,
        eye: vr::EVREye,
        texture: *const vr::Texture_t,
        bounds: *const vr::VRTextureBounds_t,
        submit_flags: vr::EVRSubmitFlags,
    ) -> vr::EVRCompositorError;
    fn set_tracking_space_as_standing(&mut self);
    fn set_tracking_space_as_seated(&mut self);
    fn unk_07(&mut self);
    fn get_projection_raw(
        &mut self,
        eye: vr::EVREye,
        left: *mut f32,
        right: *mut f32,
        top: *mut f32,
        bottom: *mut f32,
    );
    fn get_eye_to_head_transform(
        &mut self,
        out: &mut NiTransform,
        get_right_eye: bool,
    ) -> *mut NiTransform;
    fn unk_0a(
        &mut self,
        out: &mut NiTransform,
        get_right_controller: bool,
        unk1: bool,
    ) -> *mut NiTransform;
    fn unk_0b(&mut self);
    fn get_tracked_device_index_for_hmd(&mut self) -> vr::TrackedDeviceIndex_t;
    fn get_tracked_device_index_for_hand(
        &mut self,
        get_right_hand: bool,
    ) -> vr::TrackedDeviceIndex_t;
    fn trigger_haptic_pulse(&mut self, do_right_controller: bool, duration: f32);
    fn unk_0f(&mut self);
    fn unk_10(&mut self);
    fn unk_11(&mut self);
    fn get_render_target_size(&mut self, width: *mut u32, height: *mut u32);
    fn unk_13(&mut self);
    fn get_controller_node(
        &mut self,
        out: &mut NiPointer<NiNode>,
        hand: BSVRInterfaceHand,
    ) -> *mut NiPointer<NiNode>;
    fn unk_15(&mut self);
    fn get_hmd_device_type(&mut self) -> BSVRInterfaceHMDDeviceType;
    fn create_controller_node(
        &mut self,
        out: &mut NiPointer<NiNode>,
        hand: BSVRInterfaceHand,
    ) -> *mut NiPointer<NiNode>;
    unsafe fn add_vr_overlay_change_sink(&mut self, sink: *mut BSTEventSink<VROverlayChange>);
    unsafe fn remove_vr_overlay_change_sink(&mut self, sink: *mut BSTEventSink<VROverlayChange>);
    unsafe fn add_vr_device_connection_change_sink(
        &mut self,
        sink: *mut BSTEventSink<VRDeviceConnectionChange>,
    );
    unsafe fn remove_vr_device_connection_change_sink(
        &mut self,
        sink: *mut BSTEventSink<VRDeviceConnectionChange>,
    );
    unsafe fn add_vr_reset_hmd_height_sink(&mut self, sink: *mut BSTEventSink<VRResetHMDHeight>);
    unsafe fn remove_vr_reset_hmd_height_sink(&mut self, sink: *mut BSTEventSink<VRResetHMDHeight>);
}

impl<T> BSVRInterfaceExt for T
where
    T: AsRef<BSVRInterface> + AsMut<BSVRInterface>,
{
    #[inline(always)]
    fn initialize_vr(&mut self) {
        BSVRInterface::initialize_vr(self.as_mut())
    }

    #[inline(always)]
    fn shutdown_vr(&mut self) {
        BSVRInterface::shutdown_vr(self.as_mut())
    }

    #[inline(always)]
    fn post_present_handoff(&mut self) {
        BSVRInterface::post_present_handoff(self.as_mut())
    }

    #[inline(always)]
    fn submit(
        &mut self,
        texture: *const vr::Texture_t,
        bounds: *const vr::VRTextureBounds_t,
        submit_flags: vr::EVRSubmitFlags,
    ) -> vr::EVRCompositorError {
        BSVRInterface::submit(self.as_mut(), texture, bounds, submit_flags)
    }

    #[inline(always)]
    fn submit_for_eye(
        &mut self,
        eye: vr::EVREye,
        texture: *const vr::Texture_t,
        bounds: *const vr::VRTextureBounds_t,
        submit_flags: vr::EVRSubmitFlags,
    ) -> vr::EVRCompositorError {
        BSVRInterface::submit_for_eye(self.as_mut(), eye, texture, bounds, submit_flags)
    }

    #[inline(always)]
    fn set_tracking_space_as_standing(&mut self) {
        BSVRInterface::set_tracking_space_as_standing(self.as_mut())
    }

    #[inline(always)]
    fn set_tracking_space_as_seated(&mut self) {
        BSVRInterface::set_tracking_space_as_seated(self.as_mut())
    }

    #[inline(always)]
    fn unk_07(&mut self) {
        BSVRInterface::unk_07(self.as_mut())
    }

    #[inline(always)]
    fn get_projection_raw(
        &mut self,
        eye: vr::EVREye,
        left: *mut f32,
        right: *mut f32,
        top: *mut f32,
        bottom: *mut f32,
    ) {
        BSVRInterface::get_projection_raw(self.as_mut(), eye, left, right, top, bottom)
    }

    #[inline(always)]
    fn get_eye_to_head_transform(
        &mut self,
        out: &mut NiTransform,
        get_right_eye: bool,
    ) -> *mut NiTransform {
        BSVRInterface::get_eye_to_head_transform(self.as_mut(), out, get_right_eye)
    }

    #[inline(always)]
    fn unk_0a(
        &mut self,
        out: &mut NiTransform,
        get_right_controller: bool,
        unk1: bool,
    ) -> *mut NiTransform {
        BSVRInterface::unk_0a(self.as_mut(), out, get_right_controller, unk1)
    }

    #[inline(always)]
    fn unk_0b(&mut self) {
        BSVRInterface::unk_0b(self.as_mut())
    }

    #[inline(always)]
    fn get_tracked_device_index_for_hmd(&mut self) -> vr::TrackedDeviceIndex_t {
        BSVRInterface::get_tracked_device_index_for_hmd(self.as_mut())
    }

    #[inline(always)]
    fn get_tracked_device_index_for_hand(
        &mut self,
        get_right_hand: bool,
    ) -> vr::TrackedDeviceIndex_t {
        BSVRInterface::get_tracked_device_index_for_hand(self.as_mut(), get_right_hand)
    }

    #[inline(always)]
    fn trigger_haptic_pulse(&mut self, do_right_controller: bool, duration: f32) {
        BSVRInterface::trigger_haptic_pulse(self.as_mut(), do_right_controller, duration)
    }

    #[inline(always)]
    fn unk_0f(&mut self) {
        BSVRInterface::unk_0f(self.as_mut())
    }

    #[inline(always)]
    fn unk_10(&mut self) {
        BSVRInterface::unk_10(self.as_mut())
    }

    #[inline(always)]
    fn unk_11(&mut self) {
        BSVRInterface::unk_11(self.as_mut())
    }

    #[inline(always)]
    fn get_render_target_size(&mut self, width: *mut u32, height: *mut u32) {
        BSVRInterface::get_render_target_size(self.as_mut(), width, height)
    }

    #[inline(always)]
    fn unk_13(&mut self) {
        BSVRInterface::unk_13(self.as_mut())
    }

    #[inline(always)]
    fn get_controller_node(
        &mut self,
        out: &mut NiPointer<NiNode>,
        hand: BSVRInterfaceHand,
    ) -> *mut NiPointer<NiNode> {
        BSVRInterface::get_controller_node(self.as_mut(), out, hand)
    }

    #[inline(always)]
    fn unk_15(&mut self) {
        BSVRInterface::unk_15(self.as_mut())
    }

    #[inline(always)]
    fn get_hmd_device_type(&mut self) -> BSVRInterfaceHMDDeviceType {
        BSVRInterface::get_hmd_device_type(self.as_mut())
    }

    #[inline(always)]
    fn create_controller_node(
        &mut self,
        out: &mut NiPointer<NiNode>,
        hand: BSVRInterfaceHand,
    ) -> *mut NiPointer<NiNode> {
        BSVRInterface::create_controller_node(self.as_mut(), out, hand)
    }

    #[inline(always)]
    unsafe fn add_vr_overlay_change_sink(&mut self, sink: *mut BSTEventSink<VROverlayChange>) {
        unsafe { BSVRInterface::add_vr_overlay_change_sink(self.as_mut(), sink) }
    }

    #[inline(always)]
    unsafe fn remove_vr_overlay_change_sink(&mut self, sink: *mut BSTEventSink<VROverlayChange>) {
        unsafe { BSVRInterface::remove_vr_overlay_change_sink(self.as_mut(), sink) }
    }

    #[inline(always)]
    unsafe fn add_vr_device_connection_change_sink(
        &mut self,
        sink: *mut BSTEventSink<VRDeviceConnectionChange>,
    ) {
        unsafe { BSVRInterface::add_vr_device_connection_change_sink(self.as_mut(), sink) }
    }

    #[inline(always)]
    unsafe fn remove_vr_device_connection_change_sink(
        &mut self,
        sink: *mut BSTEventSink<VRDeviceConnectionChange>,
    ) {
        unsafe { BSVRInterface::remove_vr_device_connection_change_sink(self.as_mut(), sink) }
    }

    #[inline(always)]
    unsafe fn add_vr_reset_hmd_height_sink(&mut self, sink: *mut BSTEventSink<VRResetHMDHeight>) {
        unsafe { BSVRInterface::add_vr_reset_hmd_height_sink(self.as_mut(), sink) }
    }

    #[inline(always)]
    unsafe fn remove_vr_reset_hmd_height_sink(
        &mut self,
        sink: *mut BSTEventSink<VRResetHMDHeight>,
    ) {
        unsafe { BSVRInterface::remove_vr_reset_hmd_height_sink(self.as_mut(), sink) }
    }
}
