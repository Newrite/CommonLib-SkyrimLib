#![allow(non_camel_case_types)]

use core::ffi::c_void;

use core_util::EnumSet;

use crate::re::{
    ActiveEffect, ActorHandle, ActorPackage, AnimResponse, BGSEquipSlot, BGSPerkEntry,
    BSAnimationGraphManager, BSAnimationGraphVariableCache, BSFaceGenAnimationData,
    BSFaceGenNiNode, BSLightingShaderProperty, BSSimpleList, BSSpinLock, BSTArray, BSTEventSource,
    BSTSmartPointer, ExtraDataList, HitData, InventoryEntryData, MagicItem, NiAVObject, NiNode,
    NiPoint3, NiPointer, ObjectRefHandle, QueuedFile, TESBoundObject, TESIdleForm, WardState,
    bhkCharacterController,
};

crate::core_util::abstract_type! {
    pub type bhkRagdollPenetrationUtil;
    pub type BSCloneReserver;
}

/// C++ `RE::RESET_3D_FLAGS`
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RESET_3D_FLAGS {
    None = 0,
    Model = 1 << 0,
    Skin = 1 << 1,
    Head = 1 << 2,
    Face = 1 << 3,
    Scale = 1 << 4,
    Skeleton = 1 << 5,
    InitDefault = 1 << 6,
    SkyCellSkin = 1 << 7,
}

core_util::impl_enumset_type!(RESET_3D_FLAGS => u8);

/// C++ `RE::CommandedActorData`
#[repr(C)]
pub struct CommandedActorData {
    pub commanded_actor: ActorHandle,     // 00
    pub pad04: u32,                       // 04
    pub active_effect: *mut ActiveEffect, // 08
}

const _: () = assert!(core::mem::size_of::<CommandedActorData>() == 0x10);

/// C++ `RE::ObjectEquipParams`
#[repr(C)]
pub struct ObjectEquipParams {
    pub extra_data_list: *mut ExtraDataList, // 00
    pub count: i32,                          // 08
    pub pad0c: u32,                          // 0C
    pub equip_slot: *const BGSEquipSlot,     // 10
    pub unk18: *mut c_void,                  // 18
    pub play_equip_sounds: bool,             // 20
    pub force_equip: bool,                   // 21
    pub show_message: bool,                  // 22
    pub unk23: bool,                         // 23
    pub unk24: bool,                         // 24
    pub pad25: u8,                           // 25
    pub pad26: u16,                          // 26
}

const _: () = assert!(core::mem::size_of::<ObjectEquipParams>() == 0x28);

/// C++ `RE::QueuedItem`
#[repr(C)]
pub struct QueuedItem {
    pub next: *mut QueuedItem,           // 00
    pub object: *mut TESBoundObject,     // 08
    pub equip_params: ObjectEquipParams, // 10
    // TODO: Replace with `NiPointer<QueuedFile>` once `QueuedFile` has a
    // source-backed `NiRef` layer instead of an RTTI-only abstract stub.
    pub queued_files: *mut QueuedFile, // 38 - NiPointer
    pub equip: bool,                   // 40
    pub pad41: u8,                     // 41
    pub pad42: u16,                    // 42
    pub pad44: u32,                    // 44
}

const _: () = assert!(core::mem::size_of::<QueuedItem>() == 0x48);

/// C++ `RE::DeferredHideLimb`
#[repr(C)]
pub struct DeferredHideLimb {
    pub dismember_timer: f32,                     // 00
    pub limb_index: u32,                          // 04
    pub dismembered_limb_root: NiPointer<NiNode>, // 08
    pub replacement_limb: NiPointer<NiNode>,      // 10
    pub next: *mut DeferredHideLimb,              // 18
    pub explosion: bool,                          // 20
    pub pad21: u8,                                // 21
    pub pad22: u16,                               // 22
    pub pad24: u32,                               // 24
}

const _: () = assert!(core::mem::size_of::<DeferredHideLimb>() == 0x28);

/// C++ `RE::AIPerkData`
#[repr(C)]
pub struct AIPerkData {
    pub perk_entry_arrays: [BSTArray<*mut BGSPerkEntry>; 92], // 000
}

const _: () = assert!(core::mem::size_of::<AIPerkData>() == 0x8A0);

impl AIPerkData {
    #[inline(always)]
    pub fn at(&self, pos: usize) -> &BSTArray<*mut BGSPerkEntry> {
        assert!(pos < self.perk_entry_arrays.len());
        &self.perk_entry_arrays[pos]
    }

    #[inline(always)]
    pub fn at_mut(&mut self, pos: usize) -> &mut BSTArray<*mut BGSPerkEntry> {
        assert!(pos < self.perk_entry_arrays.len());
        &mut self.perk_entry_arrays[pos]
    }
}

/// C++ `RE::MiddleHighProcessData`
#[repr(C)]
pub struct MiddleHighProcessData {
    pub event_source: BSTEventSource<*mut c_void>, // 000
    pub run_once_package: ActorPackage,            // 058
    pub dead_detect_list: BSTArray<ActorHandle>,   // 088
    pub ref_list_chair_bed: BSSimpleList<*mut crate::re::TESObjectREFR>, // 0A0
    pub rotation: NiPoint3,                        // 0B0
    pub rotation_speed: NiPoint3,                  // 0BC
    pub actor_mount_position: NiPoint3,            // 0C8
    pub furniture_path_point: NiPoint3,            // 0D4
    pub last_seen_position: NiPoint3,              // 0E0
    pub bleedout_attacker: u32,                    // 0EC
    pub ward_state: WardState,                     // 0F0
    pub pad0f4: u32,                               // 0F4
    pub anim_response: BSTSmartPointer<AnimResponse>, // 0F8
    pub commanded_actors: BSTArray<CommandedActorData>, // 100
    pub damage_root_node: [*mut NiNode; 6],        // 118
    pub unk148: *mut NiAVObject,                   // 148
    pub weapon_bone: *mut NiNode,                  // 150
    pub head_node: *mut NiAVObject,                // 158
    pub torso_node: *mut NiAVObject,               // 160
    pub face_target_source_node: *mut NiAVObject,  // 168
    pub face_node_skinned: *mut BSFaceGenNiNode,   // 170
    pub lighting_property: NiPointer<BSLightingShaderProperty>, // 178
    pub unk180: u64,                               // 180
    pub items_to_equip_unequip: *mut QueuedItem,   // 188
    pub last_hit_data: *mut HitData,               // 190
    pub head_deferred_hide_limb: *mut DeferredHideLimb, // 198
    pub active_effects: *mut BSSimpleList<*mut ActiveEffect>, // 1A0
    pub animation_graph_manager: BSTSmartPointer<BSAnimationGraphManager>, // 1A8
    pub animation_variable_cache: *mut BSAnimationGraphVariableCache, // 1B0
    pub unk1b8: BSTArray<*mut c_void>,             // 1B8
    pub unk1d0: BSTArray<*mut c_void>,             // 1D0
    pub unk1e8: BSSpinLock,                        // 1E8
    // TODO: Resolve the concrete smart-pointer target type from source before
    // replacing this raw stand-in with a nontrivial smart-pointer field.
    pub unk1f0: *mut c_void,                 // 1F0 - smart ptr
    pub unk1f8: u16,                         // 1F8
    pub unk1fa: u16,                         // 1FA
    pub unk1fc: u32,                         // 1FC
    pub unk200: u32,                         // 200
    pub head_height_offset: f32,             // 204
    pub occupied_furniture: ObjectRefHandle, // 208
    pub unk20c: u32,                         // 20C
    pub unk210: *mut TESIdleForm,            // 210
    pub commanding_actor: ActorHandle,       // 218
    pub pad21c: u32,                         // 21C
    pub left_hand: *mut InventoryEntryData,  // 220
    pub furniture_idle: *mut TESIdleForm,    // 228
    // TODO: Resolve the concrete smart-pointer target type from source before
    // replacing this raw stand-in with a nontrivial smart-pointer field.
    pub unk230: *mut c_void,                                // 230 - smart ptr
    pub face_animation_data: *mut BSFaceGenAnimationData,   // 238
    pub current_package_spell: *mut MagicItem,              // 240
    pub unk248: u64,                                        // 248
    pub char_controller: NiPointer<bhkCharacterController>, // 250
    // TODO: Replace with `BSTSmartPointer<bhkRagdollPenetrationUtil>` once the
    // pointee type gains source-backed intrusive ownership semantics.
    pub penetration_detect_util: *mut bhkRagdollPenetrationUtil, // 258 - BSTSmartPointer
    pub right_hand: *mut InventoryEntryData,                     // 260
    pub both_hands: *mut InventoryEntryData,                     // 268
    // TODO: Replace with `NiPointer<QueuedFile>` once `QueuedFile` has a
    // source-backed `NiRef` layer instead of an RTTI-only abstract stub.
    pub body_part_preload: *mut QueuedFile, // 270 - NiPointer
    // TODO: Replace with `NiPointer<BSCloneReserver>` once the pointee type is
    // translated with source-backed `NiRef` semantics.
    pub unk278: *mut BSCloneReserver,       // 278 - NiPointer
    pub last_idle_played: *mut TESIdleForm, // 280
    pub perk_data: *mut AIPerkData,         // 288
    pub unk290: u32,                        // 290
    pub current_furniture_subgraph_id: u32, // 294
    pub unk298: f32,                        // 298
    pub unk29c: f32,                        // 29C
    pub unk2a0: f32,                        // 2A0
    pub unk2a4: f32,                        // 2A4
    pub current_movement_speed: f32,        // 2A8
    pub unk2ac: f32,                        // 2AC
    pub unk2b0: f32,                        // 2B0
    pub bleedout_rate: f32,                 // 2B4
    pub unk2b8: f32,                        // 2B8
    pub maximum_ward_power: f32,            // 2BC
    pub unk2c0: f32,                        // 2C0
    pub torch_evaluation_timer: f32,        // 2C4
    pub alpha_mult: f32,                    // 2C8
    pub script_refract_power: f32,          // 2CC
    pub unk2d0: f32,                        // 2D0
    pub deferred_kill_timer: f32,           // 2D4
    pub kill_move_timer: f32,               // 2D8
    pub unk2dc: f32,                        // 2DC
    pub unk2e0: u32,                        // 2E0
    pub reservation_slot: u32,              // 2E4
    pub current_furniture_marker_id: u32,   // 2E8
    pub unk2ec: u32,                        // 2EC
    pub unk2f0: u64,                        // 2F0
    pub unk2f8: u32,                        // 2F8
    pub unk2fc: u32,                        // 2FC
    pub unk300: u32,                        // 300
    pub unk304: u16,                        // 304
    pub unk306: u16,                        // 306
    pub unk308: u64,                        // 308
    pub unk310: u8,                         // 310
    pub update_3d_model: EnumSet<RESET_3D_FLAGS, u8>, // 311
    pub unk312: u16,                        // 312
    pub unk314: u16,                        // 314
    pub unk316: bool,                       // 316
    pub unk317: bool,                       // 317
    pub unk318: bool,                       // 318
    pub unk319: bool,                       // 319
    pub unk31a: bool,                       // 31A
    pub pick_pocketed: bool,                // 31B
    pub summoned_creature: bool,            // 31C
    pub force_next_update: bool,            // 31D
    pub unk31e: bool,                       // 31E
    pub unk31f: bool,                       // 31F
    pub unk320: bool,                       // 320
    pub unk321: bool,                       // 321
    pub been_attacked: bool,                // 322
    pub always_hit: bool,                   // 323
    pub do_no_damage: bool,                 // 324
    pub soul_trapped: bool,                 // 325
    pub unk326: bool,                       // 326
    pub unk327: bool,                       // 327
    pub unk328: bool,                       // 328
    pub prevent_combat: bool,               // 329
    pub unk32a: bool,                       // 32A
    pub is_fleeing: bool,                   // 32B
    pub unk32c: bool,                       // 32C
    pub hostile_guard: bool,                // 32D
    pub unk32e: bool,                       // 32E
    pub unk32f: bool,                       // 32F
    pub unk330: bool,                       // 330
    pub kill_queued: bool,                  // 331
    pub in_deferred_kill: bool,             // 332
    pub pad333: bool,                       // 333
    pub pad334: u32,                        // 334
}

const _: () = assert!(core::mem::size_of::<MiddleHighProcessData>() == 0x338);
const _: () = assert!(core::mem::offset_of!(MiddleHighProcessData, event_source) == 0x000);
const _: () = assert!(core::mem::offset_of!(MiddleHighProcessData, run_once_package) == 0x058);
const _: () = assert!(core::mem::offset_of!(MiddleHighProcessData, occupied_furniture) == 0x208);
const _: () = assert!(core::mem::offset_of!(MiddleHighProcessData, commanding_actor) == 0x218);
const _: () = assert!(core::mem::offset_of!(MiddleHighProcessData, char_controller) == 0x250);
const _: () = assert!(core::mem::offset_of!(MiddleHighProcessData, script_refract_power) == 0x2CC);
const _: () = assert!(core::mem::offset_of!(MiddleHighProcessData, update_3d_model) == 0x311);
const _: () = assert!(core::mem::offset_of!(MiddleHighProcessData, summoned_creature) == 0x31C);
