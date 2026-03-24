use crate::re::cfilter::CFilter;

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
