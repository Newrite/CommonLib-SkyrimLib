use crate::re::{BGSLocalizedStringDL, TESCondition, TESQuest, TESQuestStage};
use crate::relocation::RelocationID;

/// C++ `RE::TESQuestStageItem`
#[repr(C)]
pub struct TESQuestStageItem {
    pub obj_conditions: TESCondition,     // 00
    pub next_quest: *mut TESQuest,        // 08
    pub log_entry: BGSLocalizedStringDL,  // 10
    pub data: u8,                         // 14
    pub index: u8,                        // 15
    pub has_log_entry: bool,              // 16
    pub pad17: u8,                        // 17
    pub owner: *mut TESQuest,             // 18
    pub owning_stage: *mut TESQuestStage, // 20
}

const _: () = assert!(core::mem::size_of::<TESQuestStageItem>() == 0x28);
const _: () = assert!(core::mem::offset_of!(TESQuestStageItem, obj_conditions) == 0x00);
const _: () = assert!(core::mem::offset_of!(TESQuestStageItem, next_quest) == 0x08);
const _: () = assert!(core::mem::offset_of!(TESQuestStageItem, log_entry) == 0x10);
const _: () = assert!(core::mem::offset_of!(TESQuestStageItem, data) == 0x14);
const _: () = assert!(core::mem::offset_of!(TESQuestStageItem, index) == 0x15);
const _: () = assert!(core::mem::offset_of!(TESQuestStageItem, has_log_entry) == 0x16);
const _: () = assert!(core::mem::offset_of!(TESQuestStageItem, owner) == 0x18);
const _: () = assert!(core::mem::offset_of!(TESQuestStageItem, owning_stage) == 0x20);

impl TESQuestStageItem {
    crate::relocation_func! {
        pub fn get_log_entry(&self, a_owner_quest: *const TESQuest) -> *const i8 => RelocationID::new(24778, 25259)
    }
}
