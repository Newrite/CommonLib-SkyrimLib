use core::marker::PhantomData;

/// C++ `RE::NiTListItem<T>`
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct NiTListItem<T> {
    pub next: *mut NiTListItem<T>, // 00
    pub prev: *mut NiTListItem<T>, // 08
    pub element: T,                // 10
}

const _: () = assert!(core::mem::size_of::<NiTListItem<*mut u8>>() == 0x18);

/// C++ `RE::NiTList<T>`
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct NiTList<T> {
    pub head: *mut NiTListItem<T>, // 00
    pub tail: *mut NiTListItem<T>, // 08
    pub size: u32,                 // 10
    pub pad14: u32,                // 14
    pub _marker: PhantomData<T>,
}

const _: () = assert!(core::mem::size_of::<NiTList<*mut u8>>() == 0x18);

impl<T> NiTList<T> {
    #[inline(always)]
    pub const fn len(&self) -> u32 {
        self.size
    }

    #[inline(always)]
    pub const fn is_empty(&self) -> bool {
        self.size == 0
    }
}
