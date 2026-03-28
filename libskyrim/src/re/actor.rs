#![allow(non_camel_case_types)]

use bitflags::bitflags;
use core_util::{EnumSet, inherit};

use crate::offsets::offsets_rtti::{RTTI_Actor, RTTI_Actor__ForEachSpellVisitor};
use crate::offsets::offsets_vtable::{VTABLE_Actor, VTABLE_Actor__ForEachSpellVisitor};
use crate::re::{
    ACTOR_LOS_LOCATION, AIProcess, AITimeStamp, ActorHandle, ActorMagicCaster,
    ActorMotionFeedbackData, ActorMotionFeedbackOutput, ActorMover, ActorState, ActorValue,
    ActorValueModifier, ActorValueOwner, BGSAttackData, BGSDefaultObjectManager, BGSDialogueBranch,
    BGSEquipSlot, BGSKeyword, BGSPerk, BSAnimationGraphEvent, BSAnimationGraphManager,
    BSContainerForEachResult, BSExtraData, BSFaceGenAnimationData, BSFixedString, BSTEventSink,
    BSTSmartPointer, BSTransformDeltaEvent, BipedAnim, CFilter, CombatController, CombatGroup,
    DETECTION_PRIORITY, EmotionType, ExtraCanTalkToPlayer, ExtraDataList, ExtraDataType,
    ExtraLeveledCreature, FIGHT_REACTION, FormCastable, FormType, HighProcessData,
    IAnimationGraphManagerHolder, IAnimationGraphManagerHolderExt,
    IPostAnimationChannelUpdateFunctor, InventoryEntryData, MagicCaster, MagicItem, MagicTarget,
    MiddleHighProcessData, MovementControllerNPC, NiPoint3, NiPointer, NiRef, ObjectRefHandle,
    PROCESS_TYPE, PackageLocation, PerkEntryVisitor, ProcessLists, SpellItem, TESBoundObject,
    TESFaction, TESForm, TESIdleForm, TESNPC, TESObjectCELL, TESObjectMISC, TESObjectREFR,
    TESPackage, TESRace, TESShout, TESTopicInfo, TESWordOfPower, TrespassPackage,
    bhkCharacterController, bhkCharacterMoveFinishEvent,
};
use crate::relocation::{RelocationID, RttiType, VariantID, VariantOffset, skyrim_cast};
use crate::version::RUNTIME_SSE_1_6_629;

pub type ActorHandlePtr = NiPointer<Actor>;
pub type ACTOR_VALUE_MODIFIER = ActorValueModifier;
pub type EntryPoint = u32;

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SKILL_ACTION {
    NormalUse = 0,
    PowerAttack = 1,
    Bash = 2,
    LockpickSuccess = 3,
    LockpickBroken = 4,
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ACTOR_CRITICAL_STAGE {
    None = 0,
    GooStart = 1,
    GooEnd = 2,
    DisintegrateStart = 3,
    DisintegrateEnd = 4,
    Total = 5,
}

core_util::impl_enumset_type!(ACTOR_CRITICAL_STAGE => u32);

pub struct SlotTypes;

impl SlotTypes {
    pub const LEFT_HAND: usize = 0;
    pub const RIGHT_HAND: usize = 1;
    pub const UNKNOWN: usize = 2;
    pub const POWER_OR_SHOUT: usize = 3;
    pub const TOTAL: usize = 4;
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BOOL_BITS {
    None = 0,
    DelayUpdateScenegraph = 1 << 0,
    ProcessMe = 1 << 1,
    MurderAlarm = 1 << 2,
    HasSceneExtra = 1 << 3,
    HeadingFixed = 1 << 4,
    SpeakingDone = 1 << 5,
    IgnoreChangeAnimationCall = 1 << 6,
    SoundFileDone = 1 << 7,
    VoiceFileDone = 1 << 8,
    InTempChangeList = 1 << 9,
    DoNotRunSayToCallback = 1 << 10,
    Dead = 1 << 11,
    ForceGreetingPlayer = 1 << 12,
    ForceUpdateQuestTarget = 1 << 13,
    SearchingInCombat = 1 << 14,
    AttackOnNextTheft = 1 << 15,
    EvpBuffered = 1 << 16,
    ResetAI = 1 << 17,
    InWater = 1 << 18,
    Swimming = 1 << 19,
    VoicePausedByScript = 1 << 20,
    WasInFrustrum = 1 << 21,
    ShouldRotateToTrack = 1 << 22,
    SetOnDeath = 1 << 23,
    DoNotPadVoice = 1 << 24,
    FootIKInRange = 1 << 25,
    PlayerTeammate = 1 << 26,
    GivePlayerXP = 1 << 27,
    SoundCallbackSuccess = 1 << 28,
    UseEmotion = 1 << 29,
    Guard = 1 << 30,
    Paralyzed = 1 << 31,
}

core_util::impl_enumset_type!(BOOL_BITS => u32);

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BOOL_FLAGS {
    None = 0,
    IsAMount = 1 << 1,
    ScenePackage = 1 << 0,
    MountPointClear = 1 << 2,
    GettingOnOffMount = 1 << 3,
    InRandomScene = 1 << 4,
    NoBleedoutRecovery = 1 << 5,
    InBleedoutAnimation = 1 << 6,
    CanDoFavor = 1 << 7,
    ShouldAnimGraphUpdate = 1 << 8,
    CanSpeakToEssentialDown = 1 << 9,
    BribedByPlayer = 1 << 10,
    AngryWithPlayer = 1 << 11,
    IsTrespassing = 1 << 12,
    CanSpeak = 1 << 13,
    IsInKillMove = 1 << 14,
    AttackOnSight = 1 << 15,
    IsCommandedActor = 1 << 16,
    ForceOneAnimgraphUpdate = 1 << 17,
    Essential = 1 << 18,
    Protected = 1 << 19,
    AttackingDisabled = 1 << 20,
    CastingDisabled = 1 << 21,
    SceneHeadTrackRotation = 1 << 22,
    ForceIncMinBoneUpdate = 1 << 23,
    CrimeSearch = 1 << 24,
    MovingIntoLoadedArea = 1 << 25,
    DoNotShowOnStealthMeter = 1 << 26,
    MovementBlocked = 1 << 27,
    AllowInstantFurniturePopInPlayerCell = 1 << 28,
    ForceAnimGraphUpdate = 1 << 29,
    CheckAddEffectDualCast = 1 << 30,
    Underwater = 1 << 31,
}

core_util::impl_enumset_type!(BOOL_FLAGS => u32);

#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct Modifiers {
    pub modifiers: [f32; 3], // 00
}

const _: () = assert!(core::mem::size_of::<Modifiers>() == 0x0C);

#[repr(C)]
pub struct ActorValueStorageLocalMap<T> {
    pub actor_values: BSFixedString, // 00
    pub entries: *mut T,             // 08
}

const _: () = assert!(core::mem::size_of::<ActorValueStorageLocalMap<f32>>() == 0x10);

impl<T> ActorValueStorageLocalMap<T> {
    #[inline(always)]
    fn get_at_index(&self, actor_value: ActorValue) -> Option<usize> {
        let actor_values = self.actor_values.as_ptr();
        if actor_values.is_null() || self.entries.is_null() {
            return None;
        }

        let actor_value = actor_value as i8 as u8;
        let mut idx = 0usize;
        unsafe {
            while *actor_values.add(idx) != 0 {
                if *actor_values.add(idx) as u8 == actor_value {
                    return Some(idx);
                }
                idx += 1;
            }
        }
        None
    }

    #[inline(always)]
    pub fn get(&self, actor_value: ActorValue) -> Option<&T> {
        let idx = self.get_at_index(actor_value)?;
        unsafe { self.entries.add(idx).as_ref() }
    }

    #[inline(always)]
    pub fn get_mut(&mut self, actor_value: ActorValue) -> Option<&mut T> {
        let idx = self.get_at_index(actor_value)?;
        unsafe { self.entries.add(idx).as_mut() }
    }
}

#[repr(C)]
pub struct ActorValueStorage {
    pub base_values: ActorValueStorageLocalMap<f32>, // 00
    pub modifiers: ActorValueStorageLocalMap<Modifiers>, // 10
}

const _: () = assert!(core::mem::size_of::<ActorValueStorage>() == 0x20);

bitflags! {
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct ActorChangeFlags: u32 {
        const LIFE_STATE = 1 << 10;
        const PACKAGE_EXTRA_DATA = 1 << 11;
        const MERCHANT_CONTAINER = 1 << 12;
        const DISMEMBERED_LIMBS = 1 << 17;
        const LEVELED_ACTOR = 1 << 18;
        const DISP_MODIFIERS = 1 << 19;
        const TEMP_MODIFIERS = 1 << 20;
        const DAMAGE_MODIFIERS = 1 << 21;
        const OVERRIDE_MODIFIERS = 1 << 22;
        const PERMANENT_MODIFIERS = 1 << 23;
    }
}

bitflags! {
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct ActorRecordFlags: u32 {
        const DELETED = 1 << 5;
        const STARTS_DEAD = 1 << 9;
        const PERSISTENT = 1 << 10;
        const INITIALLY_DISABLED = 1 << 11;
        const IGNORED = 1 << 12;
        const NO_AI_ACQUIRE = 1 << 25;
        const DONT_HAVOK_SETTLE = 1 << 29;
    }
}

#[repr(C)]
pub struct ActorForEachSpellVisitor {
    pub vtable: *const usize, // 00
}

const _: () = assert!(core::mem::size_of::<ActorForEachSpellVisitor>() == 0x08);

impl RttiType for ActorForEachSpellVisitor {
    const RTTI: VariantID = RTTI_Actor__ForEachSpellVisitor;
}

impl ActorForEachSpellVisitor {
    pub const RTTI: VariantID = RTTI_Actor__ForEachSpellVisitor;
    pub const VTABLE: &'static [VariantID] = &VTABLE_Actor__ForEachSpellVisitor;

    crate::virtual_method! {
        pub const VFUNC_DTOR: usize = 0x00;
        pub fn dtor()
    }

    crate::virtual_method! {
        pub const VFUNC_VISIT: usize = 0x01;
        pub fn visit(spell: *mut SpellItem) -> BSContainerForEachResult
    }
}

#[repr(C)]
pub struct ACTOR_RUNTIME_DATA {
    pub bool_bits: EnumSet<BOOL_BITS, u32>,                 // 000
    pub update_target_timer: f32,                           // 004
    pub critical_stage: EnumSet<ACTOR_CRITICAL_STAGE, u32>, // 008
    pub pad00c: u32,                                        // 00C
    pub current_process: *mut AIProcess,                    // 010
    pub dialogue_item_target: ObjectRefHandle,              // 018
    pub current_combat_target: ActorHandle,                 // 01C
    pub my_killer: ActorHandle,                             // 020
    pub check_my_dead_body_timer: f32,                      // 024
    pub voice_timer: f32,                                   // 028
    pub under_water_timer: f32,                             // 02C
    pub thief_crime_stamp: i32,                             // 030
    pub action_value: i32,                                  // 034
    pub timer_on_action: f32,                               // 038
    pub unk03c: u32,                                        // 03C
    pub editor_loc_coord: NiPoint3,                         // 040
    pub editor_loc_rot: f32,                                // 04C
    pub editor_loc_form: *mut TESForm,                      // 050
    pub editor_location: *mut crate::re::BGSLocation,       // 058
    pub actor_mover: *mut ActorMover,                       // 060
    pub movement_controller: BSTSmartPointer<MovementControllerNPC>, // 068
    pub unk070: *mut TESPackage,                            // 070
    pub combat_controller: *mut CombatController,           // 078
    pub vendor_faction: *mut TESFaction,                    // 080
    pub calculate_vendor_faction_timer: AITimeStamp,        // 088
    pub emotion_type: EmotionType,                          // 08C
    pub emotion_value: u32,                                 // 090
    pub unk094: u32,                                        // 094
    pub unk098: u32,                                        // 098
    pub intimidate_bribe_day_stamp: u32,                    // 09C
    pub unk0a0: u64,                                        // 0A0
    pub added_spells:
        crate::re::BSTSmallArray<*mut SpellItem, { core::mem::size_of::<*mut SpellItem>() }>, // 0A8
    pub magic_casters: [*mut ActorMagicCaster; SlotTypes::TOTAL], // 0C0
    pub selected_spells: [*mut MagicItem; SlotTypes::TOTAL], // 0E0
    pub selected_power: *mut TESForm,                       // 100
    pub unk108: u32,                                        // 108
    pub pad10c: u32,                                        // 10C
    pub race: *mut TESRace,                                 // 110
    pub equipped_weight: f32,                               // 118
    pub bool_flags: EnumSet<BOOL_FLAGS, u32>,               // 11C
    pub av_storage: ActorValueStorage,                      // 120
    pub exclusive_branch: *mut BGSDialogueBranch,           // 140
    pub health_modifiers: Modifiers,                        // 148
    pub magicka_modifiers: Modifiers,                       // 154
    pub stamina_modifiers: Modifiers,                       // 160
    pub voice_points_modifiers: Modifiers,                  // 16C
    pub last_update: f32,                                   // 178
    pub last_seen_time: u32,                                // 17C
    pub biped: BSTSmartPointer<BipedAnim>,                  // 180
    pub armor_rating: f32,                                  // 188
    pub armor_base_factor_sum: f32,                         // 18C
    pub sound_callback_set: i8,                             // 190
    pub unk191: u8,                                         // 191
    pub unk192: u8,                                         // 192
    pub unk193: u8,                                         // 193
    pub unk194: u32,                                        // 194
    pub unk198: u64,                                        // 198
    pub unk1a0: u64,                                        // 1A0
    pub unk1a8: crate::rex::W32::CRITICAL_SECTION,          // 1A8
}

const _: () = assert!(core::mem::size_of::<ACTOR_RUNTIME_DATA>() == 0x1D0);
const _: () = assert!(core::mem::offset_of!(ACTOR_RUNTIME_DATA, current_process) == 0x10);
const _: () = assert!(core::mem::offset_of!(ACTOR_RUNTIME_DATA, movement_controller) == 0x68);
const _: () = assert!(core::mem::offset_of!(ACTOR_RUNTIME_DATA, magic_casters) == 0xC0);
const _: () = assert!(core::mem::offset_of!(ACTOR_RUNTIME_DATA, race) == 0x110);
const _: () = assert!(core::mem::offset_of!(ACTOR_RUNTIME_DATA, bool_flags) == 0x11C);
const _: () = assert!(core::mem::offset_of!(ACTOR_RUNTIME_DATA, exclusive_branch) == 0x140);
const _: () = assert!(core::mem::offset_of!(ACTOR_RUNTIME_DATA, biped) == 0x180);
const _: () = assert!(core::mem::offset_of!(ACTOR_RUNTIME_DATA, unk1a8) == 0x1A8);

#[repr(C)]
pub struct Actor {
    pub base: TESObjectREFR, // 00
}

const _: () = assert!(core::mem::size_of::<Actor>() == 0x80);
const _: () = assert!(core::mem::offset_of!(Actor, base) == 0x00);

impl RttiType for Actor {
    const RTTI: VariantID = RTTI_Actor;
}

impl FormCastable for Actor {
    const TARGET_FORM_TYPE: FormType = FormType::ActorCharacter;
}

impl AsRef<MagicTarget> for Actor {
    #[inline(always)]
    fn as_ref(&self) -> &MagicTarget {
        self.as_magic_target()
    }
}

impl AsMut<MagicTarget> for Actor {
    #[inline(always)]
    fn as_mut(&mut self) -> &mut MagicTarget {
        self.as_magic_target_mut()
    }
}

impl AsRef<ActorValueOwner> for Actor {
    #[inline(always)]
    fn as_ref(&self) -> &ActorValueOwner {
        self.as_actor_value_owner()
    }
}

impl AsMut<ActorValueOwner> for Actor {
    #[inline(always)]
    fn as_mut(&mut self) -> &mut ActorValueOwner {
        self.as_actor_value_owner_mut()
    }
}

impl AsRef<ActorState> for Actor {
    #[inline(always)]
    fn as_ref(&self) -> &ActorState {
        self.as_actor_state()
    }
}

impl AsMut<ActorState> for Actor {
    #[inline(always)]
    fn as_mut(&mut self) -> &mut ActorState {
        self.as_actor_state_mut()
    }
}

impl AsRef<IPostAnimationChannelUpdateFunctor> for Actor {
    #[inline(always)]
    fn as_ref(&self) -> &IPostAnimationChannelUpdateFunctor {
        self.as_ipost_animation_channel_update_functor()
    }
}

impl AsMut<IPostAnimationChannelUpdateFunctor> for Actor {
    #[inline(always)]
    fn as_mut(&mut self) -> &mut IPostAnimationChannelUpdateFunctor {
        self.as_ipost_animation_channel_update_functor_mut()
    }
}

impl AsRef<IAnimationGraphManagerHolder> for Actor {
    #[inline(always)]
    fn as_ref(&self) -> &IAnimationGraphManagerHolder {
        self.base.as_ref()
    }
}

impl AsMut<IAnimationGraphManagerHolder> for Actor {
    #[inline(always)]
    fn as_mut(&mut self) -> &mut IAnimationGraphManagerHolder {
        self.base.as_mut()
    }
}

impl NiRef for Actor {
    #[inline(always)]
    fn inc_ref(&self) {
        self.base.inc_ref();
    }

    #[inline(always)]
    fn dec_ref(&self) {
        self.base.dec_ref();
    }
}

inherit!(Actor : TESObjectREFR);

impl Actor {
    pub const RTTI: VariantID = RTTI_Actor;
    pub const VTABLE: &'static [VariantID] = &VTABLE_Actor;
    pub const FORMTYPE: FormType = FormType::ActorCharacter;
    pub const MAGIC_TARGET_OFFSET: VariantOffset = VariantOffset::new(0x98, 0xA0, 0x98);
    pub const ACTOR_VALUE_OWNER_OFFSET: VariantOffset = VariantOffset::new(0xB0, 0xB8, 0xB0);
    pub const ACTOR_STATE_OFFSET: VariantOffset = VariantOffset::new(0xB8, 0xC0, 0xB8);
    pub const BSTRANSFORM_DELTA_EVENT_SINK_OFFSET: VariantOffset =
        VariantOffset::new(0xC8, 0xD0, 0xC8);
    pub const CHARACTER_MOVE_FINISH_EVENT_SINK_OFFSET: VariantOffset =
        VariantOffset::new(0xD0, 0xD8, 0xD0);
    pub const IPOST_ANIMATION_CHANNEL_UPDATE_FUNCTOR_OFFSET: VariantOffset =
        VariantOffset::new(0xD8, 0xE0, 0xD8);
    pub const ACTOR_RUNTIME_DATA_OFFSET: VariantOffset = VariantOffset::new(0xE0, 0xE8, 0xE0);
    pub const FULL_SIZE: VariantOffset = VariantOffset::new(0x2B0, 0x2B8, 0x2B0);

    crate::runtime_data_accessor! {
        fn actor_runtime_data() -> ACTOR_RUNTIME_DATA {
            version: RUNTIME_SSE_1_6_629,
            older: 0xE0,
            newer: 0xE8
        }
    }

    crate::runtime_data_mut_accessor! {
        fn actor_runtime_data_mut() -> ACTOR_RUNTIME_DATA {
            version: RUNTIME_SSE_1_6_629,
            older: 0xE0,
            newer: 0xE8
        }
    }

    crate::runtime_cast_accessor! {
        pub fn as_magic_target() -> MagicTarget {
            offset: Self::MAGIC_TARGET_OFFSET
        }
    }

    crate::runtime_cast_mut_accessor! {
        pub fn as_magic_target_mut() -> MagicTarget {
            offset: Self::MAGIC_TARGET_OFFSET
        }
    }

    crate::runtime_cast_accessor! {
        pub fn as_actor_value_owner() -> ActorValueOwner {
            offset: Self::ACTOR_VALUE_OWNER_OFFSET
        }
    }

    crate::runtime_cast_mut_accessor! {
        pub fn as_actor_value_owner_mut() -> ActorValueOwner {
            offset: Self::ACTOR_VALUE_OWNER_OFFSET
        }
    }

    crate::runtime_cast_accessor! {
        pub fn as_actor_state() -> ActorState {
            offset: Self::ACTOR_STATE_OFFSET
        }
    }

    crate::runtime_cast_mut_accessor! {
        pub fn as_actor_state_mut() -> ActorState {
            offset: Self::ACTOR_STATE_OFFSET
        }
    }

    crate::runtime_cast_accessor! {
        pub fn as_bs_transform_delta_event_sink() -> BSTEventSink<BSTransformDeltaEvent> {
            offset: Self::BSTRANSFORM_DELTA_EVENT_SINK_OFFSET
        }
    }

    crate::runtime_cast_accessor! {
        pub fn as_character_move_finish_event_sink() -> BSTEventSink<bhkCharacterMoveFinishEvent> {
            offset: Self::CHARACTER_MOVE_FINISH_EVENT_SINK_OFFSET
        }
    }

    crate::runtime_cast_accessor! {
        pub fn as_ipost_animation_channel_update_functor() -> IPostAnimationChannelUpdateFunctor {
            offset: Self::IPOST_ANIMATION_CHANNEL_UPDATE_FUNCTOR_OFFSET
        }
    }

    crate::runtime_cast_mut_accessor! {
        pub fn as_ipost_animation_channel_update_functor_mut() -> IPostAnimationChannelUpdateFunctor {
            offset: Self::IPOST_ANIMATION_CHANNEL_UPDATE_FUNCTOR_OFFSET
        }
    }

    #[inline(always)]
    pub fn get_actor_runtime_data(&self) -> &ACTOR_RUNTIME_DATA {
        crate::runtime_assert_size!(ACTOR_RUNTIME_DATA, se: 0x1D0, ae: 0x1D0, vr: 0x1D0);
        crate::runtime_assert_offset!(ACTOR_RUNTIME_DATA, current_process, se: 0x10, ae: 0x10, vr: 0x10);
        crate::runtime_assert_offset!(ACTOR_RUNTIME_DATA, race, se: 0x110, ae: 0x110, vr: 0x110);
        crate::runtime_assert_offset!(ACTOR_RUNTIME_DATA, bool_flags, se: 0x11C, ae: 0x11C, vr: 0x11C);
        crate::runtime_assert_offset!(ACTOR_RUNTIME_DATA, exclusive_branch, se: 0x140, ae: 0x140, vr: 0x140);
        crate::runtime_assert_offset!(ACTOR_RUNTIME_DATA, biped, se: 0x180, ae: 0x180, vr: 0x180);
        self.actor_runtime_data()
    }

    #[inline(always)]
    pub fn get_actor_runtime_data_mut(&mut self) -> &mut ACTOR_RUNTIME_DATA {
        crate::runtime_assert_size!(ACTOR_RUNTIME_DATA, se: 0x1D0, ae: 0x1D0, vr: 0x1D0);
        self.actor_runtime_data_mut()
    }

    #[inline(always)]
    fn current_process(&self) -> Option<&AIProcess> {
        unsafe { self.get_actor_runtime_data().current_process.as_ref() }
    }

    #[inline(always)]
    pub fn lookup_by_handle(ref_handle: u32) -> NiPointer<Actor> {
        ActorHandle { handle: ref_handle }.get()
    }

    #[inline(always)]
    pub fn lookup_by_handle_into(ref_handle: u32, refr_out: &mut NiPointer<Actor>) -> bool {
        *refr_out = Self::lookup_by_handle(ref_handle);
        !refr_out.get().is_null()
    }

    #[inline(always)]
    pub fn create_ref_handle(&self) -> ActorHandle {
        ActorHandle::from_ptr(self as *const Self as *mut Self)
    }

    #[inline(always)]
    pub fn get_handle(&self) -> ActorHandle {
        ActorHandle::from_ptr(self as *const Self as *mut Self)
    }

    #[inline(always)]
    pub fn get_actor_base(&self) -> *mut TESNPC {
        unsafe { skyrim_cast::<TESBoundObject, TESNPC>(self.base.get_base_object()) }
    }

    #[inline(always)]
    pub fn get_actor_value_max(&self, value: ActorValue) -> f32 {
        self.as_actor_value_owner().get_permanent_actor_value(value)
            + self.get_actor_value_modifier(ActorValueModifier::Temporary, value)
    }

    #[inline(always)]
    pub fn get_commanding_actor(&self) -> NiPointer<Actor> {
        self.current_process()
            .map(AIProcess::get_commanding_actor)
            .unwrap_or_else(ActorHandle::new)
            .get()
    }

    #[inline(always)]
    pub fn get_current_package(&self) -> *mut TESPackage {
        self.current_process()
            .map(AIProcess::get_running_package)
            .unwrap_or(core::ptr::null_mut())
    }

    #[inline(always)]
    pub fn get_current_shout(&self) -> *mut TESShout {
        self.current_process()
            .map(AIProcess::get_current_shout)
            .unwrap_or(core::ptr::null_mut())
    }

    #[inline(always)]
    pub fn get_equipped_entry_data(&self, left_hand: bool) -> *mut InventoryEntryData {
        let Some(process) = self.current_process() else {
            return core::ptr::null_mut();
        };

        let Some(middle_high) = (unsafe { process.middle_high.as_ref() }) else {
            return core::ptr::null_mut();
        };

        if left_hand {
            middle_high.left_hand
        } else {
            middle_high.right_hand
        }
    }

    #[inline(always)]
    pub fn get_equipped_object(&self, left_hand: bool) -> *mut TESForm {
        match self.current_process() {
            Some(process) if left_hand => process.get_equipped_left_hand(),
            Some(process) => process.get_equipped_right_hand(),
            None => core::ptr::null_mut(),
        }
    }

    #[inline(always)]
    pub fn get_equipped_object_in_slot(&self, slot: *const BGSEquipSlot) -> *mut TESForm {
        let Some(process) = self.current_process() else {
            return core::ptr::null_mut();
        };

        for equipped_object in unsafe { process.equipped_forms.as_slice() } {
            if core::ptr::eq(equipped_object.slot as *const BGSEquipSlot, slot) {
                return equipped_object.object;
            }
        }

        core::ptr::null_mut()
    }

    #[inline(always)]
    pub fn get_equipped_weight(&self) -> f32 {
        let runtime_data = self.get_actor_runtime_data();
        if runtime_data.equipped_weight < 0.0 {
            self.calc_equipped_weight()
        } else {
            runtime_data.equipped_weight
        }
    }

    #[inline(always)]
    pub fn get_killer(&self) -> *mut Actor {
        if self.base.is_dead(false) {
            core::ptr::null_mut()
        } else {
            self.get_actor_runtime_data().my_killer.get().get()
        }
    }

    #[inline(always)]
    pub fn get_high_process(&self) -> *mut HighProcessData {
        self.current_process()
            .map(|process| process.high)
            .unwrap_or(core::ptr::null_mut())
    }

    #[inline(always)]
    pub fn get_occupied_furniture(&self) -> ObjectRefHandle {
        self.current_process()
            .map(AIProcess::get_occupied_furniture)
            .unwrap_or_else(ObjectRefHandle::new)
    }

    #[inline(always)]
    pub fn get_process_level(&self) -> PROCESS_TYPE {
        self.current_process()
            .map(AIProcess::get_process_level)
            .unwrap_or(PROCESS_TYPE::None)
    }

    #[inline(always)]
    pub fn get_race(&self) -> *mut TESRace {
        let runtime_data = self.get_actor_runtime_data();
        if !runtime_data.race.is_null() {
            runtime_data.race
        } else {
            let base = self.get_actor_base();
            if base.is_null() {
                core::ptr::null_mut()
            } else {
                unsafe { (*base).race_form.race }
            }
        }
    }

    #[inline(always)]
    pub fn get_middle_high_process(&self) -> *mut MiddleHighProcessData {
        self.current_process()
            .map(|process| process.middle_high)
            .unwrap_or(core::ptr::null_mut())
    }

    #[inline(always)]
    pub fn get_tracked_damage(&self) -> f32 {
        self.current_process()
            .map(AIProcess::get_tracked_damage)
            .unwrap_or(0.0)
    }

    #[inline(always)]
    pub fn get_template_base(&self) -> *mut TESNPC {
        let leveled_creature = self
            .base
            .extra_list
            .get_by_type_typed::<ExtraLeveledCreature>();
        if leveled_creature.is_null() {
            self.get_actor_base()
        } else {
            unsafe { (*leveled_creature).template_base.cast::<TESNPC>() }
        }
    }

    #[inline(always)]
    pub fn get_vendor_faction(&self) -> *mut TESFaction {
        if self.get_actor_runtime_data().vendor_faction.is_null() {
            self.calculate_current_vendor_faction();
        }
        self.get_actor_runtime_data().vendor_faction
    }

    #[inline(always)]
    pub fn get_voice_recovery_time(&self) -> f32 {
        self.current_process()
            .map(AIProcess::get_voice_recovery_time)
            .unwrap_or(0.0)
    }

    #[inline(always)]
    pub fn get_char_controller(&self) -> *mut bhkCharacterController {
        self.current_process()
            .map(AIProcess::get_char_controller)
            .unwrap_or(core::ptr::null_mut())
    }

    #[inline(always)]
    pub fn calculate_los_location(&self, location: ACTOR_LOS_LOCATION) -> NiPoint3 {
        type Func = extern "C" fn(*const Actor, &mut NiPoint3, ACTOR_LOS_LOCATION) -> *mut NiPoint3;
        let func: Func = unsafe { core::mem::transmute(RelocationID::new(36755, 37771).address()) };
        let mut result = NiPoint3::default();
        func(self, &mut result, location);
        result
    }

    #[inline(always)]
    pub fn can_fly_here(&self) -> bool {
        let world_space = self.base.get_worldspace();
        !world_space.is_null() && unsafe { (*world_space).has_max_height_data() }
    }

    #[inline(always)]
    pub fn can_offer_services(&self) -> bool {
        let vendor_faction = self.get_vendor_faction();
        !vendor_faction.is_null() && unsafe { (*vendor_faction).offers_services() }
    }

    #[inline(always)]
    pub fn can_pickpocket(&self) -> bool {
        let race = self.get_actor_runtime_data().race;
        !race.is_null() && unsafe { (*race).allows_pickpocket() } && !self.is_player_teammate()
    }

    #[inline(always)]
    pub fn get_aim_angle(&self) -> f32 {
        let mut aim_active = false;
        let variable = BSFixedString::from_str("bAimActive");
        self.get_graph_variable_bool(&variable, &mut aim_active);
        if !aim_active {
            return self.base.get_angle_x();
        }

        let mut aim_pitch_current = 0.0;
        let variable = BSFixedString::from_str("aimPitchCurrent");
        self.get_graph_variable_float(&variable, &mut aim_pitch_current);
        -aim_pitch_current
    }

    #[inline(always)]
    pub fn get_aim_heading(&self) -> f32 {
        let heading = self.get_heading(false);
        let mut aim_heading_current = 0.0;
        let variable = BSFixedString::from_str("aimHeadingCurrent");
        self.get_graph_variable_float(&variable, &mut aim_heading_current);
        heading - aim_heading_current
    }

    crate::virtual_method! {
        pub const VFUNC_SET_DIALOGUE_WITH_PLAYER: usize = 0x041;
        pub fn set_dialogue_with_player(
            flag: bool,
            force_greet: bool,
            topic: *mut TESTopicInfo
        ) -> bool
    }

    crate::virtual_method! {
        pub const VFUNC_GET_FACE_GEN_ANIMATION_DATA: usize = 0x063;
        pub fn get_face_gen_animation_data(&mut self) -> *mut BSFaceGenAnimationData
    }

    pub fn add_animation_graph_event_sink(
        &self,
        sink: *mut BSTEventSink<BSAnimationGraphEvent>,
    ) -> bool {
        let mut graph_manager = BSTSmartPointer::<BSAnimationGraphManager>::null();
        if !self.get_animation_graph_manager(&mut graph_manager) || graph_manager.get().is_null() {
            return false;
        }

        let graph_manager = graph_manager.get();
        let graphs = unsafe { (*graph_manager).graphs.as_slice() };
        let mut sinked = false;
        for animation_graph in graphs {
            if sinked {
                break;
            }

            let animation_graph = animation_graph.get();
            if animation_graph.is_null() {
                continue;
            }

            let event_source = unsafe { (&*animation_graph).animation_graph_event_source() };
            for &existing_sink in unsafe { event_source.sinks.as_slice() } {
                if existing_sink == sink {
                    sinked = true;
                    break;
                }
            }
        }

        if sinked {
            return false;
        }

        let Some(first_graph) = graphs.first() else {
            return false;
        };
        let first_graph = first_graph.get();
        if first_graph.is_null() {
            return false;
        }

        unsafe { (&mut *first_graph).add_animation_graph_event_sink(sink) };
        true
    }

    pub fn remove_animation_graph_event_sink(
        &self,
        sink: *mut BSTEventSink<BSAnimationGraphEvent>,
    ) {
        let mut graph_manager = BSTSmartPointer::<BSAnimationGraphManager>::null();
        if !self.get_animation_graph_manager(&mut graph_manager) || graph_manager.get().is_null() {
            return;
        }

        let graph_manager = graph_manager.get();
        let graphs = unsafe { (*graph_manager).graphs.as_slice() };
        let mut sinked = true;
        for animation_graph in graphs {
            if !sinked {
                break;
            }

            let animation_graph = animation_graph.get();
            if animation_graph.is_null() {
                continue;
            }

            let event_source = unsafe { (&*animation_graph).animation_graph_event_source() };
            for &existing_sink in unsafe { event_source.sinks.as_slice() } {
                if existing_sink == sink {
                    unsafe { (&mut *animation_graph).remove_animation_graph_event_sink(sink) };
                    sinked = false;
                    break;
                }
            }
        }
    }

    #[inline(always)]
    pub fn allow_bleedout_dialogue(&mut self, can_talk: bool) {
        if can_talk {
            self.get_actor_runtime_data_mut()
                .bool_flags
                .set(BOOL_FLAGS::CanSpeakToEssentialDown);
        } else {
            self.get_actor_runtime_data_mut()
                .bool_flags
                .reset(BOOL_FLAGS::CanSpeakToEssentialDown);
        }
    }

    #[inline(always)]
    pub fn allow_pc_dialogue(&mut self, talk: bool) {
        let mut x_talk = self
            .base
            .extra_list
            .get_by_type_typed::<ExtraCanTalkToPlayer>();
        if x_talk.is_null() {
            x_talk = BSExtraData::create_typed::<ExtraCanTalkToPlayer>(
                ExtraCanTalkToPlayer::VTABLE[0].address(),
            );
            unsafe {
                *x_talk = ExtraCanTalkToPlayer::new(false);
            }
            self.base.extra_list.add(x_talk.cast());
        }

        unsafe {
            (*x_talk).talk = talk;
        }
    }

    #[inline(always)]
    pub fn can_talk_to_player(&self) -> bool {
        let x_talk = self
            .base
            .extra_list
            .get_by_type_typed::<ExtraCanTalkToPlayer>();
        if !x_talk.is_null() {
            unsafe { (*x_talk).talk }
        } else {
            let race = self.get_actor_runtime_data().race;
            !race.is_null() && unsafe { (*race).allows_pc_dialogue() }
        }
    }

    #[inline(always)]
    pub fn clear_arrested(&mut self) {
        let Some(current_process) =
            (unsafe { self.get_actor_runtime_data_mut().current_process.as_mut() })
        else {
            return;
        };

        if current_process.is_arrested() {
            current_process.set_arrested(false);
            self.evaluate_package(false, false);
            if let Some(process_manager) = unsafe { ProcessLists::get_singleton().as_mut() } {
                process_manager.stop_combat_and_alarm_on_actor(self, true);
            }
        }
    }

    #[inline(always)]
    pub fn clear_expression_override(&mut self) {
        let face_gen = self.get_face_gen_animation_data();
        if !face_gen.is_null() {
            unsafe { (&mut *face_gen).clear_expression_override() };
        }
    }

    #[inline(always)]
    pub fn enable_ai(&mut self, enable: bool) {
        if enable {
            self.get_actor_runtime_data_mut()
                .bool_bits
                .set(BOOL_BITS::ProcessMe);
        } else {
            self.get_actor_runtime_data_mut()
                .bool_bits
                .reset(BOOL_BITS::ProcessMe);
        }
    }

    #[inline(always)]
    pub fn get_gold_amount(&mut self, no_init: bool) -> i32 {
        let inventory = self
            .base
            .get_inventory_with(|object: &TESBoundObject| object.is_gold(), no_init);
        let default_object_manager = BGSDefaultObjectManager::get_singleton();
        if default_object_manager.is_null() {
            return 0;
        }

        let gold = unsafe {
            (&mut *default_object_manager)
                .get_object_ptr_as::<TESObjectMISC>(crate::re::DefaultObjectID::kGold)
        };
        if gold.is_null() || unsafe { (*gold).is_null() } {
            return 0;
        }

        inventory
            .get(&(unsafe { *gold } as *mut TESBoundObject))
            .map(|(count, _)| *count)
            .unwrap_or(0)
    }

    #[inline(always)]
    pub fn get_player_controls(&self) -> bool {
        let mut movement_controller = self.get_actor_runtime_data().movement_controller.clone();
        if movement_controller.get().is_null() {
            false
        } else {
            movement_controller.get_controls_driven()
        }
    }

    #[inline(always)]
    pub fn has_keyword(&self, keyword: *const BGSKeyword) -> bool {
        self.base.has_keyword_helper(keyword)
    }

    #[inline(always)]
    pub fn has_keyword_string(&self, form_editor_id: &str) -> bool {
        let base = self.get_actor_base();
        !base.is_null() && unsafe { (*base).has_applicable_keyword_string(form_editor_id) }
    }

    #[inline(always)]
    pub fn is_ai_enabled(&self) -> bool {
        self.get_actor_runtime_data()
            .bool_bits
            .all(BOOL_BITS::ProcessMe)
    }

    #[inline(always)]
    pub fn is_a_mount(&self) -> bool {
        self.get_actor_runtime_data()
            .bool_flags
            .all(BOOL_FLAGS::IsAMount)
    }

    #[inline(always)]
    pub fn is_angry_with_player(&self) -> bool {
        self.get_actor_runtime_data()
            .bool_flags
            .all(BOOL_FLAGS::AngryWithPlayer)
    }

    #[inline(always)]
    pub fn is_animation_driven(&self) -> bool {
        let mut result = false;
        let variable = BSFixedString::from_str("bAnimationDriven");
        self.get_graph_variable_bool(&variable, &mut result) && result
    }

    #[inline(always)]
    pub fn is_allow_rotation(&self) -> bool {
        let mut result = false;
        let variable = BSFixedString::from_str("bAllowRotation");
        self.get_graph_variable_bool(&variable, &mut result) && result
    }

    #[inline(always)]
    pub fn is_being_ridden(&self) -> bool {
        self.is_a_mount() && self.base.extra_list.has_type(ExtraDataType::Interaction)
    }

    #[inline(always)]
    pub fn is_commanded_actor(&self) -> bool {
        self.get_actor_runtime_data()
            .bool_flags
            .all(BOOL_FLAGS::IsCommandedActor)
    }

    #[inline(always)]
    pub fn is_doing_favor(&self) -> bool {
        self.current_process()
            .map(AIProcess::is_in_command_state)
            .unwrap_or(false)
    }

    #[inline(always)]
    pub fn is_essential(&self) -> bool {
        self.get_actor_runtime_data()
            .bool_flags
            .all(BOOL_FLAGS::Essential)
    }

    #[inline(always)]
    pub fn is_guard(&self) -> bool {
        self.get_actor_runtime_data()
            .bool_bits
            .all(BOOL_BITS::Guard)
    }

    #[inline(always)]
    pub fn is_in_kill_move(&self) -> bool {
        self.get_actor_runtime_data()
            .bool_flags
            .all(BOOL_FLAGS::IsInKillMove)
    }

    #[inline(always)]
    pub fn is_on_mount(&self) -> bool {
        !self.is_a_mount() && self.base.extra_list.has_type(ExtraDataType::Interaction)
    }

    #[inline(always)]
    pub fn is_player_teammate(&self) -> bool {
        self.get_actor_runtime_data()
            .bool_bits
            .all(BOOL_BITS::PlayerTeammate)
    }

    #[inline(always)]
    pub fn is_protected(&self) -> bool {
        self.get_actor_runtime_data()
            .bool_flags
            .all(BOOL_FLAGS::Protected)
    }

    #[inline(always)]
    pub fn is_rotation_allowed(&self) -> bool {
        let mut result = false;
        let variable = BSFixedString::from_str("bAllowRotation");
        self.get_graph_variable_bool(&variable, &mut result) && result
    }

    #[inline(always)]
    pub fn is_sneaking(&self) -> bool {
        if !self.as_actor_state().is_sneaking() {
            return false;
        }
        if self.as_actor_state().is_swimming() {
            return false;
        }
        if self.is_on_mount() {
            return false;
        }
        true
    }

    #[inline(always)]
    pub fn is_staggering(&self) -> bool {
        let mut result = false;
        let variable = BSFixedString::from_str("IsStaggering");
        if self.get_graph_variable_bool(&variable, &mut result) && result {
            return true;
        }
        self.as_actor_state().is_staggered()
    }

    #[inline(always)]
    pub fn is_summoned(&self) -> bool {
        self.current_process()
            .map(AIProcess::get_is_summoned_creature)
            .unwrap_or(false)
    }

    #[inline(always)]
    pub fn is_summoned_by_player(&self) -> bool {
        if !self.is_summoned() {
            return false;
        }

        let commanding_actor = self.get_commanding_actor().get();
        !commanding_actor.is_null() && unsafe { (*commanding_actor).base.base.is_player_ref() }
    }

    #[inline(always)]
    pub fn is_trespassing(&self) -> bool {
        self.get_actor_runtime_data()
            .bool_flags
            .all(BOOL_FLAGS::IsTrespassing)
    }

    #[inline(always)]
    pub fn not_show_on_stealth_meter(&self) -> bool {
        self.get_actor_runtime_data()
            .bool_flags
            .any(BOOL_FLAGS::DoNotShowOnStealthMeter)
    }

    #[inline(always)]
    pub fn who_is_casting(&self) -> u8 {
        let mut result = 0u8;
        for &magic_caster in &self.get_actor_runtime_data().magic_casters {
            if magic_caster.is_null() {
                continue;
            }

            let magic_caster = unsafe { &(*magic_caster).base };
            if !magic_caster.current_spell.is_null() {
                result |= 1u8 << (magic_caster.get_casting_source() as u8);
            }
        }
        result
    }

    #[inline(always)]
    pub fn set_player_controls(&mut self, enable: bool) {
        let mut movement_controller = self.get_actor_runtime_data().movement_controller.clone();
        if movement_controller.get().is_null() {
            return;
        }

        self.enable_ai(!enable);
        if enable {
            movement_controller.set_controls_driven();
        } else {
            movement_controller.set_ai_driven();
        }
    }

    #[inline(always)]
    pub fn stop_alarm_on_actor(&mut self) {
        self.end_interrupt_package(false);
        if let Some(current_process) =
            unsafe { self.get_actor_runtime_data_mut().current_process.as_mut() }
        {
            current_process.clear_action_headtrack_target(true);
        }
    }

    #[inline(always)]
    pub fn update_regen_delay(&mut self, actor_value: ActorValue, regen_delay: f32) {
        if let Some(current_process) =
            unsafe { self.get_actor_runtime_data_mut().current_process.as_mut() }
        {
            current_process.update_regen_delay(actor_value, regen_delay);
        }
    }

    crate::relocated_virtual_method! {
        pub const VFUNC_UNK_A2: VariantOffset = VariantOffset::new_se_ae(0x0A2, 0x0A3);
        pub fn unk_a2(&mut self)
    }

    crate::relocated_virtual_method! {
        pub const VFUNC_PLAY_PICK_UP_SOUND: VariantOffset = VariantOffset::new_se_ae(0x0A3, 0x0A4);
        pub fn play_pick_up_sound(&mut self, object: *mut TESBoundObject, pickup: bool, use_item: bool)
    }

    crate::relocated_virtual_method! {
        pub const VFUNC_GET_HEADING: VariantOffset = VariantOffset::new_se_ae(0x0A4, 0x0A5);
        pub fn get_heading(&self, ignore_race_settings: bool) -> f32
    }

    crate::relocated_virtual_method! {
        pub const VFUNC_SET_AVOIDANCE_DISABLED: VariantOffset = VariantOffset::new_se_ae(0x0A5, 0x0A6);
        pub fn set_avoidance_disabled(&mut self, set: bool)
    }

    crate::relocated_virtual_method! {
        pub const VFUNC_DRAW_WEAPON_MAGIC_HANDS: VariantOffset = VariantOffset::new_se_ae(0x0A6, 0x0A8);
        pub fn draw_weapon_magic_hands(&mut self, draw: bool)
    }

    crate::relocated_virtual_method! {
        pub const VFUNC_DETACH_CHAR_CONTROLLER: VariantOffset = VariantOffset::new_se_ae(0x0A7, 0x0A9);
        pub fn detach_char_controller(&mut self)
    }

    crate::relocated_virtual_method! {
        pub const VFUNC_REMOVE_CHAR_CONTROLLER: VariantOffset = VariantOffset::new_se_ae(0x0A8, 0x0AA);
        pub fn remove_char_controller(&mut self)
    }

    crate::relocated_virtual_method! {
        pub const VFUNC_SET_POSITION: VariantOffset = VariantOffset::new_se_ae(0x0A9, 0x0AB);
        pub fn set_position(&mut self, pos: &NiPoint3, update_char_controller: bool)
    }

    crate::relocated_virtual_method! {
        pub const VFUNC_KILL_DYING: VariantOffset = VariantOffset::new_se_ae(0x0AA, 0x0AC);
        pub fn kill_dying(&mut self)
    }

    crate::relocated_virtual_method! {
        pub const VFUNC_RESURRECT: VariantOffset = VariantOffset::new_se_ae(0x0AB, 0x0AD);
        pub fn resurrect(&mut self, reset_inventory: bool, attach_3d: bool)
    }

    crate::relocated_virtual_method! {
        pub const VFUNC_PUT_ACTOR_ON_MOUNT_QUICK: VariantOffset = VariantOffset::new_se_ae(0x0AC, 0x0AE);
        pub fn put_actor_on_mount_quick(&mut self) -> bool
    }

    crate::relocated_virtual_method! {
        pub const VFUNC_UPDATE: VariantOffset = VariantOffset::new_se_ae(0x0AD, 0x0AF);
        pub fn update(&mut self, delta: f32)
    }

    crate::relocated_virtual_method! {
        pub const VFUNC_UPDATE_NO_AI: VariantOffset = VariantOffset::new_se_ae(0x0AE, 0x0B0);
        pub fn update_no_ai(&mut self, delta: f32)
    }

    crate::relocated_virtual_method! {
        pub const VFUNC_UPDATE_CHARACTER_CONTROLLER_SIMULATION_SETTINGS: VariantOffset =
            VariantOffset::new_se_ae(0x0AF, 0x0B1);
        pub fn update_character_controller_simulation_settings(&mut self, controller: &mut bhkCharacterController)
    }

    crate::relocated_virtual_method! {
        pub const VFUNC_POTENTIALLY_FIX_RAGDOLL_STATE: VariantOffset = VariantOffset::new_se_ae(0x0B0, 0x0B2);
        pub fn potentially_fix_ragdoll_state(&mut self)
    }

    crate::relocated_virtual_method! {
        pub const VFUNC_UPDATE_NON_RENDER_SAFE: VariantOffset = VariantOffset::new_se_ae(0x0B1, 0x0B3);
        pub fn update_non_render_safe(&mut self, delta: f32)
    }

    crate::relocated_virtual_method! {
        pub const VFUNC_ON_ITEM_EQUIPPED: VariantOffset = VariantOffset::new_se_ae(0x0B2, 0x0B4);
        pub fn on_item_equipped(&mut self, play_anim: bool)
    }

    crate::relocated_virtual_method! {
        pub const VFUNC_UNK_B3: VariantOffset = VariantOffset::new_se_ae(0x0B3, 0x0B5);
        pub fn unk_b3(&mut self)
    }

    crate::relocated_virtual_method! {
        pub const VFUNC_UNK_B4: VariantOffset = VariantOffset::new_se_ae(0x0B4, 0x0B6);
        pub fn unk_b4(&mut self)
    }

    crate::relocated_virtual_method! {
        pub const VFUNC_SET_CRIME_GOLD_VALUE: VariantOffset = VariantOffset::new_se_ae(0x0B5, 0x0B7);
        pub fn set_crime_gold_value(&mut self, faction: *mut TESFaction, violent: bool, amount: u32)
    }

    crate::relocated_virtual_method! {
        pub const VFUNC_MOD_CRIME_GOLD_VALUE: VariantOffset = VariantOffset::new_se_ae(0x0B6, 0x0B8);
        pub fn mod_crime_gold_value(&mut self, faction: *mut TESFaction, violent: bool, amount: i32)
    }

    crate::relocated_virtual_method! {
        pub const VFUNC_REMOVE_CRIME_GOLD_VALUE: VariantOffset = VariantOffset::new_se_ae(0x0B7, 0x0B9);
        pub fn remove_crime_gold_value(&mut self, faction: *mut TESFaction, violent: bool, amount: i32)
    }

    crate::relocated_virtual_method! {
        pub const VFUNC_GET_CRIME_GOLD_VALUE: VariantOffset = VariantOffset::new_se_ae(0x0B8, 0x0BA);
        pub fn get_crime_gold_value(&self, faction: *const TESFaction) -> u32
    }

    crate::relocated_virtual_method! {
        pub const VFUNC_GO_TO_PRISON: VariantOffset = VariantOffset::new_se_ae(0x0B9, 0x0BB);
        pub fn go_to_prison(&mut self, faction: *mut TESFaction, remove_inventory: bool, real_jail: bool)
    }

    crate::relocated_virtual_method! {
        pub const VFUNC_SERVE_PRISON_TIME: VariantOffset = VariantOffset::new_se_ae(0x0BA, 0x0BC);
        pub fn serve_prison_time(&mut self)
    }

    crate::relocated_virtual_method! {
        pub const VFUNC_PAY_FINE: VariantOffset = VariantOffset::new_se_ae(0x0BB, 0x0BD);
        pub fn pay_fine(&mut self, faction: *mut TESFaction, go_to_jail: bool, remove_stolen_items: bool)
    }

    crate::relocated_virtual_method! {
        pub const VFUNC_GET_CANNIBAL: VariantOffset = VariantOffset::new_se_ae(0x0BC, 0x0BE);
        pub fn get_cannibal(&mut self) -> bool
    }

    crate::relocated_virtual_method! {
        pub const VFUNC_SET_CANNIBAL: VariantOffset = VariantOffset::new_se_ae(0x0BD, 0x0BF);
        pub fn set_cannibal(&mut self, set: bool)
    }

    crate::relocated_virtual_method! {
        pub const VFUNC_GET_VAMPIRE_FEED: VariantOffset = VariantOffset::new_se_ae(0x0BE, 0x0C0);
        pub fn get_vampire_feed(&mut self) -> bool
    }

    crate::relocated_virtual_method! {
        pub const VFUNC_SET_VAMPIRE_FEED: VariantOffset = VariantOffset::new_se_ae(0x0BF, 0x0C1);
        pub fn set_vampire_feed(&mut self, set: bool)
    }

    crate::relocated_virtual_method! {
        pub const VFUNC_INITIATE_VAMPIRE_FEED_PACKAGE: VariantOffset = VariantOffset::new_se_ae(0x0C0, 0x0C2);
        pub fn initiate_vampire_feed_package(&mut self, target: *mut Actor, furniture: *mut TESObjectREFR)
    }

    crate::relocated_virtual_method! {
        pub const VFUNC_INITIATE_CANNIBAL_PACKAGE: VariantOffset = VariantOffset::new_se_ae(0x0C1, 0x0C3);
        pub fn initiate_cannibal_package(&mut self, target: *mut Actor)
    }

    crate::relocated_virtual_method! {
        pub const VFUNC_GET_EYE_VECTOR: VariantOffset = VariantOffset::new_se_ae(0x0C2, 0x0C4);
        pub fn get_eye_vector(&mut self, origin: &mut NiPoint3, direction: &mut NiPoint3, include_camera_offset: bool)
    }

    crate::relocated_virtual_method! {
        pub const VFUNC_SET_REFRACTION: VariantOffset = VariantOffset::new_se_ae(0x0C3, 0x0C5);
        pub fn set_refraction(&mut self, enable: bool, refraction: f32)
    }

    crate::relocated_virtual_method! {
        pub const VFUNC_UNK_C4: VariantOffset = VariantOffset::new_se_ae(0x0C4, 0x0C6);
        pub fn unk_c4(&mut self)
    }

    crate::relocated_virtual_method! {
        pub const VFUNC_UNK_C5: VariantOffset = VariantOffset::new_se_ae(0x0C5, 0x0C7);
        pub fn unk_c5(&mut self)
    }

    crate::relocated_virtual_method! {
        pub const VFUNC_UNK_C6: VariantOffset = VariantOffset::new_se_ae(0x0C6, 0x0C8);
        pub fn unk_c6(&mut self)
    }

    crate::relocated_virtual_method! {
        pub const VFUNC_GET_ACROBATICS: VariantOffset = VariantOffset::new_se_ae(0x0C7, 0x0C9);
        pub fn get_acrobatics(&self) -> f32
    }

    crate::relocated_virtual_method! {
        pub const VFUNC_MOVE: VariantOffset = VariantOffset::new_se_ae(0x0C8, 0x0CA);
        pub fn r#move(&mut self, arg2: f32, position: &NiPoint3) -> *mut bhkCharacterController
    }

    crate::relocated_virtual_method! {
        pub const VFUNC_UNK_C9: VariantOffset = VariantOffset::new_se_ae(0x0C9, 0x0CB);
        pub fn unk_c9(&mut self)
    }

    crate::relocated_virtual_method! {
        pub const VFUNC_ON_ARMOR_ACTOR_VALUE_CHANGED: VariantOffset = VariantOffset::new_se_ae(0x0CA, 0x0CC);
        pub fn on_armor_actor_value_changed(&mut self)
    }

    crate::relocated_virtual_method! {
        pub const VFUNC_DROP_OBJECT: VariantOffset = VariantOffset::new_se_ae(0x0CB, 0x0CD);
        pub fn drop_object(&mut self, object: *const TESBoundObject, extra_list: *mut ExtraDataList, count: i32, drop_loc: *const NiPoint3, rotate: *const NiPoint3) -> ObjectRefHandle
    }

    crate::relocated_virtual_method! {
        pub const VFUNC_PICK_UP_OBJECT: VariantOffset = VariantOffset::new_se_ae(0x0CC, 0x0CE);
        pub fn pick_up_object(&mut self, object: *mut TESObjectREFR, count: i32, arg3: bool, play_sound: bool)
    }

    crate::relocated_virtual_method! {
        pub const VFUNC_ATTACH_ARROW: VariantOffset = VariantOffset::new_se_ae(0x0CD, 0x0CF);
        pub fn attach_arrow(&mut self, biped: &BSTSmartPointer<BipedAnim>)
    }

    crate::relocated_virtual_method! {
        pub const VFUNC_DETACH_ARROW: VariantOffset = VariantOffset::new_se_ae(0x0CE, 0x0D0);
        pub fn detach_arrow(&mut self, biped: &BSTSmartPointer<BipedAnim>)
    }

    crate::relocated_virtual_method! {
        pub const VFUNC_ADD_SHOUT: VariantOffset = VariantOffset::new_se_ae(0x0CF, 0x0D1);
        pub fn add_shout(&mut self, shout: *mut TESShout) -> bool
    }

    crate::relocated_virtual_method! {
        pub const VFUNC_USE_AMMO: VariantOffset = VariantOffset::new_se_ae(0x0D2, 0x0D4);
        pub fn use_ammo(&mut self, shot_count: u32) -> u32
    }

    crate::relocated_virtual_method! {
        pub const VFUNC_CALCULATE_CACHED_OWNER_IS_IN_COMBATANT_FACTION: VariantOffset =
            VariantOffset::new_se_ae(0x0D3, 0x0D5);
        pub fn calculate_cached_owner_is_in_combatant_faction(&self) -> bool
    }

    crate::relocated_virtual_method! {
        pub const VFUNC_UNLOCK_WORD: VariantOffset = VariantOffset::new_se_ae(0x0D0, 0x0D2);
        pub fn unlock_word(&mut self, power: *mut TESWordOfPower)
    }

    crate::relocated_virtual_method! {
        pub const VFUNC_UNK_D1: VariantOffset = VariantOffset::new_se_ae(0x0D1, 0x0D3);
        pub fn unk_d1(&mut self)
    }

    crate::relocated_virtual_method! {
        pub const VFUNC_GET_COMBAT_GROUP: VariantOffset = VariantOffset::new_se_ae(0x0D4, 0x0D6);
        pub fn get_combat_group(&self) -> *mut CombatGroup
    }

    crate::relocated_virtual_method! {
        pub const VFUNC_SET_COMBAT_GROUP: VariantOffset = VariantOffset::new_se_ae(0x0D5, 0x0D7);
        pub fn set_combat_group(&mut self, group: *mut CombatGroup)
    }

    crate::relocated_virtual_method! {
        pub const VFUNC_CHECK_VALID_TARGET: VariantOffset = VariantOffset::new_se_ae(0x0D6, 0x0D8);
        pub fn check_valid_target(&mut self, target: &mut TESObjectREFR) -> bool
    }

    crate::relocated_virtual_method! {
        pub const VFUNC_INITIATE_TRESPASS_PACKAGE: VariantOffset =
            VariantOffset::new_se_ae(0x0D7, 0x0D9);
        pub fn initiate_trespass_package(
            &mut self,
            trespass_package: *mut TrespassPackage
        ) -> bool
    }

    crate::relocated_virtual_method! {
        pub const VFUNC_INITIATE_DIALOGUE: VariantOffset =
            VariantOffset::new_se_ae(0x0D8, 0x0DA);
        pub fn initiate_dialogue(
            &mut self,
            target: *mut Actor,
            loc1: *mut PackageLocation,
            loc2: *mut PackageLocation
        )
    }

    crate::relocated_virtual_method! {
        pub const VFUNC_IS_IN_FACTION: VariantOffset = VariantOffset::new_se_ae(0x0F9, 0x0FB);
        pub fn is_in_faction(&self, faction: *const TESFaction) -> bool
    }

    crate::relocated_virtual_method! {
        pub const VFUNC_FOR_EACH_PERK: VariantOffset = VariantOffset::new_se_ae(0x0FA, 0x0FC);
        pub fn for_each_perk(&self, visitor: &mut PerkEntryVisitor)
    }

    crate::relocated_virtual_method! {
        pub const VFUNC_ADD_PERK: VariantOffset = VariantOffset::new_se_ae(0x0FB, 0x0FD);
        pub fn add_perk(&mut self, perk: *mut BGSPerk, rank: u32)
    }

    crate::relocated_virtual_method! {
        pub const VFUNC_REMOVE_PERK: VariantOffset = VariantOffset::new_se_ae(0x0FC, 0x0FE);
        pub fn remove_perk(&mut self, perk: *mut BGSPerk)
    }

    crate::relocated_virtual_method! {
        pub const VFUNC_APPLY_TEMPORARY_PERK: VariantOffset =
            VariantOffset::new_se_ae(0x0FD, 0x0FF);
        pub fn apply_temporary_perk(&mut self, perk: *mut BGSPerk)
    }

    crate::relocated_virtual_method! {
        pub const VFUNC_REMOVE_TEMPORARY_PERK: VariantOffset =
            VariantOffset::new_se_ae(0x0FE, 0x100);
        pub fn remove_temporary_perk(&mut self, perk: *mut BGSPerk)
    }

    crate::relocated_virtual_method! {
        pub const VFUNC_HAS_PERK_ENTRIES: VariantOffset =
            VariantOffset::new_se_ae(0x0FF, 0x101);
        pub fn has_perk_entries(&self, entry_type: EntryPoint) -> bool
    }

    crate::relocated_virtual_method! {
        pub const VFUNC_FOR_EACH_PERK_ENTRY: VariantOffset =
            VariantOffset::new_se_ae(0x100, 0x102);
        pub fn for_each_perk_entry(
            &self,
            entry_type: EntryPoint,
            visitor: &mut PerkEntryVisitor
        )
    }

    crate::relocated_virtual_method! {
        pub const VFUNC_APPLY_PERKS_FROM_BASE: VariantOffset =
            VariantOffset::new_se_ae(0x101, 0x103);
        pub fn apply_perks_from_base(&mut self)
    }

    crate::relocated_virtual_method! {
        pub const VFUNC_Q_SPEAKING_DONE: VariantOffset = VariantOffset::new_se_ae(0x107, 0x109);
        pub fn q_speaking_done(&self) -> bool
    }

    crate::relocated_virtual_method! {
        pub const VFUNC_SET_SPEAKING_DONE: VariantOffset =
            VariantOffset::new_se_ae(0x108, 0x10A);
        pub fn set_speaking_done(&mut self, set: bool)
    }

    crate::relocated_virtual_method! {
        pub const VFUNC_CREATE_MOVEMENT_CONTROLLER: VariantOffset =
            VariantOffset::new_se_ae(0x109, 0x10B);
        pub fn create_movement_controller(&mut self)
    }

    crate::relocated_virtual_method! {
        pub const VFUNC_COMPUTE_MOTION_FEEDBACK_SPEED_AND_DIRECTION: VariantOffset =
            VariantOffset::new_se_ae(0x11D, 0x11F);
        pub fn compute_motion_feedback_speed_and_direction(
            &mut self,
            data: &ActorMotionFeedbackData,
            delta: f32,
            output: &mut ActorMotionFeedbackOutput
        ) -> bool
    }

    crate::relocated_virtual_method! {
        pub const VFUNC_UPDATE_FEEDBACK_GRAPH_SPEED_AND_DIRECTION: VariantOffset =
            VariantOffset::new_se_ae(0x11E, 0x120);
        pub fn update_feedback_graph_speed_and_direction(
            &mut self,
            output: &ActorMotionFeedbackOutput
        ) -> bool
    }

    crate::relocation_func! {
        pub fn add_cast_power(&mut self, power: *mut SpellItem) => RelocationID::new(37787, 38736)
    }

    crate::relocation_func! {
        pub fn add_death_items(&mut self) => RelocationID::new(36218, 37198)
    }

    crate::relocation_func! {
        pub fn add_spell(&mut self, spell: *mut SpellItem) -> bool => RelocationID::new(37771, 38716)
    }

    crate::relocation_func! {
        pub fn add_to_faction(&mut self, faction: *mut TESFaction, rank: i8)
            => RelocationID::new(36678, 37686)
    }

    crate::relocation_func! {
        pub fn can_attack_actor(&self, actor: *mut Actor) -> bool => RelocationID::new(36532, 37532)
    }

    crate::relocation_func! {
        pub fn can_fly(&self) -> bool => RelocationID::new(36238, 0)
    }

    crate::relocation_func! {
        pub fn can_navigate_to_position(
            &self,
            pos: &NiPoint3,
            new_pos: &NiPoint3,
            speed: f32,
            distance: f32
        ) -> bool => RelocationID::new(46050, 47314)
    }

    crate::relocation_func! {
        pub fn can_use_idle(&self, idle: *mut TESIdleForm) -> bool => RelocationID::new(36224, 37205)
    }

    crate::relocation_func! {
        pub fn cast_permanent_magic(
            &mut self,
            worn_item_enchantments: bool,
            base_spells: bool,
            race_spells: bool,
            every_actor_ability: bool
        ) => RelocationID::new(37804, 38753)
    }

    crate::relocation_func! {
        pub fn clear_death_state(&mut self) => RelocationID::new(36605, 37613)
    }

    crate::relocation_func! {
        pub fn decapitate(&mut self) -> bool => RelocationID::new(36631, 37639)
    }

    crate::relocation_func! {
        pub fn deselect_spell(&mut self, spell: *mut SpellItem) => RelocationID::new(37820, 38769)
    }

    crate::relocation_func! {
        pub fn dispel_worn_item_enchantments(&mut self) => RelocationID::new(33828, 34620)
    }

    crate::relocation_func! {
        pub fn do_reset_3d(&mut self, update_weight: bool) => RelocationID::new(39181, 40255)
    }

    crate::relocation_func! {
        pub fn do_damage(
            &mut self,
            health_damage: f32,
            source: *mut Actor,
            dont_adjust_difficulty: bool
        ) -> bool => RelocationID::new(36345, 37335)
    }

    crate::relocation_func! {
        pub fn end_interrupt_package(&mut self, skip_dialogue: bool)
            => RelocationID::new(36475, 37474)
    }

    crate::relocation_func! {
        pub fn evaluate_package(&mut self, immediate: bool, reset_ai: bool)
            => RelocationID::new(36407, 37401)
    }

    crate::relocation_func! {
        pub fn fights_in_water(&self) -> bool => RelocationID::new(36236, 0)
    }

    crate::relocation_func! {
        pub fn get_actor_value_modifier(
            &self,
            modifier: ACTOR_VALUE_MODIFIER,
            value: ActorValue
        ) -> f32 => RelocationID::new(37524, 38469)
    }

    crate::relocation_func! {
        pub fn get_attack_chance(&self, target: *mut Actor, attack_data: *mut BGSAttackData) -> f32
            => RelocationID::new(49748, 50675)
    }

    crate::relocation_func! {
        pub fn get_attack_reach(&self) -> f32 => RelocationID::new(37588, 38538)
    }

    crate::relocation_func! {
        pub fn get_bound_radius(&self) -> f32 => RelocationID::new(36444, 37439)
    }

    crate::relocation_func! {
        pub fn get_collision_filter_info(&self, out_collision_filter_info: &mut CFilter)
            => RelocationID::new(36559, 37560)
    }

    crate::relocation_func! {
        pub fn get_faction_rank(&mut self, faction: *mut TESFaction, is_player: bool) -> i32
            => RelocationID::new(36668, 37676)
    }

    crate::relocation_func! {
        pub fn get_faction_reaction(&self, other: *mut Actor) -> FIGHT_REACTION
            => RelocationID::new(36658, 37666)
    }

    crate::relocation_func! {
        pub fn get_level(&self) -> u16 => RelocationID::new(36344, 37334)
    }

    crate::relocation_func! {
        pub fn get_reach(&self) -> f32 => RelocationID::new(37588, 38538)
    }

    crate::relocation_func! {
        pub fn get_mount(&mut self, out_mount: &mut NiPointer<Actor>) -> bool
            => RelocationID::new(37757, 38702)
    }

    crate::relocation_func! {
        pub fn get_mounted_by(&mut self, out_rider: &mut NiPointer<Actor>) -> bool
            => RelocationID::new(37758, 38703)
    }

    crate::relocation_func! {
        pub fn get_move_direction_relative_to_facing(&mut self) -> f64
            => RelocationID::new(36935, 37960)
    }

    crate::relocation_func! {
        fn calculate_current_vendor_faction(&self) => RelocationID::new(36392, 37383)
    }

    crate::relocation_func! {
        fn calc_equipped_weight(&self) -> f32 => RelocationID::new(37016, 38044)
    }

    crate::relocation_func! {
        pub fn get_submerged_level(&self, z_pos: f32, cell: *mut TESObjectCELL) -> f32
            => RelocationID::new(36452, 37448)
    }

    crate::relocation_func! {
        pub fn get_total_carry_weight(&self) -> f32 => RelocationID::new(36456, 37452)
    }

    crate::relocation_func! {
        pub fn process_vats_attack(
            &mut self,
            caster: *mut MagicCaster,
            has_target_anim: bool,
            target: *mut TESObjectREFR,
            left_hand: bool
        ) => RelocationID::new(40230, 41233)
    }

    crate::relocation_func! {
        pub fn refresh_equipped_actor_value_charge(
            &mut self,
            object: *const TESForm,
            extra_list: *const ExtraDataList,
            is_left: bool
        ) => RelocationID::new(38752, 37803)
    }

    #[inline(always)]
    pub fn get_warmth_rating(&self) -> f32 {
        crate::runtime::require_non_vr("get_warmth_rating");
        type Func = extern "C" fn(*const Actor) -> f32;
        let func: Func = unsafe { core::mem::transmute(RelocationID::new(25834, 26394).address()) };
        func(self)
    }

    crate::relocation_func! {
        pub fn has_perk(&self, perk: *mut BGSPerk) -> bool => RelocationID::new(36690, 37698)
    }

    crate::relocation_func! {
        pub fn has_shout(&self, shout: *mut TESShout) -> bool => RelocationID::new(37829, 38783)
    }

    crate::relocation_func! {
        pub fn has_spell(&self, spell: *mut SpellItem) -> bool => RelocationID::new(37828, 38782)
    }

    crate::relocation_func! {
        pub fn remove_from_faction(&mut self, faction: *mut TESFaction) => RelocationID::new(36680, 37688)
    }

    crate::relocation_func! {
        pub fn remove_spell(&mut self, spell: *mut SpellItem) -> bool => RelocationID::new(37772, 38717)
    }

    crate::relocation_func! {
        pub fn request_detection_level(&mut self, target: *mut Actor, priority: DETECTION_PRIORITY) -> i32
            => RelocationID::new(36748, 37764)
    }

    crate::relocation_func! {
        pub fn request_los(&mut self, target: *mut Actor, view_cone: f32) -> i32
            => RelocationID::new(36752, 37768)
    }

    #[inline(always)]
    pub fn update_nav_pos(
        &self,
        pos: &NiPoint3,
        new_pos: &NiPoint3,
        speed: f32,
        distance: f32,
    ) -> bool {
        self.can_navigate_to_position(pos, new_pos, speed, distance)
    }

    crate::relocation_func! {
        pub fn visit_spells(&mut self, visitor: &mut ActorForEachSpellVisitor)
            => RelocationID::new(37827, 38781)
    }
}

pub trait ActorExt {
    fn as_magic_target(&self) -> &MagicTarget;
    fn as_magic_target_mut(&mut self) -> &mut MagicTarget;
    fn as_actor_value_owner(&self) -> &ActorValueOwner;
    fn as_actor_value_owner_mut(&mut self) -> &mut ActorValueOwner;
    fn as_actor_state(&self) -> &ActorState;
    fn as_actor_state_mut(&mut self) -> &mut ActorState;
    fn as_bs_transform_delta_event_sink(&self) -> &BSTEventSink<BSTransformDeltaEvent>;
    fn as_character_move_finish_event_sink(&self) -> &BSTEventSink<bhkCharacterMoveFinishEvent>;
    fn as_ipost_animation_channel_update_functor(&self) -> &IPostAnimationChannelUpdateFunctor;
    fn as_ipost_animation_channel_update_functor_mut(
        &mut self,
    ) -> &mut IPostAnimationChannelUpdateFunctor;
    fn add_animation_graph_event_sink(
        &self,
        sink: *mut BSTEventSink<BSAnimationGraphEvent>,
    ) -> bool;
    fn remove_animation_graph_event_sink(&self, sink: *mut BSTEventSink<BSAnimationGraphEvent>);
    fn get_actor_runtime_data(&self) -> &ACTOR_RUNTIME_DATA;
    fn get_actor_runtime_data_mut(&mut self) -> &mut ACTOR_RUNTIME_DATA;
    fn create_ref_handle(&self) -> ActorHandle;
    fn get_handle(&self) -> ActorHandle;
    fn get_actor_base(&self) -> *mut TESNPC;
    fn get_actor_value_max(&self, value: ActorValue) -> f32;
    fn get_actor_value_modifier(&self, modifier: ACTOR_VALUE_MODIFIER, value: ActorValue) -> f32;
    fn get_aim_angle(&self) -> f32;
    fn get_aim_heading(&self) -> f32;
    fn get_commanding_actor(&self) -> NiPointer<Actor>;
    fn get_current_package(&self) -> *mut TESPackage;
    fn get_current_shout(&self) -> *mut TESShout;
    fn get_equipped_entry_data(&self, left_hand: bool) -> *mut InventoryEntryData;
    fn get_equipped_object(&self, left_hand: bool) -> *mut TESForm;
    fn get_equipped_object_in_slot(&self, slot: *const BGSEquipSlot) -> *mut TESForm;
    fn get_equipped_weight(&self) -> f32;
    fn get_killer(&self) -> *mut Actor;
    fn get_high_process(&self) -> *mut HighProcessData;
    fn get_occupied_furniture(&self) -> ObjectRefHandle;
    fn get_process_level(&self) -> PROCESS_TYPE;
    fn get_race(&self) -> *mut TESRace;
    fn get_middle_high_process(&self) -> *mut MiddleHighProcessData;
    fn get_tracked_damage(&self) -> f32;
    fn get_vendor_faction(&self) -> *mut TESFaction;
    fn get_voice_recovery_time(&self) -> f32;
    fn get_char_controller(&self) -> *mut bhkCharacterController;
    fn calculate_los_location(&self, location: ACTOR_LOS_LOCATION) -> NiPoint3;
    fn can_fly_here(&self) -> bool;
    fn can_offer_services(&self) -> bool;
    fn can_pickpocket(&self) -> bool;
    fn clear_arrested(&mut self);
    fn enable_ai(&mut self, enable: bool);
    fn get_gold_amount(&mut self, no_init: bool) -> i32;
    fn get_player_controls(&self) -> bool;
    fn get_face_gen_animation_data(&mut self) -> *mut BSFaceGenAnimationData;
    fn get_template_base(&self) -> *mut TESNPC;
    fn has_keyword(&self, keyword: *const BGSKeyword) -> bool;
    fn has_keyword_string(&self, form_editor_id: &str) -> bool;
    fn is_in_faction(&self, faction: *const TESFaction) -> bool;
    fn can_talk_to_player(&self) -> bool;
    fn is_ai_enabled(&self) -> bool;
    fn is_a_mount(&self) -> bool;
    fn is_angry_with_player(&self) -> bool;
    fn is_animation_driven(&self) -> bool;
    fn is_allow_rotation(&self) -> bool;
    fn is_being_ridden(&self) -> bool;
    fn is_commanded_actor(&self) -> bool;
    fn is_doing_favor(&self) -> bool;
    fn is_essential(&self) -> bool;
    fn is_guard(&self) -> bool;
    fn is_in_kill_move(&self) -> bool;
    fn is_on_mount(&self) -> bool;
    fn is_player_teammate(&self) -> bool;
    fn is_protected(&self) -> bool;
    fn is_rotation_allowed(&self) -> bool;
    fn is_sneaking(&self) -> bool;
    fn is_staggering(&self) -> bool;
    fn is_summoned(&self) -> bool;
    fn is_summoned_by_player(&self) -> bool;
    fn is_trespassing(&self) -> bool;
    fn not_show_on_stealth_meter(&self) -> bool;
    fn who_is_casting(&self) -> u8;
    fn allow_bleedout_dialogue(&mut self, can_talk: bool);
    fn allow_pc_dialogue(&mut self, talk: bool);
    fn clear_expression_override(&mut self);
    fn set_player_controls(&mut self, enable: bool);
    fn stop_alarm_on_actor(&mut self);
    fn update_regen_delay(&mut self, actor_value: ActorValue, regen_delay: f32);
    fn add_spell(&mut self, spell: *mut SpellItem) -> bool;
    fn add_to_faction(&mut self, faction: *mut TESFaction, rank: i8);
    fn can_attack_actor(&self, actor: *mut Actor) -> bool;
    fn can_fly(&self) -> bool;
    fn can_use_idle(&self, idle: *mut TESIdleForm) -> bool;
    fn clear_death_state(&mut self);
    fn evaluate_package(&mut self, immediate: bool, reset_ai: bool);
    fn get_level(&self) -> u16;
    fn get_mount(&mut self, out_mount: &mut NiPointer<Actor>) -> bool;
    fn get_mounted_by(&mut self, out_rider: &mut NiPointer<Actor>) -> bool;
    fn get_total_carry_weight(&self) -> f32;
    fn get_warmth_rating(&self) -> f32;
    fn has_perk(&self, perk: *mut BGSPerk) -> bool;
    fn has_perk_entries(&self, entry_type: EntryPoint) -> bool;
    fn has_shout(&self, shout: *mut TESShout) -> bool;
    fn has_spell(&self, spell: *mut SpellItem) -> bool;
    fn process_vats_attack(
        &mut self,
        caster: *mut MagicCaster,
        has_target_anim: bool,
        target: *mut TESObjectREFR,
        left_hand: bool,
    );
    fn refresh_equipped_actor_value_charge(
        &mut self,
        object: *const TESForm,
        extra_list: *const ExtraDataList,
        is_left: bool,
    );
    fn remove_from_faction(&mut self, faction: *mut TESFaction);
    fn add_perk(&mut self, perk: *mut BGSPerk, rank: u32);
    fn remove_perk(&mut self, perk: *mut BGSPerk);
    fn apply_temporary_perk(&mut self, perk: *mut BGSPerk);
    fn remove_temporary_perk(&mut self, perk: *mut BGSPerk);
    fn remove_spell(&mut self, spell: *mut SpellItem) -> bool;
    fn request_detection_level(&mut self, target: *mut Actor, priority: DETECTION_PRIORITY) -> i32;
    fn request_los(&mut self, target: *mut Actor, view_cone: f32) -> i32;
    fn for_each_perk(&self, visitor: &mut PerkEntryVisitor);
    fn for_each_perk_entry(&self, entry_type: EntryPoint, visitor: &mut PerkEntryVisitor);
    fn apply_perks_from_base(&mut self);
    fn q_speaking_done(&self) -> bool;
    fn set_speaking_done(&mut self, set: bool);
    fn create_movement_controller(&mut self);
    fn update_nav_pos(&self, pos: &NiPoint3, new_pos: &NiPoint3, speed: f32, distance: f32)
    -> bool;
    fn visit_spells(&mut self, visitor: &mut ActorForEachSpellVisitor);
}

impl<T: AsRef<Actor> + AsMut<Actor>> ActorExt for T {
    #[inline(always)]
    fn as_magic_target(&self) -> &MagicTarget {
        Actor::as_magic_target(self.as_ref())
    }

    #[inline(always)]
    fn as_magic_target_mut(&mut self) -> &mut MagicTarget {
        Actor::as_magic_target_mut(self.as_mut())
    }

    #[inline(always)]
    fn as_actor_value_owner(&self) -> &ActorValueOwner {
        Actor::as_actor_value_owner(self.as_ref())
    }

    #[inline(always)]
    fn as_actor_value_owner_mut(&mut self) -> &mut ActorValueOwner {
        Actor::as_actor_value_owner_mut(self.as_mut())
    }

    #[inline(always)]
    fn as_actor_state(&self) -> &ActorState {
        Actor::as_actor_state(self.as_ref())
    }

    #[inline(always)]
    fn as_actor_state_mut(&mut self) -> &mut ActorState {
        Actor::as_actor_state_mut(self.as_mut())
    }

    #[inline(always)]
    fn as_bs_transform_delta_event_sink(&self) -> &BSTEventSink<BSTransformDeltaEvent> {
        Actor::as_bs_transform_delta_event_sink(self.as_ref())
    }

    #[inline(always)]
    fn as_character_move_finish_event_sink(&self) -> &BSTEventSink<bhkCharacterMoveFinishEvent> {
        Actor::as_character_move_finish_event_sink(self.as_ref())
    }

    #[inline(always)]
    fn as_ipost_animation_channel_update_functor(&self) -> &IPostAnimationChannelUpdateFunctor {
        Actor::as_ipost_animation_channel_update_functor(self.as_ref())
    }

    #[inline(always)]
    fn as_ipost_animation_channel_update_functor_mut(
        &mut self,
    ) -> &mut IPostAnimationChannelUpdateFunctor {
        Actor::as_ipost_animation_channel_update_functor_mut(self.as_mut())
    }

    #[inline(always)]
    fn add_animation_graph_event_sink(
        &self,
        sink: *mut BSTEventSink<BSAnimationGraphEvent>,
    ) -> bool {
        Actor::add_animation_graph_event_sink(self.as_ref(), sink)
    }

    #[inline(always)]
    fn remove_animation_graph_event_sink(&self, sink: *mut BSTEventSink<BSAnimationGraphEvent>) {
        Actor::remove_animation_graph_event_sink(self.as_ref(), sink)
    }

    #[inline(always)]
    fn get_actor_runtime_data(&self) -> &ACTOR_RUNTIME_DATA {
        Actor::get_actor_runtime_data(self.as_ref())
    }

    #[inline(always)]
    fn get_actor_runtime_data_mut(&mut self) -> &mut ACTOR_RUNTIME_DATA {
        Actor::get_actor_runtime_data_mut(self.as_mut())
    }

    #[inline(always)]
    fn create_ref_handle(&self) -> ActorHandle {
        Actor::create_ref_handle(self.as_ref())
    }

    #[inline(always)]
    fn get_handle(&self) -> ActorHandle {
        Actor::get_handle(self.as_ref())
    }

    #[inline(always)]
    fn get_actor_base(&self) -> *mut TESNPC {
        Actor::get_actor_base(self.as_ref())
    }

    #[inline(always)]
    fn get_actor_value_max(&self, value: ActorValue) -> f32 {
        Actor::get_actor_value_max(self.as_ref(), value)
    }

    #[inline(always)]
    fn get_actor_value_modifier(&self, modifier: ACTOR_VALUE_MODIFIER, value: ActorValue) -> f32 {
        Actor::get_actor_value_modifier(self.as_ref(), modifier, value)
    }

    #[inline(always)]
    fn get_aim_angle(&self) -> f32 {
        Actor::get_aim_angle(self.as_ref())
    }

    #[inline(always)]
    fn get_aim_heading(&self) -> f32 {
        Actor::get_aim_heading(self.as_ref())
    }

    #[inline(always)]
    fn get_commanding_actor(&self) -> NiPointer<Actor> {
        Actor::get_commanding_actor(self.as_ref())
    }

    #[inline(always)]
    fn get_current_package(&self) -> *mut TESPackage {
        Actor::get_current_package(self.as_ref())
    }

    #[inline(always)]
    fn get_current_shout(&self) -> *mut TESShout {
        Actor::get_current_shout(self.as_ref())
    }

    #[inline(always)]
    fn get_equipped_entry_data(&self, left_hand: bool) -> *mut InventoryEntryData {
        Actor::get_equipped_entry_data(self.as_ref(), left_hand)
    }

    #[inline(always)]
    fn get_equipped_object(&self, left_hand: bool) -> *mut TESForm {
        Actor::get_equipped_object(self.as_ref(), left_hand)
    }

    #[inline(always)]
    fn get_equipped_object_in_slot(&self, slot: *const BGSEquipSlot) -> *mut TESForm {
        Actor::get_equipped_object_in_slot(self.as_ref(), slot)
    }

    #[inline(always)]
    fn get_equipped_weight(&self) -> f32 {
        Actor::get_equipped_weight(self.as_ref())
    }

    #[inline(always)]
    fn get_killer(&self) -> *mut Actor {
        Actor::get_killer(self.as_ref())
    }

    #[inline(always)]
    fn get_high_process(&self) -> *mut HighProcessData {
        Actor::get_high_process(self.as_ref())
    }

    #[inline(always)]
    fn get_occupied_furniture(&self) -> ObjectRefHandle {
        Actor::get_occupied_furniture(self.as_ref())
    }

    #[inline(always)]
    fn get_process_level(&self) -> PROCESS_TYPE {
        Actor::get_process_level(self.as_ref())
    }

    #[inline(always)]
    fn get_race(&self) -> *mut TESRace {
        Actor::get_race(self.as_ref())
    }

    #[inline(always)]
    fn get_middle_high_process(&self) -> *mut MiddleHighProcessData {
        Actor::get_middle_high_process(self.as_ref())
    }

    #[inline(always)]
    fn get_tracked_damage(&self) -> f32 {
        Actor::get_tracked_damage(self.as_ref())
    }

    #[inline(always)]
    fn get_vendor_faction(&self) -> *mut TESFaction {
        Actor::get_vendor_faction(self.as_ref())
    }

    #[inline(always)]
    fn get_voice_recovery_time(&self) -> f32 {
        Actor::get_voice_recovery_time(self.as_ref())
    }

    #[inline(always)]
    fn get_char_controller(&self) -> *mut bhkCharacterController {
        Actor::get_char_controller(self.as_ref())
    }

    #[inline(always)]
    fn calculate_los_location(&self, location: ACTOR_LOS_LOCATION) -> NiPoint3 {
        Actor::calculate_los_location(self.as_ref(), location)
    }

    #[inline(always)]
    fn can_fly_here(&self) -> bool {
        Actor::can_fly_here(self.as_ref())
    }

    #[inline(always)]
    fn can_offer_services(&self) -> bool {
        Actor::can_offer_services(self.as_ref())
    }

    #[inline(always)]
    fn can_pickpocket(&self) -> bool {
        Actor::can_pickpocket(self.as_ref())
    }

    #[inline(always)]
    fn clear_arrested(&mut self) {
        Actor::clear_arrested(self.as_mut())
    }

    #[inline(always)]
    fn enable_ai(&mut self, enable: bool) {
        Actor::enable_ai(self.as_mut(), enable)
    }

    #[inline(always)]
    fn get_gold_amount(&mut self, no_init: bool) -> i32 {
        Actor::get_gold_amount(self.as_mut(), no_init)
    }

    #[inline(always)]
    fn get_player_controls(&self) -> bool {
        Actor::get_player_controls(self.as_ref())
    }

    #[inline(always)]
    fn get_face_gen_animation_data(&mut self) -> *mut BSFaceGenAnimationData {
        Actor::get_face_gen_animation_data(self.as_mut())
    }

    #[inline(always)]
    fn get_template_base(&self) -> *mut TESNPC {
        Actor::get_template_base(self.as_ref())
    }

    #[inline(always)]
    fn has_keyword(&self, keyword: *const BGSKeyword) -> bool {
        Actor::has_keyword(self.as_ref(), keyword)
    }

    #[inline(always)]
    fn has_keyword_string(&self, form_editor_id: &str) -> bool {
        Actor::has_keyword_string(self.as_ref(), form_editor_id)
    }

    #[inline(always)]
    fn is_in_faction(&self, faction: *const TESFaction) -> bool {
        Actor::is_in_faction(self.as_ref(), faction)
    }

    #[inline(always)]
    fn can_talk_to_player(&self) -> bool {
        Actor::can_talk_to_player(self.as_ref())
    }

    #[inline(always)]
    fn is_ai_enabled(&self) -> bool {
        Actor::is_ai_enabled(self.as_ref())
    }

    #[inline(always)]
    fn is_a_mount(&self) -> bool {
        Actor::is_a_mount(self.as_ref())
    }

    #[inline(always)]
    fn is_angry_with_player(&self) -> bool {
        Actor::is_angry_with_player(self.as_ref())
    }

    #[inline(always)]
    fn is_animation_driven(&self) -> bool {
        Actor::is_animation_driven(self.as_ref())
    }

    #[inline(always)]
    fn is_allow_rotation(&self) -> bool {
        Actor::is_allow_rotation(self.as_ref())
    }

    #[inline(always)]
    fn is_being_ridden(&self) -> bool {
        Actor::is_being_ridden(self.as_ref())
    }

    #[inline(always)]
    fn is_commanded_actor(&self) -> bool {
        Actor::is_commanded_actor(self.as_ref())
    }

    #[inline(always)]
    fn is_doing_favor(&self) -> bool {
        Actor::is_doing_favor(self.as_ref())
    }

    #[inline(always)]
    fn is_essential(&self) -> bool {
        Actor::is_essential(self.as_ref())
    }

    #[inline(always)]
    fn is_guard(&self) -> bool {
        Actor::is_guard(self.as_ref())
    }

    #[inline(always)]
    fn is_in_kill_move(&self) -> bool {
        Actor::is_in_kill_move(self.as_ref())
    }

    #[inline(always)]
    fn is_on_mount(&self) -> bool {
        Actor::is_on_mount(self.as_ref())
    }

    #[inline(always)]
    fn is_player_teammate(&self) -> bool {
        Actor::is_player_teammate(self.as_ref())
    }

    #[inline(always)]
    fn is_protected(&self) -> bool {
        Actor::is_protected(self.as_ref())
    }

    #[inline(always)]
    fn is_rotation_allowed(&self) -> bool {
        Actor::is_rotation_allowed(self.as_ref())
    }

    #[inline(always)]
    fn is_sneaking(&self) -> bool {
        Actor::is_sneaking(self.as_ref())
    }

    #[inline(always)]
    fn is_staggering(&self) -> bool {
        Actor::is_staggering(self.as_ref())
    }

    #[inline(always)]
    fn is_summoned(&self) -> bool {
        Actor::is_summoned(self.as_ref())
    }

    #[inline(always)]
    fn is_summoned_by_player(&self) -> bool {
        Actor::is_summoned_by_player(self.as_ref())
    }

    #[inline(always)]
    fn is_trespassing(&self) -> bool {
        Actor::is_trespassing(self.as_ref())
    }

    #[inline(always)]
    fn not_show_on_stealth_meter(&self) -> bool {
        Actor::not_show_on_stealth_meter(self.as_ref())
    }

    #[inline(always)]
    fn who_is_casting(&self) -> u8 {
        Actor::who_is_casting(self.as_ref())
    }

    #[inline(always)]
    fn allow_bleedout_dialogue(&mut self, can_talk: bool) {
        Actor::allow_bleedout_dialogue(self.as_mut(), can_talk)
    }

    #[inline(always)]
    fn allow_pc_dialogue(&mut self, talk: bool) {
        Actor::allow_pc_dialogue(self.as_mut(), talk)
    }

    #[inline(always)]
    fn clear_expression_override(&mut self) {
        Actor::clear_expression_override(self.as_mut())
    }

    #[inline(always)]
    fn set_player_controls(&mut self, enable: bool) {
        Actor::set_player_controls(self.as_mut(), enable)
    }

    #[inline(always)]
    fn stop_alarm_on_actor(&mut self) {
        Actor::stop_alarm_on_actor(self.as_mut())
    }

    #[inline(always)]
    fn update_regen_delay(&mut self, actor_value: ActorValue, regen_delay: f32) {
        Actor::update_regen_delay(self.as_mut(), actor_value, regen_delay)
    }

    #[inline(always)]
    fn add_spell(&mut self, spell: *mut SpellItem) -> bool {
        Actor::add_spell(self.as_mut(), spell)
    }

    #[inline(always)]
    fn add_to_faction(&mut self, faction: *mut TESFaction, rank: i8) {
        Actor::add_to_faction(self.as_mut(), faction, rank)
    }

    #[inline(always)]
    fn can_attack_actor(&self, actor: *mut Actor) -> bool {
        Actor::can_attack_actor(self.as_ref(), actor)
    }

    #[inline(always)]
    fn can_fly(&self) -> bool {
        Actor::can_fly(self.as_ref())
    }

    #[inline(always)]
    fn can_use_idle(&self, idle: *mut TESIdleForm) -> bool {
        Actor::can_use_idle(self.as_ref(), idle)
    }

    #[inline(always)]
    fn clear_death_state(&mut self) {
        Actor::clear_death_state(self.as_mut())
    }

    #[inline(always)]
    fn evaluate_package(&mut self, immediate: bool, reset_ai: bool) {
        Actor::evaluate_package(self.as_mut(), immediate, reset_ai)
    }

    #[inline(always)]
    fn get_level(&self) -> u16 {
        Actor::get_level(self.as_ref())
    }

    #[inline(always)]
    fn get_mount(&mut self, out_mount: &mut NiPointer<Actor>) -> bool {
        Actor::get_mount(self.as_mut(), out_mount)
    }

    #[inline(always)]
    fn get_mounted_by(&mut self, out_rider: &mut NiPointer<Actor>) -> bool {
        Actor::get_mounted_by(self.as_mut(), out_rider)
    }

    #[inline(always)]
    fn get_total_carry_weight(&self) -> f32 {
        Actor::get_total_carry_weight(self.as_ref())
    }

    #[inline(always)]
    fn get_warmth_rating(&self) -> f32 {
        Actor::get_warmth_rating(self.as_ref())
    }

    #[inline(always)]
    fn has_perk(&self, perk: *mut BGSPerk) -> bool {
        Actor::has_perk(self.as_ref(), perk)
    }

    #[inline(always)]
    fn has_perk_entries(&self, entry_type: EntryPoint) -> bool {
        Actor::has_perk_entries(self.as_ref(), entry_type)
    }

    #[inline(always)]
    fn has_shout(&self, shout: *mut TESShout) -> bool {
        Actor::has_shout(self.as_ref(), shout)
    }

    #[inline(always)]
    fn has_spell(&self, spell: *mut SpellItem) -> bool {
        Actor::has_spell(self.as_ref(), spell)
    }

    #[inline(always)]
    fn process_vats_attack(
        &mut self,
        caster: *mut MagicCaster,
        has_target_anim: bool,
        target: *mut TESObjectREFR,
        left_hand: bool,
    ) {
        Actor::process_vats_attack(self.as_mut(), caster, has_target_anim, target, left_hand)
    }

    #[inline(always)]
    fn refresh_equipped_actor_value_charge(
        &mut self,
        object: *const TESForm,
        extra_list: *const ExtraDataList,
        is_left: bool,
    ) {
        Actor::refresh_equipped_actor_value_charge(self.as_mut(), object, extra_list, is_left)
    }

    #[inline(always)]
    fn remove_from_faction(&mut self, faction: *mut TESFaction) {
        Actor::remove_from_faction(self.as_mut(), faction)
    }

    #[inline(always)]
    fn add_perk(&mut self, perk: *mut BGSPerk, rank: u32) {
        Actor::add_perk(self.as_mut(), perk, rank)
    }

    #[inline(always)]
    fn remove_perk(&mut self, perk: *mut BGSPerk) {
        Actor::remove_perk(self.as_mut(), perk)
    }

    #[inline(always)]
    fn apply_temporary_perk(&mut self, perk: *mut BGSPerk) {
        Actor::apply_temporary_perk(self.as_mut(), perk)
    }

    #[inline(always)]
    fn remove_temporary_perk(&mut self, perk: *mut BGSPerk) {
        Actor::remove_temporary_perk(self.as_mut(), perk)
    }

    #[inline(always)]
    fn remove_spell(&mut self, spell: *mut SpellItem) -> bool {
        Actor::remove_spell(self.as_mut(), spell)
    }

    #[inline(always)]
    fn request_detection_level(&mut self, target: *mut Actor, priority: DETECTION_PRIORITY) -> i32 {
        Actor::request_detection_level(self.as_mut(), target, priority)
    }

    #[inline(always)]
    fn request_los(&mut self, target: *mut Actor, view_cone: f32) -> i32 {
        Actor::request_los(self.as_mut(), target, view_cone)
    }

    #[inline(always)]
    fn for_each_perk(&self, visitor: &mut PerkEntryVisitor) {
        Actor::for_each_perk(self.as_ref(), visitor)
    }

    #[inline(always)]
    fn for_each_perk_entry(&self, entry_type: EntryPoint, visitor: &mut PerkEntryVisitor) {
        Actor::for_each_perk_entry(self.as_ref(), entry_type, visitor)
    }

    #[inline(always)]
    fn apply_perks_from_base(&mut self) {
        Actor::apply_perks_from_base(self.as_mut())
    }

    #[inline(always)]
    fn q_speaking_done(&self) -> bool {
        Actor::q_speaking_done(self.as_ref())
    }

    #[inline(always)]
    fn set_speaking_done(&mut self, set: bool) {
        Actor::set_speaking_done(self.as_mut(), set)
    }

    #[inline(always)]
    fn create_movement_controller(&mut self) {
        Actor::create_movement_controller(self.as_mut())
    }

    #[inline(always)]
    fn update_nav_pos(
        &self,
        pos: &NiPoint3,
        new_pos: &NiPoint3,
        speed: f32,
        distance: f32,
    ) -> bool {
        Actor::update_nav_pos(self.as_ref(), pos, new_pos, speed, distance)
    }

    #[inline(always)]
    fn visit_spells(&mut self, visitor: &mut ActorForEachSpellVisitor) {
        Actor::visit_spells(self.as_mut(), visitor)
    }
}
