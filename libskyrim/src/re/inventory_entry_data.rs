use crate::re::tes_bound_object::TESBoundObject;
use crate::relocation::RelocationID;

/// C++ `RE::InventoryEntryData`
#[repr(C)]
pub struct InventoryEntryData {
    pub object: *mut TESBoundObject,         // 00
    pub extra_lists: *mut core::ffi::c_void, // 08 - `BSSimpleList<ExtraDataList*>*`
    pub count_delta: i32,                    // 10
    pub pad14: u32,                          // 14
}

const _: () = assert!(core::mem::size_of::<InventoryEntryData>() == 0x18);

impl InventoryEntryData {
    #[inline(always)]
    pub const fn new(object: *mut TESBoundObject, count_delta: i32) -> Self {
        Self {
            object,
            extra_lists: core::ptr::null_mut(),
            count_delta,
            pad14: 0,
        }
    }

    // RELOCATION_ID SE: 15757, AE: 15995
    crate::relocation_func! {
        pub fn get_value(&self) -> i32 => RelocationID::new(15757, 15995)
    }
}
