use core::ffi::c_void;

use crate::ffi;

use super::{TrampolineInterface, api};

/// High-level Rust facade over `SKSE::GetTrampoline()` / `SKSE::AllocTrampoline(...)`.
pub struct Trampoline;

impl Trampoline {
    #[inline(always)]
    pub fn get_interface() -> *mut TrampolineInterface {
        api::get_trampoline_interface()
    }

    #[inline(always)]
    pub fn alloc(size: usize) {
        api::alloc_trampoline(size);
    }

    #[inline(always)]
    pub unsafe fn allocate(size: usize) -> *mut u8 {
        unsafe { ffi::commonlib_trampoline_allocate(size) }
    }

    #[inline(always)]
    pub unsafe fn allocate_as<T>() -> *mut T {
        unsafe { Self::allocate(core::mem::size_of::<T>()).cast() }
    }

    #[inline(always)]
    pub unsafe fn allocate_from_branch_pool(size: usize) -> *mut c_void {
        let intfc = Self::get_interface();
        if intfc.is_null() {
            core::ptr::null_mut()
        } else {
            unsafe { (*intfc).allocate_from_branch_pool(size) }
        }
    }

    #[inline(always)]
    pub unsafe fn allocate_from_local_pool(size: usize) -> *mut c_void {
        let intfc = Self::get_interface();
        if intfc.is_null() {
            core::ptr::null_mut()
        } else {
            unsafe { (*intfc).allocate_from_local_pool(size) }
        }
    }

    #[inline(always)]
    pub unsafe fn write_branch<const N: usize>(src: usize, dst: usize) -> usize {
        match N {
            5 => unsafe { ffi::commonlib_write_branch5(src, dst) },
            6 => unsafe { ffi::commonlib_write_branch6(src, dst) },
            _ => panic!("invalid branch size"),
        }
    }

    #[inline(always)]
    pub unsafe fn write_call<const N: usize>(src: usize, dst: usize) -> usize {
        match N {
            5 => unsafe { ffi::commonlib_write_call5(src, dst) },
            6 => unsafe { ffi::commonlib_write_call6(src, dst) },
            _ => panic!("invalid call size"),
        }
    }
}

#[inline(always)]
pub fn alloc(size: usize) {
    Trampoline::alloc(size);
}

#[inline(always)]
pub unsafe fn allocate(size: usize) -> *mut u8 {
    unsafe { Trampoline::allocate(size) }
}

#[inline(always)]
pub unsafe fn allocate_from_branch_pool(size: usize) -> *mut c_void {
    unsafe { Trampoline::allocate_from_branch_pool(size) }
}

#[inline(always)]
pub unsafe fn allocate_from_local_pool(size: usize) -> *mut c_void {
    unsafe { Trampoline::allocate_from_local_pool(size) }
}

#[inline(always)]
pub unsafe fn write_branch<const N: usize>(src: usize, dst: usize) -> usize {
    unsafe { Trampoline::write_branch::<N>(src, dst) }
}

#[inline(always)]
pub unsafe fn write_call<const N: usize>(src: usize, dst: usize) -> usize {
    unsafe { Trampoline::write_call::<N>(src, dst) }
}
