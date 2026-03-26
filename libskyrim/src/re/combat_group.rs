use crate::re::bs_atomic::BSReadWriteLock;
use crate::re::{
    AITimeStamp, AITimer, ActorHandle, BGSWorldLocation, BSPathingLOSGridMap, BSTArray,
    CombatGroupDetectionListener, CombatSearchLocation, ObjectRefHandle,
};
use crate::relocation::RelocationID;

/// C++ `RE::CombatTarget::Flags`
#[repr(u16)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CombatTargetFlag {
    None = 0,
    TargetKnown = 1 << 0,
    TargetLost = 1 << 1,
}

core_util::impl_enumset_type!(CombatTargetFlag => u16);

/// C++ `RE::CombatTarget`
#[repr(C)]
pub struct CombatTarget {
    pub target_handle: ActorHandle,
    pub detect_level: i32,
    pub stealth_points: f32,
    pub unk0c: f32,
    pub last_known_loc: BGSWorldLocation,
    pub unk28: BGSWorldLocation,
    pub unk40: BGSWorldLocation,
    pub search_loc: BGSWorldLocation,
    pub unk70: BGSWorldLocation,
    pub unk88: AITimeStamp,
    pub unk8c: AITimeStamp,
    pub last_known_time_stamp: AITimeStamp,
    pub unk94: AITimeStamp,
    pub unk98: AITimeStamp,
    pub unk9c: AITimeStamp,
    pub attacked_member: ActorHandle,
    pub attacker_count: u16,
    pub flags: core_util::EnumSet<CombatTargetFlag, u16>,
}

const _: () = assert!(core::mem::size_of::<CombatTarget>() == 0xA8);

/// C++ `RE::CombatMember`
#[repr(C)]
pub struct CombatMember {
    pub member_handle: ActorHandle,
    pub group_strength_update_timer: f32,
    pub threat_value: f32,
}

const _: () = assert!(core::mem::size_of::<CombatMember>() == 0x0C);

/// C++ `RE::CombatSearchDoor`
#[repr(C)]
pub struct CombatSearchDoor {
    pub door_handle: ObjectRefHandle,
    pub linked_door_handle: ObjectRefHandle,
    pub unk08: u8,
    pub unk09: u8,
    pub unk0a: u8,
    pub pad0b: u8,
}

const _: () = assert!(core::mem::size_of::<CombatSearchDoor>() == 0x0C);

/// C++ `RE::CombatGroup`
#[repr(C)]
pub struct CombatGroup {
    pub group_id: u32,
    pub group_index: u32,
    pub targets: BSTArray<CombatTarget>,
    pub members: BSTArray<CombatMember>,
    pub detection_listener: *mut CombatGroupDetectionListener,
    pub ally_killed_timer: AITimer,
    pub avoid_threath_timer: AITimer,
    pub unk50: AITimer,
    pub detection_dialogue_timers: [AITimer; 11],
    pub update_timer: AITimer,
    pub music_threat_ratio_timer: AITimer,
    pub unkc0: AITimer,
    pub unkc8: f32,
    pub unkcc: f32,
    pub unkd0: f32,
    pub unkd4: f32,
    pub search_state: u32,
    pub paddc: u32,
    pub grid_map: *mut BSPathingLOSGridMap,
    pub search_update_timer: AITimer,
    pub search_area_update_timer: AITimer,
    pub search_started_time_stamp: AITimeStamp,
    pub target_to_search_for: ActorHandle,
    pub search_target_loc: BGSWorldLocation,
    pub search_radius: f32,
    pub pad11c: u32,
    pub search_locations: BSTArray<CombatSearchLocation>,
    pub search_doors: BSTArray<CombatSearchDoor>,
    pub initialized_member_count: u32,
    pub flee_count: u32,
    pub fight_count: u32,
    pub music_state: u8,
    pub unk15d: u8,
    pub unk15e: u8,
    pub unk15f: u8,
    pub lock: BSReadWriteLock,
}

const _: () = assert!(core::mem::size_of::<CombatGroup>() == 0x168);
const _: () = assert!(core::mem::offset_of!(CombatGroup, detection_listener) == 0x38);
const _: () = assert!(core::mem::offset_of!(CombatGroup, grid_map) == 0xE0);
const _: () = assert!(core::mem::offset_of!(CombatGroup, search_locations) == 0x120);
const _: () = assert!(core::mem::offset_of!(CombatGroup, lock) == 0x160);

impl CombatGroup {
    crate::relocation_func! {
        pub fn is_searching(&self) -> bool => RelocationID::new(48023, 0)
    }
}
