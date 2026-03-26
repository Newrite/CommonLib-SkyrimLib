#![allow(non_snake_case)]

use crate::re::{ActorValue, BSTEventSource, PlayerCharacter};
use crate::relocation::RelocationID;

pub mod SkillIncrease {
    use super::*;

    /// C++ `RE::SkillIncrease::Event`
    #[repr(C)]
    pub struct Event {
        pub player: *mut PlayerCharacter, // 00
        pub actor_value: ActorValue,      // 08
        pub pad0c: u32,                   // 0C
    }

    const _: () = assert!(core::mem::size_of::<Event>() == 0x10);

    crate::relocation_func! {
        pub fn get_event_source() -> *mut BSTEventSource<Event> => RelocationID::new(39248, 40320)
    }
}
