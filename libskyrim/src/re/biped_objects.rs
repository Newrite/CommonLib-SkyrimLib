//! Translation of `RE::BipedObjects.h`.

/// C++ `RE::BIPED_OBJECTS::BIPED_OBJECT`
///
/// Enum representing biped object slots for armor, weapons, etc.
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BipedObject {
    Head = 0,
    Hair = 1,
    Body = 2,
    Hands = 3,
    Forearms = 4,
    Amulet = 5,
    Ring = 6,
    Feet = 7,
    Calves = 8,
    Shield = 9,
    Tail = 10,
    LongHair = 11,
    Circlet = 12,
    Ears = 13,
    ModMouth = 14,
    ModNeck = 15,
    ModChestPrimary = 16,
    ModBack = 17,
    ModMisc1 = 18,
    ModPelvisPrimary = 19,
    DecapitateHead = 20,
    Decapitate = 21,
    ModPelvisSecondary = 22,
    ModLegRight = 23,
    ModLegLeft = 24,
    ModFaceJewelry = 25,
    ModChestSecondary = 26,
    ModShoulder = 27,
    ModArmLeft = 28,
    ModArmRight = 29,
    ModMisc2 = 30,
    FX01 = 31,

    // kEditorTotal = 32
    HandToHandMelee = 32,
    OneHandSword = 33,
    OneHandDagger = 34,
    OneHandAxe = 35,
    OneHandMace = 36,
    TwoHandMelee = 37,
    Bow = 38,
    Staff = 39,
    Crossbow = 40,
    Quiver = 41,
}

core_util::impl_enumset_type!(BipedObject => u32);

impl BipedObject {
    pub const NONE: u32 = u32::MAX;
    pub const EDITOR_TOTAL: usize = 32;
    pub const TOTAL: usize = 42;
}

/// C++ `RE::BIPED_OBJECT` type alias
#[allow(non_camel_case_types)]
pub type BIPED_OBJECT = BipedObject;

/// Total number of biped object slots (kTotal = 42)
pub const BIPED_OBJECTS_TOTAL: usize = 42;
