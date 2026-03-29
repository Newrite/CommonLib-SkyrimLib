#![allow(non_camel_case_types)]

/// C++ `RE::GRect<T>`
#[repr(C)]
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct GRect<T> {
    pub left: T,   // 00
    pub top: T,    // ??
    pub right: T,  // ??
    pub bottom: T, // ??
}

pub type GRectF = GRect<f32>;
pub type GRectD = GRect<f64>;

const _: () = assert!(core::mem::size_of::<GRectF>() == 0x10);
const _: () = assert!(core::mem::size_of::<GRectD>() == 0x20);

impl<T> GRect<T> {
    #[inline(always)]
    pub const fn new(left: T, top: T, right: T, bottom: T) -> Self {
        Self {
            left,
            top,
            right,
            bottom,
        }
    }
}
