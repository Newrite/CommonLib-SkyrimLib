#![allow(non_camel_case_types)]

use crate::core_util::inherit;
use crate::offsets::offsets_rtti::RTTI_ActorState;
use crate::offsets::offsets_vtable::VTABLE_ActorState;
use crate::re::IMovementState;
use crate::relocation::{RttiType, VariantID};
use crate::virtual_method;
use core_util::Enum;

/// C++ `RE::ACTOR_LIFE_STATE`
#[libskyrim_macros::open_enum]
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ACTOR_LIFE_STATE {
    Alive = 0,
    Dying = 1,
    Dead = 2,
    Unconcious = 3,
    Reanimate = 4,
    Recycle = 5,
    Restrained = 6,
    EssentialDown = 7,
    Bleedout = 8,
}

/// C++ `RE::ATTACK_STATE_ENUM`
#[libskyrim_macros::open_enum]
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ATTACK_STATE_ENUM {
    None = 0,
    Draw = 1,
    Swing = 2,
    Hit = 3,
    NextAttack = 4,
    FollowThrough = 5,
    Bash = 6,
    BowDraw = 8,
    BowAttached = 9,
    BowDrawn = 10,
    BowReleasing = 11,
    BowReleased = 12,
    BowNextAttack = 13,
    BowFollowThrough = 14,
    Fire = 15,
    Firing = 16,
    Fired = 17,
}

/// C++ `RE::FLY_STATE`
#[libskyrim_macros::open_enum]
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FLY_STATE {
    None = 0,
    TakeOff = 1,
    Cruising = 2,
    Hovering = 3,
    Landing = 4,
    Perching = 5,
    Action = 6,
}

/// C++ `RE::KNOCK_STATE_ENUM`
#[libskyrim_macros::open_enum]
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum KNOCK_STATE_ENUM {
    Normal = 0,
    Explode = 1,
    ExplodeLeadIn = 2,
    Out = 3,
    OutLeadIn = 4,
    Queued = 5,
    GetUp = 6,
    Down = 7,
    WaitForTaskQueue = 8,
}

/// C++ `RE::SIT_SLEEP_STATE`
#[libskyrim_macros::open_enum]
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SIT_SLEEP_STATE {
    Normal = 0,
    WantToSit = 1,
    WaitingForSitAnim = 2,
    IsSitting = 3,
    WantToStand = 4,
    WantToSleep = 5,
    WaitingForSleepAnim = 6,
    IsSleeping = 7,
    WantToWake = 8,
}

impl SIT_SLEEP_STATE {
    pub const RIDING_MOUNT: Self = Self::IsSitting;
}

/// C++ `RE::WEAPON_STATE`
#[libskyrim_macros::open_enum]
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum WEAPON_STATE {
    Sheathed = 0,
    WantToDraw = 1,
    Drawing = 2,
    Drawn = 3,
    WantToSheathe = 4,
    Sheathing = 5,
}

/// C++ `RE::ActorState::ActorState1`
///
/// CommonLib stores these named fields as C++ bitfields packed into one `u32`.
#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ActorState1 {
    pub bits: u32, // 00
}

const _: () = assert!(core::mem::size_of::<ActorState1>() == 0x4);
const _: () = assert!(core::mem::offset_of!(ActorState1, bits) == 0x00);

impl ActorState1 {
    #[inline(always)]
    const fn test_bit(value: u32, bit: u32) -> bool {
        value & (1u32 << bit) != 0
    }

    #[inline(always)]
    const fn get_bits(value: u32, shift: u32, width: u32) -> u32 {
        (value >> shift) & ((1u32 << width) - 1)
    }

    #[inline(always)]
    pub const fn moving_back(&self) -> bool {
        Self::test_bit(self.bits, 0)
    }

    #[inline(always)]
    pub const fn moving_forward(&self) -> bool {
        Self::test_bit(self.bits, 1)
    }

    #[inline(always)]
    pub const fn moving_right(&self) -> bool {
        Self::test_bit(self.bits, 2)
    }

    #[inline(always)]
    pub const fn moving_left(&self) -> bool {
        Self::test_bit(self.bits, 3)
    }

    #[inline(always)]
    pub const fn unk04(&self) -> u32 {
        Self::get_bits(self.bits, 4, 2)
    }

    #[inline(always)]
    pub const fn walking(&self) -> bool {
        Self::test_bit(self.bits, 6)
    }

    #[inline(always)]
    pub const fn running(&self) -> bool {
        Self::test_bit(self.bits, 7)
    }

    #[inline(always)]
    pub const fn sprinting(&self) -> bool {
        Self::test_bit(self.bits, 8)
    }

    #[inline(always)]
    pub const fn sneaking(&self) -> bool {
        Self::test_bit(self.bits, 9)
    }

    #[inline(always)]
    pub const fn swimming(&self) -> bool {
        Self::test_bit(self.bits, 10)
    }

    #[inline(always)]
    pub const fn unk11(&self) -> u32 {
        Self::get_bits(self.bits, 11, 3)
    }

    #[inline(always)]
    pub const fn sit_sleep_state_storage(&self) -> Enum<SIT_SLEEP_STATE, u32> {
        Enum::from_underlying(Self::get_bits(self.bits, 14, 4))
    }

    #[inline(always)]
    pub fn try_get_sit_sleep_state(&self) -> Option<SIT_SLEEP_STATE> {
        self.sit_sleep_state_storage().get()
    }

    #[inline(always)]
    pub fn sit_sleep_state(&self) -> SIT_SLEEP_STATE {
        self.try_get_sit_sleep_state()
            .unwrap_or(SIT_SLEEP_STATE::Normal)
    }

    #[inline(always)]
    pub const fn fly_state_storage(&self) -> Enum<FLY_STATE, u32> {
        Enum::from_underlying(Self::get_bits(self.bits, 18, 3))
    }

    #[inline(always)]
    pub fn try_get_fly_state(&self) -> Option<FLY_STATE> {
        self.fly_state_storage().get()
    }

    #[inline(always)]
    pub fn fly_state(&self) -> FLY_STATE {
        self.try_get_fly_state().unwrap_or(FLY_STATE::None)
    }

    #[inline(always)]
    pub const fn life_state_storage(&self) -> Enum<ACTOR_LIFE_STATE, u32> {
        Enum::from_underlying(Self::get_bits(self.bits, 21, 4))
    }

    #[inline(always)]
    pub fn try_get_life_state(&self) -> Option<ACTOR_LIFE_STATE> {
        self.life_state_storage().get()
    }

    #[inline(always)]
    pub fn life_state(&self) -> ACTOR_LIFE_STATE {
        self.try_get_life_state().unwrap_or(ACTOR_LIFE_STATE::Alive)
    }

    #[inline(always)]
    pub const fn knock_state_storage(&self) -> Enum<KNOCK_STATE_ENUM, u32> {
        Enum::from_underlying(Self::get_bits(self.bits, 25, 3))
    }

    #[inline(always)]
    pub fn try_get_knock_state(&self) -> Option<KNOCK_STATE_ENUM> {
        self.knock_state_storage().get()
    }

    #[inline(always)]
    pub fn knock_state(&self) -> KNOCK_STATE_ENUM {
        self.try_get_knock_state()
            .unwrap_or(KNOCK_STATE_ENUM::Normal)
    }

    #[inline(always)]
    pub const fn melee_attack_state_storage(&self) -> Enum<ATTACK_STATE_ENUM, u32> {
        Enum::from_underlying(Self::get_bits(self.bits, 28, 4))
    }

    #[inline(always)]
    pub fn try_get_melee_attack_state(&self) -> Option<ATTACK_STATE_ENUM> {
        self.melee_attack_state_storage().get()
    }

    #[inline(always)]
    pub fn melee_attack_state(&self) -> ATTACK_STATE_ENUM {
        self.try_get_melee_attack_state()
            .unwrap_or(ATTACK_STATE_ENUM::None)
    }
}

/// C++ `RE::ActorState::ActorState2`
///
/// CommonLib stores these named fields as C++ bitfields packed into one `u32`.
#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ActorState2 {
    pub bits: u32, // 00
}

const _: () = assert!(core::mem::size_of::<ActorState2>() == 0x4);
const _: () = assert!(core::mem::offset_of!(ActorState2, bits) == 0x00);

impl ActorState2 {
    #[inline(always)]
    const fn test_bit(value: u32, bit: u32) -> bool {
        value & (1u32 << bit) != 0
    }

    #[inline(always)]
    const fn get_bits(value: u32, shift: u32, width: u32) -> u32 {
        (value >> shift) & ((1u32 << width) - 1)
    }

    #[inline(always)]
    pub const fn talking_to_player(&self) -> bool {
        Self::test_bit(self.bits, 0)
    }

    #[inline(always)]
    pub const fn force_run(&self) -> bool {
        Self::test_bit(self.bits, 1)
    }

    #[inline(always)]
    pub const fn force_sneak(&self) -> bool {
        Self::test_bit(self.bits, 2)
    }

    #[inline(always)]
    pub const fn head_tracking(&self) -> bool {
        Self::test_bit(self.bits, 3)
    }

    #[inline(always)]
    pub const fn reanimating(&self) -> bool {
        Self::test_bit(self.bits, 4)
    }

    #[inline(always)]
    pub const fn weapon_state_storage(&self) -> Enum<WEAPON_STATE, u32> {
        Enum::from_underlying(Self::get_bits(self.bits, 5, 3))
    }

    #[inline(always)]
    pub fn try_get_weapon_state(&self) -> Option<WEAPON_STATE> {
        self.weapon_state_storage().get()
    }

    #[inline(always)]
    pub fn weapon_state(&self) -> WEAPON_STATE {
        self.try_get_weapon_state()
            .unwrap_or(WEAPON_STATE::Sheathed)
    }

    #[inline(always)]
    pub const fn want_blocking(&self) -> bool {
        Self::test_bit(self.bits, 8)
    }

    #[inline(always)]
    pub const fn flight_blocked(&self) -> bool {
        Self::test_bit(self.bits, 9)
    }

    #[inline(always)]
    pub const fn recoil(&self) -> u32 {
        Self::get_bits(self.bits, 10, 2)
    }

    #[inline(always)]
    pub const fn allow_flying(&self) -> bool {
        Self::test_bit(self.bits, 12)
    }

    #[inline(always)]
    pub const fn staggered(&self) -> bool {
        Self::test_bit(self.bits, 13)
    }

    #[inline(always)]
    pub const fn unk14(&self) -> u32 {
        Self::get_bits(self.bits, 14, 18)
    }
}

/// C++ `RE::ActorState`
#[repr(C)]
pub struct ActorState {
    pub base: IMovementState,      // 00
    pub actor_state1: ActorState1, // 08
    pub actor_state2: ActorState2, // 0C
}

const _: () = assert!(core::mem::size_of::<ActorState>() == 0x10);
const _: () = assert!(core::mem::offset_of!(ActorState, base) == 0x00);
const _: () = assert!(core::mem::offset_of!(ActorState, actor_state1) == 0x08);
const _: () = assert!(core::mem::offset_of!(ActorState, actor_state2) == 0x0C);

impl RttiType for ActorState {
    const RTTI: VariantID = RTTI_ActorState;
}

inherit!(ActorState : IMovementState, base);

impl AsRef<ActorState> for ActorState {
    #[inline(always)]
    fn as_ref(&self) -> &Self {
        self
    }
}

impl AsMut<ActorState> for ActorState {
    #[inline(always)]
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl ActorState {
    pub const RTTI: VariantID = RTTI_ActorState;
    pub const VTABLE: &'static [VariantID] = &VTABLE_ActorState;

    // override (IMovementState)
    // void  Unk_01() override;                // 01
    // void  Unk_02() override;                // 02
    // void  Unk_03() override;                // 03
    // void  Unk_04() override;                // 04
    // float DoGetMovementSpeed() override;    // 05
    // float DoGetRotationSpeed() override;    // 06
    // void  Unk_07() override;                // 07
    // void  Unk_08() override;                // 08

    virtual_method! {
        pub const VFUNC_DO_SET_SIT_SLEEP_STATE: usize = 0x14;
        pub fn do_set_sit_sleep_state(&mut self, a_state: SIT_SLEEP_STATE) -> bool
    }

    virtual_method! {
        pub const VFUNC_UNK_15: usize = 0x15;
        pub fn unk_15(&mut self)
    }

    #[inline(always)]
    pub fn get_attack_state(&self) -> ATTACK_STATE_ENUM {
        self.actor_state1.melee_attack_state()
    }

    #[inline(always)]
    pub fn get_fly_state(&self) -> FLY_STATE {
        self.actor_state1.fly_state()
    }

    #[inline(always)]
    pub fn get_knock_state(&self) -> KNOCK_STATE_ENUM {
        self.actor_state1.knock_state()
    }

    #[inline(always)]
    pub fn get_life_state(&self) -> ACTOR_LIFE_STATE {
        self.actor_state1.life_state()
    }

    #[inline(always)]
    pub fn get_sit_sleep_state(&self) -> SIT_SLEEP_STATE {
        self.actor_state1.sit_sleep_state()
    }

    #[inline(always)]
    pub fn get_weapon_state(&self) -> WEAPON_STATE {
        self.actor_state2.weapon_state()
    }

    #[inline(always)]
    pub fn is_bleeding_out(&self) -> bool {
        matches!(
            self.get_life_state(),
            ACTOR_LIFE_STATE::EssentialDown | ACTOR_LIFE_STATE::Bleedout
        )
    }

    #[inline(always)]
    pub fn is_flying(&self) -> bool {
        !matches!(self.get_fly_state(), FLY_STATE::None | FLY_STATE::Perching)
    }

    #[inline(always)]
    pub fn is_reanimated(&self) -> bool {
        self.get_life_state() == ACTOR_LIFE_STATE::Reanimate
    }

    #[inline(always)]
    pub fn is_sitting(&self) -> bool {
        matches!(
            self.get_sit_sleep_state(),
            SIT_SLEEP_STATE::IsSitting | SIT_SLEEP_STATE::WantToStand
        )
    }

    #[inline(always)]
    pub fn is_running(&self) -> bool {
        self.actor_state1.running()
    }

    #[inline(always)]
    pub fn is_sneaking(&self) -> bool {
        self.actor_state1.sneaking()
    }

    #[inline(always)]
    pub fn is_sprinting(&self) -> bool {
        self.actor_state1.sprinting()
    }

    #[inline(always)]
    pub fn is_staggered(&self) -> bool {
        self.actor_state2.staggered()
    }

    #[inline(always)]
    pub fn is_swimming(&self) -> bool {
        self.actor_state1.swimming()
    }

    #[inline(always)]
    pub fn is_unconscious(&self) -> bool {
        self.get_life_state() == ACTOR_LIFE_STATE::Unconcious
    }

    #[inline(always)]
    pub fn is_walking(&self) -> bool {
        self.actor_state1.walking()
    }

    #[inline(always)]
    pub fn is_weapon_drawn(&self) -> bool {
        matches!(
            self.get_weapon_state(),
            WEAPON_STATE::Drawn | WEAPON_STATE::WantToSheathe | WEAPON_STATE::Sheathing
        )
    }
}

pub trait ActorStateExt {
    fn do_set_sit_sleep_state(&mut self, a_state: SIT_SLEEP_STATE) -> bool;
    fn unk_15(&mut self);
    fn get_attack_state(&self) -> ATTACK_STATE_ENUM;
    fn get_fly_state(&self) -> FLY_STATE;
    fn get_knock_state(&self) -> KNOCK_STATE_ENUM;
    fn get_life_state(&self) -> ACTOR_LIFE_STATE;
    fn get_sit_sleep_state(&self) -> SIT_SLEEP_STATE;
    fn get_weapon_state(&self) -> WEAPON_STATE;
    fn is_bleeding_out(&self) -> bool;
    fn is_flying(&self) -> bool;
    fn is_reanimated(&self) -> bool;
    fn is_sitting(&self) -> bool;
    fn is_running(&self) -> bool;
    fn is_sneaking(&self) -> bool;
    fn is_sprinting(&self) -> bool;
    fn is_staggered(&self) -> bool;
    fn is_swimming(&self) -> bool;
    fn is_unconscious(&self) -> bool;
    fn is_walking(&self) -> bool;
    fn is_weapon_drawn(&self) -> bool;
}

impl<T: AsRef<ActorState> + AsMut<ActorState>> ActorStateExt for T {
    #[inline(always)]
    fn do_set_sit_sleep_state(&mut self, a_state: SIT_SLEEP_STATE) -> bool {
        ActorState::do_set_sit_sleep_state(self.as_mut(), a_state)
    }

    #[inline(always)]
    fn unk_15(&mut self) {
        ActorState::unk_15(self.as_mut())
    }

    #[inline(always)]
    fn get_attack_state(&self) -> ATTACK_STATE_ENUM {
        ActorState::get_attack_state(self.as_ref())
    }

    #[inline(always)]
    fn get_fly_state(&self) -> FLY_STATE {
        ActorState::get_fly_state(self.as_ref())
    }

    #[inline(always)]
    fn get_knock_state(&self) -> KNOCK_STATE_ENUM {
        ActorState::get_knock_state(self.as_ref())
    }

    #[inline(always)]
    fn get_life_state(&self) -> ACTOR_LIFE_STATE {
        ActorState::get_life_state(self.as_ref())
    }

    #[inline(always)]
    fn get_sit_sleep_state(&self) -> SIT_SLEEP_STATE {
        ActorState::get_sit_sleep_state(self.as_ref())
    }

    #[inline(always)]
    fn get_weapon_state(&self) -> WEAPON_STATE {
        ActorState::get_weapon_state(self.as_ref())
    }

    #[inline(always)]
    fn is_bleeding_out(&self) -> bool {
        ActorState::is_bleeding_out(self.as_ref())
    }

    #[inline(always)]
    fn is_flying(&self) -> bool {
        ActorState::is_flying(self.as_ref())
    }

    #[inline(always)]
    fn is_reanimated(&self) -> bool {
        ActorState::is_reanimated(self.as_ref())
    }

    #[inline(always)]
    fn is_sitting(&self) -> bool {
        ActorState::is_sitting(self.as_ref())
    }

    #[inline(always)]
    fn is_running(&self) -> bool {
        ActorState::is_running(self.as_ref())
    }

    #[inline(always)]
    fn is_sneaking(&self) -> bool {
        ActorState::is_sneaking(self.as_ref())
    }

    #[inline(always)]
    fn is_sprinting(&self) -> bool {
        ActorState::is_sprinting(self.as_ref())
    }

    #[inline(always)]
    fn is_staggered(&self) -> bool {
        ActorState::is_staggered(self.as_ref())
    }

    #[inline(always)]
    fn is_swimming(&self) -> bool {
        ActorState::is_swimming(self.as_ref())
    }

    #[inline(always)]
    fn is_unconscious(&self) -> bool {
        ActorState::is_unconscious(self.as_ref())
    }

    #[inline(always)]
    fn is_walking(&self) -> bool {
        ActorState::is_walking(self.as_ref())
    }

    #[inline(always)]
    fn is_weapon_drawn(&self) -> bool {
        ActorState::is_weapon_drawn(self.as_ref())
    }
}
