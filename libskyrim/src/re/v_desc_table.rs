use crate::re::{BSFixedString, BSTTuple, SimpleArray, TypeInfo};

/// C++ `RE::BSScript::Internal::VDescTable`
#[repr(C)]
pub struct VDescTable {
    pub entries: SimpleArray<BSTTuple<BSFixedString, TypeInfo>>, // 00
    pub param_count: u16,                                        // 08
    pub total_entries: u16,                                      // 0A
    pub pad0c: u32,                                              // 0C
}

const _: () = assert!(core::mem::size_of::<VDescTable>() == 0x10);
const _: () = assert!(core::mem::offset_of!(VDescTable, entries) == 0x00);
const _: () = assert!(core::mem::offset_of!(VDescTable, param_count) == 0x08);
const _: () = assert!(core::mem::offset_of!(VDescTable, total_entries) == 0x0A);
const _: () = assert!(core::mem::offset_of!(VDescTable, pad0c) == 0x0C);

impl VDescTable {
    #[inline(always)]
    pub fn new(num_params: u16, num_locals: u16) -> Self {
        let total = num_params.wrapping_add(num_locals);
        Self {
            entries: SimpleArray::with_count(total as usize),
            param_count: num_params,
            total_entries: total,
            pad0c: 0,
        }
    }
}
