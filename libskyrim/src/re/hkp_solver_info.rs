#![allow(non_camel_case_types)]

use crate::re::{hkHalf, hkVector4};

/// C++ `RE::hkpSolverInfo::DeactivationClass`
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum hkpSolverInfoDeactivationClass {
    Invalid = 0,
    Off = 1,
    Low = 2,
    Medium = 3,
    High = 4,
    Aggressive = 5,
    Total = 6,
}

impl hkpSolverInfoDeactivationClass {
    pub const TOTAL_COUNT: usize = 6;
}

/// C++ `RE::hkpSolverInfo::DeactivationInfo`
#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct hkpSolverInfoDeactivationInfo {
    pub linear_velocity_threshold_inv: f32,     // 00
    pub angular_velocity_threshold_inv: f32,    // 04
    pub slow_object_velocity_multiplier: f32,   // 08
    pub relative_sleep_velocity_threshold: f32, // 0C
    pub max_dist_sqrd: [f32; 2],                // 10
    pub max_rot_sqrd: [hkHalf; 2],              // 18
}

const _: () = assert!(core::mem::size_of::<hkpSolverInfoDeactivationInfo>() == 0x1C);
const _: () = assert!(
    core::mem::offset_of!(hkpSolverInfoDeactivationInfo, linear_velocity_threshold_inv) == 0x00
);
const _: () = assert!(
    core::mem::offset_of!(
        hkpSolverInfoDeactivationInfo,
        angular_velocity_threshold_inv
    ) == 0x04
);
const _: () = assert!(
    core::mem::offset_of!(
        hkpSolverInfoDeactivationInfo,
        slow_object_velocity_multiplier
    ) == 0x08
);
const _: () = assert!(
    core::mem::offset_of!(
        hkpSolverInfoDeactivationInfo,
        relative_sleep_velocity_threshold
    ) == 0x0C
);
const _: () = assert!(core::mem::offset_of!(hkpSolverInfoDeactivationInfo, max_dist_sqrd) == 0x10);
const _: () = assert!(core::mem::offset_of!(hkpSolverInfoDeactivationInfo, max_rot_sqrd) == 0x18);

/// C++ `RE::hkpSolverInfo`
#[repr(C, align(16))]
pub struct hkpSolverInfo {
    pub one: f32,                                    // 000
    pub tau: f32,                                    // 004
    pub damping: f32,                                // 008
    pub friction_tau: f32,                           // 00C
    pub global_acceleration_per_sub_step: hkVector4, // 010
    pub global_acceleration_per_step: hkVector4,     // 020
    pub integrate_velocity_factor: hkVector4,        // 030
    pub inv_integrate_velocity_factor: hkVector4,    // 040
    pub damp_div_tau: f32,                           // 050
    pub tau_div_damp: f32,                           // 054
    pub damp_div_friction_tau: f32,                  // 058
    pub friction_tau_div_damp: f32,                  // 05C
    pub contact_resting_velocity: f32,               // 060
    pub deactivation_info:
        [hkpSolverInfoDeactivationInfo; hkpSolverInfoDeactivationClass::TOTAL_COUNT], // 064
    pub delta_time: f32,                             // 10C
    pub inv_delta_time: f32,                         // 110
    pub num_steps: i32,                              // 114
    pub num_micro_steps: i32,                        // 118
    pub inv_num_micro_steps: f32,                    // 11C
    pub inv_num_steps: f32,                          // 120
    pub force_coherent_constraint_ordering_in_solver: bool, // 124
    pub deactivation_num_inactive_frames_select_flag: [u8; 2], // 125
    pub deactivation_integrate_counter: u8,          // 127
    pub max_constraint_violation_sqrd: f32,          // 128
    pub pad12c: u32,                                 // 12C
}

const _: () = assert!(core::mem::size_of::<hkpSolverInfo>() == 0x130);
const _: () = assert!(core::mem::offset_of!(hkpSolverInfo, one) == 0x000);
const _: () =
    assert!(core::mem::offset_of!(hkpSolverInfo, global_acceleration_per_sub_step) == 0x010);
const _: () = assert!(core::mem::offset_of!(hkpSolverInfo, deactivation_info) == 0x064);
const _: () = assert!(core::mem::offset_of!(hkpSolverInfo, delta_time) == 0x10C);
const _: () = assert!(
    core::mem::offset_of!(hkpSolverInfo, force_coherent_constraint_ordering_in_solver) == 0x124
);
const _: () = assert!(core::mem::offset_of!(hkpSolverInfo, max_constraint_violation_sqrd) == 0x128);
