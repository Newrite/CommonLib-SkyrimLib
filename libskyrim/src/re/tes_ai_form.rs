use core_util::inherit;

use crate::offsets::offsets_rtti::RTTI_TESAIForm;
use crate::offsets::offsets_vtable::VTABLE_TESAIForm;
use crate::re::BSSimpleList;
use crate::re::TESPackage;
use crate::re::base_form_component::BaseFormComponent;
use crate::relocation::{RttiType, VariantID};

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ActorAggression {
    Calmed = -1,
    Unaggressive = 0,
    Aggressive = 1,
    VeryAggressive = 2,
    Frenzied = 3,
}

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ActorAssistance {
    HelpsNobody = 0,
    HelpsAllies = 1,
    HelpsFriends = 2,
}

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ActorConfidence {
    Cowardly = 0,
    Cautious = 1,
    Average = 2,
    Brave = 3,
    Foolhardy = 4,
}

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ActorMood {
    Neutral = 0,
    Angry = 1,
    Fear = 2,
    Happy = 3,
    Sad = 4,
    Surprised = 5,
    Puzzled = 6,
    Disgusted = 7,
}

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ActorMorality {
    AnyCrime = 0,
    ViolenceAgainstEnemy = 1,
    PropertyCrimeOnly = 2,
    NoCrime = 3,
}

pub mod actor_aggro_radius {
    pub const WARN: usize = 0;
    pub const WARN_AND_ATTACK: usize = 1;
    pub const ATTACK: usize = 2;
    pub const TOTAL: usize = 3;
}

#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct AIDataGame {
    pub bits0: u8,                                      // 00
    pub bits1: u8,                                      // 01
    pub bits2: u8,                                      // 02
    pub pad3: u8,                                       // 03
    pub aggro_radius: [u16; actor_aggro_radius::TOTAL], // 04
    pub bits_a: u8,                                     // 0A
    pub padb: u8,                                       // 0B
}

const _: () = assert!(core::mem::size_of::<AIDataGame>() == 0x0C);
const _: () = assert!(core::mem::offset_of!(AIDataGame, aggro_radius) == 0x04);

impl AIDataGame {
    #[inline(always)]
    const fn test_bit(value: u8, bit: u8) -> bool {
        value & (1 << bit) != 0
    }

    #[inline(always)]
    const fn set_bit(value: &mut u8, bit: u8, enabled: bool) {
        if enabled {
            *value |= 1 << bit;
        } else {
            *value &= !(1 << bit);
        }
    }

    #[inline(always)]
    pub const fn aggression1(&self) -> bool {
        Self::test_bit(self.bits0, 0)
    }
    #[inline(always)]
    pub const fn aggression2(&self) -> bool {
        Self::test_bit(self.bits0, 1)
    }
    #[inline(always)]
    pub const fn confidence1(&self) -> bool {
        Self::test_bit(self.bits0, 2)
    }
    #[inline(always)]
    pub const fn confidence2(&self) -> bool {
        Self::test_bit(self.bits0, 3)
    }
    #[inline(always)]
    pub const fn confidence3(&self) -> bool {
        Self::test_bit(self.bits0, 4)
    }
    #[inline(always)]
    pub const fn energy_level1(&self) -> bool {
        Self::test_bit(self.bits0, 5)
    }
    #[inline(always)]
    pub const fn energy_level2(&self) -> bool {
        Self::test_bit(self.bits0, 6)
    }
    #[inline(always)]
    pub const fn energy_level3(&self) -> bool {
        Self::test_bit(self.bits0, 7)
    }
    #[inline(always)]
    pub const fn energy_level4(&self) -> bool {
        Self::test_bit(self.bits1, 0)
    }
    #[inline(always)]
    pub const fn energy_level5(&self) -> bool {
        Self::test_bit(self.bits1, 1)
    }
    #[inline(always)]
    pub const fn energy_level6(&self) -> bool {
        Self::test_bit(self.bits1, 2)
    }
    #[inline(always)]
    pub const fn energy_level7(&self) -> bool {
        Self::test_bit(self.bits1, 3)
    }
    #[inline(always)]
    pub const fn energy_level8(&self) -> bool {
        Self::test_bit(self.bits1, 4)
    }
    #[inline(always)]
    pub const fn morality1(&self) -> bool {
        Self::test_bit(self.bits1, 5)
    }
    #[inline(always)]
    pub const fn mood1(&self) -> bool {
        Self::test_bit(self.bits1, 7)
    }
    #[inline(always)]
    pub const fn mood2(&self) -> bool {
        Self::test_bit(self.bits2, 0)
    }
    #[inline(always)]
    pub const fn mood3(&self) -> bool {
        Self::test_bit(self.bits2, 1)
    }
    #[inline(always)]
    pub const fn assistance1(&self) -> bool {
        Self::test_bit(self.bits2, 2)
    }
    #[inline(always)]
    pub const fn assistance2(&self) -> bool {
        Self::test_bit(self.bits2, 3)
    }
    #[inline(always)]
    pub const fn aggro_radius_behaviour(&self) -> bool {
        Self::test_bit(self.bits2, 4)
    }
    #[inline(always)]
    pub const fn no_slow_approach(&self) -> bool {
        Self::test_bit(self.bits_a, 0)
    }

    #[inline(always)]
    pub fn set_aggression_bits(&mut self, value: i32) {
        Self::set_bit(&mut self.bits0, 0, value & 1 != 0);
        Self::set_bit(&mut self.bits0, 1, value & 2 != 0);
    }

    #[inline(always)]
    pub fn set_assistance_bits(&mut self, value: i32) {
        Self::set_bit(&mut self.bits2, 2, value & 1 != 0);
        Self::set_bit(&mut self.bits2, 3, value & 2 != 0);
    }

    #[inline(always)]
    pub fn set_confidence_bits(&mut self, value: i32) {
        Self::set_bit(&mut self.bits0, 2, value & 1 != 0);
        Self::set_bit(&mut self.bits0, 3, value & 2 != 0);
        Self::set_bit(&mut self.bits0, 4, value & 4 != 0);
    }
}

#[repr(C)]
pub struct PackageList {
    pub packages: BSSimpleList<*mut TESPackage>, // 00
}

const _: () = assert!(core::mem::size_of::<PackageList>() == 0x10);

#[repr(C)]
pub struct TESAIForm {
    pub base: BaseFormComponent,  // 00
    pub ai_data: AIDataGame,      // 08
    pub ai_packages: PackageList, // 18
}

const _: () = assert!(core::mem::size_of::<TESAIForm>() == 0x28);
const _: () = assert!(core::mem::offset_of!(TESAIForm, ai_data) == 0x08);
const _: () = assert!(core::mem::offset_of!(TESAIForm, ai_packages) == 0x18);

impl RttiType for TESAIForm {
    const RTTI: VariantID = RTTI_TESAIForm;
}

impl AsRef<TESAIForm> for TESAIForm {
    #[inline(always)]
    fn as_ref(&self) -> &Self {
        self
    }
}

impl AsMut<TESAIForm> for TESAIForm {
    #[inline(always)]
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

inherit!(TESAIForm : BaseFormComponent);

impl TESAIForm {
    pub const RTTI: VariantID = RTTI_TESAIForm;
    pub const VTABLE: &'static [VariantID] = &VTABLE_TESAIForm;

    // override (BaseFormComponent)
    // void InitializeDataComponent() override;                // 01
    // void ClearDataComponent() override;                     // 02
    // void CopyComponent(BaseFormComponent* a_rhs) override;  // 03

    #[inline]
    pub const fn aggro_radius_behaviour_is_enabled(&self) -> bool {
        self.ai_data.aggro_radius_behaviour()
    }

    #[inline]
    pub fn get_aggression_level(&self) -> ActorAggression {
        match (self.ai_data.aggression1() as i32) | ((self.ai_data.aggression2() as i32) << 1) {
            0 => ActorAggression::Unaggressive,
            1 => ActorAggression::Aggressive,
            2 => ActorAggression::VeryAggressive,
            _ => ActorAggression::Frenzied,
        }
    }

    #[inline]
    pub fn get_assistance_level(&self) -> ActorAssistance {
        match (self.ai_data.assistance1() as i32) | ((self.ai_data.assistance2() as i32) << 1) {
            0 => ActorAssistance::HelpsNobody,
            1 => ActorAssistance::HelpsAllies,
            _ => ActorAssistance::HelpsFriends,
        }
    }

    #[inline]
    pub fn get_confidence_level(&self) -> ActorConfidence {
        match (self.ai_data.confidence1() as i32)
            | ((self.ai_data.confidence2() as i32) << 1)
            | ((self.ai_data.confidence3() as i32) << 2)
        {
            0 => ActorConfidence::Cowardly,
            1 => ActorConfidence::Cautious,
            2 => ActorConfidence::Average,
            3 => ActorConfidence::Brave,
            _ => ActorConfidence::Foolhardy,
        }
    }

    #[inline]
    pub fn get_energy_level(&self) -> u8 {
        (self.ai_data.energy_level1() as u8)
            | ((self.ai_data.energy_level2() as u8) << 1)
            | ((self.ai_data.energy_level3() as u8) << 2)
            | ((self.ai_data.energy_level4() as u8) << 3)
            | ((self.ai_data.energy_level5() as u8) << 4)
            | ((self.ai_data.energy_level6() as u8) << 5)
            | ((self.ai_data.energy_level7() as u8) << 6)
            | ((self.ai_data.energy_level8() as u8) << 7)
    }

    #[inline]
    pub fn get_mood_level(&self) -> ActorMood {
        match (self.ai_data.mood1() as i32)
            | ((self.ai_data.mood2() as i32) << 1)
            | ((self.ai_data.mood3() as i32) << 2)
        {
            0 => ActorMood::Neutral,
            1 => ActorMood::Angry,
            2 => ActorMood::Fear,
            3 => ActorMood::Happy,
            4 => ActorMood::Sad,
            5 => ActorMood::Surprised,
            6 => ActorMood::Puzzled,
            _ => ActorMood::Disgusted,
        }
    }

    #[inline]
    pub fn get_morality_level(&self) -> ActorMorality {
        match (self.ai_data.morality1() as i32) | ((self.ai_data.morality1() as i32) << 1) {
            0 => ActorMorality::AnyCrime,
            1 => ActorMorality::ViolenceAgainstEnemy,
            2 => ActorMorality::PropertyCrimeOnly,
            _ => ActorMorality::NoCrime,
        }
    }

    #[inline]
    pub const fn no_slow_approach(&self) -> bool {
        self.ai_data.no_slow_approach()
    }

    #[inline]
    pub fn set_aggression_level(&mut self, level: ActorAggression) {
        self.ai_data.set_aggression_bits(level as i32);
    }

    #[inline]
    pub fn set_assistance_level(&mut self, level: ActorAssistance) {
        self.ai_data.set_assistance_bits(level as i32);
    }

    #[inline]
    pub fn set_confidence_level(&mut self, level: ActorConfidence) {
        self.ai_data.set_confidence_bits(level as i32);
    }
}

pub trait TESAIFormExt {
    fn aggro_radius_behaviour_is_enabled(&self) -> bool;
    fn get_aggression_level(&self) -> ActorAggression;
    fn get_assistance_level(&self) -> ActorAssistance;
    fn get_confidence_level(&self) -> ActorConfidence;
    fn get_energy_level(&self) -> u8;
    fn get_mood_level(&self) -> ActorMood;
    fn get_morality_level(&self) -> ActorMorality;
    fn no_slow_approach(&self) -> bool;
    fn set_aggression_level(&mut self, level: ActorAggression);
    fn set_assistance_level(&mut self, level: ActorAssistance);
    fn set_confidence_level(&mut self, level: ActorConfidence);
}

impl<T: AsRef<TESAIForm> + AsMut<TESAIForm>> TESAIFormExt for T {
    #[inline(always)]
    fn aggro_radius_behaviour_is_enabled(&self) -> bool {
        self.as_ref().aggro_radius_behaviour_is_enabled()
    }

    #[inline(always)]
    fn get_aggression_level(&self) -> ActorAggression {
        self.as_ref().get_aggression_level()
    }

    #[inline(always)]
    fn get_assistance_level(&self) -> ActorAssistance {
        self.as_ref().get_assistance_level()
    }

    #[inline(always)]
    fn get_confidence_level(&self) -> ActorConfidence {
        self.as_ref().get_confidence_level()
    }

    #[inline(always)]
    fn get_energy_level(&self) -> u8 {
        self.as_ref().get_energy_level()
    }

    #[inline(always)]
    fn get_mood_level(&self) -> ActorMood {
        self.as_ref().get_mood_level()
    }

    #[inline(always)]
    fn get_morality_level(&self) -> ActorMorality {
        self.as_ref().get_morality_level()
    }

    #[inline(always)]
    fn no_slow_approach(&self) -> bool {
        self.as_ref().no_slow_approach()
    }

    #[inline(always)]
    fn set_aggression_level(&mut self, level: ActorAggression) {
        TESAIForm::set_aggression_level(self.as_mut(), level)
    }

    #[inline(always)]
    fn set_assistance_level(&mut self, level: ActorAssistance) {
        TESAIForm::set_assistance_level(self.as_mut(), level)
    }

    #[inline(always)]
    fn set_confidence_level(&mut self, level: ActorConfidence) {
        TESAIForm::set_confidence_level(self.as_mut(), level)
    }
}
