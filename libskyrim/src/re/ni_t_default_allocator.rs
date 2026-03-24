use core::ffi::c_void;
use core::marker::PhantomData;

use crate::re::ni_t_collection::{ni_free, ni_malloc};

/// C++ `RE::NiTDefaultAllocator<T>` / `RE::DFALL<T>`
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct NiTDefaultAllocator<T> {
    _marker: PhantomData<fn() -> T>,
}

const _: () = assert!(core::mem::size_of::<NiTDefaultAllocator<*mut c_void>>() == 0x0);

pub type DFALL<T> = NiTDefaultAllocator<T>;

impl<T> NiTDefaultAllocator<T> {
    #[inline(always)]
    pub fn allocate(&self) -> *mut c_void {
        ni_malloc(core::mem::size_of::<T>())
    }

    #[inline(always)]
    pub fn deallocate(&self, ptr: *mut c_void) {
        ni_free(ptr);
    }
}
