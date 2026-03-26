#![allow(non_camel_case_types)]

use core::ffi::c_void;

use core_util::EnumSet;

use crate::re::bs_atomic::BSReadWriteLock;
use crate::re::bs_core_types::{FormID, RefHandle};
use crate::re::bst_smart_pointer::{BSTSmartPointer, BSTSmartPointerIntrusiveRefCountable};
use crate::re::{
    AITimeStamp, ActorHandle, ActorKnowledge, BGSAnimationSequencer, BGSAttackData, BGSProjectile,
    BSFixedString, BSIntrusiveRefCounted, BSSimpleList, BSSoundHandle, BSTArray, BSTSmallArray,
    BSTTuple, Crime, DialogueItem, IAnimationSetCallbackFunctor, MagicItem, ModelDBHandle,
    Movement, NiAVObject, NiBillboardNode, NiPoint3, NiPointLight, NiPointer, NiRefObject,
    ObjectRefHandle, PLAYER_ACTION, SpellItem, TESIdleForm, TESObjectREFR, TESObjectWEAP, TESShout,
    TESTopicInfo,
};
use crate::relocation::RelocationID;

crate::core_util::abstract_type! {
    pub type StandardDetectionListener;
    pub type QueuedDialogueType;
}

/// C++ `RE::VOICE_STATE`
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum VOICE_STATE {
    None = 0,
    Request = 1,
    Start = 2,
    Continue = 3,
    Precast = 4,
    Postcast = 5,
    Fail = 6,
}

core_util::impl_enumset_type!(VOICE_STATE => u32);

/// C++ `RE::DetectionEvent`
#[repr(C)]
pub union DetectionEventData {
    pub action_value: u32,
    pub location: NiPoint3,
}

/// C++ `RE::DetectionEvent`
#[repr(C)]
pub union DetectionEventContext {
    pub time_stamp: f32,
    pub reference: ObjectRefHandle,
}

/// C++ `RE::DetectionEvent`
#[repr(C)]
pub struct DetectionEvent {
    pub data: DetectionEventData,       // 00
    pub pad0c: u32,                     // 0C
    pub context: DetectionEventContext, // 10
    pub pad14: u32,                     // 14
}

const _: () = assert!(core::mem::size_of::<DetectionEvent>() == 0x18);

/// C++ `RE::HighProcessData::FADE_STATE`
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HighProcessDataFADE_STATE {
    Normal = 0,
    In = 1,
    Out = 2,
    TeleportIn = 3,
    TeleportOut = 4,
    OutDisable = 5,
    OutDelete = 6,
}

core_util::impl_enumset_type!(HighProcessDataFADE_STATE => u32);

/// C++ `RE::HighProcessData::BUMP_TYPE`
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HighProcessDataBUMP_TYPE {
    None = -1,
    Small = 0,
    Big = 1,
}

/// C++ `RE::HighProcessData::HEAD_TRACK_TYPES::HEAD_TRACK_TYPE`
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HighProcessDataHEAD_TRACK_TYPE {
    Default = 0,
    Action = 1,
    Script = 2,
    Combat = 3,
    Dialogue = 4,
    Procedure = 5,
    Total = 6,
}

/// C++ `RE::HighProcessData::Data190::Data::UnkData`
#[repr(C)]
pub struct HighProcessDataData190DataUnkData {
    pub data: [u64; 16], // 00
}

const _: () = assert!(core::mem::size_of::<HighProcessDataData190DataUnkData>() == 0x80);

/// C++ `RE::HighProcessData::Data190::Data`
#[repr(C)]
pub struct HighProcessDataData190Data {
    pub unk00: *mut HighProcessDataData190DataUnkData, // 00
    pub unk08: u64,                                    // 08
}

const _: () = assert!(core::mem::size_of::<HighProcessDataData190Data>() == 0x10);

/// C++ `RE::HighProcessData::Data190`
#[repr(C)]
pub struct HighProcessDataData190 {
    pub ref_counted_base: BSIntrusiveRefCounted, // 00
    pub unk04: u32,                              // 04
    pub unk08: BSTSmallArray<
        HighProcessDataData190Data,
        { core::mem::size_of::<HighProcessDataData190Data>() },
    >, // 08
    pub unk28: u64,                              // 28
}

const _: () = assert!(core::mem::size_of::<HighProcessDataData190>() == 0x30);

impl BSTSmartPointerIntrusiveRefCountable for HighProcessDataData190 {
    #[inline(always)]
    fn bst_inc_ref(&self) {
        self.ref_counted_base.inc_ref();
    }

    #[inline(always)]
    fn bst_dec_ref(&self) -> u32 {
        self.ref_counted_base.dec_ref()
    }

    #[inline(always)]
    unsafe fn bst_delete(&self) {
        let this = self as *const Self as *mut Self;
        unsafe {
            core::ptr::drop_in_place(this);
            crate::ffi::commonlib_free(this.cast());
        }
    }
}

/// C++ `RE::HighProcessData::Data208`
#[repr(C)]
pub struct HighProcessDataData208 {
    pub unk00: u64,                        // 00
    pub unk08: u64,                        // 08
    pub unk10: u64,                        // 10
    pub unk18: u64,                        // 18
    pub unk20: u64,                        // 20
    pub unk28: u64,                        // 28
    pub next: *mut HighProcessDataData208, // 30
}

const _: () = assert!(core::mem::size_of::<HighProcessDataData208>() == 0x38);

/// C++ `RE::HighProcessData::MuzzleFlash`
#[repr(C)]
pub struct HighProcessDataMuzzleFlash {
    pub enabled: bool,                           // 00
    pub mps_enabled: bool,                       // 01
    pub update_light: bool,                      // 02
    pub unk03: bool,                             // 03
    pub enable_timer: f32,                       // 04
    pub muzzle_flash_duration: f32,              // 08
    pub unk0c: u32,                              // 0C
    pub projectile_3d: NiPointer<NiAVObject>,    // 10
    pub projectile_node: NiPointer<NiAVObject>,  // 18
    pub attached_light: NiPointer<NiPointLight>, // 20
    pub base_projectile: *mut BGSProjectile,     // 28
    pub weapon_source: *mut TESObjectWEAP,       // 30
    pub shooter_handle: ActorHandle,             // 38
    pub unk3c: u32,                              // 3C
}

const _: () = assert!(core::mem::size_of::<HighProcessDataMuzzleFlash>() == 0x40);

/// C++ `RE::HighProcessData`
#[repr(C)]
pub struct HighProcessData {
    pub voice_state: EnumSet<VOICE_STATE, u32>,      // 000
    pub pad004: u32,                                 // 004
    pub current_shout: *mut TESShout,                // 008
    pub current_shout_variation: u32,                // 010 - TESShout::VariationID
    pub voice_time_elapsed: f32,                     // 014
    pub voice_recovery_time: f32,                    // 018
    pub health_regen_delay: f32,                     // 01C
    pub stamina_regen_delay: f32,                    // 020
    pub magicka_regen_delay: f32,                    // 024
    pub unk028: f32,                                 // 028
    pub unk02c: u32,                                 // 02C
    pub last_spoken_to_array: BSTArray<ActorHandle>, // 030
    pub unk048: u64,                                 // 048
    pub anim_sequencer: BGSAnimationSequencer,       // 050
    pub pathing_current_movement_speed: NiPoint3,    // 088
    pub pathing_current_rotation_speed: NiPoint3,    // 094
    pub pathing_desired_position: NiPoint3,          // 0A0
    pub pathing_desired_orientation: NiPoint3,       // 0AC
    pub pathing_desired_movement_speed: NiPoint3,    // 0B8
    pub pathing_desired_rotation_speed: NiPoint3,    // 0C4
    pub unk0d0: u32,                                 // 0D0
    pub last_bump_direction: f32,                    // 0D4
    pub last_ext_door_activated: ObjectRefHandle,    // 0D8
    pub activation_height: f32,                      // 0DC
    pub reanimate_caster: ActorHandle,               // 0E0
    pub pad0e4: u32,                                 // 0E4
    pub reanimate_spell: *mut MagicItem,             // 0E8
    pub current_movement_type: Movement::TypeData,   // 0F0
    pub fade_state: EnumSet<HighProcessDataFADE_STATE, u32>, // 130
    pub fade_alpha: f32,                             // 134
    pub fade_trigger: *mut TESObjectREFR,            // 138
    pub head_track_target: [ObjectRefHandle; 6],     // 140
    pub head_tracked: [bool; 6],                     // 158
    pub unk15e: u16,                                 // 15E
    pub head_track_target_timer: f32,                // 160
    pub head_track_target_offset: NiPoint3,          // 164
    pub head_track_hold_offset_hold_timer: f32,      // 170
    pub head_track_target_offset_timer: f32,         // 174
    pub last_target: ObjectRefHandle,                // 178
    pub path_look_at_target: ObjectRefHandle,        // 17C
    // TODO: Resolve the concrete smart-pointer target type from source before
    // replacing this raw stand-in with a nontrivial smart-pointer field.
    pub unk180: *mut c_void, // 180 - smart ptr
    // TODO: Resolve the concrete smart-pointer target type from source before
    // replacing this raw stand-in with a nontrivial smart-pointer field.
    pub unk188: *mut c_void,                             // 188 - smart ptr
    pub unk190: BSTSmartPointer<HighProcessDataData190>, // 190
    pub unk198: BSTSmartPointer<HighProcessDataData190>, // 198
    pub unk1a0: NiPoint3,                                // 1A0
    pub unk1ac: u32,                                     // 1AC
    pub unk1b0: u64,                                     // 1B0
    pub unk1b8: u64,                                     // 1B8
    pub unk1c0: u64,                                     // 1C0
    pub unk1c8: u64,                                     // 1C8
    pub unk1d0: u64,                                     // 1D0
    pub unk1d8: u64,                                     // 1D8
    pub unk1e0: f32,                                     // 1E0
    pub cached_actor_height: f32,                        // 1E4
    pub unk1e8: NiPointer<NiRefObject>,                  // 1E8
    pub unk1f0: u32,                                     // 1F0
    pub bump_timer: AITimeStamp,                         // 1F4
    pub unk1f8: AITimeStamp,                             // 1F8
    pub bumped_state: HighProcessDataBUMP_TYPE,          // 1FC
    pub take_back_timer: f32,                            // 200
    pub pad204: u32,                                     // 204
    pub unk208: *mut HighProcessDataData208,             // 208
    pub avoid_wait_timer: f32,                           // 210
    pub player_action_reaction: PLAYER_ACTION,           // 214
    pub voice_subtitle: BSFixedString,                   // 218
    pub knowledge_array: BSTArray<BSTTuple<FormID, NiPointer<ActorKnowledge>>>, // 220
    pub knowledge_lock: BSReadWriteLock,                 // 238
    pub queue_of_greetings: BSTArray<*mut QueuedDialogueType>, // 240
    pub attack_data: NiPointer<BGSAttackData>,           // 258
    pub location_offset_by_water_point: NiPoint3,        // 260
    pub unk26c: u32,                                     // 26C
    pub unk270: u64,                                     // 270
    pub unk278: f32,                                     // 278
    pub death_force_direction: NiPoint3,                 // 27C
    pub death_force: f32,                                // 288
    pub unk28c: f32,                                     // 28C
    pub unk290: f32,                                     // 290
    pub unk294: f32,                                     // 294
    pub unk298: f32,                                     // 298
    pub clear_talk_to_list_timer: f32,                   // 29C
    pub max_alpha: f32,                                  // 2A0
    pub unk2a4: f32,                                     // 2A4
    pub unk2a8: u64,                                     // 2A8
    pub check_to_talk_timer: f32,                        // 2B0
    pub unk2b4: u32,                                     // 2B4
    pub unk2b8: u64,                                     // 2B8
    pub unk2c0: u64,                                     // 2C0
    pub unk2c8: u64,                                     // 2C8
    pub unk2d0: u64,                                     // 2D0
    pub anim_action: u32,                                // 2D8
    pub left_weapon_last_pos: NiPoint3,                  // 2DC
    pub right_weapon_last_pos: NiPoint3,                 // 2E8
    pub greet_actor: ObjectRefHandle,                    // 2F4
    pub sound_delay: f32,                                // 2F8
    pub sound_handles: [BSSoundHandle; 2],               // 2FC
    pub greeting_timer: f32,                             // 314
    pub exclusive_timer: f32,                            // 318
    pub idle_timer: f32,                                 // 31C
    pub detect_greet_timer: f32,                         // 320
    pub breath_timer: f32,                               // 324
    pub voice_timer: f32,                                // 328
    pub dying_timer: f32,                                // 32C
    pub last_greeting: *mut TESTopicInfo,                // 330
    pub unk338: u64,                                     // 338
    pub aware_player_timer: f32,                         // 340
    pub unk344: u32,                                     // 344
    pub current_process_idle: *mut TESIdleForm,          // 348
    pub unk350: RefHandle,                               // 350
    pub unk354: u32,                                     // 354
    pub unk358: u64,                                     // 358
    pub greet_topic: BSTSmartPointer<DialogueItem>,      // 360
    pub unk368: u32,                                     // 368
    pub unk36c: RefHandle,                               // 36C
    pub face_gen_geom_handle: ModelDBHandle,             // 370
    pub unk378: u64,                                     // 378
    pub health_bar_node: NiPointer<NiBillboardNode>,     // 380
    pub unk388: f32,                                     // 388
    pub unk38c: u32,                                     // 38C
    pub number_guards_pursuing: u32,                     // 390
    pub re_equip_armor_timer: f32,                       // 394
    pub leveled_spell_list: *mut BSSimpleList<*mut SpellItem>, // 398
    pub detection_modifier: f32,                         // 3A0
    pub detection_modifier_timer: f32,                   // 3A4
    pub light_level: f32,                                // 3A8
    pub scene_head_track_timer: f32,                     // 3AC
    pub p_cap_voice_failsafe_timer: f32,                 // 3B0
    pub pad3b4: u32,                                     // 3B4
    pub unk3b8: *mut c_void,                             // 3B8
    pub unk3c0: u64,                                     // 3C0
    pub muzzle_flash: *mut HighProcessDataMuzzleFlash,   // 3C8
    pub unk3d0: u32,                                     // 3D0
    pub pad3d4: u32,                                     // 3D4
    pub actors_generated_detection_event: *mut DetectionEvent, // 3D8
    // TODO: Replace with `NiPointer<StandardDetectionListener>` once the
    // pointee type is translated with source-backed `NiRef` semantics.
    pub detection_listener: *mut StandardDetectionListener, // 3E0 - NiPointer
    pub unk3e8: u64,                                        // 3E8
    // TODO: Resolve the concrete smart-pointer target type from source before
    // replacing this raw stand-in with a nontrivial smart-pointer field.
    pub unk3f0: *mut c_void, // 3F0 - smart ptr
    pub unk3f8: u64,         // 3F8
    pub unk400: BSTSmallArray<u64, { core::mem::size_of::<u64>() }>, // 400
    pub animation_delta: NiPoint3, // 418
    pub animation_angle_mod: NiPoint3, // 424
    // TODO: Replace with `BSTSmartPointer<IAnimationSetCallbackFunctor>` once a
    // source-backed intrusive-refcount mapping exists for this interface layer.
    // `IAnimationSetCallbackFunctor` itself exposes only a vtable root; the
    // concrete refcount currently lives in derived movement-controller objects.
    pub unk430: *mut IAnimationSetCallbackFunctor, // 430 - BSTSmartPointer
    pub absorb_timer: f32,                         // 438
    pub unk43c: f32,                               // 43C
    pub crime_to_react_to: *mut Crime,             // 440
    pub unk448: u64,                               // 448
    pub unk450: bool,                              // 450
    pub unk451: u8,                                // 451
    pub unk452: u8,                                // 452
    pub unk453: u8,                                // 453
    pub greeting_player: bool,                     // 454
    pub unk455: u8,                                // 455
    pub unk456: u8,                                // 456
    pub detect_alert: bool,                        // 457
    pub talking_to_pc: bool,                       // 458
    pub in_command_state: bool,                    // 459
    pub unk45a: u8,                                // 45A
    pub unk45b: u8,                                // 45B
    pub unk45c: u8,                                // 45C
    pub unk45d: u8,                                // 45D
    pub unk45e: u8,                                // 45E
    pub is_dual_casting: bool,                     // 45F
    pub get_planted_explosive: bool,               // 460
    pub approaching_auto_teleport_door: bool,      // 461
    pub arrested: bool,                            // 462
    pub unk463: bool,                              // 463
    pub unk464: bool,                              // 464
    pub unk465: bool,                              // 465
    pub unk466: bool,                              // 466
    pub unk467: bool,                              // 467
    pub far_geometry: bool,                        // 468
    pub unk469: bool,                              // 469
    pub unk46a: bool,                              // 46A
    pub death_dialogue: bool,                      // 46B
    pub fists_drawn: bool,                         // 46C
    pub unk46d: bool,                              // 46D
    pub unk46e: bool,                              // 46E
    pub allow_rotation: bool,                      // 46F
    pub door_activated: bool,                      // 470
    pub unk471: bool,                              // 471
    pub aggro_radius_started: bool,                // 472
    pub pad473: u8,                                // 473
    pub pad474: u32,                               // 474
}

const _: () = assert!(core::mem::size_of::<HighProcessData>() == 0x478);
const _: () = assert!(core::mem::offset_of!(HighProcessData, current_shout) == 0x008);
const _: () = assert!(core::mem::offset_of!(HighProcessData, voice_recovery_time) == 0x018);
const _: () = assert!(core::mem::offset_of!(HighProcessData, head_track_target) == 0x140);
const _: () = assert!(core::mem::offset_of!(HighProcessData, cached_actor_height) == 0x1E4);
const _: () = assert!(core::mem::offset_of!(HighProcessData, player_action_reaction) == 0x214);
const _: () = assert!(core::mem::offset_of!(HighProcessData, face_gen_geom_handle) == 0x370);
const _: () = assert!(core::mem::offset_of!(HighProcessData, in_command_state) == 0x459);

impl HighProcessData {
    crate::relocation_func! {
        pub fn clear_headtrack_target(&mut self, headtrack_type: HighProcessDataHEAD_TRACK_TYPE, default_hold: bool) => RelocationID::new(38726, 39756)
    }

    crate::relocation_func! {
        pub fn set_headtrack_target(&mut self, headtrack_type: HighProcessDataHEAD_TRACK_TYPE, target: *mut TESObjectREFR) => RelocationID::new(38760, 39783)
    }
}
