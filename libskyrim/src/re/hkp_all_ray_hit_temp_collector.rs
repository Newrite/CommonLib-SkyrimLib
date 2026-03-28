#![allow(non_camel_case_types)]

use crate::core_util::inherit;
use crate::offsets::offsets_rtti::RTTI_hkpAllRayHitTempCollector;
use crate::offsets::offsets_vtable::VTABLE_hkpAllRayHitTempCollector;
use crate::re::{
    hkInplaceArray, hkpCdBody, hkpRayHitCollector, hkpShapeRayCastCollectorOutput,
    hkpWorldRayCastOutput,
};
use crate::relocation::{RelocationID, RttiType, VariantID};

/// C++ `RE::hkpAllRayHitTempCollector`
#[repr(C)]
pub struct hkpAllRayHitTempCollector {
    pub base: hkpRayHitCollector,                       // 00
    pub hits: hkInplaceArray<hkpWorldRayCastOutput, 8>, // 10
}

const _: () = assert!(core::mem::size_of::<hkpAllRayHitTempCollector>() == 0x320);
const _: () = assert!(core::mem::offset_of!(hkpAllRayHitTempCollector, base) == 0x00);
const _: () = assert!(core::mem::offset_of!(hkpAllRayHitTempCollector, hits) == 0x10);

impl RttiType for hkpAllRayHitTempCollector {
    const RTTI: VariantID = RTTI_hkpAllRayHitTempCollector;
}

inherit!(hkpAllRayHitTempCollector : hkpRayHitCollector, base);

impl hkpAllRayHitTempCollector {
    pub const RTTI: VariantID = RTTI_hkpAllRayHitTempCollector;
    pub const VTABLE: &'static [VariantID] = &VTABLE_hkpAllRayHitTempCollector;

    // override (hkpRayHitCollector)
    crate::relocation_func! {
        pub fn add_ray_hit(&mut self, body: &hkpCdBody, hit_info: &hkpShapeRayCastCollectorOutput)
            => RelocationID::new(76693, 78566)
    }

    crate::relocation_func! {
        pub fn reset(&mut self) => RelocationID::new(15073, 15250)
    }

    #[inline(always)]
    pub fn new() -> Self {
        Self {
            base: hkpRayHitCollector {
                vtable: Self::VTABLE[0].address() as *const usize,
                early_out_hit_fraction: 1.0,
                pad0c: 0,
            },
            hits: hkInplaceArray {
                base: Default::default(),
                storage: core::array::from_fn(|_| hkpWorldRayCastOutput::default()),
            },
        }
    }
}

impl Default for hkpAllRayHitTempCollector {
    #[inline(always)]
    fn default() -> Self {
        Self::new()
    }
}
