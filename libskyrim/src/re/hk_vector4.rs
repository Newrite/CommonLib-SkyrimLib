use core::ops::{Add, Div, Mul, Sub};

use crate::re::NiPoint3;
use crate::re::hkQuadReal;

/// C++ `RE::hkVector4`
#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct hkVector4 {
    pub quad: hkQuadReal, // 00
}

const _: () = assert!(core::mem::size_of::<hkVector4>() == 0x10);

impl hkVector4 {
    #[inline(always)]
    pub const fn zero() -> Self {
        Self {
            quad: hkQuadReal::splat(0.0),
        }
    }

    #[inline(always)]
    pub const fn splat(value: f32) -> Self {
        Self {
            quad: hkQuadReal::splat(value),
        }
    }

    #[inline(always)]
    pub const fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self {
            quad: hkQuadReal::new(x, y, z, w),
        }
    }

    #[inline(always)]
    pub const fn from_quad(quad: hkQuadReal) -> Self {
        Self { quad }
    }

    #[inline(always)]
    pub fn is_equal(&self, other: &Self, epsilon: f32) -> bool {
        self.quad
            .values
            .iter()
            .zip(other.quad.values.iter())
            .all(|(lhs, rhs)| (lhs - rhs).abs() <= epsilon)
    }

    #[inline(always)]
    pub fn cross(&self, other: &Self) -> Self {
        Self::new(
            self.quad[1] * other.quad[2] - self.quad[2] * other.quad[1],
            self.quad[2] * other.quad[0] - self.quad[0] * other.quad[2],
            self.quad[0] * other.quad[1] - self.quad[1] * other.quad[0],
            0.0,
        )
    }

    #[inline(always)]
    pub fn dot3(&self, other: &Self) -> f32 {
        self.quad[0] * other.quad[0] + self.quad[1] * other.quad[1] + self.quad[2] * other.quad[2]
    }

    #[inline(always)]
    pub fn dot4(&self, other: &Self) -> f32 {
        self.quad[0] * other.quad[0]
            + self.quad[1] * other.quad[1]
            + self.quad[2] * other.quad[2]
            + self.quad[3] * other.quad[3]
    }

    #[inline(always)]
    pub fn get_distance3(&self, other: &Self) -> f32 {
        sqrtf32(self.get_squared_distance3(other))
    }

    #[inline(always)]
    pub fn get_squared_distance3(&self, other: &Self) -> f32 {
        let dx = self.quad[0] - other.quad[0];
        let dy = self.quad[1] - other.quad[1];
        let dz = self.quad[2] - other.quad[2];
        dx * dx + dy * dy + dz * dz
    }

    #[inline(always)]
    pub fn length3(&self) -> f32 {
        sqrtf32(self.sqr_length3())
    }

    #[inline(always)]
    pub fn sqr_length3(&self) -> f32 {
        self.dot3(self)
    }

    #[inline(always)]
    pub fn length4(&self) -> f32 {
        sqrtf32(self.sqr_length4())
    }

    #[inline(always)]
    pub fn sqr_length4(&self) -> f32 {
        self.dot4(self)
    }
}

impl From<NiPoint3> for hkVector4 {
    #[inline(always)]
    fn from(value: NiPoint3) -> Self {
        Self::new(value.x, value.y, value.z, 0.0)
    }
}

impl Add for hkVector4 {
    type Output = Self;

    #[inline(always)]
    fn add(self, rhs: Self) -> Self::Output {
        Self::new(
            self.quad[0] + rhs.quad[0],
            self.quad[1] + rhs.quad[1],
            self.quad[2] + rhs.quad[2],
            self.quad[3] + rhs.quad[3],
        )
    }
}

impl Sub for hkVector4 {
    type Output = Self;

    #[inline(always)]
    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(
            self.quad[0] - rhs.quad[0],
            self.quad[1] - rhs.quad[1],
            self.quad[2] - rhs.quad[2],
            self.quad[3] - rhs.quad[3],
        )
    }
}

impl Mul for hkVector4 {
    type Output = Self;

    #[inline(always)]
    fn mul(self, rhs: Self) -> Self::Output {
        Self::new(
            self.quad[0] * rhs.quad[0],
            self.quad[1] * rhs.quad[1],
            self.quad[2] * rhs.quad[2],
            self.quad[3] * rhs.quad[3],
        )
    }
}

impl Div for hkVector4 {
    type Output = Self;

    #[inline(always)]
    fn div(self, rhs: Self) -> Self::Output {
        Self::new(
            self.quad[0] / rhs.quad[0],
            self.quad[1] / rhs.quad[1],
            self.quad[2] / rhs.quad[2],
            self.quad[3] / rhs.quad[3],
        )
    }
}

#[inline]
fn sqrtf32(x: f32) -> f32 {
    unsafe { sqrtf(x) }
}

unsafe extern "C" {
    fn sqrtf(x: f32) -> f32;
}
