#![allow(non_camel_case_types)]

use crate::re::{ColLayer, hkAabbUint32, hkpCdBody, hkpShapeKey, hkpTypedBroadPhaseHandle};
use core_util::Enum;

/// C++ `RE::hkpCollidable::BelongsTo`
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum hkpCollidableBelongsTo {
    Terrain = 1 << 16,
}

core_util::impl_enumset_type!(hkpCollidableBelongsTo => u32);

/// C++ `RE::hkpCollidable::CollisionFilterInfo`
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum hkpCollidableCollisionFilterInfo {
    BelongsTo = 0xFFFF_0000,
    CollidesWith = 0x0000_FFFF,
}

core_util::impl_enumset_type!(hkpCollidableCollisionFilterInfo => u32);

/// C++ `RE::hkpCollidable::ForceCollideOntoPpuReasons`
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum hkpCollidableForceCollideOntoPpuReasons {
    UserRequest = 1 << 0,
    ShapeRequest = 1 << 1,
    ModifierRequest = 1 << 2,
    ShapeUnchecked = 1 << 3,
}

core_util::impl_enumset_type!(hkpCollidableForceCollideOntoPpuReasons => u32);

/// C++ `RE::hkpCollidable::BoundingVolumeData`
#[repr(C)]
pub struct hkpCollidableBoundingVolumeData {
    pub min: [u32; 3],                        // 00
    pub expansion_min: [u8; 3],               // 0C
    pub expansion_shift: u8,                  // 0F
    pub max: [u32; 3],                        // 10
    pub expansion_max: [u8; 3],               // 1C
    pub pad1f: u8,                            // 1F
    pub num_child_shape_aabbs: u16,           // 20
    pub capacity_child_shape_aabbs: u16,      // 22
    pub pad24: u32,                           // 24
    pub child_shape_aabbs: *mut hkAabbUint32, // 28
    pub child_shape_keys: *mut hkpShapeKey,   // 30
}

const _: () = assert!(core::mem::size_of::<hkpCollidableBoundingVolumeData>() == 0x38);
const _: () =
    assert!(core::mem::offset_of!(hkpCollidableBoundingVolumeData, child_shape_aabbs) == 0x28);
const _: () =
    assert!(core::mem::offset_of!(hkpCollidableBoundingVolumeData, child_shape_keys) == 0x30);

/// C++ `RE::hkpCollidable`
#[repr(C)]
pub struct hkpCollidable {
    pub base: hkpCdBody,                                       // 00
    pub owner_offset: i8,                                      // 20
    pub force_collide_onto_ppu: u8,                            // 21
    pub shape_size_on_spu: u16,                                // 22
    pub broad_phase_handle: hkpTypedBroadPhaseHandle,          // 24
    pub bounding_volume_data: hkpCollidableBoundingVolumeData, // 30
    pub allowed_penetration_depth: f32,                        // 68
    pub pad6c: u32,                                            // 6C
}

const _: () = assert!(core::mem::size_of::<hkpCollidable>() == 0x70);
const _: () = assert!(core::mem::offset_of!(hkpCollidable, base) == 0x00);
const _: () = assert!(core::mem::offset_of!(hkpCollidable, broad_phase_handle) == 0x24);
const _: () = assert!(core::mem::offset_of!(hkpCollidable, bounding_volume_data) == 0x30);
const _: () = assert!(core::mem::offset_of!(hkpCollidable, allowed_penetration_depth) == 0x68);

impl AsRef<hkpCollidable> for hkpCollidable {
    #[inline(always)]
    fn as_ref(&self) -> &hkpCollidable {
        self
    }
}

impl AsMut<hkpCollidable> for hkpCollidable {
    #[inline(always)]
    fn as_mut(&mut self) -> &mut hkpCollidable {
        self
    }
}

impl hkpCollidable {
    #[inline(always)]
    pub const fn collision_layer_storage(&self) -> Enum<ColLayer, i32> {
        self.broad_phase_handle
            .collision_filter_info
            .collision_layer_storage()
    }

    #[inline(always)]
    pub fn try_get_collision_layer(&self) -> Option<ColLayer> {
        self.collision_layer_storage().get()
    }

    #[inline(always)]
    pub fn get_collision_layer(&self) -> ColLayer {
        self.try_get_collision_layer().unwrap_or(ColLayer::Invalid)
    }

    #[inline(always)]
    pub fn get_owner(&self) -> *mut core::ffi::c_void {
        (self as *const Self as *const u8).wrapping_offset(self.owner_offset as isize)
            as *mut core::ffi::c_void
    }

    #[inline(always)]
    pub fn get_owner_as<T>(&self) -> *mut T {
        self.get_owner().cast()
    }
}

pub trait hkpCollidableExt {
    fn collision_layer_storage(&self) -> Enum<ColLayer, i32>;
    fn try_get_collision_layer(&self) -> Option<ColLayer>;
    fn get_collision_layer(&self) -> ColLayer;
    fn get_owner(&self) -> *mut core::ffi::c_void;
    fn get_owner_as<T>(&self) -> *mut T;
}

impl<T: AsRef<hkpCollidable>> hkpCollidableExt for T {
    #[inline(always)]
    fn collision_layer_storage(&self) -> Enum<ColLayer, i32> {
        hkpCollidable::collision_layer_storage(self.as_ref())
    }

    #[inline(always)]
    fn try_get_collision_layer(&self) -> Option<ColLayer> {
        hkpCollidable::try_get_collision_layer(self.as_ref())
    }

    #[inline(always)]
    fn get_collision_layer(&self) -> ColLayer {
        hkpCollidable::get_collision_layer(self.as_ref())
    }

    #[inline(always)]
    fn get_owner(&self) -> *mut core::ffi::c_void {
        hkpCollidable::get_owner(self.as_ref())
    }

    #[inline(always)]
    fn get_owner_as<U>(&self) -> *mut U {
        hkpCollidable::get_owner_as(self.as_ref())
    }
}
