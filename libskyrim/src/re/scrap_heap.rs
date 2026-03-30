use core::ffi::c_void;

use core_util::inherit;

use crate::offsets::offsets_rtti::RTTI_ScrapHeap;
use crate::offsets::offsets_vtable::VTABLE_ScrapHeap;
use crate::re::IMemoryStore;
use crate::relocation::{RelocationID, RttiType, VariantID};

/// C++ `RE::ScrapHeap::Block`
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct ScrapHeapBlock {
    pub size_flags: usize,         // 00
    pub prev: *mut ScrapHeapBlock, // 08
}

const _: () = assert!(core::mem::size_of::<ScrapHeapBlock>() == 0x10);
const _: () = assert!(core::mem::offset_of!(ScrapHeapBlock, size_flags) == 0x00);
const _: () = assert!(core::mem::offset_of!(ScrapHeapBlock, prev) == 0x08);

/// C++ `RE::ScrapHeap::FreeBlock`
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct ScrapHeapFreeBlock {
    pub base: ScrapHeapBlock,           // 00
    pub left: *mut ScrapHeapFreeBlock,  // 10
    pub right: *mut ScrapHeapFreeBlock, // 18
}

const _: () = assert!(core::mem::size_of::<ScrapHeapFreeBlock>() == 0x20);
const _: () = assert!(core::mem::offset_of!(ScrapHeapFreeBlock, base) == 0x00);
const _: () = assert!(core::mem::offset_of!(ScrapHeapFreeBlock, left) == 0x10);
const _: () = assert!(core::mem::offset_of!(ScrapHeapFreeBlock, right) == 0x18);

inherit!(ScrapHeapFreeBlock : ScrapHeapBlock);

/// C++ `RE::ScrapHeap::FreeTreeNode`
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct ScrapHeapFreeTreeNode {
    pub base: ScrapHeapBlock,                   // 00
    pub root: *mut *mut ScrapHeapFreeTreeNode,  // 10
    pub left_node: *mut ScrapHeapFreeTreeNode,  // 18
    pub right_node: *mut ScrapHeapFreeTreeNode, // 20
    pub parent_and_black: usize,                // 28
}

const _: () = assert!(core::mem::size_of::<ScrapHeapFreeTreeNode>() == 0x30);
const _: () = assert!(core::mem::offset_of!(ScrapHeapFreeTreeNode, base) == 0x00);
const _: () = assert!(core::mem::offset_of!(ScrapHeapFreeTreeNode, root) == 0x10);
const _: () = assert!(core::mem::offset_of!(ScrapHeapFreeTreeNode, parent_and_black) == 0x28);

inherit!(ScrapHeapFreeTreeNode : ScrapHeapBlock);

/// C++ `RE::ScrapHeap`
#[repr(C)]
pub struct ScrapHeap {
    pub base: IMemoryStore,                         // 00
    pub small_blocks: [*mut ScrapHeapFreeBlock; 6], // 08
    pub free_list: *mut ScrapHeapFreeTreeNode,      // 38
    pub last_block: *mut ScrapHeapBlock,            // 40
    pub base_address: *mut c_void,                  // 48
    pub end_address: *mut c_void,                   // 50
    pub commit_end: *mut c_void,                    // 58
    pub reserve_size: usize,                        // 60
    pub min_commit: usize,                          // 68
    pub total_allocated: usize,                     // 70
    pub keep_pages_request: u32,                    // 78
    pub total_free_blocks: u32,                     // 7C
    pub free_small_blocks: u32,                     // 80
    pub total_allocated_blocks: u32,                // 84
    pub pmp_barrier: u32,                           // 88
}

const _: () = assert!(core::mem::size_of::<ScrapHeap>() == 0x90);
const _: () = assert!(core::mem::offset_of!(ScrapHeap, base) == 0x00);
const _: () = assert!(core::mem::offset_of!(ScrapHeap, small_blocks) == 0x08);
const _: () = assert!(core::mem::offset_of!(ScrapHeap, free_list) == 0x38);
const _: () = assert!(core::mem::offset_of!(ScrapHeap, base_address) == 0x48);
const _: () = assert!(core::mem::offset_of!(ScrapHeap, pmp_barrier) == 0x88);

impl RttiType for ScrapHeap {
    const RTTI: VariantID = RTTI_ScrapHeap;
}

inherit!(ScrapHeap : IMemoryStore);

impl ScrapHeap {
    pub const RTTI: VariantID = RTTI_ScrapHeap;
    pub const VTABLE: &'static [VariantID] = &VTABLE_ScrapHeap;

    crate::relocation_func! {
        pub fn allocate(&mut self, size: usize, alignment: usize) -> *mut c_void
            => RelocationID::new(66884, 68144)
    }

    crate::relocation_func! {
        pub fn deallocate(&mut self, mem: *mut c_void)
            => RelocationID::new(66885, 68146)
    }
}
