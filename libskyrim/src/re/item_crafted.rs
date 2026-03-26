#![allow(non_snake_case)]

use crate::re::{BSTEventSource, TESForm};
use crate::relocation::RelocationID;

pub mod ItemCrafted {
    use super::*;

    /// C++ `RE::ItemCrafted::Event`
    #[repr(C)]
    pub struct Event {
        pub item: *mut TESForm, // 00
        pub unk08: bool,        // 08
        pub unk09: bool,        // 09
        pub unk0a: bool,        // 0A
        pub pad0b: u8,          // 0B
        pub pad0c: u32,         // 0C
    }

    const _: () = assert!(core::mem::size_of::<Event>() == 0x10);

    crate::relocation_func! {
        pub fn get_event_source() -> *mut BSTEventSource<Event> => RelocationID::new(50515, 51403)
    }
}
