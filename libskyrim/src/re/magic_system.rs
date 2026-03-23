#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum CannotCastReason {
    OK = 0,
    Magicka = 1,
    PowerUsed = 2,
    RangedUnderWater = 3,
    MultipleCast = 4,
    ItemCharge = 5,
    CastWhileShouting = 6,
    ShoutWhileCasting = 7,
    ShoutWhileRecovering = 8,
    CustomReasonNoStart = 100,
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum CastingSource {
    LeftHand = 0,
    RightHand = 1,
    Other = 2,
    Instant = 3,
    None = 4,
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum CastingType {
    ConstantEffect = 0,
    FireAndForget = 1,
    Concentration = 2,
    Scroll = 3,
}

core_util::impl_enumset_type!(CastingType => u16);

impl TryFrom<u16> for CastingType {
    type Error = ();

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        if value <= Self::Scroll as u16 {
            Ok(unsafe { core::mem::transmute::<u32, Self>(value as u32) })
        } else {
            Err(())
        }
    }
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Delivery {
    Self_ = 0,
    Touch = 1,
    Aimed = 2,
    TargetActor = 3,
    TargetLocation = 4,
    None = 5,
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum SoundID {
    DrawSheatheLPM = 0,
    Charge = 1,
    ReadyLoop = 2,
    Release = 3,
    CastLoop = 4,
    Hit = 5,
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum SpellType {
    Spell = 0,
    Disease = 1,
    Power = 2,
    LesserPower = 3,
    Ability = 4,
    Poison = 5,
    Enchantment = 6,
    Potion = 7,
    WortCraft = 8,
    LeveledSpell = 9,
    Addiction = 10,
    VoicePower = 11,
    StaffEnchantment = 12,
    Scroll = 13,
}

impl SpellType {
    pub const ALCHEMY: Self = Self::Potion;
    pub const INGREDIENT: Self = Self::WortCraft;
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum WardState {
    None = 0,
    Absorb = 1,
    Break = 2,
    Total = 3,
}
