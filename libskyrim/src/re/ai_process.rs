use core_util::EnumSet;

use crate::re::{
    Actor, ActorHandle, ActorPackage, ActorValue, BGSEquipSlot, BSFixedString, BSSimpleList,
    BSTArray, BipedAnim, DEFAULT_OBJECT, DEFAULT_OBJECT_ACTION_IDLE, HighProcessData,
    HighProcessDataHEAD_TRACK_TYPE, InventoryEntryData, MiddleHighProcessData, NiAVObject,
    NiPoint3, ObjectRefHandle, PROCESS_TYPE, RESET_3D_FLAGS, TESForm, TESIdleForm, TESObjectREFR,
    TESPackage, TESShout, bhkCharacterController,
};
use crate::relocation::{ID, RelocationID, skyrim_cast};
use crate::version::RUNTIME_VR_1_4_15;

/// C++ `RE::MiddleLowProcessData`
#[repr(C)]
pub struct MiddleLowProcessData {
    pub hour_package_evaluated: i32, // 00
}

const _: () = assert!(core::mem::size_of::<MiddleLowProcessData>() == 0x4);

/// C++ `RE::CachedValueData`
#[repr(C)]
pub struct CachedValueData {
    pub value: f32,    // 00
    pub invalid: bool, // 04
    pub pad5: u8,      // 05
    pub pad6: u16,     // 06
}

const _: () = assert!(core::mem::size_of::<CachedValueData>() == 0x8);

/// C++ `RE::CachedValues::Flags`
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CachedValuesFlags {
    None = 0,
    Radius = 1 << 0,
    Width = 1 << 1,
    Length = 1 << 2,
    DPS = 1 << 3,
    MedicineEffectivenessMult = 1 << 4,
    EyeLevel = 1 << 9,
    ConditionPreventsRun = 1 << 10,
    ForwardLength = 1 << 11,
    ActorIsGhost = 1 << 20,
    HealthDamaged = 1 << 21,
    MagickaPointsDamaged = 1 << 22,
    StaminaDamaged = 1 << 23,
    OwnerIsNPC = 1 << 25,
    OwnerIsUndead = 1 << 26,
    OwnerIsInCombatantFaction = 1 << 27,
}

core_util::impl_enumset_type!(CachedValuesFlags => u32);

/// C++ `RE::CachedValues::BooleanValue`
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CachedValuesBooleanValue {
    None = 0,
    ConditionPreventsRun = 1 << 0,
    OwnerIsNPC = 1 << 1,
    OwnerIsUndead = 1 << 2,
    OwnerIsInCombatantFaction = 1 << 3,
}

core_util::impl_enumset_type!(CachedValuesBooleanValue => u32);

/// C++ `RE::CachedValues`
#[repr(C)]
pub struct CachedValues {
    pub cached_radius: f32,                                     // 00
    pub cached_width: f32,                                      // 04
    pub cached_length: f32,                                     // 08
    pub cached_forward_length: f32,                             // 0C
    pub cached_dps: f32,                                        // 10
    pub cached_eye_level: f32,                                  // 14
    pub cached_walk_speed: f32,                                 // 18
    pub cached_run_speed: f32,                                  // 1C
    pub cached_jog_speed: f32,                                  // 20
    pub cached_fast_walk_speed: f32,                            // 24
    pub boolean_values: EnumSet<CachedValuesBooleanValue, u32>, // 28
    pub flags: EnumSet<CachedValuesFlags, u32>,                 // 2C
    pub actor_value_cache: BSTArray<CachedValueData>,           // 30
    pub max_actor_value_cache: BSTArray<CachedValueData>,       // 48
}

const _: () = assert!(core::mem::size_of::<CachedValues>() == 0x60);

/// C++ `RE::ObjectstoAcquire`
#[repr(C)]
pub struct ObjectsToAcquire {
    pub unk00: u64, // 00
    pub unk08: u64, // 08
    pub unk10: u64, // 10
    pub unk18: u64, // 18
    pub unk20: u64, // 20
}

const _: () = assert!(core::mem::size_of::<ObjectsToAcquire>() == 0x28);

/// C++ `RE::AIProcess::LowProcessFlags`
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AIProcessLowProcessFlags {
    None = 0,
    TargetActivated = 1 << 0,
    CurrentActionComplete = 1 << 1,
    IsAggressor = 1 << 2,
    Alert = 1 << 3,
    Follower = 1 << 4,
    PackageDoneOnce = 1 << 5,
    PackageIdleDone = 1 << 6,
}

core_util::impl_enumset_type!(AIProcessLowProcessFlags => u8);

/// C++ `RE::AIProcess::Hands::Hand`
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AIProcessHand {
    Left = 0,
    Right = 1,
    Total = 2,
}

/// C++ `RE::AIProcess::EquippedObject`
#[repr(C)]
pub struct AIProcessEquippedObject {
    pub object: *mut TESForm,    // 00
    pub slot: *mut BGSEquipSlot, // 08
}

const _: () = assert!(core::mem::size_of::<AIProcessEquippedObject>() == 0x10);

/// C++ `RE::AIProcess::Data0B8`
#[repr(C)]
pub struct AIProcessData0B8 {
    pub unk00: *mut core::ffi::c_void, // 00
    pub unk08: *mut AIProcessData0B8,  // 08
    pub unk10: *mut core::ffi::c_void, // 10
    pub unk18: *mut core::ffi::c_void, // 18
    pub unk20: u64,                    // 20
    pub unk28: *mut core::ffi::c_void, // 28
    pub unk30: u32,                    // 30
    pub pad34: u32,                    // 34
}

const _: () = assert!(core::mem::size_of::<AIProcessData0B8>() == 0x38);

/// C++ `RE::AIProcess`
#[repr(C)]
pub struct AIProcess {
    pub middle_low: *mut MiddleLowProcessData,        // 000
    pub middle_high: *mut MiddleHighProcessData,      // 008
    pub high: *mut HighProcessData,                   // 010
    pub current_package: ActorPackage,                // 018
    pub hour_last_processed: f32,                     // 048
    pub date_last_processed: f32,                     // 04C
    pub cached_values: *mut CachedValues,             // 050
    pub number_items_activate: i32,                   // 058
    pub pad05c: u32,                                  // 05C
    pub objects: BSSimpleList<*mut ObjectsToAcquire>, // 060
    pub generic_locations: BSSimpleList<*mut TESObjectREFR>, // 070
    pub acquire_object: *mut ObjectsToAcquire,        // 080
    pub saved_acquire_object: *mut ObjectsToAcquire,  // 088
    pub essential_down_timer: f32,                    // 090
    pub death_time: f32,                              // 094
    pub tracked_damage: f32,                          // 098
    pub pad09c: u32,                                  // 09C
    pub equipped_forms: BSTArray<AIProcessEquippedObject>, // 0A0
    pub unk0b8: AIProcessData0B8,                     // 0B8
    pub equipped_objects: [*mut TESForm; 2],          // 0F0
    pub unk100: u64,                                  // 100
    pub unk108: u64,                                  // 108
    pub follow_target: u32,                           // 110 - RefHandle
    pub target: u32,                                  // 114 - RefHandle
    pub arrest_target: u32,                           // 118 - RefHandle
    pub unk120: u64,                                  // 120
    pub unk128: u64,                                  // 128
    pub unk130: u32,                                  // 130
    pub unk134: u16,                                  // 134
    pub low_process_flags: EnumSet<AIProcessLowProcessFlags, u8>, // 136
    pub process_level: EnumSet<PROCESS_TYPE, u8>,     // 137
    pub skipped_time_stamp_for_pathing: bool,         // 138
    pub ignoring_combat: bool,                        // 139
    pub end_alarm_on_actor: bool,                     // 13A
    pub escorting_player: bool,                       // 13B
    pub pad13c: u32,                                  // 13C
}

const _: () = assert!(core::mem::size_of::<AIProcess>() == 0x140);
const _: () = assert!(core::mem::offset_of!(AIProcess, middle_high) == 0x08);
const _: () = assert!(core::mem::offset_of!(AIProcess, high) == 0x10);
const _: () = assert!(core::mem::offset_of!(AIProcess, current_package) == 0x18);
const _: () = assert!(core::mem::offset_of!(AIProcess, tracked_damage) == 0x98);
const _: () = assert!(core::mem::offset_of!(AIProcess, equipped_objects) == 0xF0);
const _: () = assert!(core::mem::offset_of!(AIProcess, process_level) == 0x137);

impl AIProcess {
    #[inline(always)]
    pub fn clear_action_headtrack_target(&mut self, default_hold: bool) {
        unsafe {
            if let Some(high) = self.high.as_mut() {
                high.clear_headtrack_target(HighProcessDataHEAD_TRACK_TYPE::Action, default_hold);
            }
        }
    }

    #[inline(always)]
    pub fn get_cached_height(&self) -> f32 {
        unsafe {
            self.high
                .as_ref()
                .map(|high| high.cached_actor_height)
                .unwrap_or(-1.0)
        }
    }

    #[inline(always)]
    pub fn get_char_controller(&self) -> *mut bhkCharacterController {
        unsafe {
            self.middle_high
                .as_ref()
                .map(|middle_high| middle_high.char_controller.get())
                .unwrap_or(core::ptr::null_mut())
        }
    }

    #[inline(always)]
    pub fn get_commanding_actor(&self) -> ActorHandle {
        unsafe {
            self.middle_high
                .as_ref()
                .map(|middle_high| middle_high.commanding_actor)
                .unwrap_or_else(ActorHandle::new)
        }
    }

    #[inline(always)]
    pub fn get_current_shout(&self) -> *mut TESShout {
        unsafe {
            self.high
                .as_ref()
                .map(|high| high.current_shout)
                .unwrap_or(core::ptr::null_mut())
        }
    }

    crate::relocation_func! {
        pub fn get_current_weapon(&mut self, left_hand: bool) -> *mut InventoryEntryData => RelocationID::new(38781, 39806)
    }

    #[inline(always)]
    pub fn get_equipped_left_hand(&self) -> *mut TESForm {
        self.equipped_objects[AIProcessHand::Left as usize]
    }

    #[inline(always)]
    pub fn get_equipped_right_hand(&self) -> *mut TESForm {
        self.equipped_objects[AIProcessHand::Right as usize]
    }

    #[inline(always)]
    pub fn get_headtrack_target(&self) -> ObjectRefHandle {
        let mut out = ObjectRefHandle::new();
        unsafe { self.get_headtrack_target_impl(&mut out) };
        out
    }

    crate::relocation_func! {
        fn get_headtrack_target_impl(&self, out: &mut ObjectRefHandle) => RelocationID::new(38483, 39484)
    }

    #[inline(always)]
    pub fn get_is_summoned_creature(&self) -> bool {
        unsafe {
            self.middle_high
                .as_ref()
                .map(|middle_high| middle_high.summoned_creature)
                .unwrap_or(false)
        }
    }

    pub fn get_magic_node(&self, biped: *const BipedAnim) -> *mut NiAVObject {
        self.get_biped_node_by_name(biped, "NPCRMagicNode[RMag]")
    }

    #[inline(always)]
    pub fn get_occupied_furniture(&self) -> ObjectRefHandle {
        unsafe {
            self.middle_high
                .as_ref()
                .map(|middle_high| middle_high.occupied_furniture)
                .unwrap_or_else(ObjectRefHandle::new)
        }
    }

    #[inline(always)]
    pub fn get_process_level(&self) -> PROCESS_TYPE {
        self.process_level.get().unwrap_or(PROCESS_TYPE::None)
    }

    #[inline(always)]
    pub fn get_regen_delay(&self, actor_value: ActorValue) -> f32 {
        unsafe {
            if let Some(high) = self.high.as_ref() {
                return match actor_value {
                    ActorValue::Health => high.health_regen_delay,
                    ActorValue::Magicka => high.magicka_regen_delay,
                    ActorValue::Stamina => high.stamina_regen_delay,
                    _ => 0.0,
                };
            }
        }

        0.0
    }

    #[inline(always)]
    pub fn get_running_package(&self) -> *mut TESPackage {
        unsafe {
            if let Some(middle_high) = self.middle_high.as_ref() {
                let package = middle_high.run_once_package.package;
                if !package.is_null() {
                    return package;
                }
            }
        }

        self.current_package.package
    }

    pub fn get_torch_node(&self, biped: *const BipedAnim) -> *mut NiAVObject {
        self.get_biped_node_by_name(biped, "Shield")
    }

    #[inline(always)]
    pub fn get_tracked_damage(&self) -> f32 {
        self.tracked_damage
    }

    pub fn get_user_data(&self) -> *mut Actor {
        unsafe {
            let mut node = self
                .middle_high
                .as_ref()
                .map(|middle_high| middle_high.torso_node)
                .unwrap_or(core::ptr::null_mut());

            while !node.is_null() {
                let user_data = (*node).user_data;
                if !user_data.is_null() {
                    return skyrim_cast(user_data);
                }
                node = (*node).parent.cast();
            }
        }

        core::ptr::null_mut()
    }

    #[inline(always)]
    pub fn get_voice_recovery_time(&self) -> f32 {
        unsafe {
            self.high
                .as_ref()
                .map(|high| high.voice_recovery_time)
                .unwrap_or(0.0)
        }
    }

    pub fn get_weapon_node(&self, biped: *const BipedAnim) -> *mut NiAVObject {
        unsafe {
            if let Some(middle_high) = self.middle_high.as_ref() {
                if !biped.is_null() {
                    return self.get_biped_node_by_name(biped, "Weapon");
                }

                return middle_high.unk148;
            }
        }

        core::ptr::null_mut()
    }

    #[inline(always)]
    pub fn in_high_process(&self) -> bool {
        matches!(self.get_process_level(), PROCESS_TYPE::High)
    }

    #[inline(always)]
    pub fn in_middle_high_process(&self) -> bool {
        matches!(
            self.get_process_level(),
            PROCESS_TYPE::High | PROCESS_TYPE::MiddleHigh
        )
    }

    #[inline(always)]
    pub fn in_middle_low_process(&self) -> bool {
        matches!(
            self.get_process_level(),
            PROCESS_TYPE::High | PROCESS_TYPE::MiddleHigh | PROCESS_TYPE::MiddleLow
        )
    }

    #[inline(always)]
    pub fn in_low_process(&self) -> bool {
        matches!(
            self.get_process_level(),
            PROCESS_TYPE::High
                | PROCESS_TYPE::MiddleHigh
                | PROCESS_TYPE::MiddleLow
                | PROCESS_TYPE::Low
        )
    }

    #[inline(always)]
    pub fn is_arrested(&self) -> bool {
        unsafe {
            self.high
                .as_ref()
                .map(|high| high.arrested)
                .unwrap_or(false)
        }
    }

    #[inline(always)]
    pub fn is_ghost(&self) -> bool {
        unsafe {
            self.cached_values
                .as_ref()
                .map(|cached_values| cached_values.flags.all(CachedValuesFlags::ActorIsGhost))
                .unwrap_or(false)
        }
    }

    #[inline(always)]
    pub fn is_in_command_state(&self) -> bool {
        unsafe {
            self.high
                .as_ref()
                .map(|high| high.in_command_state)
                .unwrap_or(false)
        }
    }

    crate::relocation_func! {
        pub fn add_to_procedure_index_running(&mut self, actor: *mut Actor, num: u32) => RelocationID::new(38198, 39158)
    }

    crate::relocation_func! {
        pub fn clear_furniture(&mut self) => RelocationID::new(38773, 39798)
    }

    crate::relocation_func! {
        pub fn clear_muzzle_flashes(&mut self) => RelocationID::new(38495, 39504)
    }

    crate::relocation_func! {
        pub fn compute_last_time_processed(&mut self) => RelocationID::new(38158, 39116)
    }

    crate::relocation_func! {
        pub fn knock_explosion(&mut self, actor: *mut Actor, location: &NiPoint3, magnitude: f32) => RelocationID::new(38858, 39895)
    }

    crate::relocation_func! {
        pub fn knock_paralyze(&mut self, actor: *mut Actor) => RelocationID::new(38857, 39894)
    }

    #[inline(always)]
    pub fn play_idle(
        &mut self,
        actor: *mut Actor,
        idle: *mut TESIdleForm,
        target: *mut TESObjectREFR,
    ) -> bool {
        self.setup_special_idle(actor, DEFAULT_OBJECT_ACTION_IDLE, idle, true, false, target)
    }

    crate::relocation_func! {
        pub fn randomly_play_special_idles(&mut self, actor: *mut Actor) => RelocationID::new(38308, 39281)
    }

    crate::relocation_func! {
        pub fn set_actors_detection_event(&mut self, actor: *mut Actor, location: &NiPoint3, sound_level: i32, reference: *mut TESObjectREFR) => RelocationID::new(38311, 39286)
    }

    #[inline(always)]
    pub fn set_arrested(&mut self, arrested: bool) {
        unsafe {
            if let Some(high) = self.high.as_mut() {
                high.arrested = arrested;
            }
        }
    }

    #[inline(always)]
    pub fn set_cached_height(&mut self, height: f32) {
        unsafe {
            if let Some(high) = self.high.as_mut() {
                high.cached_actor_height = height;
            }
        }
    }

    crate::relocation_func! {
        pub fn set_headtrack_target(&mut self, owner: *mut Actor, target_position: &mut NiPoint3) => RelocationID::new(38850, 39887)
    }

    #[inline(always)]
    pub fn set_refraction(&mut self, refraction: f32) {
        unsafe {
            if let Some(middle_high) = self.middle_high.as_mut() {
                middle_high.script_refract_power = refraction;
            }
        }
    }

    #[inline(always)]
    pub fn set_3d_update_flag(&mut self, flags: RESET_3D_FLAGS) {
        unsafe {
            if let Some(middle_high) = self.middle_high.as_mut() {
                middle_high.update_3d_model.set(flags);
            }
        }
    }

    crate::relocation_func! {
        pub fn set_run_once_package(&mut self, package: *mut TESPackage, actor: *mut Actor) => RelocationID::new(38819, 39849)
    }

    crate::relocation_func! {
        pub fn setup_special_idle(&mut self, actor: *mut Actor, action: DEFAULT_OBJECT, idle: *mut TESIdleForm, arg5: bool, arg6: bool, target: *mut TESObjectREFR) -> bool => RelocationID::new(38290, 39256)
    }

    crate::relocation_func! {
        pub fn stop_current_idle(&mut self, actor: *mut Actor, force_idle_stop: bool) => RelocationID::new(38291, 39257)
    }

    crate::relocation_func! {
        pub fn update_3d_model_impl(&mut self, actor: *mut Actor) => RelocationID::new(38404, 39395)
    }

    #[inline(always)]
    pub fn update_3d_model(&mut self, actor: *mut Actor) {
        // CommonLib also emits SKSE::NiNodeUpdateEvent after the engine-side
        // update. libskyrim does not currently expose that SKSE event source.
        self.update_3d_model_impl(actor);
    }

    #[inline(always)]
    pub fn update_regen_delay(&mut self, actor_value: ActorValue, regen_delay: f32) {
        unsafe {
            if let Some(high) = self.high.as_mut() {
                match actor_value {
                    ActorValue::Health => high.health_regen_delay = regen_delay,
                    ActorValue::Magicka => high.magicka_regen_delay = regen_delay,
                    ActorValue::Stamina => high.stamina_regen_delay = regen_delay,
                    _ => {}
                }
            }
        }
    }

    #[inline(always)]
    pub fn set_actor_refraction(&mut self, refraction: f32) {
        if crate::runtime::is_vr() && crate::runtime::is_at_least(RUNTIME_VR_1_4_15) {
            let func: extern "C" fn(*mut Self, f32) =
                unsafe { core::mem::transmute(ID::new(5375528368usize).address()) };
            func(self, refraction);
        }
    }

    fn get_biped_node_by_name(&self, biped: *const BipedAnim, node_name: &str) -> *mut NiAVObject {
        if biped.is_null() {
            return core::ptr::null_mut();
        }

        unsafe {
            let root = (*biped).root.cast::<NiAVObject>();
            if root.is_null() {
                return core::ptr::null_mut();
            }

            let node_name = BSFixedString::from_str(node_name);
            (&mut *root).get_object_by_name(&node_name as *const _)
        }
    }
}
