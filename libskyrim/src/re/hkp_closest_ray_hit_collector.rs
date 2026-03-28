#![allow(non_camel_case_types)]

use crate::core_util::inherit;
use crate::offsets::offsets_rtti::RTTI_hkpClosestRayHitCollector;
use crate::offsets::offsets_vtable::VTABLE_hkpClosestRayHitCollector;
use crate::re::{
    hkpCdBody, hkpRayHitCollector, hkpShapeRayCastCollectorOutput, hkpWorldRayCastOutput,
};
use crate::relocation::{RelocationID, RttiType, VariantID};

/// C++ `RE::hkpClosestRayHitCollector`
#[repr(C)]
pub struct hkpClosestRayHitCollector {
    pub base: hkpRayHitCollector,       // 00
    pub ray_hit: hkpWorldRayCastOutput, // 10
}

const _: () = assert!(core::mem::size_of::<hkpClosestRayHitCollector>() == 0x70);
const _: () = assert!(core::mem::offset_of!(hkpClosestRayHitCollector, base) == 0x00);
const _: () = assert!(core::mem::offset_of!(hkpClosestRayHitCollector, ray_hit) == 0x10);

impl RttiType for hkpClosestRayHitCollector {
    const RTTI: VariantID = RTTI_hkpClosestRayHitCollector;
}

inherit!(hkpClosestRayHitCollector : hkpRayHitCollector, base);

impl hkpClosestRayHitCollector {
    pub const RTTI: VariantID = RTTI_hkpClosestRayHitCollector;
    pub const VTABLE: &'static [VariantID] = &VTABLE_hkpClosestRayHitCollector;

    // override (hkpRayHitCollector)
    crate::relocation_func! {
        pub fn add_ray_hit(&mut self, body: &hkpCdBody, hit_info: &hkpShapeRayCastCollectorOutput)
            => RelocationID::new(59653, 60338)
    }

    #[inline(always)]
    pub fn new() -> Self {
        Self {
            base: hkpRayHitCollector {
                vtable: Self::VTABLE[0].address() as *const usize,
                early_out_hit_fraction: 1.0,
                pad0c: 0,
            },
            ray_hit: hkpWorldRayCastOutput::default(),
        }
    }

    #[inline(always)]
    pub const fn has_hit(&self) -> bool {
        self.ray_hit.has_hit()
    }

    #[inline(always)]
    pub fn reset(&mut self) {
        self.base.reset();
        self.ray_hit.reset();
    }
}

impl Default for hkpClosestRayHitCollector {
    #[inline(always)]
    fn default() -> Self {
        Self::new()
    }
}
