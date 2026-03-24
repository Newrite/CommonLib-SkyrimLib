#![allow(non_camel_case_types)]

use crate::core_util::EnumSet;

pub type hkObjectIndex = u16;
pub type hkTime = f32;
pub type hkEnum<E, U> = EnumSet<E, U>;

/// C++ `RE::hkResult`
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum hkResult {
    Success = 0,
    Failure = 1,
}

/// C++ `RE::hkHalf`
#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct hkHalf {
    pub value: i16, // 00
}

const _: () = assert!(core::mem::size_of::<hkHalf>() == 0x2);

impl hkHalf {
    #[inline(always)]
    pub const fn new() -> Self {
        Self { value: 0 }
    }

    #[inline(always)]
    pub fn from_float(value: f32) -> Self {
        let mut half = Self::new();
        half.set_float(value);
        half
    }

    #[inline(always)]
    pub fn set_float(&mut self, value: f32) {
        self.value = (value.to_bits() >> 16) as i16;
    }

    #[inline(always)]
    pub fn get_float(self) -> f32 {
        f32::from_bits((self.value as i32 as u32) << 16)
    }
}

impl From<f32> for hkHalf {
    #[inline(always)]
    fn from(value: f32) -> Self {
        Self::from_float(value)
    }
}

impl From<hkHalf> for f32 {
    #[inline(always)]
    fn from(value: hkHalf) -> Self {
        value.get_float()
    }
}

/// C++ `RE::hkUFloat8`
#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct hkUFloat8 {
    pub value: u8, // 00
}

const _: () = assert!(core::mem::size_of::<hkUFloat8>() == 0x1);
