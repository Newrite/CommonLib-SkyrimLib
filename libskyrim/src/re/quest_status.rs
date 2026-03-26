#![allow(non_snake_case)]

use crate::re::{BSTEventSource, TESQuest};
use crate::relocation::RelocationID;

pub mod QuestStatus {
    use super::*;

    /// C++ `RE::QuestStatus::Status`
    #[repr(i32)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub enum Status {
        kCompleted = 0,
        kStarted = 1,
        kReseted = 2,
    }

    /// C++ `RE::QuestStatus::Event`
    #[repr(C)]
    pub struct Event {
        pub quest: *mut TESQuest, // 00
        pub status: Status,       // 08
        pub pad0c: u32,           // 0C
    }

    const _: () = assert!(core::mem::size_of::<Event>() == 0x10);

    crate::relocation_func! {
        pub fn get_event_source() -> *mut BSTEventSource<Event> => RelocationID::new(24719, 25196)
    }
}
