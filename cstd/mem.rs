use alloc::alloc::{GlobalAlloc, Layout};
use core::ffi::c_void;

/// Global allocator backed by the CommonLib bridge.
pub struct SkyrimAllocator;

unsafe extern "C" {
    fn commonlib_aligned_alloc(alignment: usize, size: usize) -> *mut c_void;
    fn commonlib_aligned_free(ptr: *mut c_void);
}

unsafe impl GlobalAlloc for SkyrimAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        unsafe { commonlib_aligned_alloc(layout.align(), layout.size()) as *mut u8 }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        unsafe { commonlib_aligned_free(ptr as *mut c_void) };
    }
}

#[global_allocator]
pub static A: SkyrimAllocator = SkyrimAllocator;
