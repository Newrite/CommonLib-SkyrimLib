#![allow(non_camel_case_types)]

use crate::relocation::RelocationID;

/// C++ `RE::VRControls::VR_DEVICE`
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum VR_DEVICE {
    kLeftController = 0,
    kRightController = 1,
    kHeadset = 2,
    kTotal = 3,
    kNone = u32::MAX,
}

crate::core_util::abstract_type! { pub type CrosshairPickData; }

impl CrosshairPickData {
    crate::relocation_variable! {
        fn singleton() -> *mut *mut CrosshairPickData => RelocationID::new(515446, 401585), is_ptr
    }

    #[inline(always)]
    pub fn get_singleton() -> *mut CrosshairPickData {
        let singleton = Self::singleton();
        if singleton.is_null() {
            core::ptr::null_mut()
        } else {
            unsafe { *singleton }
        }
    }

    // TODO: `CrosshairPickData` has a full runtime-divergent layout in `CrosshairPickData.h`
    // (0x38 flat, 0x88 VR). This partial translation currently carries only the source-backed
    // `VR_DEVICE` surface and `GetSingleton()` because that is all `PlayerCharacter` needs.
    // Replace this opaque type with an honest runtime-layout translation when field access beyond
    // `VR_DEVICE`/singleton is required.
}
