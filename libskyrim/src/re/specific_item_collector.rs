#![allow(non_camel_case_types)]

use crate::core_util::inherit;
use crate::offsets::offsets_rtti::RTTI_SpecificItemCollector;
use crate::offsets::offsets_vtable::VTABLE_SpecificItemCollector;
use crate::re::{CFilter, hkpClosestRayHitCollector};
use crate::relocation::{RttiType, VariantID};

/// C++ `RE::SpecificItemCollector`
#[repr(C)]
pub struct SpecificItemCollector {
    pub base: hkpClosestRayHitCollector, // 00
    pub filter: CFilter,                 // 70
    pub pad74: [u8; 0x0C],               // 74
}

const _: () = assert!(core::mem::size_of::<SpecificItemCollector>() == 0x80);
const _: () = assert!(core::mem::offset_of!(SpecificItemCollector, base) == 0x00);
const _: () = assert!(core::mem::offset_of!(SpecificItemCollector, filter) == 0x70);

impl RttiType for SpecificItemCollector {
    const RTTI: VariantID = RTTI_SpecificItemCollector;
}

inherit!(SpecificItemCollector : hkpClosestRayHitCollector, base);

impl SpecificItemCollector {
    pub const RTTI: VariantID = RTTI_SpecificItemCollector;
    pub const VTABLE: &'static [VariantID] = &VTABLE_SpecificItemCollector;

    crate::virtual_method! {
        pub const VFUNC_DTOR: usize = 0x00;
        pub fn dtor(&mut self)
    }

    // override (hkpClosestRayHitCollector)
    crate::virtual_method! {
        pub const VFUNC_ADD_RAY_HIT: usize = 0x01;
        pub fn add_ray_hit(&mut self, body: &crate::re::hkpCdBody, hit_info: &crate::re::hkpShapeRayCastCollectorOutput)
    }

    #[inline(always)]
    pub fn new() -> Self {
        let mut base = hkpClosestRayHitCollector::new();
        base.base.vtable = Self::VTABLE[0].address() as *const usize;

        Self {
            base,
            filter: CFilter::default(),
            pad74: [0; 0x0C],
        }
    }
}

impl Default for SpecificItemCollector {
    #[inline(always)]
    fn default() -> Self {
        Self::new()
    }
}
