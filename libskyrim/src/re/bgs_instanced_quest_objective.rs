use crate::re::quest_objective_states::QUEST_OBJECTIVE_STATE;
use crate::re::tes_quest::BGSQuestObjective;

/// C++ `RE::BGSInstancedQuestObjective`
#[repr(C)]
pub struct BGSInstancedQuestObjective {
    pub objective: *mut BGSQuestObjective,     // 00
    pub instance_id: u32,                      // 08
    pub instance_state: QUEST_OBJECTIVE_STATE, // 0C
}

const _: () = assert!(core::mem::size_of::<BGSInstancedQuestObjective>() == 0x10);
const _: () = assert!(core::mem::offset_of!(BGSInstancedQuestObjective, objective) == 0x00);
const _: () = assert!(core::mem::offset_of!(BGSInstancedQuestObjective, instance_id) == 0x08);
const _: () = assert!(core::mem::offset_of!(BGSInstancedQuestObjective, instance_state) == 0x0C);
