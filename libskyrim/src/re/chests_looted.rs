#![allow(non_snake_case)]

use crate::re::BSTEventSource;
use crate::relocation::RelocationID;

pub mod ChestsLooted {
    use super::*;

    /// C++ `RE::ChestsLooted::Event`
    #[repr(C)]
    pub struct Event {
        pub _data: u8,
    }

    const _: () = assert!(core::mem::size_of::<Event>() == 0x1);

    crate::relocation_func! {
        pub fn get_event_source() -> *mut BSTEventSource<Event> => RelocationID::new(50257, 51182)
    }

    #[inline(always)]
    pub unsafe fn send_event() {
        let event = Event { _data: 0 };
        let source = get_event_source();
        if !source.is_null() {
            unsafe { (*source).send_event(core::ptr::addr_of!(event)) };
        }
    }
}
