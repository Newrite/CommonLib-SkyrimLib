#![allow(non_snake_case)]

use crate::re::{BSTEventSource, TESObjectREFR, TESObjectWEAP};
use crate::relocation::RelocationID;

pub mod CriticalHit {
    use super::*;

    /// C++ `RE::CriticalHit::Event`
    #[repr(C)]
    pub struct Event {
        pub aggressor: *mut TESObjectREFR, // 00
        pub weapon: *mut TESObjectWEAP,    // 08
        pub sneak_hit: bool,               // 10
        pub pad11: u8,                     // 11
        pub pad12: u16,                    // 12
        pub pad14: u32,                    // 14
    }

    const _: () = assert!(core::mem::size_of::<Event>() == 0x18);

    crate::relocation_func! {
        pub fn get_event_source() -> *mut BSTEventSource<Event> => RelocationID::new(37726, 38671)
    }
}
