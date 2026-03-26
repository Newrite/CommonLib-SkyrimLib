#![allow(non_snake_case)]

use crate::re::BSTEventSource;
use crate::relocation::RelocationID;

pub mod DragonSoulsGained {
    use super::*;

    /// C++ `RE::DragonSoulsGained::Event`
    #[repr(C)]
    pub struct Event {
        pub souls: f32, // 00
        pub pad04: u32, // 04
    }

    const _: () = assert!(core::mem::size_of::<Event>() == 0x08);

    crate::relocation_func! {
        pub fn get_event_source() -> *mut BSTEventSource<Event> => RelocationID::new(37571, 38520)
    }
}
