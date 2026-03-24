use core_util::EnumSet;

use crate::re::actor_values::ActorValue;
use crate::re::form_type::FormType;
use crate::relocation::RelocationID;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum EffectArchetypeId {
    None = -1,
    ValueModifier = 0,
    Script = 1,
    Dispel = 2,
    CureDisease = 3,
    Absorb = 4,
    DualValueModifier = 5,
    Calm = 6,
    Demoralize = 7,
    Frenzy = 8,
    Disarm = 9,
    CommandSummoned = 10,
    Invisibility = 11,
    Light = 12,
    Darkness = 13,
    NightEye = 14,
    Lock = 15,
    Open = 16,
    BoundWeapon = 17,
    SummonCreature = 18,
    DetectLife = 19,
    Telekinesis = 20,
    Paralysis = 21,
    Reanimate = 22,
    SoulTrap = 23,
    TurnUndead = 24,
    Guide = 25,
    WerewolfFeed = 26,
    CureParalysis = 27,
    CureAddiction = 28,
    CurePoison = 29,
    Concussion = 30,
    ValueAndParts = 31,
    AccumulateMagnitude = 32,
    Stagger = 33,
    PeakValueModifier = 34,
    Cloak = 35,
    Werewolf = 36,
    SlowTime = 37,
    Rally = 38,
    EnhanceWeapon = 39,
    SpawnHazard = 40,
    Etherealize = 41,
    Banish = 42,
    SpawnScriptedRef = 43,
    Disguise = 44,
    GrabActor = 45,
    VampireLord = 46,
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EffectArchetypeFlags {
    None = 0,
    HiddenInEditor = 1 << 0,
    IsActorValueUsed = 1 << 1,
    IsFormUsed = 1 << 2,
    Unk3 = 1 << 3,
    AllowStacking = 1 << 4,
    CannotMultiCast = 1 << 5,
    CreatesRef = 1 << 6,
    CustomSkillUse = 1 << 7,
    RewardsSkillUseWithoutTarget = 1 << 8,
    AddsEffectToCaster = 1 << 9,
}

core_util::impl_enumset_type!(EffectArchetypeFlags => u32);

#[repr(C)]
pub struct EffectArchetypeDef {
    pub name: *const core::ffi::c_char,            // 0x00
    pub flags: EnumSet<EffectArchetypeFlags, u32>, // 0x08
    pub fixed_actor_value: ActorValue,             // 0x0C
    pub associated_form_type: FormType,            // 0x10
    pub pad14: u32,                                // 0x14
}

const _: () = assert!(core::mem::size_of::<EffectArchetypeDef>() == 0x18);
const _: () = assert!(core::mem::offset_of!(EffectArchetypeDef, name) == 0x00);
const _: () = assert!(core::mem::offset_of!(EffectArchetypeDef, flags) == 0x08);
const _: () = assert!(core::mem::offset_of!(EffectArchetypeDef, fixed_actor_value) == 0x0C);
const _: () = assert!(core::mem::offset_of!(EffectArchetypeDef, associated_form_type) == 0x10);
const _: () = assert!(core::mem::offset_of!(EffectArchetypeDef, pad14) == 0x14);

pub struct EffectArchetypes;

pub type EffectArchetype = EffectArchetypeId;

impl EffectArchetypes {
    pub const COUNT: usize = EffectArchetypeId::VampireLord as usize + 1;

    crate::relocation_variable! {
        fn archetypes() -> &'static *mut EffectArchetypeDef => RelocationID::new(500623, 358289)
    }

    #[inline]
    pub fn try_get_archetype_def(id: EffectArchetypeId) -> Option<&'static EffectArchetypeDef> {
        let index = id as i32;
        if index < 0 || index as usize >= Self::COUNT {
            return None;
        }

        let archetypes = *Self::archetypes();
        if archetypes.is_null() {
            return None;
        }

        unsafe { archetypes.add(index as usize).as_ref() }
    }

    #[inline]
    pub fn get_archetype_def(id: EffectArchetypeId) -> &'static EffectArchetypeDef {
        debug_assert!(id as i32 >= 0);
        debug_assert!((id as usize) < Self::COUNT);
        Self::try_get_archetype_def(id)
            .expect("EffectArchetypes::get_archetype_def called with invalid archetype id")
    }

    #[inline]
    pub fn get_archetype_name(id: EffectArchetypeId) -> *const core::ffi::c_char {
        Self::get_archetype_def(id).name
    }

    #[inline]
    pub fn get_archetype_name_as_str(id: EffectArchetypeId) -> &'static str {
        core_util::ptr_to_str(Self::get_archetype_name(id))
    }

    #[inline]
    pub fn get_associated_form_type(id: EffectArchetypeId) -> FormType {
        Self::get_archetype_def(id).associated_form_type
    }

    #[inline]
    pub fn get_fixed_actor_value(id: EffectArchetypeId) -> ActorValue {
        Self::get_archetype_def(id).fixed_actor_value
    }

    #[inline]
    pub fn is_flag_set(id: EffectArchetypeId, flag: EffectArchetypeFlags) -> bool {
        Self::get_archetype_def(id).flags.all(flag)
    }
}
