#![allow(non_snake_case)]

use crate::re::{BGSQuestObjective, BSTEventSource, QUEST_OBJECTIVE_STATE};
use crate::relocation::RelocationID;

pub mod ObjectiveState {
    use super::*;

    /// C++ `RE::ObjectiveState::Event`
    #[repr(C)]
    pub struct Event {
        pub objective: *mut BGSQuestObjective, // 00
        pub old_state: QUEST_OBJECTIVE_STATE,  // 08
        pub new_state: QUEST_OBJECTIVE_STATE,  // 0C
    }

    const _: () = assert!(core::mem::size_of::<Event>() == 0x10);
    const _: () = assert!(core::mem::offset_of!(Event, objective) == 0x00);
    const _: () = assert!(core::mem::offset_of!(Event, old_state) == 0x08);
    const _: () = assert!(core::mem::offset_of!(Event, new_state) == 0x0C);

    crate::relocation_func! {
        pub fn get_event_source() -> *mut BSTEventSource<Event> => RelocationID::new(23486, 23951)
    }
}
