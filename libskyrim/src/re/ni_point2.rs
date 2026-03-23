use core::ops::{
    Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub, SubAssign,
};

/// C++ `RE::NiPoint2`
#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct NiPoint2 {
    pub x: f32, // 00
    pub y: f32, // 04
}

const _: () = assert!(core::mem::size_of::<NiPoint2>() == 0x8);

impl NiPoint2 {
    #[inline(always)]
    pub const fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    #[inline(always)]
    pub const fn cross(self, pt: Self) -> f32 {
        self.x * pt.y - self.y * pt.x
    }

    #[inline(always)]
    pub const fn dot(self, pt: Self) -> f32 {
        self.x * pt.x + self.y * pt.y
    }

    #[inline(always)]
    pub fn get_squared_distance(self, pt: Self) -> f32 {
        let dx = pt.x - self.x;
        let dy = pt.y - self.y;
        dx * dx + dy * dy
    }

    #[inline(always)]
    pub fn get_distance(self, pt: Self) -> f32 {
        sqrtf32(self.get_squared_distance(pt))
    }

    #[inline(always)]
    pub fn length(self) -> f32 {
        sqrtf32(self.x * self.x + self.y * self.y)
    }

    #[inline(always)]
    pub fn sqr_length(self) -> f32 {
        self.x * self.x + self.y * self.y
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
            len = 0.0;
        }

        len
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

impl Index<usize> for NiPoint2 {
    type Output = f32;

    #[inline(always)]
    fn index(&self, index: usize) -> &Self::Output {
        debug_assert!(index < 2);
        if index == 0 { &self.x } else { &self.y }
    }
}

impl IndexMut<usize> for NiPoint2 {
    #[inline(always)]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        debug_assert!(index < 2);
        if index == 0 { &mut self.x } else { &mut self.y }
    }
}

impl Add for NiPoint2 {
    type Output = Self;

    #[inline(always)]
    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl Sub for NiPoint2 {
    type Output = Self;

    #[inline(always)]
    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl Mul for NiPoint2 {
    type Output = f32;

    #[inline(always)]
    fn mul(self, rhs: Self) -> Self::Output {
        self.dot(rhs)
    }
}

impl Mul<f32> for NiPoint2 {
    type Output = Self;

    #[inline(always)]
    fn mul(self, rhs: f32) -> Self::Output {
        Self::new(self.x * rhs, self.y * rhs)
    }
}

impl Div<f32> for NiPoint2 {
    type Output = Self;

    #[inline(always)]
    fn div(self, rhs: f32) -> Self::Output {
        self * (1.0 / rhs)
    }
}

impl Neg for NiPoint2 {
    type Output = Self;

    #[inline(always)]
    fn neg(self) -> Self::Output {
        Self::new(-self.x, -self.y)
    }
}

impl AddAssign for NiPoint2 {
    #[inline(always)]
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl SubAssign for NiPoint2 {
    #[inline(always)]
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl MulAssign<f32> for NiPoint2 {
    #[inline(always)]
    fn mul_assign(&mut self, rhs: f32) {
        self.x *= rhs;
        self.y *= rhs;
    }
}

impl DivAssign<f32> for NiPoint2 {
    #[inline(always)]
    fn div_assign(&mut self, rhs: f32) {
        *self *= 1.0 / rhs;
    }
}
