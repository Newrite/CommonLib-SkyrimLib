#![allow(non_camel_case_types)]

use crate::re::{HitData, NiAVObject, TESObjectREFR, bhkRigidBody, hkVector4, hkpCollidable};
use crate::relocation::RelocationID;

/// C++ `namespace RE::TESHavokUtilities`
pub struct TESHavokUtilities;

impl TESHavokUtilities {
    crate::relocation_func! {
        pub fn add_explosion_impulse(
            obj_3d: *mut NiAVObject,
            pos: *mut hkVector4,
            force: f32,
            hit_data: *const HitData,
        ) => RelocationID::new(25468, 26005)
    }

    crate::relocation_func! {
        pub fn find_collidable_ref(linked_collidable: &hkpCollidable) -> *mut TESObjectREFR
            => RelocationID::new(25466, 26003)
    }

    crate::relocation_func! {
        pub fn find_collidable_object(linked_collidable: &hkpCollidable) -> *mut NiAVObject
            => RelocationID::new(15644, 15870)
    }

    crate::relocation_func! {
        pub fn get_damage_for_impact(mass: f32, speed: f32) -> f32
            => RelocationID::new(25478, 26018)
    }

    crate::relocation_func! {
        pub fn pop_temporary_mass(body: *mut bhkRigidBody) => RelocationID::new(25484, 26024)
    }

    crate::relocation_func! {
        pub fn push_temporary_mass(body: *mut bhkRigidBody, mass: f32)
            => RelocationID::new(25483, 26023)
    }

    crate::relocation_func! {
        pub fn scale_gameplay_impulse_force(
            input_force: f32,
            body: *mut bhkRigidBody,
            factor_mass: bool,
        ) -> f32 => RelocationID::new(25467, 26004)
    }
}
