#![allow(non_camel_case_types)]

use core::ffi::c_void;
use core::marker::PhantomData;

use crate::re::{GAllocatorBaseGH, GAllocatorBaseLH, GConstructorMov};

pub trait GAllocatorTraits<T> {
    const STAT_ID: u32;

    fn alloc(heap_addr: *const c_void, size: usize) -> *mut c_void;
    fn realloc(ptr: *mut c_void, new_size: usize) -> *mut c_void;
    fn free(ptr: *mut c_void);

    #[inline(always)]
    unsafe fn construct(ptr: *mut T)
    where
        T: Default,
    {
        unsafe {
            GConstructorMov::<T>::construct(ptr);
        }
    }

    #[inline(always)]
    unsafe fn construct_copy(ptr: *mut T, source: &T)
    where
        T: Clone,
    {
        unsafe {
            GConstructorMov::<T>::construct_copy(ptr, source);
        }
    }

    #[inline(always)]
    unsafe fn construct_alt<S>(ptr: *mut T, source: S)
    where
        S: Into<T>,
    {
        unsafe {
            GConstructorMov::<T>::construct_alt(ptr, source);
        }
    }

    #[inline(always)]
    unsafe fn construct_array(ptr: *mut T, count: usize)
    where
        T: Default,
    {
        unsafe {
            GConstructorMov::<T>::construct_array(ptr, count);
        }
    }

    #[inline(always)]
    unsafe fn construct_array_fill(ptr: *mut T, count: usize, source: &T)
    where
        T: Clone,
    {
        unsafe {
            GConstructorMov::<T>::construct_array_fill(ptr, count, source);
        }
    }

    #[inline(always)]
    unsafe fn construct_array_from(ptr: *mut T, count: usize, source: *const T)
    where
        T: Clone,
    {
        unsafe {
            GConstructorMov::<T>::construct_array_from(ptr, count, source);
        }
    }

    #[inline(always)]
    unsafe fn destruct(ptr: *mut T) {
        unsafe {
            GConstructorMov::<T>::destruct(ptr);
        }
    }

    #[inline(always)]
    unsafe fn destruct_array(ptr: *mut T, count: usize) {
        unsafe {
            GConstructorMov::<T>::destruct_array(ptr, count);
        }
    }

    #[inline(always)]
    unsafe fn copy_array_forward(dst: *mut T, src: *const T, count: usize) {
        unsafe {
            GConstructorMov::<T>::copy_array_forward(dst, src, count);
        }
    }

    #[inline(always)]
    unsafe fn copy_array_backward(dst: *mut T, src: *const T, count: usize) {
        unsafe {
            GConstructorMov::<T>::copy_array_backward(dst, src, count);
        }
    }

    #[inline(always)]
    fn is_movable() -> bool {
        GConstructorMov::<T>::is_movable()
    }
}

/// C++ `RE::GAllocatorGH<T, SID>`
#[repr(C)]
pub struct GAllocatorGH<T, const SID: u32 = 0> {
    pub _pad0: u8,
    pub _marker: PhantomData<fn() -> T>,
}

const _: () = assert!(core::mem::size_of::<GAllocatorGH<*mut c_void>>() == 0x1);

impl<T, const SID: u32> GAllocatorTraits<T> for GAllocatorGH<T, SID> {
    const STAT_ID: u32 = SID;

    #[inline(always)]
    fn alloc(heap_addr: *const c_void, size: usize) -> *mut c_void {
        GAllocatorBaseGH::<SID>::alloc(heap_addr, size)
    }

    #[inline(always)]
    fn realloc(ptr: *mut c_void, new_size: usize) -> *mut c_void {
        GAllocatorBaseGH::<SID>::realloc(ptr, new_size)
    }

    #[inline(always)]
    fn free(ptr: *mut c_void) {
        GAllocatorBaseGH::<SID>::free(ptr)
    }
}

/// C++ `RE::GAllocatorLH<T, SID>`
#[repr(C)]
pub struct GAllocatorLH<T, const SID: u32 = 0> {
    pub _pad0: u8,
    pub _marker: PhantomData<fn() -> T>,
}

const _: () = assert!(core::mem::size_of::<GAllocatorLH<*mut c_void>>() == 0x1);

impl<T, const SID: u32> GAllocatorTraits<T> for GAllocatorLH<T, SID> {
    const STAT_ID: u32 = SID;

    #[inline(always)]
    fn alloc(heap_addr: *const c_void, size: usize) -> *mut c_void {
        GAllocatorBaseLH::<SID>::alloc(heap_addr, size)
    }

    #[inline(always)]
    fn realloc(ptr: *mut c_void, new_size: usize) -> *mut c_void {
        GAllocatorBaseLH::<SID>::realloc(ptr, new_size)
    }

    #[inline(always)]
    fn free(ptr: *mut c_void) {
        GAllocatorBaseLH::<SID>::free(ptr)
    }
}
