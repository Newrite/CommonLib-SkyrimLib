use core::marker::PhantomData;

/// C++ `RE::NiTArray<T, Allocator>`
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct NiTArray<T> {
    pub vtable: *const usize, // 00
    pub data: *mut T,         // 08
    pub capacity: u16,        // 10
    pub free_idx: u16,        // 12
    pub size: u16,            // 14
    pub growth_size: u16,     // 16
    pub _marker: PhantomData<T>,
}

const _: () = assert!(core::mem::size_of::<NiTArray<*mut u8>>() == 0x18);

impl<T> NiTArray<T> {
    #[inline(always)]
    pub const fn len(&self) -> u16 {
        self.size
    }

    #[inline(always)]
    pub const fn is_empty(&self) -> bool {
        self.size == 0
    }

    #[inline(always)]
    pub const fn as_ptr(&self) -> *const T {
        self.data
    }

    #[inline(always)]
    pub const fn as_mut_ptr(&mut self) -> *mut T {
        self.data
    }
}

/// C++ `RE::NiTPrimitiveArray<T>`
pub type NiTPrimitiveArray<T> = NiTArray<T>;

/// C++ `RE::NiTObjectArray<T>`
pub type NiTObjectArray<T> = NiTArray<T>;
