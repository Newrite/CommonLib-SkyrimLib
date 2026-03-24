/// C++ `RE::QUEST_OBJECTIVE_STATE`
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum QuestObjectiveState {
    Dormant = 0,
    Displayed = 1,
    Completed = 2,
    CompletedDisplayed = 3,
    Failed = 4,
    FailedDisplayed = 5,
}

core_util::impl_enumset_type!(QuestObjectiveState => u8);

pub use QuestObjectiveState as QUEST_OBJECTIVE_STATE;
