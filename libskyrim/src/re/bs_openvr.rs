use core_util::Enum;

use core::ptr;

use crate::re::{
    BSVRInterface, BSVRInterfaceCOpenVRContext, BSVRInterfaceHMDDeviceType, BSVRInterfaceHand,
    NiNode, NiPointer, NiSourceTexture, NiTransform,
};
use crate::relocation::{Offset, Relocation, RttiType, VariantID};
use crate::rex::openvr::vr;

/// C++ `RE::BSOpenVR::Unk238`
#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct BSOpenVRUnk238 {
    pub unk00: u64, // 00
    pub unk08: u64, // 08
    pub unk10: u64, // 10
    pub unk18: u64, // 18
    pub unk20: u64, // 20
    pub unk28: u64, // 28
    pub unk30: u64, // 30
    pub unk38: u64, // 38
}

const _: () = assert!(core::mem::size_of::<BSOpenVRUnk238>() == 0x40);

/// C++ `RE::BSOpenVR`
#[repr(C)]
pub struct BSOpenVR {
    pub base: BSVRInterface,                                             // 000
    pub vr_system: *mut vr::IVRSystem,                                   // 208
    pub unk210: *mut core::ffi::c_void,                                  // 210
    pub unk218: u64,                                                     // 218
    pub unk220: u64,                                                     // 220
    pub unk228: u64,                                                     // 228
    pub unk230: NiPointer<NiSourceTexture>,                              // 230
    pub unk238: [BSOpenVRUnk238; 4],                                     // 238
    pub unk338: u64,                                                     // 338
    pub unk340: [u64; 9],                                                // 340
    pub controller_nodes: [NiPointer<NiNode>; BSVRInterfaceHand::TOTAL], // 388
    pub hmd_device_type: Enum<BSVRInterfaceHMDDeviceType, u32>,          // 398
    pub eye_to_head_transform: [NiTransform; 2],                         // 39C
    pub pad404: u32,                                                     // 404
}

const _: () = assert!(core::mem::size_of::<BSOpenVR>() == 0x408);
const _: () = assert!(core::mem::offset_of!(BSOpenVR, base) == 0x000);
const _: () = assert!(core::mem::offset_of!(BSOpenVR, vr_system) == 0x208);
const _: () = assert!(core::mem::offset_of!(BSOpenVR, unk230) == 0x230);
const _: () = assert!(core::mem::offset_of!(BSOpenVR, unk238) == 0x238);
const _: () = assert!(core::mem::offset_of!(BSOpenVR, unk338) == 0x338);
const _: () = assert!(core::mem::offset_of!(BSOpenVR, unk340) == 0x340);
const _: () = assert!(core::mem::offset_of!(BSOpenVR, controller_nodes) == 0x388);
const _: () = assert!(core::mem::offset_of!(BSOpenVR, hmd_device_type) == 0x398);
const _: () = assert!(core::mem::offset_of!(BSOpenVR, eye_to_head_transform) == 0x39C);

impl RttiType for BSOpenVR {
    const RTTI: VariantID = VariantID::new(0, 0, 0x1F5FC78);
}

core_util::inherit!(BSOpenVR : BSVRInterface, base);

impl BSOpenVR {
    pub const RTTI: VariantID = VariantID::new(0, 0, 0x1F5FC78);
    pub const VTABLE: &'static [VariantID] = &[VariantID::new(0, 0, 0x17E6AA0)];
    pub const HAPTIC_PULSE_SCALE_DEFAULT: f32 = 3999.0;

    crate::relocation_func! {
        fn get_ivr_compositor_raw() -> *mut vr::IVRCompositor => Offset::new(0xC57880)
    }

    crate::relocation_func! {
        fn get_ivr_overlay_from_context_raw(
            vr_context: *mut BSVRInterfaceCOpenVRContext
        ) -> *mut vr::IVROverlay => Offset::new(0x8A0110)
    }

    crate::relocation_func! {
        fn get_ivr_render_models_raw() -> *mut vr::IVRRenderModels => Offset::new(0xC57920)
    }

    crate::relocation_func! {
        fn get_ivr_settings_raw() -> *mut vr::IVRSettings => Offset::new(0xC579C0)
    }

    crate::relocation_func! {
        fn get_ivr_system_raw() -> *mut vr::IVRSystem => Offset::new(0xC57A60)
    }

    #[inline(always)]
    fn singleton_storage() -> *mut *mut BSOpenVR {
        if !crate::runtime::is_vr() {
            return ptr::null_mut();
        }
        unsafe {
            Relocation::<*mut *mut BSOpenVR>::try_new(Offset::new(0x2FEB9B0))
                .ok()
                .map(|relocation| relocation.get())
                .unwrap_or(ptr::null_mut())
        }
    }

    #[inline(always)]
    fn haptic_pulse_scale_ptr() -> *mut f32 {
        if !crate::runtime::is_vr() {
            return ptr::null_mut();
        }
        unsafe {
            Relocation::<*mut f32>::try_new(Offset::new(0x17E6E50))
                .ok()
                .map(|relocation| relocation.get())
                .unwrap_or(ptr::null_mut())
        }
    }

    #[inline(always)]
    pub fn get_singleton() -> *mut BSOpenVR {
        let storage = Self::singleton_storage();
        if storage.is_null() {
            ptr::null_mut()
        } else {
            unsafe { *storage }
        }
    }

    #[inline(always)]
    pub fn get_ivr_compositor() -> *mut vr::IVRCompositor {
        if crate::runtime::is_vr() {
            Self::get_ivr_compositor_raw()
        } else {
            ptr::null_mut()
        }
    }

    #[inline(always)]
    pub fn get_ivr_overlay_from_context(
        vr_context: *mut BSVRInterfaceCOpenVRContext,
    ) -> *mut vr::IVROverlay {
        if crate::runtime::is_vr() {
            Self::get_ivr_overlay_from_context_raw(vr_context)
        } else {
            ptr::null_mut()
        }
    }

    #[inline(always)]
    pub fn get_ivr_render_models() -> *mut vr::IVRRenderModels {
        if crate::runtime::is_vr() {
            Self::get_ivr_render_models_raw()
        } else {
            ptr::null_mut()
        }
    }

    #[inline(always)]
    pub fn get_ivr_settings() -> *mut vr::IVRSettings {
        if crate::runtime::is_vr() {
            Self::get_ivr_settings_raw()
        } else {
            ptr::null_mut()
        }
    }

    #[inline(always)]
    pub fn get_ivr_system() -> *mut vr::IVRSystem {
        if crate::runtime::is_vr() {
            Self::get_ivr_system_raw()
        } else {
            ptr::null_mut()
        }
    }

    #[inline(always)]
    pub fn get_haptic_pulse_scale() -> f32 {
        let haptic_pulse_scale = Self::haptic_pulse_scale_ptr();
        if haptic_pulse_scale.is_null() {
            Self::HAPTIC_PULSE_SCALE_DEFAULT
        } else {
            unsafe { *haptic_pulse_scale }
        }
    }

    #[inline(always)]
    pub fn set_haptic_pulse_scale(mut value: f32) {
        let haptic_pulse_scale = Self::haptic_pulse_scale_ptr();
        if haptic_pulse_scale.is_null() {
            return;
        }
        value = value.clamp(0.0, 20000.0);
        unsafe {
            *haptic_pulse_scale = value;
        }
    }

    #[inline(always)]
    pub fn get_clean_ivr_overlay() -> *mut vr::IVROverlay {
        // TODO: `BSOpenVR::GetCleanIVROverlay()` depends on the external
        // OpenVR SDK constant `vr::IVROverlay_Version`, which is not vendored
        // in this repository. Keep returning null until `rex::openvr` gains
        // that exact source-backed C-string constant instead of guessing it.
        ptr::null_mut()
    }

    #[inline(always)]
    pub const fn hmd_device_type_storage(&self) -> Enum<BSVRInterfaceHMDDeviceType, u32> {
        self.hmd_device_type
    }

    #[inline(always)]
    pub fn try_get_hmd_device_type(&self) -> Option<BSVRInterfaceHMDDeviceType> {
        self.hmd_device_type_storage().get()
    }

    #[inline(always)]
    pub fn get_hmd_device_type(&self) -> BSVRInterfaceHMDDeviceType {
        self.try_get_hmd_device_type()
            .unwrap_or(BSVRInterfaceHMDDeviceType::Lighthouse)
    }
}
