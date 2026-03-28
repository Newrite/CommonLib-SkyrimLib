#![allow(non_camel_case_types)]

use crate::core_util::inherit;
use crate::re::hkp_shape::hkpShapeKey;
use crate::re::{HK_INVALID_SHAPE_KEY, hkpShapeRayCastCollectorOutput};

/// C++ `RE::hkpShapeRayCastOutput`
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct hkpShapeRayCastOutput {
    pub base: hkpShapeRayCastCollectorOutput, // 00
    pub shape_keys: [hkpShapeKey; 8],         // 20
    pub shape_key_index: i32,                 // 40
    pub pad44: u32,                           // 44
    pub pad48: u64,                           // 48
}

const _: () = assert!(core::mem::size_of::<hkpShapeRayCastOutput>() == 0x50);
const _: () = assert!(core::mem::offset_of!(hkpShapeRayCastOutput, base) == 0x00);
const _: () = assert!(core::mem::offset_of!(hkpShapeRayCastOutput, shape_keys) == 0x20);
const _: () = assert!(core::mem::offset_of!(hkpShapeRayCastOutput, shape_key_index) == 0x40);
const _: () = assert!(core::mem::offset_of!(hkpShapeRayCastOutput, pad44) == 0x44);
const _: () = assert!(core::mem::offset_of!(hkpShapeRayCastOutput, pad48) == 0x48);

inherit!(hkpShapeRayCastOutput : hkpShapeRayCastCollectorOutput, base);

impl AsRef<hkpShapeRayCastOutput> for hkpShapeRayCastOutput {
    #[inline(always)]
    fn as_ref(&self) -> &Self {
        self
    }
}

impl AsMut<hkpShapeRayCastOutput> for hkpShapeRayCastOutput {
    #[inline(always)]
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl Default for hkpShapeRayCastOutput {
    #[inline(always)]
    fn default() -> Self {
        let mut shape_keys = [0; Self::K_MAX_HIERARCHY_DEPTH];
        shape_keys[0] = HK_INVALID_SHAPE_KEY;

        Self {
            base: hkpShapeRayCastCollectorOutput::default(),
            shape_keys,
            shape_key_index: 0,
            pad44: 0,
            pad48: 0,
        }
    }
}

impl hkpShapeRayCastOutput {
    pub const K_MAX_HIERARCHY_DEPTH: usize = 8;

    #[inline(always)]
    pub fn change_level(&mut self, delta: i32) {
        let next = self.shape_key_index + delta;
        debug_assert!(next < Self::K_MAX_HIERARCHY_DEPTH as i32);
        self.shape_key_index = next;
    }

    #[inline(always)]
    pub fn set_key(&mut self, key: hkpShapeKey) {
        let index = self.shape_key_index as usize;
        debug_assert!(index < Self::K_MAX_HIERARCHY_DEPTH);
        self.shape_keys[index] = key;
    }

    #[inline(always)]
    pub fn reset(&mut self) {
        self.base.reset();
        self.shape_keys[0] = HK_INVALID_SHAPE_KEY;
        self.shape_key_index = 0;
    }
}

pub trait hkpShapeRayCastOutputExt {
    fn has_hit(&self) -> bool;
    fn change_level(&mut self, delta: i32);
    fn set_key(&mut self, key: hkpShapeKey);
    fn reset(&mut self);
}

impl<T: AsRef<hkpShapeRayCastOutput> + AsMut<hkpShapeRayCastOutput>> hkpShapeRayCastOutputExt
    for T
{
    #[inline(always)]
    fn has_hit(&self) -> bool {
        hkpShapeRayCastCollectorOutput::has_hit(self.as_ref().as_ref())
    }

    #[inline(always)]
    fn change_level(&mut self, delta: i32) {
        hkpShapeRayCastOutput::change_level(self.as_mut(), delta)
    }

    #[inline(always)]
    fn set_key(&mut self, key: hkpShapeKey) {
        hkpShapeRayCastOutput::set_key(self.as_mut(), key)
    }

    #[inline(always)]
    fn reset(&mut self) {
        hkpShapeRayCastOutput::reset(self.as_mut())
    }
}
