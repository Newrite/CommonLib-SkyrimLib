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
