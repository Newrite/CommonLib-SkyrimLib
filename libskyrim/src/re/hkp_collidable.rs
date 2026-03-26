use crate::re::cfilter::CFilter;
use crate::re::{hkpCdBody, hkpTypedBroadPhaseHandle};

/// C++ `RE::hkpCollidable::BoundingVolumeData`
#[repr(C)]
pub struct hkpCollidableBoundingVolumeData {
    pub min: [u32; 3],                             // 00
    pub expansion_min: [u8; 3],                    // 0C
    pub expansion_shift: u8,                       // 0F
    pub max: [u32; 3],                             // 10
    pub expansion_max: [u8; 3],                    // 1C
    pub pad1f: u8,                                 // 1F
    pub num_child_shape_aabbs: u16,                // 20
    pub capacity_child_shape_aabbs: u16,           // 22
    pub pad24: u32,                                // 24
    pub child_shape_aabbs: *mut core::ffi::c_void, // 28
    pub child_shape_keys: *mut u32,                // 30
}

const _: () = assert!(core::mem::size_of::<hkpCollidableBoundingVolumeData>() == 0x38);

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

/// Source-backed partial view of `RE::hkpTypedBroadPhaseHandle` exposing
/// `collisionFilterInfo`.
#[repr(C)]
pub struct hkpTypedBroadPhaseHandleCollisionView {
    pub pad00: [u8; 0x8],               // 00
    pub collision_filter_info: CFilter, // 08
}

const _: () = assert!(core::mem::size_of::<hkpTypedBroadPhaseHandleCollisionView>() == 0xC);
const _: () = assert!(
    core::mem::offset_of!(hkpTypedBroadPhaseHandleCollisionView, collision_filter_info) == 0x8
);

/// Source-backed partial view of `RE::hkpCollidable` exposing
/// `broadPhaseHandle.collisionFilterInfo`.
#[repr(C)]
pub struct hkpCollidableCollisionView {
    pub pad00: [u8; 0x24],                                         // 00
    pub broad_phase_handle: hkpTypedBroadPhaseHandleCollisionView, // 24
}

const _: () = assert!(core::mem::size_of::<hkpCollidableCollisionView>() == 0x30);
const _: () =
    assert!(core::mem::offset_of!(hkpCollidableCollisionView, broad_phase_handle) == 0x24);
