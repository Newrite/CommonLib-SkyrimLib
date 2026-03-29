#![allow(non_camel_case_types)]

/// C++ `RE::GPoint<T>::BoundsType`
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GPointBoundsType {
    kMin = 0,
    kMax = 1,
}

/// C++ `RE::GPoint<T>`
#[repr(C)]
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct GPoint<T> {
    pub x: T, // 00
    pub y: T, // ??
}

pub type GPointF = GPoint<f32>;
pub type GPointD = GPoint<f64>;

const _: () = assert!(core::mem::size_of::<GPointF>() == 0x8);
const _: () = assert!(core::mem::size_of::<GPointD>() == 0x10);

impl<T> GPoint<T> {
    #[inline(always)]
    pub const fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}
