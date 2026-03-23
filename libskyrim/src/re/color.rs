use alloc::string::String;
use core::ops::{Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Sub, SubAssign};

use crate::re::NiColor;

#[repr(C)]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, bytemuck::Zeroable)]
pub struct Color {
    pub red: u8,   // 0x00
    pub green: u8, // 0x01
    pub blue: u8,  // 0x02
    pub alpha: u8, // 0x03
}

const _: () = assert!(core::mem::size_of::<Color>() == 0x4);

impl Color {
    #[inline(always)]
    pub const fn new(red: u8, green: u8, blue: u8, alpha: u8) -> Self {
        Self {
            red,
            green,
            blue,
            alpha,
        }
    }

    #[inline(always)]
    pub const fn from_hex_value(hex_value: u32) -> Self {
        Self::new(
            ((hex_value >> 16) & 0xFF) as u8,
            ((hex_value >> 8) & 0xFF) as u8,
            (hex_value & 0xFF) as u8,
            0,
        )
    }

    #[inline(always)]
    pub const fn to_int(self) -> u32 {
        ((self.red as u32 & 0xFF) << 24)
            + ((self.green as u32 & 0xFF) << 16)
            + ((self.blue as u32 & 0xFF) << 8)
            + (self.alpha as u32 & 0xFF)
    }

    #[inline]
    pub fn to_hex(self) -> String {
        alloc::format!(
            "{:X}{:X}{:X}{:X}",
            self.red,
            self.green,
            self.blue,
            self.alpha
        )
    }
}

impl From<NiColor> for Color {
    #[inline(always)]
    fn from(value: NiColor) -> Self {
        Self::from(&value)
    }
}

impl From<&NiColor> for Color {
    #[inline(always)]
    fn from(value: &NiColor) -> Self {
        // Matches the current CommonLibVR Color.cpp implementation.
        Self::new(
            (255.0 / value.red) as u8,
            (255.0 / value.green) as u8,
            (255.0 / value.blue) as u8,
            0,
        )
    }
}

impl Index<usize> for Color {
    type Output = u8;

    #[inline(always)]
    fn index(&self, index: usize) -> &Self::Output {
        debug_assert!(index < 4);
        match index {
            0 => &self.red,
            1 => &self.green,
            2 => &self.blue,
            _ => &self.alpha,
        }
    }
}

impl IndexMut<usize> for Color {
    #[inline(always)]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        debug_assert!(index < 4);
        match index {
            0 => &mut self.red,
            1 => &mut self.green,
            2 => &mut self.blue,
            _ => &mut self.alpha,
        }
    }
}

impl Add for Color {
    type Output = Self;

    #[inline(always)]
    fn add(self, rhs: Self) -> Self::Output {
        Self::new(
            self.red.wrapping_add(rhs.red),
            self.green.wrapping_add(rhs.green),
            self.blue.wrapping_add(rhs.blue),
            self.alpha.wrapping_add(rhs.alpha),
        )
    }
}

impl AddAssign for Color {
    #[inline(always)]
    fn add_assign(&mut self, rhs: Self) {
        self.red = self.red.wrapping_add(rhs.red);
        self.green = self.green.wrapping_add(rhs.green);
        self.blue = self.blue.wrapping_add(rhs.blue);
        self.alpha = self.alpha.wrapping_add(rhs.alpha);
    }
}

impl Sub for Color {
    type Output = Self;

    #[inline(always)]
    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(
            self.red.wrapping_sub(rhs.red),
            self.green.wrapping_sub(rhs.green),
            self.blue.wrapping_sub(rhs.blue),
            self.alpha.wrapping_sub(rhs.alpha),
        )
    }
}

impl SubAssign for Color {
    #[inline(always)]
    fn sub_assign(&mut self, rhs: Self) {
        self.red = self.red.wrapping_sub(rhs.red);
        self.green = self.green.wrapping_sub(rhs.green);
        self.blue = self.blue.wrapping_sub(rhs.blue);
        self.alpha = self.alpha.wrapping_sub(rhs.alpha);
    }
}

impl Mul for Color {
    type Output = Self;

    #[inline(always)]
    fn mul(self, rhs: Self) -> Self::Output {
        Self::new(
            self.red.wrapping_mul(rhs.red),
            self.green.wrapping_mul(rhs.green),
            self.blue.wrapping_mul(rhs.blue),
            self.alpha.wrapping_mul(rhs.alpha),
        )
    }
}

impl MulAssign for Color {
    #[inline(always)]
    fn mul_assign(&mut self, rhs: Self) {
        self.red = self.red.wrapping_mul(rhs.red);
        self.green = self.green.wrapping_mul(rhs.green);
        self.blue = self.blue.wrapping_mul(rhs.blue);
        self.alpha = self.alpha.wrapping_mul(rhs.alpha);
    }
}

impl Div for Color {
    type Output = Self;

    #[inline(always)]
    fn div(self, rhs: Self) -> Self::Output {
        Self::new(
            self.red / rhs.red,
            self.green / rhs.green,
            self.blue / rhs.blue,
            self.alpha / rhs.alpha,
        )
    }
}

impl DivAssign for Color {
    #[inline(always)]
    fn div_assign(&mut self, rhs: Self) {
        self.red /= rhs.red;
        self.green /= rhs.green;
        self.blue /= rhs.blue;
        self.alpha /= rhs.alpha;
    }
}

impl Add<u8> for Color {
    type Output = Self;

    #[inline(always)]
    fn add(self, rhs: u8) -> Self::Output {
        Self::new(
            self.red.wrapping_add(rhs),
            self.green.wrapping_add(rhs),
            self.blue.wrapping_add(rhs),
            self.alpha.wrapping_add(rhs),
        )
    }
}

impl AddAssign<u8> for Color {
    #[inline(always)]
    fn add_assign(&mut self, rhs: u8) {
        self.red = self.red.wrapping_add(rhs);
        self.green = self.green.wrapping_add(rhs);
        self.blue = self.blue.wrapping_add(rhs);
        self.alpha = self.alpha.wrapping_add(rhs);
    }
}

impl Sub<u8> for Color {
    type Output = Self;

    #[inline(always)]
    fn sub(self, rhs: u8) -> Self::Output {
        Self::new(
            self.red.wrapping_sub(rhs),
            self.green.wrapping_sub(rhs),
            self.blue.wrapping_sub(rhs),
            self.alpha.wrapping_sub(rhs),
        )
    }
}

impl SubAssign<u8> for Color {
    #[inline(always)]
    fn sub_assign(&mut self, rhs: u8) {
        self.red = self.red.wrapping_sub(rhs);
        self.green = self.green.wrapping_sub(rhs);
        self.blue = self.blue.wrapping_sub(rhs);
        self.alpha = self.alpha.wrapping_sub(rhs);
    }
}

impl Mul<u8> for Color {
    type Output = Self;

    #[inline(always)]
    fn mul(self, rhs: u8) -> Self::Output {
        Self::new(
            self.red.wrapping_mul(rhs),
            self.green.wrapping_mul(rhs),
            self.blue.wrapping_mul(rhs),
            self.alpha.wrapping_mul(rhs),
        )
    }
}

impl MulAssign<u8> for Color {
    #[inline(always)]
    fn mul_assign(&mut self, rhs: u8) {
        self.red = self.red.wrapping_mul(rhs);
        self.green = self.green.wrapping_mul(rhs);
        self.blue = self.blue.wrapping_mul(rhs);
        self.alpha = self.alpha.wrapping_mul(rhs);
    }
}

impl Div<u8> for Color {
    type Output = Self;

    #[inline(always)]
    fn div(self, rhs: u8) -> Self::Output {
        Self::new(
            self.red / rhs,
            self.green / rhs,
            self.blue / rhs,
            self.alpha / rhs,
        )
    }
}

impl DivAssign<u8> for Color {
    #[inline(always)]
    fn div_assign(&mut self, rhs: u8) {
        self.red /= rhs;
        self.green /= rhs;
        self.blue /= rhs;
        self.alpha /= rhs;
    }
}

impl Sub<Color> for u8 {
    type Output = Color;

    #[inline(always)]
    fn sub(self, rhs: Color) -> Self::Output {
        Color::new(
            self.wrapping_sub(rhs.red),
            self.wrapping_sub(rhs.green),
            self.wrapping_sub(rhs.blue),
            self.wrapping_sub(rhs.alpha),
        )
    }
}

impl Mul<Color> for u8 {
    type Output = Color;

    #[inline(always)]
    fn mul(self, rhs: Color) -> Self::Output {
        Color::new(
            self.wrapping_mul(rhs.red),
            self.wrapping_mul(rhs.green),
            self.wrapping_mul(rhs.blue),
            self.wrapping_mul(rhs.alpha),
        )
    }
}

impl Div<Color> for u8 {
    type Output = Color;

    #[inline(always)]
    fn div(self, rhs: Color) -> Self::Output {
        Color::new(
            self / rhs.red,
            self / rhs.green,
            self / rhs.blue,
            self / rhs.alpha,
        )
    }
}
