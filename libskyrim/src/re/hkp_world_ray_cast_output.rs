#![allow(non_camel_case_types)]

use crate::core_util::inherit;
use crate::re::{hkpCollidable, hkpShapeRayCastOutput};

/// C++ `RE::hkpWorldRayCastOutput`
#[repr(C)]
#[derive(Clone, Copy, PartialEq)]
pub struct hkpWorldRayCastOutput {
    pub base: hkpShapeRayCastOutput,           // 00
    pub root_collidable: *const hkpCollidable, // 50
    pub pad58: u64,                            // 58
}

const _: () = assert!(core::mem::size_of::<hkpWorldRayCastOutput>() == 0x60);
const _: () = assert!(core::mem::offset_of!(hkpWorldRayCastOutput, base) == 0x00);
const _: () = assert!(core::mem::offset_of!(hkpWorldRayCastOutput, root_collidable) == 0x50);
const _: () = assert!(core::mem::offset_of!(hkpWorldRayCastOutput, pad58) == 0x58);

inherit!(hkpWorldRayCastOutput : hkpShapeRayCastOutput, base);

impl Default for hkpWorldRayCastOutput {
    #[inline(always)]
    fn default() -> Self {
        Self {
            base: hkpShapeRayCastOutput::default(),
            root_collidable: core::ptr::null(),
            pad58: 0,
        }
    }
}

impl hkpWorldRayCastOutput {
    #[inline(always)]
    pub const fn has_hit(&self) -> bool {
        !self.root_collidable.is_null()
    }

    #[inline(always)]
    pub fn reset(&mut self) {
        self.base.reset();
        self.root_collidable = core::ptr::null();
    }
}
