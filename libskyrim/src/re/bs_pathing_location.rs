use crate::re::{BSNavmeshInfo, BSPathingCell, BSTArray, BSTSmartPointer, NiPoint3};

/// C++ `RE::BSPathingLocation`
#[repr(C)]
pub struct BSPathingLocation {
    pub location: NiPoint3,                                     // 00
    pub nav_mesh_info: *mut BSNavmeshInfo,                      // 10
    pub nav_mesh_info_array: *mut BSTArray<*mut BSNavmeshInfo>, // 18
    pub pathing_cell: BSTSmartPointer<BSPathingCell>,           // 20
    pub triangle: u16,                                          // 28
    pub flags: u8,                                              // 2A
    pub client_data: u8,                                        // 2B
    pub pad2c: u32,                                             // 2C
}

const _: () = assert!(core::mem::size_of::<BSPathingLocation>() == 0x30);
const _: () = assert!(core::mem::offset_of!(BSPathingLocation, location) == 0x00);
const _: () = assert!(core::mem::offset_of!(BSPathingLocation, nav_mesh_info) == 0x10);
const _: () = assert!(core::mem::offset_of!(BSPathingLocation, nav_mesh_info_array) == 0x18);
const _: () = assert!(core::mem::offset_of!(BSPathingLocation, pathing_cell) == 0x20);
const _: () = assert!(core::mem::offset_of!(BSPathingLocation, triangle) == 0x28);
const _: () = assert!(core::mem::offset_of!(BSPathingLocation, flags) == 0x2A);
const _: () = assert!(core::mem::offset_of!(BSPathingLocation, client_data) == 0x2B);
