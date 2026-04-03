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

impl TryFrom<u8> for QuestObjectiveState {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Dormant),
            1 => Ok(Self::Displayed),
            2 => Ok(Self::Completed),
            3 => Ok(Self::CompletedDisplayed),
            4 => Ok(Self::Failed),
            5 => Ok(Self::FailedDisplayed),
            _ => Err(()),
        }
    }
}
