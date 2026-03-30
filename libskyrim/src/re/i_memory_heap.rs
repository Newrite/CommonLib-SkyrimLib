use core::ffi::c_void;

use core_util::inherit;

use crate::offsets::offsets_rtti::RTTI_IMemoryHeap;
use crate::offsets::offsets_vtable::VTABLE_IMemoryHeap;
use crate::re::IMemoryStore;
use crate::relocation::{RttiType, VariantID};

/// C++ `RE::MEM_CONTEXT`
pub type MEM_CONTEXT = i32;

/// C++ `RE::HeapStats`
#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct HeapStats {
    pub heap_name: *const i8,           // 00
    pub mem_heap_size: usize,           // 08
    pub mem_heap_committed: usize,      // 10
    pub mem_allocated_to_blocks: usize, // 18
    pub num_blocks: i32,                // 20
    pub num_free_blocks: i32,           // 24
    pub mem_free_in_blocks: usize,      // 28
    pub mem_used_in_blocks: usize,      // 30
    pub smallest_free_block: usize,     // 38
    pub largest_free_block: usize,      // 40
    pub heap_overhead: usize,           // 48
    pub free_list_overhead: usize,      // 50
    pub block_overhead: usize,          // 58
    pub total_free: usize,              // 60
}

const _: () = assert!(core::mem::size_of::<HeapStats>() == 0x68);
const _: () = assert!(core::mem::offset_of!(HeapStats, heap_name) == 0x00);
const _: () = assert!(core::mem::offset_of!(HeapStats, total_free) == 0x60);

/// C++ `RE::IMemoryHeap`
#[repr(C)]
pub struct IMemoryHeap {
    pub base: IMemoryStore, // 00
}

const _: () = assert!(core::mem::size_of::<IMemoryHeap>() == 0x08);
const _: () = assert!(core::mem::offset_of!(IMemoryHeap, base) == 0x00);

impl RttiType for IMemoryHeap {
    const RTTI: VariantID = RTTI_IMemoryHeap;
}

inherit!(IMemoryHeap : IMemoryStore);

impl IMemoryHeap {
    pub const RTTI: VariantID = RTTI_IMemoryHeap;
    pub const VTABLE: &'static [VariantID] = &VTABLE_IMemoryHeap;

    crate::virtual_method! {
        pub const VFUNC_GET_NAME: usize = 0x07;
        pub fn get_name() -> *const i8
    }

    crate::virtual_method! {
        pub const VFUNC_ALLOCATE: usize = 0x08;
        pub fn allocate(size: usize, alignment: u32) -> *mut c_void
    }

    crate::virtual_method! {
        pub const VFUNC_DEALLOCATE: usize = 0x09;
        pub fn deallocate(mem: *mut c_void, arg1: u32)
    }

    crate::virtual_method! {
        pub const VFUNC_POINTER_IN_HEAP: usize = 0x0A;
        pub fn pointer_in_heap(pointer: *const c_void) -> bool
    }

    crate::virtual_method! {
        pub const VFUNC_TOTAL_SIZE: usize = 0x0B;
        pub fn total_size(pointer: *const c_void) -> usize
    }

    crate::virtual_method! {
        pub const VFUNC_GET_HEAP_STATS: usize = 0x0C;
        pub fn get_heap_stats(stats: &mut HeapStats, full_block_info: bool)
    }

    crate::virtual_method! {
        pub const VFUNC_SHOULD_TRY_SMALL_BLOCK_POOLS: usize = 0x0D;
        pub fn should_try_small_block_pools(size: usize, context: MEM_CONTEXT) -> bool
    }

    crate::virtual_method! {
        pub const VFUNC_GET_PAGE_SIZE: usize = 0x0E;
        pub fn get_page_size() -> u32
    }
}
