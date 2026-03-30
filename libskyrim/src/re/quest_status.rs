use crate::re::{BSTEventSource, TESQuest};
use crate::relocation::RelocationID;

/// C++ `RE::QuestStatus::Status`
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum QuestStatus {
    Completed = 0,
    Started = 1,
    Reseted = 2,
}

/// C++ `RE::QuestStatus::Event`
#[repr(C)]
pub struct QuestStatusEvent {
    pub quest: *mut TESQuest, // 00
    pub status: QuestStatus,  // 08
    pub pad0c: u32,           // 0C
}

const _: () = assert!(core::mem::size_of::<QuestStatusEvent>() == 0x10);

crate::relocation_func! {
    pub fn get_quest_status_event_source() -> *mut BSTEventSource<QuestStatusEvent> => RelocationID::new(24719, 25196)
}
