/// C++ `RE::QUEST_OBJECTIVE_STATE`
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum QuestObjectiveState {
    kDormant = 0,
    kDisplayed = 1,
    kCompleted = 2,
    kCompletedDisplayed = 3,
    kFailed = 4,
    kFailedDisplayed = 5,
}

pub use QuestObjectiveState as QUEST_OBJECTIVE_STATE;
