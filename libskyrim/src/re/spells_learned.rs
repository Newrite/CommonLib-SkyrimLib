#![allow(non_snake_case)]

use crate::re::{BSTEventSource, SpellItem};
use crate::relocation::RelocationID;

pub mod SpellsLearned {
    use super::*;

    /// C++ `RE::SpellsLearned::Event`
    #[repr(C)]
    pub struct Event {
        pub spell: *mut SpellItem, // 00
    }

    const _: () = assert!(core::mem::size_of::<Event>() == 0x08);

    crate::relocation_func! {
        pub fn get_event_source() -> *mut BSTEventSource<Event> => RelocationID::new(37917, 38874)
    }

    #[inline(always)]
    pub unsafe fn send_event(spell: *mut SpellItem) {
        let event = Event { spell };
        let source = get_event_source();
        if !source.is_null() {
            unsafe { (*source).send_event(core::ptr::addr_of!(event)) };
        }
    }
}
