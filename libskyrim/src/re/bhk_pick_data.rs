#![allow(non_camel_case_types)]

use crate::re::{
    hkVector4, hkpAllRayHitCollector, hkpAllRayHitTempCollector, hkpBroadPhaseAabbCache,
    hkpClosestRayHitCollector, hkpWorldRayCastInput, hkpWorldRayCastOutput,
};

/// C++ `RE::bhkPickData`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct bhkPickData {
    pub ray_input: hkpWorldRayCastInput,         // 00
    pub ray_output: hkpWorldRayCastOutput,       // 30
    pub ray: hkVector4,                          // 90
    pub aabb_cache: *mut hkpBroadPhaseAabbCache, // A0
    pub closest_ray_hit_collector: *mut hkpClosestRayHitCollector, // A8
    pub all_ray_hit_collector: *mut hkpAllRayHitCollector, // B0
    pub all_ray_hit_temp_collector: *mut hkpAllRayHitTempCollector, // B8
    pub pick_failed: bool,                       // C0
    pub pad_c1: u8,                              // C1
    pub pad_c2: u16,                             // C2
    pub pad_c4: u32,                             // C4
    pub pad_c8: u32,                             // C8
}

const _: () = assert!(core::mem::size_of::<bhkPickData>() == 0xD0);
const _: () = assert!(core::mem::offset_of!(bhkPickData, ray_input) == 0x00);
const _: () = assert!(core::mem::offset_of!(bhkPickData, ray_output) == 0x30);
const _: () = assert!(core::mem::offset_of!(bhkPickData, ray) == 0x90);
const _: () = assert!(core::mem::offset_of!(bhkPickData, aabb_cache) == 0xA0);
const _: () = assert!(core::mem::offset_of!(bhkPickData, closest_ray_hit_collector) == 0xA8);
const _: () = assert!(core::mem::offset_of!(bhkPickData, all_ray_hit_collector) == 0xB0);
const _: () = assert!(core::mem::offset_of!(bhkPickData, all_ray_hit_temp_collector) == 0xB8);
const _: () = assert!(core::mem::offset_of!(bhkPickData, pick_failed) == 0xC0);

impl Default for bhkPickData {
    #[inline(always)]
    fn default() -> Self {
        Self {
            ray_input: hkpWorldRayCastInput::default(),
            ray_output: hkpWorldRayCastOutput::default(),
            ray: hkVector4::zero(),
            aabb_cache: core::ptr::null_mut(),
            closest_ray_hit_collector: core::ptr::null_mut(),
            all_ray_hit_collector: core::ptr::null_mut(),
            all_ray_hit_temp_collector: core::ptr::null_mut(),
            pick_failed: false,
            pad_c1: 0,
            pad_c2: 0,
            pad_c4: 0,
            pad_c8: 0,
        }
    }
}
