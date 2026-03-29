#![allow(non_camel_case_types)]

use core::ffi::c_void;

use crate::re::{GMemoryHeap, GSysAllocPaged};
use crate::relocation::RelocationID;

/// C++ `RE::GMemory`
pub struct GMemory;

impl GMemory {
    crate::relocation_variable! {
        fn global_heap_ref() -> &'static mut *mut GMemoryHeap => RelocationID::new(525584, 412058)
    }

    #[inline(always)]
    pub fn set_global_heap(heap: *mut GMemoryHeap) {
        *Self::global_heap_ref() = heap;
    }

    #[inline(always)]
    pub fn get_global_heap() -> *mut GMemoryHeap {
        *Self::global_heap_ref()
    }

    #[inline(always)]
    pub fn create_arena(arena: usize, sys_alloc: *mut GSysAllocPaged) {
        unsafe {
            (*Self::get_global_heap()).create_arena(arena, sys_alloc);
        }
    }

    #[inline(always)]
    pub fn destroy_arena(arena: usize) {
        unsafe {
            (*Self::get_global_heap()).destroy_arena(arena);
        }
    }

    #[inline(always)]
    pub fn arena_is_empty(arena: usize) -> bool {
        unsafe { (*Self::get_global_heap()).arena_is_empty(arena) }
    }

    #[inline(always)]
    pub fn alloc(count: usize) -> *mut c_void {
        unsafe { (*Self::get_global_heap()).alloc(count) }
    }

    #[inline(always)]
    pub fn alloc_aligned(count: usize, align: usize) -> *mut c_void {
        unsafe { (*Self::get_global_heap()).alloc_aligned(count, align) }
    }

    #[inline(always)]
    pub fn alloc_auto_heap(this_ptr: *const c_void, count: usize) -> *mut c_void {
        unsafe { (*Self::get_global_heap()).alloc_auto_heap(this_ptr, count) }
    }

    #[inline(always)]
    pub fn alloc_auto_heap_aligned(
        this_ptr: *const c_void,
        count: usize,
        align: usize,
    ) -> *mut c_void {
        unsafe { (*Self::get_global_heap()).alloc_auto_heap_aligned(this_ptr, count, align) }
    }

    #[inline(always)]
    pub fn alloc_in_heap(heap: *mut GMemoryHeap, count: usize) -> *mut c_void {
        debug_assert!(!heap.is_null());
        unsafe { (*heap).alloc(count) }
    }

    #[inline(always)]
    pub fn alloc_in_heap_aligned(
        heap: *mut GMemoryHeap,
        count: usize,
        align: usize,
    ) -> *mut c_void {
        debug_assert!(!heap.is_null());
        unsafe { (*heap).alloc_aligned(count, align) }
    }

    #[inline(always)]
    pub fn realloc(ptr: *mut c_void, new_count: usize) -> *mut c_void {
        unsafe { (*Self::get_global_heap()).realloc(ptr, new_count) }
    }

    #[inline(always)]
    pub fn free(ptr: *mut c_void) {
        if !ptr.is_null() {
            unsafe {
                (*Self::get_global_heap()).free(ptr);
            }
        }
    }

    #[inline(always)]
    pub fn free_in_heap(heap: *mut GMemoryHeap, ptr: *mut c_void) {
        debug_assert!(!heap.is_null());
        if !ptr.is_null() {
            unsafe {
                (*heap).free(ptr);
            }
        }
    }

    #[inline(always)]
    pub fn get_heap_by_address(ptr: *const c_void) -> *mut GMemoryHeap {
        unsafe { (*Self::get_global_heap()).get_alloc_heap(ptr) }
    }

    #[inline(always)]
    pub fn detect_memory_leaks() -> bool {
        unsafe { (*Self::get_global_heap()).dump_memory_leaks() }
    }
}
