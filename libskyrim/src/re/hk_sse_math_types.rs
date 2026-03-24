#![allow(non_camel_case_types)]

use core::ops::{Index, IndexMut};

/// C++ `RE::hkQuadReal`
#[repr(C, align(16))]
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct hkQuadReal {
    pub values: [f32; 4], // 00
}

const _: () = assert!(core::mem::size_of::<hkQuadReal>() == 0x10);

impl hkQuadReal {
    #[inline(always)]
    pub const fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self {
            values: [x, y, z, w],
        }
    }

    #[inline(always)]
    pub const fn splat(value: f32) -> Self {
        Self { values: [value; 4] }
    }
}

impl Index<usize> for hkQuadReal {
    type Output = f32;

    #[inline(always)]
    fn index(&self, index: usize) -> &Self::Output {
        &self.values[index]
    }
}

impl IndexMut<usize> for hkQuadReal {
    #[inline(always)]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.values[index]
    }
}

/// C++ `RE::hkVector4Comparison::Mask`
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum hkVector4ComparisonMask {
    None = 0,
    X = 1,
    Y = 2,
    XY = 3,
    Z = 4,
    XZ = 5,
    YZ = 6,
    XYZ = 7,
    W = 8,
    XW = 9,
    YW = 10,
    XYW = 11,
    ZW = 12,
    XZW = 13,
    YZW = 14,
    XYZW = 15,
}

/// C++ `RE::hkVector4Comparison`
#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct hkVector4Comparison {
    pub mask: hkQuadReal, // 00
}

const _: () = assert!(core::mem::size_of::<hkVector4Comparison>() == 0x10);
