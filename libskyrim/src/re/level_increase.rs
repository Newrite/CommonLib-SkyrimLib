#![allow(non_snake_case)]

use crate::re::{BSTEventSource, PlayerCharacter};
use crate::relocation::RelocationID;

pub mod LevelIncrease {
    use super::*;

    /// C++ `RE::LevelIncrease::Event`
    #[repr(C)]
    pub struct Event {
        pub player: *mut PlayerCharacter, // 00
        pub new_level: u16,               // 08
        pub pad0a: u16,                   // 0A
        pub pad0c: u32,                   // 0C
    }

    const _: () = assert!(core::mem::size_of::<Event>() == 0x10);

    crate::relocation_func! {
        pub fn get_event_source() -> *mut BSTEventSource<Event> => RelocationID::new(39247, 40319)
    }
}
