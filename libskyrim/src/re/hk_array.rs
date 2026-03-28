#![allow(non_camel_case_types)]

use core::ffi::c_void;
use core::marker::PhantomData;
use core::ops::{Index, IndexMut};

use crate::core_util::inherit;

/// C++ `RE::hkArrayBase<T>`
#[repr(C)]
pub struct hkArrayBase<T> {
    pub data: *mut T,            // 00
    pub size: i32,               // 08
    pub capacity_and_flags: i32, // 0C
}

const _: () = assert!(core::mem::size_of::<hkArrayBase<*mut c_void>>() == 0x10);
const _: () = assert!(core::mem::offset_of!(hkArrayBase<*mut c_void>, data) == 0x00);
const _: () = assert!(core::mem::offset_of!(hkArrayBase<*mut c_void>, size) == 0x08);
const _: () = assert!(core::mem::offset_of!(hkArrayBase<*mut c_void>, capacity_and_flags) == 0x0C);

impl<T> hkArrayBase<T> {
    pub const CAPACITY_MASK: u32 = 0x3FFF_FFFF;
    pub const FLAG_MASK: u32 = 0xC000_0000;
    pub const DONT_DEALLOC_FLAG: u32 = 1u32 << 31;
    pub const GROWTH_FACTOR: f32 = 1.5;

    #[inline(always)]
    pub const fn data(&self) -> *const T {
        self.data.cast_const()
    }

    #[inline(always)]
    pub const fn data_mut(&mut self) -> *mut T {
        self.data
    }

    #[inline(always)]
    pub const fn is_empty(&self) -> bool {
        self.size == 0
    }

    #[inline(always)]
    pub const fn empty(&self) -> bool {
        self.is_empty()
    }

    #[inline(always)]
    pub const fn size(&self) -> i32 {
        self.size
    }

    #[inline(always)]
    pub fn len(&self) -> usize {
        debug_assert!(self.size >= 0);
        self.size.max(0) as usize
    }

    #[inline(always)]
    pub const fn capacity(&self) -> i32 {
        ((self.capacity_and_flags as u32) & Self::CAPACITY_MASK) as i32
    }

    #[inline(always)]
    pub fn as_slice(&self) -> &[T] {
        debug_assert!(self.size >= 0);
        if self.size <= 0 {
            &[]
        } else {
            unsafe { core::slice::from_raw_parts(self.data(), self.size as usize) }
        }
    }

    #[inline(always)]
    pub fn as_mut_slice(&mut self) -> &mut [T] {
        debug_assert!(self.size >= 0);
        if self.size <= 0 {
            &mut []
        } else {
            unsafe { core::slice::from_raw_parts_mut(self.data_mut(), self.size as usize) }
        }
    }

    #[inline(always)]
    pub fn front(&self) -> Option<&T> {
        self.as_slice().first()
    }

    #[inline(always)]
    pub fn front_mut(&mut self) -> Option<&mut T> {
        self.as_mut_slice().first_mut()
    }

    #[inline(always)]
    pub fn back(&self) -> Option<&T> {
        self.as_slice().last()
    }

    #[inline(always)]
    pub fn back_mut(&mut self) -> Option<&mut T> {
        self.as_mut_slice().last_mut()
    }

    // TODO: SOURCE - add `reserve`, `resize`, and `push_back` after the
    // `hkContainerHeapAllocator::Allocator` / `hkMemoryAllocator` allocation
    // surface is translated from `hkContainerAllocators.h`; `hkArray.h` routes
    // those mutators through Havok allocators rather than a Rust-owned buffer.
}

impl<T> Default for hkArrayBase<T> {
    #[inline(always)]
    fn default() -> Self {
        Self {
            data: core::ptr::null_mut(),
            size: 0,
            capacity_and_flags: 0,
        }
    }
}

impl<T> Index<usize> for hkArrayBase<T> {
    type Output = T;

    #[inline(always)]
    fn index(&self, index: usize) -> &Self::Output {
        &self.as_slice()[index]
    }
}

impl<T> IndexMut<usize> for hkArrayBase<T> {
    #[inline(always)]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.as_mut_slice()[index]
    }
}

/// C++ `RE::hkArray<T>`
#[repr(C)]
pub struct hkArray<T, Allocator = ()> {
    pub base: hkArrayBase<T>, // 00
    pub _allocator: PhantomData<Allocator>,
}

const _: () = assert!(core::mem::size_of::<hkArray<*mut c_void>>() == 0x10);
const _: () = assert!(core::mem::offset_of!(hkArray<*mut c_void>, base) == 0x00);

inherit!(for[T, Allocator] hkArray<T, Allocator> : hkArrayBase<T>);

impl<T, Allocator> Default for hkArray<T, Allocator> {
    #[inline(always)]
    fn default() -> Self {
        Self {
            base: hkArrayBase::default(),
            _allocator: PhantomData,
        }
    }
}

/// C++ `RE::hkInplaceArray<T, N>`
#[repr(C)]
pub struct hkInplaceArray<T, const N: usize, Allocator = ()> {
    pub base: hkArray<T, Allocator>, // 00
    pub storage: [T; N],             // 10
}

const _: () = assert!(core::mem::size_of::<hkInplaceArray<*mut c_void, 1>>() == 0x18);
const _: () = assert!(core::mem::offset_of!(hkInplaceArray<*mut c_void, 1>, base) == 0x00);
const _: () = assert!(core::mem::offset_of!(hkInplaceArray<*mut c_void, 1>, storage) == 0x10);

inherit!(for[T, const N: usize, Allocator] hkInplaceArray<T, N, Allocator> : hkArray<T, Allocator>);
