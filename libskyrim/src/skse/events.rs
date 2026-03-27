use core_util::EnumSet;

use crate::re::{Actor, BSFixedString, NiPointer, TESCameraState, TESForm, TESObjectREFR};
use crate::sdk::core::{GameRef, NativeOwner};

#[repr(C)]
pub struct ModCallbackEvent {
    pub event_name: BSFixedString,
    pub str_arg: BSFixedString,
    pub num_arg: f32,
    pub sender: *mut TESForm,
}

impl ModCallbackEvent {
    #[inline(always)]
    pub fn sender_ref(&self) -> GameRef<'_, TESForm> {
        unsafe { GameRef::from_raw(self.sender) }
    }
}

#[repr(C)]
pub struct CameraEvent {
    pub old_state: *mut TESCameraState,
    pub new_state: *mut TESCameraState,
}

impl CameraEvent {
    #[inline(always)]
    pub fn old_state_ref(&self) -> GameRef<'_, TESCameraState> {
        unsafe { GameRef::from_raw(self.old_state) }
    }

    #[inline(always)]
    pub fn new_state_ref(&self) -> GameRef<'_, TESCameraState> {
        unsafe { GameRef::from_raw(self.new_state) }
    }
}

#[repr(C)]
pub struct CrosshairRefEvent {
    pub crosshair_ref: NiPointer<TESObjectREFR>,
}

impl CrosshairRefEvent {
    #[inline(always)]
    pub fn reference_ref(&self) -> GameRef<'_, TESObjectREFR> {
        self.crosshair_ref.borrow()
    }
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ActionEventType {
    WeaponSwing = 0,
    SpellCast = 1,
    SpellFire = 2,
    VoiceCast = 3,
    VoiceFire = 4,
    BowDraw = 5,
    BowRelease = 6,
    BeginDraw = 7,
    EndDraw = 8,
    BeginSheathe = 9,
    EndSheathe = 10,
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ActionEventSlot {
    Left = 0,
    Right = 1,
    Voice = 2,
}

#[repr(C)]
pub struct ActionEvent {
    pub type_: EnumSet<ActionEventType, u32>,
    pub actor: *mut Actor,
    pub source_form: *mut TESForm,
    pub slot: EnumSet<ActionEventSlot, u32>,
}

impl ActionEvent {
    #[inline(always)]
    pub fn actor_ref(&self) -> GameRef<'_, Actor> {
        unsafe { GameRef::from_raw(self.actor) }
    }

    #[inline(always)]
    pub fn source_form_ref(&self) -> GameRef<'_, TESForm> {
        unsafe { GameRef::from_raw(self.source_form) }
    }
}

#[repr(C)]
pub struct NiNodeUpdateEvent {
    pub reference: *mut TESObjectREFR,
}

impl NiNodeUpdateEvent {
    #[inline(always)]
    pub fn reference_ref(&self) -> GameRef<'_, TESObjectREFR> {
        unsafe { GameRef::from_raw(self.reference) }
    }
}

const _: () = assert!(core::mem::size_of::<ModCallbackEvent>() == 0x20);
const _: () = assert!(core::mem::size_of::<CameraEvent>() == 0x10);
const _: () = assert!(core::mem::size_of::<CrosshairRefEvent>() == 0x8);
const _: () = assert!(core::mem::size_of::<ActionEvent>() == 0x20);
const _: () = assert!(core::mem::offset_of!(ActionEvent, type_) == 0x00);
const _: () = assert!(core::mem::offset_of!(ActionEvent, actor) == 0x08);
const _: () = assert!(core::mem::offset_of!(ActionEvent, source_form) == 0x10);
const _: () = assert!(core::mem::offset_of!(ActionEvent, slot) == 0x18);
const _: () = assert!(core::mem::size_of::<NiNodeUpdateEvent>() == 0x8);
