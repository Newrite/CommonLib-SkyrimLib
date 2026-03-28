#![allow(non_camel_case_types)]

use crate::core_util::inherit;
use crate::offsets::offsets_rtti::RTTI_hkpAllRayHitCollector;
use crate::offsets::offsets_vtable::VTABLE_hkpAllRayHitCollector;
use crate::re::{
    hkInplaceArray, hkpCdBody, hkpRayHitCollector, hkpShapeRayCastCollectorOutput,
    hkpWorldRayCastOutput,
};
use crate::relocation::{RelocationID, RttiType, VariantID};

/// C++ `RE::hkpAllRayHitCollector`
#[repr(C)]
pub struct hkpAllRayHitCollector {
    pub base: hkpRayHitCollector,                       // 00
    pub hits: hkInplaceArray<hkpWorldRayCastOutput, 8>, // 10
}

const _: () = assert!(core::mem::size_of::<hkpAllRayHitCollector>() == 0x320);
const _: () = assert!(core::mem::offset_of!(hkpAllRayHitCollector, base) == 0x00);
const _: () = assert!(core::mem::offset_of!(hkpAllRayHitCollector, hits) == 0x10);

impl RttiType for hkpAllRayHitCollector {
    const RTTI: VariantID = RTTI_hkpAllRayHitCollector;
}

inherit!(hkpAllRayHitCollector : hkpRayHitCollector, base);

impl hkpAllRayHitCollector {
    pub const RTTI: VariantID = RTTI_hkpAllRayHitCollector;
    pub const VTABLE: &'static [VariantID] = &VTABLE_hkpAllRayHitCollector;

    // override (hkpRayHitCollector)
    crate::relocation_func! {
        pub fn add_ray_hit(&mut self, body: &hkpCdBody, hit_info: &hkpShapeRayCastCollectorOutput)
            => RelocationID::new(77446, 79318)
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

impl Default for hkpAllRayHitCollector {
    #[inline(always)]
    fn default() -> Self {
        Self::new()
    }
}
