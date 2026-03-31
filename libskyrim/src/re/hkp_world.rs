#![allow(non_camel_case_types)]

use crate::core_util::{EnumSet, inherit};
use crate::offsets::offsets_rtti::RTTI_hkpWorld;
use crate::offsets::offsets_vtable::VTABLE_hkpWorld;
use crate::re::{
    hkArray, hkMultiThreadCheck, hkReferencedObject, hkStepInfo, hkVector4,
    hkWorldMemoryAvailableWatchDog, hkpBroadPhase, hkpCollidable, hkpCollisionFilter,
    hkpConvexListFilter, hkpLinearCastInput, hkpRigidBody, hkpSolverInfo,
    hkpWorldCinfoContactPointGeneration, hkpWorldCinfoSimulationType, hkpWorldRayCastInput,
    hkpWorldRayCastOutput,
};
use crate::relocation::{RelocationID, RttiType, VariantID};

crate::core_util::abstract_type! {
    pub type hkCriticalSection;
    pub type hkdWorld;
    pub type hknpWorld;
    pub type hkpActionListener;
    pub type hkpBroadPhaseBorder;
    pub type hkpBroadPhaseBorderListener;
    pub type hkpCdPointCollector;
    pub type hkpCollisionDispatcher;
    pub type hkpConstraintListener;
    pub type hkpContactImpulseLimitBreachedListener;
    pub type hkpContactListener;
    pub type hkpDebugInfoOnPendingOperationQueues;
    pub type hkpEntityEntityBroadPhaseListener;
    pub type hkpEntityListener;
    pub type hkpIslandActivationListener;
    pub type hkpIslandPostCollideListener;
    pub type hkpIslandPostIntegrateListener;
    pub type hkpMtThreadStructure;
    pub type hkpPhantom;
    pub type hkpPhantomBroadPhaseListener;
    pub type hkpPhantomListener;
    pub type hkpProcessCollisionInput;
    pub type hkpSimulation;
    pub type hkpSimulationIsland;
    pub type hkpTreeWorldManager;
    pub type hkpTypedBroadPhaseDispatcher;
    pub type hkpViolatedConstraintArray;
    pub type hkpWorldDeletionListener;
    pub type hkpWorldExtension;
    pub type hkpWorldMaintenanceMgr;
    pub type hkpWorldOperationQueue;
    pub type hkpWorldPostCollideListener;
    pub type hkpWorldPostIntegrateListener;
    pub type hkpWorldPostSimulationListener;
}

/// C++ `RE::hkpWorldDynamicsStepInfo`
#[repr(C)]
pub struct hkpWorldDynamicsStepInfo {
    pub step_info: hkStepInfo,      // 000
    pub solver_info: hkpSolverInfo, // 010
}

const _: () = assert!(core::mem::size_of::<hkpWorldDynamicsStepInfo>() == 0x140);
const _: () = assert!(core::mem::offset_of!(hkpWorldDynamicsStepInfo, step_info) == 0x000);
const _: () = assert!(core::mem::offset_of!(hkpWorldDynamicsStepInfo, solver_info) == 0x010);

/// C++ `RE::hkpWorld`
#[repr(C)]
pub struct hkpWorld {
    pub base: hkReferencedObject,                                     // 000
    pub simulation: *mut hkpSimulation,                               // 010
    pub pad018: u64,                                                  // 018
    pub gravity: hkVector4,                                           // 020
    pub fixed_island: *mut hkpSimulationIsland,                       // 030
    pub fixed_rigid_body: *mut hkpRigidBody,                          // 038
    pub active_simulation_islands: hkArray<*mut hkpSimulationIsland>, // 040
    pub inactive_simulation_islands: hkArray<*mut hkpSimulationIsland>, // 050
    pub dirty_simulation_islands: hkArray<*mut hkpSimulationIsland>,  // 060
    pub maintenance_mgr: *mut hkpWorldMaintenanceMgr,                 // 070
    // TODO: SOURCE - replace this raw pointer stand-in with the header's
    // `hkRefPtr<hkWorldMemoryAvailableWatchDog>` once the watchdog ownership
    // surface is translated.
    pub memory_watch_dog: *mut hkWorldMemoryAvailableWatchDog, // 078
    pub assert_on_running_out_of_solver_memory: bool,          // 080
    pub pad081: u8,                                            // 081
    pub pad082: u16,                                           // 082
    pub pad084: u32,                                           // 084
    pub broad_phase: *mut hkpBroadPhase,                       // 088
    pub kd_tree_manager: *mut hkpTreeWorldManager,             // 090
    pub auto_update_tree: bool,                                // 098
    pub pad099: u8,                                            // 099
    pub pad09a: u16,                                           // 09A
    pub pad09c: u32,                                           // 09C
    pub broad_phase_dispatcher: *mut hkpTypedBroadPhaseDispatcher, // 0A0
    pub phantom_broad_phase_listener: *mut hkpPhantomBroadPhaseListener, // 0A8
    pub entity_entity_broad_phase_listener: *mut hkpEntityEntityBroadPhaseListener, // 0B0
    pub broad_phase_border_listener: *mut hkpBroadPhaseBorderListener, // 0B8
    pub multithreaded_simulation_job_data: *mut hkpMtThreadStructure, // 0C0
    pub collision_input: *mut hkpProcessCollisionInput,        // 0C8
    pub collision_filter: *mut hkpCollisionFilter,             // 0D0
    pub collision_dispatcher: *mut hkpCollisionDispatcher,     // 0D8
    pub convex_list_filter: *mut hkpConvexListFilter,          // 0E0
    pub pending_operations: *mut hkpWorldOperationQueue,       // 0E8
    pub pending_operations_count: i32,                         // 0F0
    pub pending_body_operations_count: i32,                    // 0F4
    pub critical_operations_lock_count: i32,                   // 0F8
    pub critical_operations_lock_count_for_phantoms: i32,      // 0FC
    pub block_executing_pending_operations: bool,              // 100
    pub critical_operations_allowed: bool,                     // 101
    pub pad102: u16,                                           // 102
    pub pad104: u32,                                           // 104
    pub pending_operation_queues: *mut hkpDebugInfoOnPendingOperationQueues, // 108
    pub pending_operation_queue_count: i32,                    // 110
    pub multi_thread_check: hkMultiThreadCheck,                // 114
    pub process_actions_in_single_thread: bool,                // 120
    pub allow_integration_of_islands_without_constraints_in_a_separate_job: bool, // 121
    pub pad122: u16,                                           // 122
    pub min_desired_island_size: u32,                          // 124
    pub modify_constraint_critical_section: *mut hkCriticalSection, // 128
    pub is_locked: i32,                                        // 130
    pub pad134: u32,                                           // 134
    pub island_dirty_list_critical_section: *mut hkCriticalSection, // 138
    pub property_master_lock: *mut hkCriticalSection,          // 140
    pub want_simulation_islands: bool,                         // 148
    pub use_hybrid_broadphase: bool,                           // 149
    pub pad14a: u16,                                           // 14A
    pub snap_collision_to_convex_edge_threshold: f32,          // 14C
    pub snap_collision_to_concave_edge_threshold: f32,         // 150
    pub enable_toi_weld_rejection: bool,                       // 154
    pub want_deactivation: bool,                               // 155
    pub should_activate_on_rigid_body_transform_change: bool,  // 156
    pub pad157: u8,                                            // 157
    pub deactivation_reference_distance: f32,                  // 158
    pub toi_collision_response_rotate_normal: f32,             // 15C
    pub max_sectors_per_midphase_collide_task: i32,            // 160
    pub max_sectors_per_narrowphase_collide_task: i32,         // 164
    pub process_tois_multithreaded: bool,                      // 168
    pub pad169: u8,                                            // 169
    pub pad16a: u16,                                           // 16A
    pub max_entries_per_toi_midphase_collide_task: i32,        // 16C
    pub max_entries_per_toi_narrowphase_collide_task: i32,     // 170
    pub max_num_toi_collision_pairs_singlethreaded: i32,       // 174
    pub simulation_type: EnumSet<hkpWorldCinfoSimulationType, u8>, // 178
    pub pad179: u8,                                            // 179
    pub pad17a: u16,                                           // 17A
    pub num_tois_till_allowed_penetration_simplified_toi: f32, // 17C
    pub num_tois_till_allowed_penetration_toi: f32,            // 180
    pub num_tois_till_allowed_penetration_toi_higher: f32,     // 184
    pub num_tois_till_allowed_penetration_toi_forced: f32,     // 188
    pub last_entity_uid: u32,                                  // 18C
    pub last_island_uid: u32,                                  // 190
    pub last_constraint_uid: u32,                              // 194
    pub phantoms: hkArray<*mut hkpPhantom>,                    // 198
    pub action_listeners: hkArray<*mut hkpActionListener>,     // 1A8
    pub entity_listeners: hkArray<*mut hkpEntityListener>,     // 1B8
    pub phantom_listeners: hkArray<*mut hkpPhantomListener>,   // 1C8
    pub constraint_listeners: hkArray<*mut hkpConstraintListener>, // 1D8
    pub world_deletion_listeners: hkArray<*mut hkpWorldDeletionListener>, // 1E8
    pub island_activation_listeners: hkArray<*mut hkpIslandActivationListener>, // 1F8
    pub world_post_simulation_listeners: hkArray<*mut hkpWorldPostSimulationListener>, // 208
    pub world_post_integrate_listeners: hkArray<*mut hkpWorldPostIntegrateListener>, // 218
    pub world_post_collide_listeners: hkArray<*mut hkpWorldPostCollideListener>, // 228
    pub island_post_integrate_listeners: hkArray<*mut hkpIslandPostIntegrateListener>, // 238
    pub island_post_collide_listeners: hkArray<*mut hkpIslandPostCollideListener>, // 248
    pub contact_listeners: hkArray<*mut hkpContactListener>,   // 258
    pub contact_impulse_limit_breached_listeners:
        hkArray<*mut hkpContactImpulseLimitBreachedListener>, // 268
    pub world_extensions: hkArray<*mut hkpWorldExtension>,     // 278
    pub violated_constraint_array: *mut hkpViolatedConstraintArray, // 288
    pub broad_phase_border: *mut hkpBroadPhaseBorder,          // 290
    pub destruction_world: *mut hkdWorld,                      // 298
    pub np_world: *mut hknpWorld,                              // 2A0
    pub pad2a8: u64,                                           // 2A8
    pub dynamics_step_info: hkpWorldDynamicsStepInfo,          // 2B0
    pub broad_phase_extents: [hkVector4; 2],                   // 3F0
    pub broad_phase_num_markers: i32,                          // 410
    pub size_of_toi_event_queue: i32,                          // 414
    pub broad_phase_query_size: i32,                           // 418
    pub broad_phase_update_size: i32,                          // 41C
    pub contact_point_generation: EnumSet<hkpWorldCinfoContactPointGeneration, u8>, // 420
    pub pad421: u8,                                            // 421
    pub pad422: u16,                                           // 422
    pub pad424: u32,                                           // 424
    pub pad428: u64,                                           // 428
}

const _: () = assert!(core::mem::size_of::<hkpWorld>() == 0x430);
const _: () = assert!(core::mem::offset_of!(hkpWorld, base) == 0x000);
const _: () = assert!(core::mem::offset_of!(hkpWorld, simulation) == 0x010);
const _: () = assert!(core::mem::offset_of!(hkpWorld, gravity) == 0x020);
const _: () = assert!(core::mem::offset_of!(hkpWorld, maintenance_mgr) == 0x070);
const _: () = assert!(core::mem::offset_of!(hkpWorld, memory_watch_dog) == 0x078);
const _: () = assert!(core::mem::offset_of!(hkpWorld, broad_phase) == 0x088);
const _: () = assert!(core::mem::offset_of!(hkpWorld, pending_operations) == 0x0E8);
const _: () = assert!(core::mem::offset_of!(hkpWorld, multi_thread_check) == 0x114);
const _: () = assert!(core::mem::offset_of!(hkpWorld, simulation_type) == 0x178);
const _: () = assert!(core::mem::offset_of!(hkpWorld, phantoms) == 0x198);
const _: () = assert!(core::mem::offset_of!(hkpWorld, dynamics_step_info) == 0x2B0);
const _: () = assert!(core::mem::offset_of!(hkpWorld, broad_phase_extents) == 0x3F0);
const _: () = assert!(core::mem::offset_of!(hkpWorld, contact_point_generation) == 0x420);

impl RttiType for hkpWorld {
    const RTTI: VariantID = RTTI_hkpWorld;
}

impl AsRef<hkpWorld> for hkpWorld {
    #[inline(always)]
    fn as_ref(&self) -> &Self {
        self
    }
}

impl AsMut<hkpWorld> for hkpWorld {
    #[inline(always)]
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

inherit!(hkpWorld : hkReferencedObject, base);

impl hkpWorld {
    pub const RTTI: VariantID = RTTI_hkpWorld;
    pub const VTABLE: &'static [VariantID] = &VTABLE_hkpWorld;

    crate::relocation_func! {
        pub fn add_phantom(&mut self, phantom: *mut hkpPhantom) -> *mut hkpPhantom => RelocationID::new(60502, 61314)
    }

    crate::relocation_func! {
        pub fn remove_phantom(&mut self, phantom: *mut hkpPhantom) => RelocationID::new(60504, 61316)
    }

    crate::relocation_func! {
        pub fn cast_ray(&self, input: &hkpWorldRayCastInput, output: &mut hkpWorldRayCastOutput) => RelocationID::new(60551, 61399)
    }

    crate::relocation_func! {
        pub fn linear_cast(
            &self,
            col_a: *const hkpCollidable,
            input: &hkpLinearCastInput,
            cast_collector: &mut hkpCdPointCollector,
            start_collector: *mut hkpCdPointCollector,
        ) => RelocationID::new(60554, 61402)
    }
}

pub trait hkpWorldExt {
    fn add_phantom(&mut self, phantom: *mut hkpPhantom) -> *mut hkpPhantom;
    fn remove_phantom(&mut self, phantom: *mut hkpPhantom);
    fn cast_ray(&self, input: &hkpWorldRayCastInput, output: &mut hkpWorldRayCastOutput);
    fn linear_cast(
        &self,
        col_a: *const hkpCollidable,
        input: &hkpLinearCastInput,
        cast_collector: &mut hkpCdPointCollector,
        start_collector: *mut hkpCdPointCollector,
    );
}

impl<T> hkpWorldExt for T
where
    T: AsRef<hkpWorld> + AsMut<hkpWorld>,
{
    #[inline(always)]
    fn add_phantom(&mut self, phantom: *mut hkpPhantom) -> *mut hkpPhantom {
        hkpWorld::add_phantom(self.as_mut(), phantom)
    }

    #[inline(always)]
    fn remove_phantom(&mut self, phantom: *mut hkpPhantom) {
        hkpWorld::remove_phantom(self.as_mut(), phantom)
    }

    #[inline(always)]
    fn cast_ray(&self, input: &hkpWorldRayCastInput, output: &mut hkpWorldRayCastOutput) {
        hkpWorld::cast_ray(self.as_ref(), input, output)
    }

    #[inline(always)]
    fn linear_cast(
        &self,
        col_a: *const hkpCollidable,
        input: &hkpLinearCastInput,
        cast_collector: &mut hkpCdPointCollector,
        start_collector: *mut hkpCdPointCollector,
    ) {
        hkpWorld::linear_cast(self.as_ref(), col_a, input, cast_collector, start_collector)
    }
}
