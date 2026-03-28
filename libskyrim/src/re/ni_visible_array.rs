use crate::re::BSGeometry;

/// C++ `RE::NiVisibleArray`
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct NiVisibleArray {
    pub array: *mut *mut BSGeometry, // 00
    pub current_size: u32,           // 08
    pub allocated_size: u32,         // 0C
    pub grow_by: u32,                // 10
}

const _: () = assert!(core::mem::size_of::<NiVisibleArray>() == 0x18);
const _: () = assert!(core::mem::offset_of!(NiVisibleArray, array) == 0x00);
const _: () = assert!(core::mem::offset_of!(NiVisibleArray, current_size) == 0x08);
const _: () = assert!(core::mem::offset_of!(NiVisibleArray, allocated_size) == 0x0C);
const _: () = assert!(core::mem::offset_of!(NiVisibleArray, grow_by) == 0x10);
