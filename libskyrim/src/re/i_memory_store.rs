use core::ffi::c_void;

use core_util::inherit;

use crate::offsets::offsets_rtti::RTTI_IMemoryStore;
use crate::offsets::offsets_vtable::VTABLE_IMemoryStore;
use crate::re::i_memory_store_base::IMemoryStoreBase;
use crate::relocation::{RttiType, VariantID};

/// C++ `RE::IMemoryStore`
#[repr(C)]
pub struct IMemoryStore {
    pub base: IMemoryStoreBase, // 00
}

const _: () = assert!(core::mem::size_of::<IMemoryStore>() == 0x08);
const _: () = assert!(core::mem::offset_of!(IMemoryStore, base) == 0x00);

impl RttiType for IMemoryStore {
    const RTTI: VariantID = RTTI_IMemoryStore;
}

inherit!(IMemoryStore : IMemoryStoreBase);

impl IMemoryStore {
    pub const RTTI: VariantID = RTTI_IMemoryStore;
    pub const VTABLE: &'static [VariantID] = &VTABLE_IMemoryStore;

    // override (IMemoryStoreBase)
    // VFUNC_DTOR            = 0x00
    // VFUNC_SIZE            = 0x01
    // VFUNC_GET_MEMORY_STATS= 0x02
    // VFUNC_CONTAINS_BLOCK_IMPL = 0x03

    crate::virtual_method! {
        pub const VFUNC_ALLOCATE_ALIGN_IMPL: usize = 0x04;
        pub fn allocate_align_impl(size: usize, alignment: u32) -> *mut c_void
    }

    crate::virtual_method! {
        pub const VFUNC_DEALLOCATE_ALIGN_IMPL: usize = 0x05;
        pub fn deallocate_align_impl(block: &mut *mut c_void)
    }

    crate::virtual_method! {
        pub const VFUNC_TRY_ALLOCATE_IMPL: usize = 0x06;
        pub fn try_allocate_impl(size: usize, alignment: u32) -> *mut c_void
    }

    #[inline(always)]
    pub fn allocate_align(&mut self, size: usize, alignment: u32) -> *mut c_void {
        self.allocate_align_impl(size, alignment)
    }

    #[inline(always)]
    pub fn deallocate_align(&mut self, block: &mut *mut c_void) {
        self.deallocate_align_impl(block)
    }
}
