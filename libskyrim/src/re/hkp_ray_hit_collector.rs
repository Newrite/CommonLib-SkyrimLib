#![allow(non_camel_case_types)]

use crate::offsets::offsets_rtti::RTTI_hkpRayHitCollector;
use crate::offsets::offsets_vtable::VTABLE_hkpRayHitCollector;
use crate::re::{hkpCdBody, hkpShapeRayCastCollectorOutput};
use crate::relocation::{RttiType, VariantID};
use crate::virtual_method;

/// C++ `RE::hkpRayHitCollector`
#[repr(C)]
pub struct hkpRayHitCollector {
    pub vtable: *const usize,        // 00
    pub early_out_hit_fraction: f32, // 08
    pub pad0c: u32,                  // 0C
}

const _: () = assert!(core::mem::size_of::<hkpRayHitCollector>() == 0x10);
const _: () = assert!(core::mem::offset_of!(hkpRayHitCollector, vtable) == 0x00);
const _: () = assert!(core::mem::offset_of!(hkpRayHitCollector, early_out_hit_fraction) == 0x08);
const _: () = assert!(core::mem::offset_of!(hkpRayHitCollector, pad0c) == 0x0C);

impl RttiType for hkpRayHitCollector {
    const RTTI: VariantID = RTTI_hkpRayHitCollector;
}

impl AsRef<hkpRayHitCollector> for hkpRayHitCollector {
    #[inline(always)]
    fn as_ref(&self) -> &Self {
        self
    }
}

impl AsMut<hkpRayHitCollector> for hkpRayHitCollector {
    #[inline(always)]
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl hkpRayHitCollector {
    pub const RTTI: VariantID = RTTI_hkpRayHitCollector;
    pub const VTABLE: &'static [VariantID] = &VTABLE_hkpRayHitCollector;

    virtual_method! {
        pub const VFUNC_ADD_RAY_HIT: usize = 0x00;
        pub fn add_ray_hit(&mut self, body: &hkpCdBody, hit_info: &hkpShapeRayCastCollectorOutput)
    }

    virtual_method! {
        pub const VFUNC_DTOR: usize = 0x01;
        pub fn dtor(&mut self)
    }

    #[inline(always)]
    pub fn reset(&mut self) {
        self.early_out_hit_fraction = 1.0;
    }
}

pub trait hkpRayHitCollectorExt {
    fn add_ray_hit(&mut self, body: &hkpCdBody, hit_info: &hkpShapeRayCastCollectorOutput);
    fn reset(&mut self);
}

impl<T: AsRef<hkpRayHitCollector> + AsMut<hkpRayHitCollector>> hkpRayHitCollectorExt for T {
    #[inline(always)]
    fn add_ray_hit(&mut self, body: &hkpCdBody, hit_info: &hkpShapeRayCastCollectorOutput) {
        hkpRayHitCollector::add_ray_hit(self.as_mut(), body, hit_info)
    }

    #[inline(always)]
    fn reset(&mut self) {
        hkpRayHitCollector::reset(self.as_mut())
    }
}
