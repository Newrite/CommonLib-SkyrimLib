#![allow(non_camel_case_types)]

use crate::re::{GMemoryHeap, GStatBag, GString};

/// C++ `RE::GFxResourceReport`
#[repr(C)]
pub struct GFxResourceReport {
    pub vtable: *const usize, // 00
}

const _: () = assert!(core::mem::size_of::<GFxResourceReport>() == 0x8);
const _: () = assert!(core::mem::offset_of!(GFxResourceReport, vtable) == 0x0);

impl GFxResourceReport {
    crate::virtual_method! {
        pub const VFUNC_DTOR: usize = 0x00;
        pub fn dtor()
    }

    crate::virtual_method! {
        pub const VFUNC_GET_RESOURCE_NAME: usize = 0x01;
        pub fn get_resource_name() -> GString
    }

    crate::virtual_method! {
        pub const VFUNC_GET_RESOURCE_HEAP: usize = 0x02;
        pub fn get_resource_heap() -> *mut GMemoryHeap
    }

    crate::virtual_method! {
        pub const VFUNC_GET_STATS: usize = 0x03;
        pub fn get_stats(bag: *mut GStatBag, reset: bool)
    }
}
