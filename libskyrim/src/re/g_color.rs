#![allow(non_camel_case_types)]

/// C++ `RE::GColor::RGB32`
#[repr(C)]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct GColorRGB32 {
    pub blue: u8,  // 00
    pub green: u8, // 01
    pub red: u8,   // 02
    pub alpha: u8, // 03
}

const _: () = assert!(core::mem::size_of::<GColorRGB32>() == 0x4);

/// C++ `RE::GColor::ColorData`
#[repr(C)]
#[derive(Clone, Copy)]
pub union GColorColorData {
    pub channels: GColorRGB32,
    pub raw: u32,
}

const _: () = assert!(core::mem::size_of::<GColorColorData>() == 0x4);

/// C++ `RE::GColor`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct GColor {
    pub color_data: GColorColorData, // 00
}

const _: () = assert!(core::mem::size_of::<GColor>() == 0x4);
const _: () = assert!(core::mem::offset_of!(GColor, color_data) == 0x0);

impl Default for GColor {
    #[inline(always)]
    fn default() -> Self {
        Self::from_raw(0)
    }
}

impl PartialEq for GColor {
    #[inline(always)]
    fn eq(&self, other: &Self) -> bool {
        self.raw() == other.raw()
    }
}

impl Eq for GColor {}

impl core::fmt::Debug for GColor {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("GColor")
            .field("raw", &self.raw())
            .field("red", &self.red())
            .field("green", &self.green())
            .field("blue", &self.blue())
            .field("alpha", &self.alpha())
            .finish()
    }
}

impl GColor {
    #[inline(always)]
    pub const fn new(red: u8, green: u8, blue: u8, alpha: u8) -> Self {
        Self::from_raw(
            ((alpha as u32) << 24) | ((red as u32) << 16) | ((green as u32) << 8) | blue as u32,
        )
    }

    #[inline(always)]
    pub const fn from_raw(raw: u32) -> Self {
        Self {
            color_data: GColorColorData { raw },
        }
    }

    #[inline(always)]
    pub fn raw(&self) -> u32 {
        unsafe { self.color_data.raw }
    }

    #[inline(always)]
    pub fn channels(&self) -> GColorRGB32 {
        unsafe { self.color_data.channels }
    }

    #[inline(always)]
    pub fn red(&self) -> u8 {
        self.channels().red
    }

    #[inline(always)]
    pub fn green(&self) -> u8 {
        self.channels().green
    }

    #[inline(always)]
    pub fn blue(&self) -> u8 {
        self.channels().blue
    }

    #[inline(always)]
    pub fn alpha(&self) -> u8 {
        self.channels().alpha
    }
}
