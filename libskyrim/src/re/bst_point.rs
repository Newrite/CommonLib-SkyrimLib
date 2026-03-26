/// Translation of `RE::BSTPoint.h`.

/// C++ `RE::BSTPointDefaultOps<T>`
#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct BSTPointDefaultOps<T> {
    pub pad0: u8,
    pub _marker: core::marker::PhantomData<fn() -> T>,
}

/// C++ `RE::BSTPoint2Base<T>`
#[repr(C)]
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct BSTPoint2Base<T> {
    pub x: T,
    pub y: T,
}

/// C++ `RE::BSTPoint2<T, Ops>`
#[repr(C)]
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct BSTPoint2<T, Ops = BSTPointDefaultOps<T>> {
    pub x: T,
    pub y: T,
    pub _ops: core::marker::PhantomData<fn() -> Ops>,
}

/// C++ `RE::BSTPoint3Base<T>`
#[repr(C)]
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct BSTPoint3Base<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

/// C++ `RE::BSTPoint3<T, Ops>`
#[repr(C)]
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct BSTPoint3<T, Ops = BSTPointDefaultOps<T>> {
    pub x: T,
    pub y: T,
    pub z: T,
    pub _ops: core::marker::PhantomData<fn() -> Ops>,
}

const _: () = assert!(core::mem::size_of::<BSTPointDefaultOps<f32>>() == 0x1);
const _: () = assert!(core::mem::size_of::<BSTPoint2Base<f32>>() == 0x8);
const _: () = assert!(core::mem::size_of::<BSTPoint2<f32>>() == 0x8);
const _: () = assert!(core::mem::size_of::<BSTPoint3Base<f32>>() == 0xC);
const _: () = assert!(core::mem::size_of::<BSTPoint3<f32>>() == 0xC);
