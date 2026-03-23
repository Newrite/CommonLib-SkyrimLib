use core::ops::{Div, DivAssign, Index, IndexMut, Mul, MulAssign};

use crate::re::Color;
use crate::re::NiColor;

#[repr(C)]
#[derive(Debug, Clone, Copy, Default, PartialEq, bytemuck::Zeroable)]
pub struct NiColorA {
    pub red: f32,   // 0x00
    pub green: f32, // 0x04
    pub blue: f32,  // 0x08
    pub alpha: f32, // 0x0C
}

const _: () = assert!(core::mem::size_of::<NiColorA>() == 0x10);

impl NiColorA {
    #[inline(always)]
    pub const fn new(red: f32, green: f32, blue: f32, alpha: f32) -> Self {
        Self {
            red,
            green,
            blue,
            alpha,
        }
    }

    #[inline(always)]
    pub fn from_color(color: Color) -> Self {
        Self::new(
            color.red as f32 / 255.0,
            color.green as f32 / 255.0,
            color.blue as f32 / 255.0,
            0.0,
        )
    }

    #[inline(always)]
    pub fn from_ni_color(color: NiColor) -> Self {
        Self::new(color.red, color.green, color.blue, 0.0)
    }
}

impl From<Color> for NiColorA {
    #[inline(always)]
    fn from(value: Color) -> Self {
        Self::from_color(value)
    }
}

impl From<&Color> for NiColorA {
    #[inline(always)]
    fn from(value: &Color) -> Self {
        Self::from_color(*value)
    }
}

impl From<NiColor> for NiColorA {
    #[inline(always)]
    fn from(value: NiColor) -> Self {
        Self::from_ni_color(value)
    }
}

impl From<&NiColor> for NiColorA {
    #[inline(always)]
    fn from(value: &NiColor) -> Self {
        Self::from_ni_color(*value)
    }
}

impl Index<usize> for NiColorA {
    type Output = f32;

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

impl IndexMut<usize> for NiColorA {
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

impl Mul<f32> for NiColorA {
    type Output = Self;

    #[inline(always)]
    fn mul(self, rhs: f32) -> Self::Output {
        Self::new(
            self.red * rhs,
            self.green * rhs,
            self.blue * rhs,
            self.alpha * rhs,
        )
    }
}

impl MulAssign<f32> for NiColorA {
    #[inline(always)]
    fn mul_assign(&mut self, rhs: f32) {
        self.red *= rhs;
        self.green *= rhs;
        self.blue *= rhs;
        self.alpha *= rhs;
    }
}

impl Div<f32> for NiColorA {
    type Output = Self;

    #[inline(always)]
    fn div(self, rhs: f32) -> Self::Output {
        Self::new(
            self.red / rhs,
            self.green / rhs,
            self.blue / rhs,
            self.alpha / rhs,
        )
    }
}

impl DivAssign<f32> for NiColorA {
    #[inline(always)]
    fn div_assign(&mut self, rhs: f32) {
        self.red /= rhs;
        self.green /= rhs;
        self.blue /= rhs;
        self.alpha /= rhs;
    }
}
