/// C++ `RE::DIALOGUE_TYPE`
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DialogueType {
    PlayerDialogue = 0,
    CommandDialogue = 1,
    SceneDialogue = 2,
    Combat = 3,
    Favors = 4,
    Detection = 5,
    Service = 6,
    Miscellaneous = 7,
}

pub const DIALOGUE_TYPE_BRANCHED_TOTAL: usize = 2;
pub const DIALOGUE_TYPE_TOTAL: usize = 8;

pub use DialogueType as DIALOGUE_TYPE;
