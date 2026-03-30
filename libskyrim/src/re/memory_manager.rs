use core::ffi::c_void;

use crate::re::bs_small_block_allocator::BSSmallBlockAllocator;
use crate::re::compacting_store::CompactingStore;
use crate::re::{IMemoryHeap, ScrapHeap};
use crate::relocation::RelocationID;

/// C++ `RE::MemoryManager::ThreadScrapHeap`
#[repr(C)]
pub struct ThreadScrapHeap {
    pub heap: ScrapHeap,            // 00
    pub next: *mut ThreadScrapHeap, // 90
    pub owning_thread: u32,         // 98
    pub pad9c: u32,                 // 9C
}

const _: () = assert!(core::mem::size_of::<ThreadScrapHeap>() == 0xA0);
const _: () = assert!(core::mem::offset_of!(ThreadScrapHeap, heap) == 0x00);
const _: () = assert!(core::mem::offset_of!(ThreadScrapHeap, next) == 0x90);
const _: () = assert!(core::mem::offset_of!(ThreadScrapHeap, owning_thread) == 0x98);

/// C++ `RE::MemoryManager`
#[repr(C)]
pub struct MemoryManager {
    pub initialized: bool,                                 // 000
    pub pad001: u8,                                        // 001
    pub num_heaps: u16,                                    // 002
    pub num_physical_heaps: u16,                           // 004
    pub pad006: u16,                                       // 006
    pub heaps: *mut *mut IMemoryHeap,                      // 008
    pub allow_other_context_allocs: *mut bool,             // 010
    pub heaps_by_context: [*mut IMemoryHeap; 127],         // 018
    pub thread_scrap_heap: *mut ThreadScrapHeap,           // 410
    pub physical_heaps: *mut *mut IMemoryHeap,             // 418
    pub big_alloc_heap: *mut IMemoryHeap,                  // 420
    pub emergency_heap: *mut IMemoryHeap,                  // 428
    pub small_block_allocator: *mut BSSmallBlockAllocator, // 430
    pub compacting_store: *mut CompactingStore::Store,     // 438
    pub external_havok_allocator: *mut IMemoryHeap,        // 440
    pub special_heaps: bool,                               // 448
    pub allow_pool_use: bool,                              // 449
    pub pad44a: u16,                                       // 44A
    pub sys_alloc_bytes: u32,                              // 44C
    pub malloc_bytes: u32,                                 // 450
    pub alignment_for_pools: u32,                          // 454
    pub main_thread_memory_problem_pass_signal: u32,       // 458
    pub pad45c: u32,                                       // 45C
    pub failed_allocation_size: usize,                     // 460
    pub num_memory_problem_passes_run: u32,                // 468
    pub pad46c: u32,                                       // 46C
    pub time_of_last_memory_problem_pass: usize,           // 470
    pub default_heap: *mut IMemoryHeap,                    // 478
}

const _: () = assert!(core::mem::size_of::<MemoryManager>() == 0x480);
const _: () = assert!(core::mem::offset_of!(MemoryManager, initialized) == 0x000);
const _: () = assert!(core::mem::offset_of!(MemoryManager, heaps) == 0x008);
const _: () = assert!(core::mem::offset_of!(MemoryManager, heaps_by_context) == 0x018);
const _: () = assert!(core::mem::offset_of!(MemoryManager, thread_scrap_heap) == 0x410);
const _: () = assert!(core::mem::offset_of!(MemoryManager, small_block_allocator) == 0x430);
const _: () = assert!(core::mem::offset_of!(MemoryManager, compacting_store) == 0x438);
const _: () = assert!(core::mem::offset_of!(MemoryManager, failed_allocation_size) == 0x460);
const _: () = assert!(core::mem::offset_of!(MemoryManager, default_heap) == 0x478);

impl MemoryManager {
    crate::relocation_func! {
        pub fn get_singleton() -> *mut MemoryManager
            => RelocationID::new(11045, 11141)
    }

    crate::relocation_func! {
        pub fn allocate(
            &mut self,
            size: usize,
            alignment: i32,
            alignment_required: bool
        ) -> *mut c_void => RelocationID::new(66859, 68115)
    }

    crate::relocation_func! {
        pub fn deallocate(&mut self, mem: *mut c_void, alignment_required: bool)
            => RelocationID::new(66861, 68117)
    }

    crate::relocation_func! {
        pub fn get_thread_scrap_heap(&mut self) -> *mut ScrapHeap
            => RelocationID::new(66841, 68088)
    }

    crate::relocation_func! {
        pub fn reallocate(
            &mut self,
            old_mem: *mut c_void,
            new_size: usize,
            alignment: i32,
            aligned: bool
        ) -> *mut c_void => RelocationID::new(66860, 68116)
    }

    crate::relocation_func! {
        pub fn register_memory_manager(&mut self)
            => RelocationID::new(35199, 36091)
    }
}

#[inline(always)]
pub fn malloc(size: usize) -> *mut c_void {
    let heap = MemoryManager::get_singleton();
    if heap.is_null() {
        core::ptr::null_mut()
    } else {
        unsafe { (*heap).allocate(size, 0, false) }
    }
}

#[inline(always)]
pub fn aligned_alloc(alignment: usize, size: usize) -> *mut c_void {
    let heap = MemoryManager::get_singleton();
    if heap.is_null() {
        core::ptr::null_mut()
    } else {
        unsafe { (*heap).allocate(size, alignment as i32, true) }
    }
}

#[inline(always)]
pub fn calloc(count: usize, size: usize) -> *mut c_void {
    let total = count.saturating_mul(size);
    let mem = malloc(total);
    if !mem.is_null() {
        unsafe {
            core::ptr::write_bytes(mem.cast::<u8>(), 0, total);
        }
    }
    mem
}

#[inline(always)]
pub fn realloc(ptr: *mut c_void, new_size: usize) -> *mut c_void {
    let heap = MemoryManager::get_singleton();
    if heap.is_null() {
        core::ptr::null_mut()
    } else {
        unsafe { (*heap).reallocate(ptr, new_size, 0, false) }
    }
}

#[inline(always)]
pub fn aligned_realloc(ptr: *mut c_void, new_size: usize, alignment: usize) -> *mut c_void {
    let heap = MemoryManager::get_singleton();
    if heap.is_null() {
        core::ptr::null_mut()
    } else {
        unsafe { (*heap).reallocate(ptr, new_size, alignment as i32, true) }
    }
}

#[inline(always)]
pub fn free(ptr: *mut c_void) {
    let heap = MemoryManager::get_singleton();
    if !heap.is_null() {
        unsafe {
            (*heap).deallocate(ptr, false);
        }
    }
}

#[inline(always)]
pub fn aligned_free(ptr: *mut c_void) {
    let heap = MemoryManager::get_singleton();
    if !heap.is_null() {
        unsafe {
            (*heap).deallocate(ptr, true);
        }
    }
}
