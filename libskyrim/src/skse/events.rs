use core_util::EnumSet;

use crate::re::{Actor, BSFixedString, NiPointer, TESCameraState, TESForm, TESObjectREFR};
use crate::sdk::core::{NativeOwner, Resolved};

#[repr(C)]
pub struct ModCallbackEvent {
    pub event_name: BSFixedString,
    pub str_arg: BSFixedString,
    pub num_arg: f32,
    pub sender: *mut TESForm,
}

impl ModCallbackEvent {
    #[inline(always)]
    pub const fn sender_ptr(&self) -> *mut TESForm {
        self.sender
    }

    #[inline(always)]
    pub fn sender(&self) -> Option<&TESForm> {
        unsafe { self.sender.as_ref() }
    }
}

#[repr(C)]
pub struct CameraEvent {
    pub old_state: *mut TESCameraState,
    pub new_state: *mut TESCameraState,
}

impl CameraEvent {
    #[inline(always)]
    pub const fn old_state_ptr(&self) -> *mut TESCameraState {
        self.old_state
    }

    #[inline(always)]
    pub fn old_state(&self) -> Option<&TESCameraState> {
        unsafe { self.old_state.as_ref() }
    }

    #[inline(always)]
    pub const fn new_state_ptr(&self) -> *mut TESCameraState {
        self.new_state
    }

    #[inline(always)]
    pub fn new_state(&self) -> Option<&TESCameraState> {
        unsafe { self.new_state.as_ref() }
    }
}

#[repr(C)]
pub struct CrosshairRefEvent {
    pub crosshair_ref: NiPointer<TESObjectREFR>,
}

impl CrosshairRefEvent {
    #[inline(always)]
    pub fn reference_owner(&self) -> &NiPointer<TESObjectREFR> {
        &self.crosshair_ref
    }

    #[inline(always)]
    pub fn reference(&self) -> Option<&TESObjectREFR> {
        self.crosshair_ref.as_ref()
    }

    #[inline(always)]
    pub fn reference_resolved(&self) -> Option<Resolved<TESObjectREFR>> {
        self.reference().and_then(Resolved::try_from_ref)
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
    pub const fn actor_ptr(&self) -> *mut Actor {
        self.actor
    }

    #[inline(always)]
    pub fn actor(&self) -> Option<&Actor> {
        unsafe { self.actor.as_ref() }
    }

    #[inline(always)]
    pub fn actor_resolved(&self) -> Option<Resolved<Actor>> {
        self.actor().and_then(Resolved::try_from_ref)
    }

    #[inline(always)]
    pub const fn source_form_ptr(&self) -> *mut TESForm {
        self.source_form
    }

    #[inline(always)]
    pub fn source_form(&self) -> Option<&TESForm> {
        unsafe { self.source_form.as_ref() }
    }
}

#[repr(C)]
pub struct NiNodeUpdateEvent {
    pub reference: *mut TESObjectREFR,
}

impl NiNodeUpdateEvent {
    #[inline(always)]
    pub const fn reference_ptr(&self) -> *mut TESObjectREFR {
        self.reference
    }

    #[inline(always)]
    pub fn reference(&self) -> Option<&TESObjectREFR> {
        unsafe { self.reference.as_ref() }
    }

    #[inline(always)]
    pub fn reference_resolved(&self) -> Option<Resolved<TESObjectREFR>> {
        self.reference().and_then(Resolved::try_from_ref)
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
