#![allow(non_snake_case)]

use crate::re::{Actor, BSTEventSource};
use crate::relocation::RelocationID;

pub mod ActorKill {
    use super::*;

    /// C++ `RE::ActorKill::Event`
    #[repr(C)]
    pub struct Event {
        pub killer: *mut Actor, // 00
        pub victim: *mut Actor, // 08
    }

    const _: () = assert!(core::mem::size_of::<Event>() == 0x10);

    crate::relocation_func! {
        pub fn get_event_source() -> *mut BSTEventSource<Event> => RelocationID::new(37390, 38338)
    }
}
