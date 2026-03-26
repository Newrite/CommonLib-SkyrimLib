#![allow(non_snake_case)]

use crate::re::BSTEventSource;
use crate::relocation::RelocationID;

pub mod LocationCleared {
    use super::*;

    /// C++ `RE::LocationCleared::Event`
    #[repr(C)]
    pub struct Event {
        pub _data: u8,
    }

    const _: () = assert!(core::mem::size_of::<Event>() == 0x1);

    crate::relocation_func! {
        pub fn get_event_source() -> *mut BSTEventSource<Event> => RelocationID::new(18046, 18435)
    }
}
