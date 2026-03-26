#![allow(non_snake_case)]

use crate::re::{Actor, BSTEventSource};
use crate::relocation::RelocationID;

pub mod DisarmedEvent {
    use super::*;

    /// C++ `RE::DisarmedEvent::Event`
    #[repr(C)]
    pub struct Event {
        pub source: *mut Actor, // 00
        pub target: *mut Actor, // 08
    }

    const _: () = assert!(core::mem::size_of::<Event>() == 0x10);

    crate::relocation_func! {
        pub fn get_event_source() -> *mut BSTEventSource<Event> => RelocationID::new(37392, 38340)
    }
}
