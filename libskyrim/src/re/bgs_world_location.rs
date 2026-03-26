use crate::re::{NiPoint3, TESForm};

/// C++ `RE::BGSWorldLocation`
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BGSWorldLocation {
    pub pos: NiPoint3,       // 00
    pub space: *mut TESForm, // 10
}

const _: () = assert!(core::mem::size_of::<BGSWorldLocation>() == 0x18);
const _: () = assert!(core::mem::offset_of!(BGSWorldLocation, pos) == 0x00);
const _: () = assert!(core::mem::offset_of!(BGSWorldLocation, space) == 0x10);

impl BGSWorldLocation {
    #[inline(always)]
    pub fn get_distance(&self, rhs: &Self) -> f32 {
        if self.space != rhs.space {
            f32::INFINITY
        } else {
            self.pos.get_distance(rhs.pos)
        }
    }

    #[inline(always)]
    pub fn get_squared_distance(&self, rhs: &Self) -> f32 {
        if self.space != rhs.space {
            f32::INFINITY
        } else {
            self.pos.get_squared_distance(rhs.pos)
        }
    }
}
