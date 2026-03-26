#![allow(non_camel_case_types)]

use crate::re::{hkpRigidBody, hkpSimpleConstraintContactMgr};

/// C++ `RE::hkpCollisionEvent::CallbackSource`
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum hkpCollisionEventCallbackSource {
    kSourceA = 0,
    kSourceB = 1,
    kSourceWorld = 2,
}

/// C++ `RE::hkpCollisionEvent`
#[repr(C)]
pub struct hkpCollisionEvent {
    pub source: hkpCollisionEventCallbackSource,         // 00
    pub pad04: u32,                                      // 04
    pub bodies: [*mut hkpRigidBody; 2],                  // 08
    pub contact_mgr: *mut hkpSimpleConstraintContactMgr, // 18
}

const _: () = assert!(core::mem::size_of::<hkpCollisionEvent>() == 0x20);

impl hkpCollisionEvent {
    #[inline(always)]
    pub fn new(
        source: hkpCollisionEventCallbackSource,
        body_a: *mut hkpRigidBody,
        body_b: *mut hkpRigidBody,
        mgr: *mut hkpSimpleConstraintContactMgr,
    ) -> Self {
        Self {
            source,
            pad04: 0,
            bodies: [body_a, body_b],
            contact_mgr: mgr,
        }
    }
}
