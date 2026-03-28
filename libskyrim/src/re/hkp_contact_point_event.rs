#![allow(non_camel_case_types)]

use core_util::inherit;

use crate::re::hkp_shape::hkpShapeKey;
use crate::re::{
    hkContactPoint, hkpCollisionEvent, hkpContactPointProperties, hkpVelocityAccumulator,
};

/// C++ `RE::hkpContactPointEvent::Type`
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum hkpContactPointEventType {
    kTOI = 0,
    kExpandManifold = 1,
    kManifold = 2,
    kManifoldAtEndOfStep = 3,
    kManifoldFromSavedContactPoint = 4,
}

/// C++ `RE::hkpContactPointEvent`
#[repr(C)]
pub struct hkpContactPointEvent {
    pub base: hkpCollisionEvent,                                  // 00
    pub type_: hkpContactPointEventType,                          // 20
    pub pad24: u32,                                               // 24
    pub contact_point: *mut hkContactPoint,                       // 28
    pub contact_point_properties: *mut hkpContactPointProperties, // 30
    pub firing_callbacks_for_full_manifold: bool,                 // 38
    pub first_callback_for_full_manifold: bool,                   // 39
    pub last_callback_for_full_manifold: bool,                    // 3A
    pub pad3b: u8,                                                // 3B
    pub pad3c: u32,                                               // 3C
    pub separating_velocity: *mut f32,                            // 40
    pub rotate_normal: *mut f32,                                  // 48
    pub shape_key_storage: *mut hkpShapeKey,                      // 50
    pub accumulators: [*mut hkpVelocityAccumulator; 2],           // 58
}

const _: () = assert!(core::mem::size_of::<hkpContactPointEvent>() == 0x68);

inherit!(hkpContactPointEvent : hkpCollisionEvent, base);

impl hkpContactPointEvent {
    #[inline(always)]
    pub fn get_shape_keys(&self, body_idx: u32) -> *mut hkpShapeKey {
        if body_idx <= 1 {
            let body = self.base.bodies[body_idx as usize];
            if !body.is_null() && unsafe { (&*body).num_shape_keys_in_contact_point_properties } > 0
            {
                let count =
                    unsafe { (&*self.base.bodies[0]).num_shape_keys_in_contact_point_properties }
                        as usize;
                return unsafe { self.shape_key_storage.add(body_idx as usize * count) };
            }
        }
        core::ptr::null_mut()
    }
}
