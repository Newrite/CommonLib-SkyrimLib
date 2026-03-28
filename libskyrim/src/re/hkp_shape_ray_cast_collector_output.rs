#![allow(non_camel_case_types)]

use crate::re::hkp_shape::hkpShapeKey;
use crate::re::{HK_INVALID_SHAPE_KEY, hkVector4};

/// C++ `RE::hkpShapeRayCastCollectorOutput`
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct hkpShapeRayCastCollectorOutput {
    pub normal: hkVector4,      // 00
    pub hit_fraction: f32,      // 10
    pub extra_info: i32,        // 14
    pub shape_key: hkpShapeKey, // 18
    pub pad1c: i32,             // 1C
}

const _: () = assert!(core::mem::size_of::<hkpShapeRayCastCollectorOutput>() == 0x20);
const _: () = assert!(core::mem::offset_of!(hkpShapeRayCastCollectorOutput, normal) == 0x00);
const _: () = assert!(core::mem::offset_of!(hkpShapeRayCastCollectorOutput, hit_fraction) == 0x10);
const _: () = assert!(core::mem::offset_of!(hkpShapeRayCastCollectorOutput, extra_info) == 0x14);
const _: () = assert!(core::mem::offset_of!(hkpShapeRayCastCollectorOutput, shape_key) == 0x18);
const _: () = assert!(core::mem::offset_of!(hkpShapeRayCastCollectorOutput, pad1c) == 0x1C);

impl Default for hkpShapeRayCastCollectorOutput {
    #[inline(always)]
    fn default() -> Self {
        Self {
            normal: hkVector4::zero(),
            hit_fraction: 1.0,
            extra_info: -1,
            shape_key: HK_INVALID_SHAPE_KEY,
            pad1c: 0,
        }
    }
}

impl AsRef<hkpShapeRayCastCollectorOutput> for hkpShapeRayCastCollectorOutput {
    #[inline(always)]
    fn as_ref(&self) -> &Self {
        self
    }
}

impl AsMut<hkpShapeRayCastCollectorOutput> for hkpShapeRayCastCollectorOutput {
    #[inline(always)]
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl hkpShapeRayCastCollectorOutput {
    #[inline(always)]
    pub const fn has_hit(&self) -> bool {
        self.hit_fraction < 1.0
    }

    #[inline(always)]
    pub fn reset(&mut self) {
        self.hit_fraction = 1.0;
        self.shape_key = HK_INVALID_SHAPE_KEY;
        self.extra_info = -1;
    }
}

pub trait hkpShapeRayCastCollectorOutputExt {
    fn has_hit(&self) -> bool;
    fn reset(&mut self);
}

impl<T: AsRef<hkpShapeRayCastCollectorOutput> + AsMut<hkpShapeRayCastCollectorOutput>>
    hkpShapeRayCastCollectorOutputExt for T
{
    #[inline(always)]
    fn has_hit(&self) -> bool {
        hkpShapeRayCastCollectorOutput::has_hit(self.as_ref())
    }

    #[inline(always)]
    fn reset(&mut self) {
        hkpShapeRayCastCollectorOutput::reset(self.as_mut())
    }
}
