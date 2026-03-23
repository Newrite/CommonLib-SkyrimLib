use core::cmp::Ordering;
use core::ops::{
    Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub, SubAssign,
};

use crate::relocation::RelocationID;

/// C++ `RE::NiPoint3`
#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct NiPoint3 {
    pub x: f32, // 00
    pub y: f32, // 04
    pub z: f32, // 08
}

const _: () = assert!(core::mem::size_of::<NiPoint3>() == 0xC);

impl NiPoint3 {
    #[inline(always)]
    pub const fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    #[inline(always)]
    pub fn cross(self, pt: Self) -> Self {
        Self::new(
            self.y * pt.z - self.z * pt.y,
            self.z * pt.x - self.x * pt.z,
            self.x * pt.y - self.y * pt.x,
        )
    }

    #[inline(always)]
    pub const fn dot(self, pt: Self) -> f32 {
        self.x * pt.x + self.y * pt.y + self.z * pt.z
    }

    #[inline(always)]
    pub fn get_squared_distance(self, pt: Self) -> f32 {
        let dx = pt.x - self.x;
        let dy = pt.y - self.y;
        let dz = pt.z - self.z;
        dx * dx + dy * dy + dz * dz
    }

    #[inline(always)]
    pub fn get_distance(self, pt: Self) -> f32 {
        sqrtf32(self.get_squared_distance(pt))
    }

    #[inline(always)]
    pub fn length(self) -> f32 {
        sqrtf32(self.x * self.x + self.y * self.y + self.z * self.z)
    }

    #[inline(always)]
    pub fn sqr_length(self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    #[inline(always)]
    pub fn unit_cross(self, pt: Self) -> Self {
        let mut cross = self.cross(pt);
        cross.unitize();
        cross
    }

    #[inline]
    pub fn unitize(&mut self) -> f32 {
        let mut len = self.length();
        if len == 1.0 {
            return len;
        }

        if len > f32::EPSILON {
            *self /= len;
        } else {
            self.x = 0.0;
            self.y = 0.0;
            self.z = 0.0;
            len = 0.0;
        }

        len
    }

    // RELOCATION_ID SE: 523887, AE: 410468
    crate::relocation_variable! {
        pub fn zero() -> &'static NiPoint3 => RelocationID::new(523887, 410468)
    }
}

#[inline]
fn sqrtf32(x: f32) -> f32 {
    if x <= 0.0 {
        return 0.0;
    }
    unsafe { sqrtf(x) }
}

unsafe extern "C" {
    fn sqrtf(x: f32) -> f32;
}

impl Index<usize> for NiPoint3 {
    type Output = f32;

    #[inline(always)]
    fn index(&self, index: usize) -> &Self::Output {
        debug_assert!(index < 3);
        if index == 0 {
            &self.x
        } else if index == 1 {
            &self.y
        } else {
            &self.z
        }
    }
}

impl IndexMut<usize> for NiPoint3 {
    #[inline(always)]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        debug_assert!(index < 3);
        if index == 0 {
            &mut self.x
        } else if index == 1 {
            &mut self.y
        } else {
            &mut self.z
        }
    }
}

impl PartialOrd for NiPoint3 {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self.x.partial_cmp(&other.x) {
            Some(Ordering::Equal) => {}
            non_eq => return non_eq,
        }

        match self.y.partial_cmp(&other.y) {
            Some(Ordering::Equal) => {}
            non_eq => return non_eq,
        }

        self.z.partial_cmp(&other.z)
    }
}

impl Add for NiPoint3 {
    type Output = Self;

    #[inline(always)]
    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl Sub for NiPoint3 {
    type Output = Self;

    #[inline(always)]
    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl Mul for NiPoint3 {
    type Output = Self;

    #[inline(always)]
    fn mul(self, rhs: Self) -> Self::Output {
        Self::new(self.x * rhs.x, self.y * rhs.y, self.z * rhs.z)
    }
}

impl Div for NiPoint3 {
    type Output = Self;

    #[inline(always)]
    fn div(self, rhs: Self) -> Self::Output {
        Self::new(self.x / rhs.x, self.y / rhs.y, self.z / rhs.z)
    }
}

impl Add<f32> for NiPoint3 {
    type Output = Self;

    #[inline(always)]
    fn add(self, rhs: f32) -> Self::Output {
        Self::new(self.x + rhs, self.y + rhs, self.z + rhs)
    }
}

impl Sub<f32> for NiPoint3 {
    type Output = Self;

    #[inline(always)]
    fn sub(self, rhs: f32) -> Self::Output {
        Self::new(self.x - rhs, self.y - rhs, self.z - rhs)
    }
}

impl Mul<f32> for NiPoint3 {
    type Output = Self;

    #[inline(always)]
    fn mul(self, rhs: f32) -> Self::Output {
        Self::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

impl Div<f32> for NiPoint3 {
    type Output = Self;

    #[inline(always)]
    fn div(self, rhs: f32) -> Self::Output {
        self * (1.0 / rhs)
    }
}

impl Neg for NiPoint3 {
    type Output = Self;

    #[inline(always)]
    fn neg(self) -> Self::Output {
        Self::new(-self.x, -self.y, -self.z)
    }
}

impl AddAssign for NiPoint3 {
    #[inline(always)]
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl SubAssign for NiPoint3 {
    #[inline(always)]
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

impl MulAssign for NiPoint3 {
    #[inline(always)]
    fn mul_assign(&mut self, rhs: Self) {
        self.x *= rhs.x;
        self.y *= rhs.y;
        self.z *= rhs.z;
    }
}

impl DivAssign for NiPoint3 {
    #[inline(always)]
    fn div_assign(&mut self, rhs: Self) {
        self.x /= rhs.x;
        self.y /= rhs.y;
        self.z /= rhs.z;
    }
}

impl AddAssign<f32> for NiPoint3 {
    #[inline(always)]
    fn add_assign(&mut self, rhs: f32) {
        self.x += rhs;
        self.y += rhs;
        self.z += rhs;
    }
}

impl SubAssign<f32> for NiPoint3 {
    #[inline(always)]
    fn sub_assign(&mut self, rhs: f32) {
        self.x -= rhs;
        self.y -= rhs;
        self.z -= rhs;
    }
}

impl MulAssign<f32> for NiPoint3 {
    #[inline(always)]
    fn mul_assign(&mut self, rhs: f32) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

impl DivAssign<f32> for NiPoint3 {
    #[inline(always)]
    fn div_assign(&mut self, rhs: f32) {
        *self *= 1.0 / rhs;
    }
}

impl Add<NiPoint3> for f32 {
    type Output = NiPoint3;

    #[inline(always)]
    fn add(self, rhs: NiPoint3) -> Self::Output {
        NiPoint3::new(self + rhs.x, self + rhs.y, self + rhs.z)
    }
}

impl Sub<NiPoint3> for f32 {
    type Output = NiPoint3;

    #[inline(always)]
    fn sub(self, rhs: NiPoint3) -> Self::Output {
        NiPoint3::new(self - rhs.x, self - rhs.y, self - rhs.z)
    }
}

impl Mul<NiPoint3> for f32 {
    type Output = NiPoint3;

    #[inline(always)]
    fn mul(self, rhs: NiPoint3) -> Self::Output {
        NiPoint3::new(self * rhs.x, self * rhs.y, self * rhs.z)
    }
}

impl Div<NiPoint3> for f32 {
    type Output = NiPoint3;

    #[inline(always)]
    fn div(self, rhs: NiPoint3) -> Self::Output {
        NiPoint3::new(self / rhs.x, self / rhs.y, self / rhs.z)
    }
}
