use alloc::string::String;
use core::ops::{
    Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub, SubAssign,
};

use crate::re::Color;
use crate::re::NiColorA;

#[repr(C)]
#[derive(Debug, Clone, Copy, Default, PartialEq, bytemuck::Zeroable)]
pub struct NiColor {
    pub red: f32,   // 0x00
    pub green: f32, // 0x04
    pub blue: f32,  // 0x08
}

const _: () = assert!(core::mem::size_of::<NiColor>() == 0xC);

impl NiColor {
    #[inline(always)]
    pub const fn new(red: f32, green: f32, blue: f32) -> Self {
        Self { red, green, blue }
    }

    #[inline(always)]
    pub const fn from_hex_value(hex_value: u32) -> Self {
        Self::new(
            ((hex_value >> 16) & 0xFF) as f32 / 255.0,
            ((hex_value >> 8) & 0xFF) as f32 / 255.0,
            (hex_value & 0xFF) as f32 / 255.0,
        )
    }

    #[inline(always)]
    pub fn to_int(self) -> u32 {
        let r = (self.red * 255.0) as u32;
        let g = (self.green * 255.0) as u32;
        let b = (self.blue * 255.0) as u32;
        ((r & 0xFF) << 16) + ((g & 0xFF) << 8) + (b & 0xFF)
    }

    #[inline]
    pub fn to_hex(self) -> String {
        let r = (self.red * 255.0) as u32;
        let g = (self.green * 255.0) as u32;
        let b = (self.blue * 255.0) as u32;
        alloc::format!("{:X}{:X}{:X}", r, g, b)
    }
}

impl From<Color> for NiColor {
    #[inline(always)]
    fn from(value: Color) -> Self {
        Self::from(&value)
    }
}

impl From<&Color> for NiColor {
    #[inline(always)]
    fn from(value: &Color) -> Self {
        Self::new(
            value.red as f32 / 255.0,
            value.green as f32 / 255.0,
            value.blue as f32 / 255.0,
        )
    }
}

impl From<NiColorA> for NiColor {
    #[inline(always)]
    fn from(value: NiColorA) -> Self {
        Self::from(&value)
    }
}

impl From<&NiColorA> for NiColor {
    #[inline(always)]
    fn from(value: &NiColorA) -> Self {
        Self::new(value.red, value.green, value.blue)
    }
}

impl Index<usize> for NiColor {
    type Output = f32;

    #[inline(always)]
    fn index(&self, index: usize) -> &Self::Output {
        debug_assert!(index < 3);
        match index {
            0 => &self.red,
            1 => &self.green,
            _ => &self.blue,
        }
    }
}

impl IndexMut<usize> for NiColor {
    #[inline(always)]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        debug_assert!(index < 3);
        match index {
            0 => &mut self.red,
            1 => &mut self.green,
            _ => &mut self.blue,
        }
    }
}

impl Add for NiColor {
    type Output = Self;

    #[inline(always)]
    fn add(self, rhs: Self) -> Self::Output {
        Self::new(
            self.red + rhs.red,
            self.green + rhs.green,
            self.blue + rhs.blue,
        )
    }
}

impl AddAssign for NiColor {
    #[inline(always)]
    fn add_assign(&mut self, rhs: Self) {
        self.red += rhs.red;
        self.green += rhs.green;
        self.blue += rhs.blue;
    }
}

impl Sub for NiColor {
    type Output = Self;

    #[inline(always)]
    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(
            self.red - rhs.red,
            self.green - rhs.green,
            self.blue - rhs.blue,
        )
    }
}

impl SubAssign for NiColor {
    #[inline(always)]
    fn sub_assign(&mut self, rhs: Self) {
        self.red -= rhs.red;
        self.green -= rhs.green;
        self.blue -= rhs.blue;
    }
}

impl Neg for NiColor {
    type Output = Self;

    #[inline(always)]
    fn neg(self) -> Self::Output {
        Self::new(-self.red, -self.green, -self.blue)
    }
}

impl Mul for NiColor {
    type Output = Self;

    #[inline(always)]
    fn mul(self, rhs: Self) -> Self::Output {
        Self::new(
            self.red * rhs.red,
            self.green * rhs.green,
            self.blue * rhs.blue,
        )
    }
}

impl MulAssign for NiColor {
    #[inline(always)]
    fn mul_assign(&mut self, rhs: Self) {
        self.red *= rhs.red;
        self.green *= rhs.green;
        self.blue *= rhs.blue;
    }
}

impl Div for NiColor {
    type Output = Self;

    #[inline(always)]
    fn div(self, rhs: Self) -> Self::Output {
        Self::new(
            self.red / rhs.red,
            self.green / rhs.green,
            self.blue / rhs.blue,
        )
    }
}

impl DivAssign for NiColor {
    #[inline(always)]
    fn div_assign(&mut self, rhs: Self) {
        self.red /= rhs.red;
        self.green /= rhs.green;
        self.blue /= rhs.blue;
    }
}

impl Add<f32> for NiColor {
    type Output = Self;

    #[inline(always)]
    fn add(self, rhs: f32) -> Self::Output {
        Self::new(self.red + rhs, self.green + rhs, self.blue + rhs)
    }
}

impl AddAssign<f32> for NiColor {
    #[inline(always)]
    fn add_assign(&mut self, rhs: f32) {
        self.red += rhs;
        self.green += rhs;
        self.blue += rhs;
    }
}

impl Sub<f32> for NiColor {
    type Output = Self;

    #[inline(always)]
    fn sub(self, rhs: f32) -> Self::Output {
        Self::new(self.red - rhs, self.green - rhs, self.blue - rhs)
    }
}

impl SubAssign<f32> for NiColor {
    #[inline(always)]
    fn sub_assign(&mut self, rhs: f32) {
        self.red -= rhs;
        self.green -= rhs;
        self.blue -= rhs;
    }
}

impl Mul<f32> for NiColor {
    type Output = Self;

    #[inline(always)]
    fn mul(self, rhs: f32) -> Self::Output {
        Self::new(self.red * rhs, self.green * rhs, self.blue * rhs)
    }
}

impl MulAssign<f32> for NiColor {
    #[inline(always)]
    fn mul_assign(&mut self, rhs: f32) {
        self.red *= rhs;
        self.green *= rhs;
        self.blue *= rhs;
    }
}

impl Div<f32> for NiColor {
    type Output = Self;

    #[inline(always)]
    fn div(self, rhs: f32) -> Self::Output {
        Self::new(self.red / rhs, self.green / rhs, self.blue / rhs)
    }
}

impl DivAssign<f32> for NiColor {
    #[inline(always)]
    fn div_assign(&mut self, rhs: f32) {
        self.red /= rhs;
        self.green /= rhs;
        self.blue /= rhs;
    }
}

impl Sub<NiColor> for f32 {
    type Output = NiColor;

    #[inline(always)]
    fn sub(self, rhs: NiColor) -> Self::Output {
        NiColor::new(self - rhs.red, self - rhs.green, self - rhs.blue)
    }
}

impl Mul<NiColor> for f32 {
    type Output = NiColor;

    #[inline(always)]
    fn mul(self, rhs: NiColor) -> Self::Output {
        NiColor::new(self * rhs.red, self * rhs.green, self * rhs.blue)
    }
}

impl Div<NiColor> for f32 {
    type Output = NiColor;

    #[inline(always)]
    fn div(self, rhs: NiColor) -> Self::Output {
        NiColor::new(self / rhs.red, self / rhs.green, self / rhs.blue)
    }
}
