use core::marker::PhantomData;

/// C++ `RE::BSTSingletonExplicit<T>`
#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct BSTSingletonExplicit<T> {
    pub pad0: u8, // 00
    pub _marker: PhantomData<fn() -> T>,
}

/// C++ `RE::BSTSingletonImplicit<T>`
#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct BSTSingletonImplicit<T> {
    pub pad0: u8, // 00
    pub _marker: PhantomData<fn() -> T>,
}

/// C++ `RE::BSTSingletonSDMOpStaticBuffer<T>`
#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct BSTSingletonSDMOpStaticBuffer<T> {
    pub pad0: u8, // 00
    pub _marker: PhantomData<fn() -> T>,
}

/// C++ `RE::BSTSDMTraits<T, Alloc>`
#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct BSTSDMTraits<T, Alloc = BSTSingletonSDMOpStaticBuffer<T>> {
    pub pad0: u8, // 00
    pub _marker: PhantomData<fn() -> (T, Alloc)>,
}

/// C++ `RE::BSTSingletonSDMBase<Traits>`
#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct BSTSingletonSDMBase<Traits> {
    pub pad0: u8, // 00
    pub _marker: PhantomData<fn() -> Traits>,
}

/// C++ `RE::BSTSingletonSDM<T, Singleton = BSTSingletonSDMOpStaticBuffer>`
#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct BSTSingletonSDM<T> {
    pub pad0: u8, // 00
    pub _marker: PhantomData<fn() -> T>,
}

const _: () = assert!(core::mem::size_of::<BSTSingletonExplicit<u8>>() == 0x1);
const _: () = assert!(core::mem::size_of::<BSTSingletonImplicit<u8>>() == 0x1);
const _: () = assert!(core::mem::size_of::<BSTSingletonSDMOpStaticBuffer<u8>>() == 0x1);
const _: () = assert!(core::mem::size_of::<BSTSDMTraits<u8>>() == 0x1);
const _: () = assert!(core::mem::size_of::<BSTSingletonSDMBase<BSTSDMTraits<u8>>>() == 0x1);
const _: () = assert!(core::mem::size_of::<BSTSingletonSDM<u8>>() == 0x1);
