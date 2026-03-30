use core::ffi::c_void;

use crate::offsets::offsets_rtti::RTTI_IMemoryStoreBase;
use crate::offsets::offsets_vtable::VTABLE_IMemoryStoreBase;
use crate::relocation::{RttiType, VariantID};

/// C++ `RE::MemoryStats`
#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct MemoryStats {
    pub name: *const i8,       // 00
    pub used_size: usize,      // 08
    pub committed_size: usize, // 10
    pub reserved_size: usize,  // 18
    pub overhead: u32,         // 20
    pub pad24: u32,            // 24
    pub free_size: usize,      // 28
}

const _: () = assert!(core::mem::size_of::<MemoryStats>() == 0x30);
const _: () = assert!(core::mem::offset_of!(MemoryStats, name) == 0x00);
const _: () = assert!(core::mem::offset_of!(MemoryStats, used_size) == 0x08);
const _: () = assert!(core::mem::offset_of!(MemoryStats, free_size) == 0x28);

/// C++ `RE::IMemoryStoreBase`
#[repr(C)]
pub struct IMemoryStoreBase {
    pub vtable: *const usize, // 00
}

const _: () = assert!(core::mem::size_of::<IMemoryStoreBase>() == 0x08);
const _: () = assert!(core::mem::offset_of!(IMemoryStoreBase, vtable) == 0x00);

impl RttiType for IMemoryStoreBase {
    const RTTI: VariantID = RTTI_IMemoryStoreBase;
}

impl IMemoryStoreBase {
    pub const RTTI: VariantID = RTTI_IMemoryStoreBase;
    pub const VTABLE: &'static [VariantID] = &VTABLE_IMemoryStoreBase;

    crate::virtual_method! {
        pub const VFUNC_DTOR: usize = 0x00;
        pub fn dtor()
    }

    crate::virtual_method! {
        pub const VFUNC_SIZE: usize = 0x01;
        pub fn size(mem: *const c_void) -> usize
    }

    crate::virtual_method! {
        pub const VFUNC_GET_MEMORY_STATS: usize = 0x02;
        pub fn get_memory_stats(stats: &mut MemoryStats)
    }

    crate::virtual_method! {
        pub const VFUNC_CONTAINS_BLOCK_IMPL: usize = 0x03;
        pub fn contains_block_impl(block: *const c_void) -> bool
    }

    #[inline(always)]
    pub fn contains_block(&self, block: *const c_void) -> bool {
        self.contains_block_impl(block)
    }
}
