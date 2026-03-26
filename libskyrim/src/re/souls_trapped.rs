#![allow(non_snake_case)]

use crate::re::{Actor, BSTEventSource};
use crate::relocation::RelocationID;

pub mod SoulsTrapped {
    use super::*;

    /// C++ `RE::SoulsTrapped::Event`
    #[repr(C)]
    pub struct Event {
        pub trapper: *mut Actor, // 00
        pub target: *mut Actor,  // 08
    }

    const _: () = assert!(core::mem::size_of::<Event>() == 0x10);

    crate::relocation_func! {
        pub fn get_event_source() -> *mut BSTEventSource<Event> => RelocationID::new(37916, 38873)
    }

    #[inline(always)]
    pub unsafe fn send_event(trapper: *mut Actor, target: *mut Actor) {
        let event = Event { trapper, target };
        let source = get_event_source();
        if !source.is_null() {
            unsafe { (*source).send_event(core::ptr::addr_of!(event)) };
        }
    }
}
