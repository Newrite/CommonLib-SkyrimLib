use crate::core_util::inherit;
use core::marker::PhantomData;

/// C++ `RE::NiTSet<T, Allocator>`
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct NiTSet<T> {
    pub data: *mut T,  // 00
    pub capacity: u32, // 08
    pub size: u32,     // 0C
    pub _marker: PhantomData<T>,
}

const _: () = assert!(core::mem::size_of::<NiTSet<*mut u8>>() == 0x10);
const _: () = assert!(core::mem::offset_of!(NiTSet<*mut u8>, data) == 0x00);
const _: () = assert!(core::mem::offset_of!(NiTSet<*mut u8>, capacity) == 0x08);
const _: () = assert!(core::mem::offset_of!(NiTSet<*mut u8>, size) == 0x0C);

impl<T> NiTSet<T> {
    #[inline(always)]
    pub const fn len(&self) -> u32 {
        self.size
    }

    #[inline(always)]
    pub const fn is_empty(&self) -> bool {
        self.capacity == 0
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

/// C++ `RE::NiTObjectSet<T>`
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct NiTObjectSet<T> {
    pub base: NiTSet<T>,
}

const _: () = assert!(core::mem::size_of::<NiTObjectSet<*mut u8>>() == 0x10);
const _: () = assert!(core::mem::offset_of!(NiTObjectSet<*mut u8>, base) == 0x00);

inherit!(for[T] NiTObjectSet<T> : NiTSet<T>, base);

/// C++ `RE::NiTPrimitiveSet<T>`
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct NiTPrimitiveSet<T> {
    pub base: NiTSet<T>,
}

const _: () = assert!(core::mem::size_of::<NiTPrimitiveSet<*mut u8>>() == 0x10);
const _: () = assert!(core::mem::offset_of!(NiTPrimitiveSet<*mut u8>, base) == 0x00);

inherit!(for[T] NiTPrimitiveSet<T> : NiTSet<T>, base);
