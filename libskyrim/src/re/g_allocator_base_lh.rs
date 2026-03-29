#![allow(non_camel_case_types)]

use core::ffi::c_void;

use crate::re::GMemory;

/// C++ `RE::GAllocatorBaseLH<SID>`
#[repr(C)]
pub struct GAllocatorBaseLH<const SID: u32 = 0> {
    pub _pad0: u8,
}

const _: () = assert!(core::mem::size_of::<GAllocatorBaseLH>() == 0x1);

impl<const SID: u32> GAllocatorBaseLH<SID> {
    pub const kStatID: u32 = SID;

    #[inline(always)]
    pub fn alloc(heap_addr: *const c_void, size: usize) -> *mut c_void {
        GMemory::alloc_auto_heap(heap_addr, size)
    }

    #[inline(always)]
    pub fn realloc(ptr: *mut c_void, new_size: usize) -> *mut c_void {
        GMemory::realloc(ptr, new_size)
    }

    #[inline(always)]
    pub fn free(ptr: *mut c_void) {
        GMemory::free(ptr)
    }
}
