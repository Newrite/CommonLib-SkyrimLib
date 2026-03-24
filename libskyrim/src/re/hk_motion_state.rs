use crate::re::{hkHalf, hkSweptTransform, hkTransform, hkUFloat8, hkVector4};

/// C++ `RE::hkMotionState`
#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct hkMotionState {
    pub transform: hkTransform,            // 00
    pub swept_transform: hkSweptTransform, // 40
    pub delta_angle: hkVector4,            // 90
    pub object_radius: f32,                // A0
    pub linear_damping: hkHalf,            // A4
    pub angular_damping: hkHalf,           // A6
    pub time_factor: hkHalf,               // A8
    pub max_linear_velocity: hkUFloat8,    // AA
    pub max_angular_velocity: hkUFloat8,   // AB
    pub deactivation_class: u8,            // AC
    pub padad: u8,                         // AD
    pub padae: u16,                        // AE
}

const _: () = assert!(core::mem::size_of::<hkMotionState>() == 0xB0);
const _: () = assert!(core::mem::offset_of!(hkMotionState, transform) == 0x00);
const _: () = assert!(core::mem::offset_of!(hkMotionState, swept_transform) == 0x40);
const _: () = assert!(core::mem::offset_of!(hkMotionState, delta_angle) == 0x90);
const _: () = assert!(core::mem::offset_of!(hkMotionState, object_radius) == 0xA0);
const _: () = assert!(core::mem::offset_of!(hkMotionState, linear_damping) == 0xA4);
const _: () = assert!(core::mem::offset_of!(hkMotionState, angular_damping) == 0xA6);
const _: () = assert!(core::mem::offset_of!(hkMotionState, time_factor) == 0xA8);
const _: () = assert!(core::mem::offset_of!(hkMotionState, max_linear_velocity) == 0xAA);
const _: () = assert!(core::mem::offset_of!(hkMotionState, max_angular_velocity) == 0xAB);
const _: () = assert!(core::mem::offset_of!(hkMotionState, deactivation_class) == 0xAC);
