#![allow(non_snake_case)]

use crate::re::BSTEventSource;
use crate::relocation::RelocationID;

pub mod ItemsPickpocketed {
    use super::*;

    /// C++ `RE::ItemsPickpocketed::Event`
    #[repr(C)]
    pub struct Event {
        pub num_items: i32, // 00
        pub pad4: u32,      // 04
    }

    const _: () = assert!(core::mem::size_of::<Event>() == 0x08);

    crate::relocation_func! {
        pub fn get_event_source() -> *mut BSTEventSource<Event> => RelocationID::new(50258, 51183)
    }

    #[inline(always)]
    pub unsafe fn send_event(num_items: i32) {
        let event = Event { num_items, pad4: 0 };
        let source = get_event_source();
        if !source.is_null() {
            unsafe { (*source).send_event(core::ptr::addr_of!(event)) };
        }
    }
}
