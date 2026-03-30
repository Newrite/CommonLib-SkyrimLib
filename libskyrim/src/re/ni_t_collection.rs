use core::ffi::{c_char, c_void};
use core::marker::PhantomData;

use crate::re::memory_manager::MemoryManager;
use crate::re::ni_allocator::NiMemEventType;
use crate::re::ni_mem_manager::NiMemManager;
use crate::re::scrap_heap::ScrapHeap;

#[inline]
pub fn ni_malloc(size_in_bytes: usize) -> *mut c_void {
    let mem_manager = NiMemManager::get_singleton();
    assert!(!mem_manager.is_null());
    unsafe {
        (*mem_manager).allocate(
            size_in_bytes,
            0,
            NiMemEventType::Malloc,
            false,
            core::ptr::null::<c_char>(),
            -1,
            core::ptr::null::<c_char>(),
        )
    }
}

#[inline]
pub fn ni_aligned_malloc(size_in_bytes: usize, alignment: usize) -> *mut c_void {
    let mem_manager = NiMemManager::get_singleton();
    assert!(!mem_manager.is_null());
    unsafe {
        (*mem_manager).allocate(
            size_in_bytes,
            alignment,
            NiMemEventType::AlignedMalloc,
            false,
            core::ptr::null::<c_char>(),
            -1,
            core::ptr::null::<c_char>(),
        )
    }
}

#[inline]
pub fn ni_alloc<T>(count: usize) -> *mut T {
    ni_malloc(core::mem::size_of::<T>() * count).cast()
}

#[inline]
pub fn ni_aligned_alloc<T>(count: usize, alignment: usize) -> *mut T {
    ni_aligned_malloc(core::mem::size_of::<T>() * count, alignment).cast()
}

#[inline]
pub fn ni_realloc(memory: *mut c_void, size_in_bytes: usize) -> *mut c_void {
    if size_in_bytes == 0 && !memory.is_null() {
        ni_free(memory);
        return core::ptr::null_mut();
    }
    if memory.is_null() {
        return ni_malloc(size_in_bytes);
    }

    let mem_manager = NiMemManager::get_singleton();
    assert!(!mem_manager.is_null());
    unsafe {
        (*mem_manager).reallocate(
            memory,
            size_in_bytes,
            4,
            NiMemEventType::Realloc,
            false,
            usize::MAX,
            core::ptr::null::<c_char>(),
            -1,
            core::ptr::null::<c_char>(),
        )
    }
}

#[inline]
pub fn ni_aligned_realloc(
    memory: *mut c_void,
    size_in_bytes: usize,
    alignment: usize,
) -> *mut c_void {
    if size_in_bytes == 0 && !memory.is_null() {
        ni_aligned_free(memory);
        return core::ptr::null_mut();
    }
    if memory.is_null() {
        return ni_aligned_malloc(size_in_bytes, alignment);
    }

    let mem_manager = NiMemManager::get_singleton();
    assert!(!mem_manager.is_null());
    unsafe {
        (*mem_manager).reallocate(
            memory,
            size_in_bytes,
            alignment,
            NiMemEventType::AlignedRealloc,
            false,
            usize::MAX,
            core::ptr::null::<c_char>(),
            -1,
            core::ptr::null::<c_char>(),
        )
    }
}

#[inline]
pub fn ni_free(memory: *mut c_void) {
    if !memory.is_null() {
        let mem_manager = NiMemManager::get_singleton();
        assert!(!mem_manager.is_null());
        unsafe {
            (*mem_manager).deallocate(memory, NiMemEventType::Free, usize::MAX);
        }
    }
}

#[inline]
pub fn ni_aligned_free(memory: *mut c_void) {
    if !memory.is_null() {
        let mem_manager = NiMemManager::get_singleton();
        assert!(!mem_manager.is_null());
        unsafe {
            (*mem_manager).deallocate(memory, NiMemEventType::AlignedFree, usize::MAX);
        }
    }
}

#[inline(always)]
pub const fn ni_track_alloc(_memory: *mut c_void, _size_in_bytes: usize) -> bool {
    false
}

#[inline(always)]
pub const fn ni_track_free(_memory: *mut c_void) -> bool {
    false
}

/// C++ `RE::NiTMallocInterface<T>`
#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct NiTMallocInterface<T> {
    _marker: PhantomData<fn() -> T>,
}

const _: () = assert!(core::mem::size_of::<NiTMallocInterface<*mut c_void>>() == 0x0);

impl<T> NiTMallocInterface<T> {
    #[inline(always)]
    pub fn allocate(num_elements: usize) -> *mut T {
        ni_alloc::<T>(num_elements)
    }

    #[inline(always)]
    pub fn deallocate(array: *mut T) {
        ni_free(array.cast());
    }
}

/// C++ `RE::NiTNewInterface<T>`
#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct NiTNewInterface<T> {
    _marker: PhantomData<fn() -> T>,
}

const _: () = assert!(core::mem::size_of::<NiTNewInterface<*mut c_void>>() == 0x0);

impl<T> NiTNewInterface<T> {
    #[inline]
    pub fn allocate(num_elements: usize) -> *mut T {
        let bytes = core::mem::size_of::<usize>() + (core::mem::size_of::<T>() * num_elements);
        let mem = unsafe { crate::ffi::commonlib_malloc(bytes) }.cast::<usize>();
        assert!(!mem.is_null(), "NiTNewInterface allocation failed");
        unsafe {
            *mem = num_elements;
            mem.add(1).cast()
        }
    }

    #[inline]
    pub fn deallocate(array: *mut T) {
        if array.is_null() {
            return;
        }

        unsafe {
            let head = (array.cast::<u8>())
                .sub(core::mem::size_of::<usize>())
                .cast::<usize>();
            for i in 0..*head {
                core::ptr::drop_in_place(array.add(i));
            }
            crate::ffi::commonlib_free(head.cast());
        }
    }
}

/// C++ `RE::NiTScrapHeapInterface<T>`
#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct NiTScrapHeapInterface<T> {
    _marker: PhantomData<fn() -> T>,
}

const _: () = assert!(core::mem::size_of::<NiTScrapHeapInterface<*mut c_void>>() == 0x0);

impl<T> NiTScrapHeapInterface<T> {
    #[inline]
    pub fn allocate(num_elements: usize) -> *mut T {
        let mgr = MemoryManager::get_singleton();
        let allocator = if mgr.is_null() {
            core::ptr::null_mut()
        } else {
            unsafe { (*mgr).get_thread_scrap_heap() }
        };
        assert!(!allocator.is_null(), "NiTScrapHeapInterface: no ScrapHeap");

        let size = core::mem::size_of::<T>() * num_elements;
        let mem = unsafe { (*allocator).allocate(size, core::mem::align_of::<*mut c_void>()) };
        assert!(!mem.is_null(), "NiTScrapHeapInterface allocation failed");
        unsafe {
            core::ptr::write_bytes(mem.cast::<u8>(), 0, size);
        }
        mem.cast()
    }

    #[inline]
    pub fn deallocate(array: *mut T) {
        if array.is_null() {
            return;
        }

        let mgr = MemoryManager::get_singleton();
        let allocator = if mgr.is_null() {
            core::ptr::null_mut()
        } else {
            unsafe { (*mgr).get_thread_scrap_heap() }
        };
        assert!(!allocator.is_null(), "NiTScrapHeapInterface: no ScrapHeap");

        unsafe { (*allocator).deallocate(array.cast()) };
    }
}
