#![allow(non_snake_case)]

use crate::re::{BSTEventSource, TESObjectBOOK};
use crate::relocation::RelocationID;

pub mod BooksRead {
    use super::*;

    /// C++ `RE::BooksRead::Event`
    #[repr(C)]
    pub struct Event {
        pub book: *mut TESObjectBOOK, // 00
        pub skill_book: bool,         // 08
        pub pad09: u8,                // 09
        pub pad0a: u16,               // 0A
        pub pad0c: u32,               // 0C
    }

    const _: () = assert!(core::mem::size_of::<Event>() == 0x10);

    crate::relocation_func! {
        pub fn get_event_source() -> *mut BSTEventSource<Event> => RelocationID::new(17470, 17865)
    }
}
