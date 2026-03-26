#![allow(non_camel_case_types)]

use crate::re::hkArray;
use crate::re::hkp_collidable::hkpCollidable as HavokCollidable;

/// C++ `RE::hkpLinkedCollidable::CollisionEntry`
#[repr(C)]
pub struct hkpLinkedCollidableCollisionEntry {
    pub agent_entry: *mut core::ffi::c_void, // 00
    pub partner: *mut hkpLinkedCollidable,   // 08
}

const _: () = assert!(core::mem::size_of::<hkpLinkedCollidableCollisionEntry>() == 0x10);

/// C++ `RE::hkpLinkedCollidable`
#[repr(C)]
pub struct hkpLinkedCollidable {
    pub base: HavokCollidable,                                         // 00
    pub collision_entries: hkArray<hkpLinkedCollidableCollisionEntry>, // 70
}

const _: () = assert!(core::mem::size_of::<hkpLinkedCollidable>() == 0x80);
