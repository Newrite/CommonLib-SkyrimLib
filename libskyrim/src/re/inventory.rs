#![allow(non_snake_case)]

use crate::re::{BSTEventSource, InventoryEntryData, TESObjectREFR};
use crate::relocation::RelocationID;

pub mod Inventory {
    use super::*;

    /// C++ `RE::Inventory::Event`
    #[repr(C)]
    pub struct Event {
        pub obj_refr: *mut TESObjectREFR,        // 00
        pub entry_data: *mut InventoryEntryData, // 08
        pub new_count: i32,                      // 10
        pub prev_count: i32,                     // 14
    }

    const _: () = assert!(core::mem::size_of::<Event>() == 0x18);

    crate::relocation_func! {
        pub fn get_event_source() -> *mut BSTEventSource<Event> => RelocationID::new(15980, 16225)
    }
}
