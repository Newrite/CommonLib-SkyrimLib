use crate::offsets::offsets_rtti::RTTI_ProcessLists__GetActorsFilter;
use crate::offsets::offsets_vtable::VTABLE_ProcessLists__GetActorsFilter;
use crate::re::Actor;
use crate::re::ActorHandle;
use crate::re::BSTempEffect;
use crate::re::Crime;
use crate::re::ModelReferenceEffect;
use crate::re::NiObject;
use crate::re::ObjectRefHandle;
use crate::re::ShaderReferenceEffect;
use crate::re::SyncQueueObj;
use crate::re::TESObjectREFR;
use crate::re::bs_atomic::{BSSemaphore, BSSpinLock, BSSpinLockGuard};
use crate::re::bs_container::BSContainerForEachResult;
use crate::re::bssimple_list::BSSimpleList;
use crate::re::bst_array::{BSScrapArray, BSTArray};
use crate::re::bst_singleton::BSTSingletonSDM;
use crate::re::bst_smart_pointer::BSTSmartPointer;
use crate::re::crime::CrimeType;
use crate::re::ni_rtti::NiRTTI;
use crate::re::ni_smart_pointer::NiPointer;
use crate::relocation::{RelocationID, RttiType, VariantID};
use crate::{relocation_variable, virtual_method};

/// C++ `RE::ProcessLists::GetActorsFilter`
#[repr(C)]
pub struct ProcessListsGetActorsFilter {
    pub vtable: *const usize, // 00
}

const _: () = assert!(core::mem::size_of::<ProcessListsGetActorsFilter>() == 0x8);

impl RttiType for ProcessListsGetActorsFilter {
    const RTTI: VariantID = RTTI_ProcessLists__GetActorsFilter;
}

impl ProcessListsGetActorsFilter {
    pub const RTTI: VariantID = RTTI_ProcessLists__GetActorsFilter;
    pub const VTABLE: &'static [VariantID] = &VTABLE_ProcessLists__GetActorsFilter;

    virtual_method! {
        pub const VFUNC_DTOR: usize = 0x00;
        pub fn dtor()
    }

    virtual_method! {
        pub const VFUNC_IS_VALID: usize = 0x01;
        pub fn is_valid(actor: *mut Actor) -> bool
    }
}

/// C++ `RE::ProcessLists`
#[repr(C)]
pub struct ProcessLists {
    pub base: BSTSingletonSDM<ProcessLists>,              // 000
    pub run_detection: bool,                              // 001
    pub show_detection_stats: bool,                       // 002
    pub pad003: u8,                                       // 003
    pub statdetect: ActorHandle,                          // 004
    pub process_high: bool,                               // 008
    pub process_low: bool,                                // 009
    pub process_m_high: bool,                             // 00A
    pub process_m_low: bool,                              // 00B
    pub run_editor_schedules: bool,                       // 00C
    pub show_dialogue_subtitles: bool,                    // 00D
    pub unk00e: u8,                                       // 00E
    pub pad00f: u8,                                       // 00F
    pub number_high_actors: i32,                          // 010
    pub unk014: f32,                                      // 014
    pub unk018: u32,                                      // 018
    pub remove_excess_dead_timer: f32,                    // 01C
    pub movement_sync_sema: BSSemaphore,                  // 020
    pub unk028: u32,                                      // 028
    pub pad02c: u32,                                      // 02C
    pub high_actor_handles: BSTArray<ActorHandle>,        // 030
    pub low_actor_handles: BSTArray<ActorHandle>,         // 048
    pub middle_high_actor_handles: BSTArray<ActorHandle>, // 060
    pub middle_low_actor_handles: BSTArray<ActorHandle>,  // 078
    pub all_processes: [*mut BSTArray<ActorHandle>; 4],   // 090
    pub global_crimes: [*mut BSSimpleList<*mut Crime>; CrimeType::TOTAL], // 0B0
    pub global_temp_effects: BSTArray<NiPointer<BSTempEffect>>, // 0E8
    pub global_effects_lock: BSSpinLock,                  // 100
    pub magic_effects: BSTArray<NiPointer<BSTempEffect>>, // 108
    pub magic_effects_lock: BSSpinLock,                   // 120
    pub interface_effects: BSTArray<NiPointer<BSTempEffect>>, // 128
    pub interface_effects_lock: BSSpinLock,               // 140
    pub unk148: u64,                                      // 148
    pub unk150: u64,                                      // 150
    pub temp_should_moves: BSTArray<ObjectRefHandle>,     // 158
    pub alive_actor_list: BSSimpleList<ActorHandle>,      // 170
    pub init_package_locations_queue: BSTArray<ActorHandle>, // 180
    pub package_locations_queue_lock: BSSpinLock,         // 198
    pub init_anim_position_queue: BSTArray<ActorHandle>,  // 1A0
    pub sync_position_queue: BSTArray<BSTSmartPointer<SyncQueueObj>>, // 1B8
    pub player_action_comment_timer: f32,                 // 1D0
    pub player_knock_object_comment_timer: f32,           // 1D4
    pub current_low_actor: u32,                           // 1D8
    pub current_middle_high_actor: u32,                   // 1DC
    pub current_middle_low_actor: u32,                    // 1E0
    pub run_schedules: bool,                              // 1E4
    pub run_movement: bool,                               // 1E5
    pub run_animations: bool,                             // 1E6
    pub update_actors_in_player_cell: bool,               // 1E7
    pub unk1e8: u64,                                      // 1E8
}

const _: () = assert!(core::mem::size_of::<ProcessLists>() == 0x1F0);
const _: () = assert!(core::mem::offset_of!(ProcessLists, high_actor_handles) == 0x030);
const _: () = assert!(core::mem::offset_of!(ProcessLists, all_processes) == 0x090);
const _: () = assert!(core::mem::offset_of!(ProcessLists, global_crimes) == 0x0B0);
const _: () = assert!(core::mem::offset_of!(ProcessLists, magic_effects) == 0x108);
const _: () = assert!(core::mem::offset_of!(ProcessLists, magic_effects_lock) == 0x120);
const _: () = assert!(core::mem::offset_of!(ProcessLists, alive_actor_list) == 0x170);
const _: () = assert!(core::mem::offset_of!(ProcessLists, sync_position_queue) == 0x1B8);

impl ProcessLists {
    relocation_variable! {
        fn singleton() -> *mut ProcessLists => RelocationID::new(514167, 400315), is_indirect_ptr
    }

    #[inline(always)]
    pub fn get_singleton() -> *mut ProcessLists {
        Self::singleton()
    }

    crate::relocation_func! {
        pub fn are_hostile_actors_near(&mut self, array_out: *mut BSScrapArray<ActorHandle>) -> bool => RelocationID::new(40388, 41402)
    }

    crate::relocation_func! {
        pub fn clear_cached_faction_fight_reactions(&self) => RelocationID::new(40396, 41410)
    }

    pub fn for_all_actors<F>(&self, mut callback: F)
    where
        F: FnMut(*mut Actor) -> BSContainerForEachResult,
    {
        for list in self.all_processes {
            if list.is_null() {
                continue;
            }

            for actor_handle in unsafe { (*list).as_slice() } {
                let actor_handle = *actor_handle;
                let actor = actor_handle.get();
                let actor = actor.get();
                if !actor.is_null() && callback(actor) == BSContainerForEachResult::Stop {
                    return;
                }
            }
        }
    }

    pub fn for_each_high_actor<F>(&self, mut callback: F)
    where
        F: FnMut(*mut Actor) -> BSContainerForEachResult,
    {
        for actor_handle in unsafe { self.high_actor_handles.as_slice() } {
            let actor_handle = *actor_handle;
            let actor = actor_handle.get();
            let actor = actor.get();
            if !actor.is_null() && callback(actor) == BSContainerForEachResult::Stop {
                break;
            }
        }
    }

    pub fn for_each_magic_temp_effect<F>(&mut self, mut callback: F)
    where
        F: FnMut(*mut BSTempEffect) -> BSContainerForEachResult,
    {
        let _locker = BSSpinLockGuard::new(&mut self.magic_effects_lock);

        for temp_effect_ptr in unsafe { self.magic_effects.as_slice() } {
            let temp_effect = temp_effect_ptr.get();
            if !temp_effect.is_null() && callback(temp_effect) == BSContainerForEachResult::Stop {
                break;
            }
        }
    }

    pub fn for_each_model_effect<F>(&mut self, mut callback: F)
    where
        F: FnMut(*mut ModelReferenceEffect) -> BSContainerForEachResult,
    {
        let model_rtti = ModelReferenceEffect::NI_RTTI.address() as *const NiRTTI;
        self.for_each_magic_temp_effect(|temp_effect| unsafe {
            let ni_object = temp_effect.cast::<NiObject>();
            let temp_rtti = (*ni_object).get_rtti();
            if !temp_rtti.is_null() && (*temp_rtti).is_kind_of(model_rtti) {
                return callback(temp_effect.cast());
            }
            BSContainerForEachResult::Continue
        });
    }

    pub fn for_each_shader_effect<F>(&mut self, mut callback: F)
    where
        F: FnMut(*mut ShaderReferenceEffect) -> BSContainerForEachResult,
    {
        let shader_rtti = ShaderReferenceEffect::NI_RTTI.address() as *const NiRTTI;
        self.for_each_magic_temp_effect(|temp_effect| unsafe {
            let ni_object = temp_effect.cast::<NiObject>();
            let temp_rtti = (*ni_object).get_rtti();
            if !temp_rtti.is_null() && (*temp_rtti).is_kind_of(shader_rtti) {
                return callback(temp_effect.cast());
            }
            BSContainerForEachResult::Continue
        });
    }

    crate::relocation_func! {
        pub fn get_system_time_clock(&self) -> f32 => RelocationID::new(40327, 41337)
    }

    crate::relocation_func! {
        pub fn request_highest_detection_level_against_actor(&mut self, actor: *mut Actor, los_count: &mut u32) -> i16 => RelocationID::new(40394, 41408)
    }

    pub fn stop_all_magic_effects(&mut self, reference: &TESObjectREFR) {
        let handle =
            ObjectRefHandle::from_ptr(reference as *const TESObjectREFR as *mut TESObjectREFR);
        self.for_each_magic_temp_effect(|temp_effect| unsafe {
            let ni_object = temp_effect.cast::<NiObject>();
            let temp_rtti = (*ni_object).get_rtti();
            let reference_rtti =
                crate::re::ReferenceEffect::NI_RTTI.address() as *const crate::re::NiRTTI;

            if !temp_rtti.is_null() && (*temp_rtti).is_kind_of(reference_rtti) {
                let reference_effect = temp_effect.cast::<crate::re::ReferenceEffect>();
                if (*reference_effect).target == handle {
                    (*reference_effect).finished = true;
                }
            }

            BSContainerForEachResult::Continue
        });
    }

    crate::relocation_func! {
        pub fn stop_combat_and_alarm_on_actor(&mut self, actor: *mut Actor, not_alarm: bool) => RelocationID::new(40330, 41340)
    }
}
