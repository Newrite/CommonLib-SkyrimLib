#![allow(non_snake_case)]

use crate::re::{BSTEventSource, TESShout};
use crate::relocation::RelocationID;

pub mod ShoutAttack {
    use super::*;

    /// C++ `RE::ShoutAttack::Event`
    #[repr(C)]
    pub struct Event {
        pub shout: *mut TESShout, // 00
    }

    const _: () = assert!(core::mem::size_of::<Event>() == 0x08);

    crate::relocation_func! {
        pub fn get_event_source() -> *mut BSTEventSource<Event> => RelocationID::new(40060, 41071)
    }
}
