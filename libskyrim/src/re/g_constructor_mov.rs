#![allow(non_camel_case_types)]

use core::marker::PhantomData;

/// C++ `RE::GConstructorMov<T>`
#[repr(C)]
pub struct GConstructorMov<T> {
    pub _pad0: u8,
    pub _marker: PhantomData<fn() -> T>,
}

const _: () = assert!(core::mem::size_of::<GConstructorMov<*mut core::ffi::c_void>>() == 0x1);

impl<T> GConstructorMov<T> {
    #[inline(always)]
    pub unsafe fn construct(ptr: *mut T)
    where
        T: Default,
    {
        unsafe {
            ptr.write(T::default());
        }
    }

    #[inline(always)]
    pub unsafe fn construct_copy(ptr: *mut T, source: &T)
    where
        T: Clone,
    {
        unsafe {
            ptr.write(source.clone());
        }
    }

    #[inline(always)]
    pub unsafe fn construct_alt<S>(ptr: *mut T, source: S)
    where
        S: Into<T>,
    {
        unsafe {
            ptr.write(source.into());
        }
    }

    #[inline(always)]
    pub unsafe fn construct_array(ptr: *mut T, count: usize)
    where
        T: Default,
    {
        for i in 0..count {
            unsafe {
                Self::construct(ptr.add(i));
            }
        }
    }

    #[inline(always)]
    pub unsafe fn construct_array_fill(ptr: *mut T, count: usize, source: &T)
    where
        T: Clone,
    {
        for i in 0..count {
            unsafe {
                Self::construct_copy(ptr.add(i), source);
            }
        }
    }

    #[inline(always)]
    pub unsafe fn construct_array_from(ptr: *mut T, count: usize, source: *const T)
    where
        T: Clone,
    {
        for i in 0..count {
            unsafe {
                Self::construct_copy(ptr.add(i), &*source.add(i));
            }
        }
    }

    #[inline(always)]
    pub unsafe fn destruct(ptr: *mut T) {
        unsafe {
            core::ptr::drop_in_place(ptr);
        }
    }

    #[inline(always)]
    pub unsafe fn destruct_array(ptr: *mut T, count: usize) {
        for i in (0..count).rev() {
            unsafe {
                Self::destruct(ptr.add(i));
            }
        }
    }

    #[inline(always)]
    pub unsafe fn copy_array_forward(dst: *mut T, src: *const T, count: usize) {
        unsafe {
            core::ptr::copy(src, dst, count);
        }
    }

    #[inline(always)]
    pub unsafe fn copy_array_backward(dst: *mut T, src: *const T, count: usize) {
        unsafe {
            core::ptr::copy(src, dst, count);
        }
    }

    #[inline(always)]
    pub const fn is_movable() -> bool {
        true
    }
}
