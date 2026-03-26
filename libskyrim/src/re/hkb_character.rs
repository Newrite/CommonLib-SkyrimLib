#![allow(non_camel_case_types)]

use core::ffi::c_void;

use core_util::inherit;

use crate::offsets::offsets_rtti::RTTI_hkbCharacter;
use crate::offsets::offsets_vtable::VTABLE_hkbCharacter;
use crate::re::{
    hkArray, hkRefPtr, hkRefVariant, hkReferencedObject, hkStringPtr, hkbBehaviorGraph,
    hkbCharacterSetup, hkbProjectData, hkbRagdollDriver,
};
use crate::relocation::{RttiType, VariantID};

/// Partial C++ `RE::hkbCharacter`
#[repr(C)]
pub struct hkbCharacter {
    pub base: hkReferencedObject,                      // 00
    pub nearby_characters: hkArray<*mut hkbCharacter>, // 10
    pub current_lod: i16,                              // 20
    pub num_tracks_in_lod: i16,                        // 22
    pub pad24: u32,                                    // 24
    pub name: hkStringPtr,                             // 28
    pub ragdoll_driver: hkRefPtr<hkbRagdollDriver>,    // 30
    pub character_controller_driver: hkRefVariant,     // 38
    pub foot_ik_driver: hkRefVariant,                  // 40
    pub hand_ik_driver: hkRefVariant,                  // 48
    pub setup: hkRefPtr<hkbCharacterSetup>,            // 50
    pub behavior_graph: hkRefPtr<hkbBehaviorGraph>,    // 58
    pub project_data: hkRefPtr<hkbProjectData>,        // 60
    pub animation_binding_set: hkRefVariant,           // 68
    pub raycast_interface: hkRefVariant,               // 70
    pub world: hkRefVariant,                           // 78
    pub event_queue: hkRefVariant,                     // 80
    pub world_from_model: hkRefVariant,                // 88
    pub pose_local: *mut *const c_void,                // 90
    pub num_pose_local: i32,                           // 98
    pub delete_world_from_model: bool,                 // 9C
    pub delete_pose_local: bool,                       // 9D
    pub pad9e: u16,                                    // 9E
}

const _: () = assert!(core::mem::size_of::<hkbCharacter>() == 0xA0);
const _: () = assert!(core::mem::offset_of!(hkbCharacter, base) == 0x00);
const _: () = assert!(core::mem::offset_of!(hkbCharacter, nearby_characters) == 0x10);
const _: () = assert!(core::mem::offset_of!(hkbCharacter, name) == 0x28);
const _: () = assert!(core::mem::offset_of!(hkbCharacter, ragdoll_driver) == 0x30);
const _: () = assert!(core::mem::offset_of!(hkbCharacter, character_controller_driver) == 0x38);
const _: () = assert!(core::mem::offset_of!(hkbCharacter, setup) == 0x50);
const _: () = assert!(core::mem::offset_of!(hkbCharacter, behavior_graph) == 0x58);
const _: () = assert!(core::mem::offset_of!(hkbCharacter, project_data) == 0x60);
const _: () = assert!(core::mem::offset_of!(hkbCharacter, pose_local) == 0x90);
const _: () = assert!(core::mem::offset_of!(hkbCharacter, num_pose_local) == 0x98);

impl RttiType for hkbCharacter {
    const RTTI: VariantID = RTTI_hkbCharacter;
}

inherit!(hkbCharacter : hkReferencedObject, base);

impl hkbCharacter {
    pub const RTTI: VariantID = RTTI_hkbCharacter;
    pub const VTABLE: &'static [VariantID] = &VTABLE_hkbCharacter;

    crate::virtual_method! {
        pub const VFUNC_UNK_03: usize = 0x03;
        pub fn unk_03(&mut self)
    }

    crate::virtual_method! {
        pub const VFUNC_GET_NEARBY_CHARACTERS: usize = 0x04;
        pub fn get_nearby_characters(&mut self, max_distance: f32, characters: &mut hkArray<*mut hkbCharacter>)
    }
}
