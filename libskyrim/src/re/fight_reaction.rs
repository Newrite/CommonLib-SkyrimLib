/// C++ `RE::FIGHT_REACTION`
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FightReaction {
    Neutral = 0,
    Enemy = 1,
    Ally = 2,
    Friend = 3,
}

pub use FightReaction as FIGHT_REACTION;
