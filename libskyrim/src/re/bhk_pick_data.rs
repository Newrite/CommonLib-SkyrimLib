use crate::re::hkp_collidable::hkpCollidableCollisionView;

/// Source-backed partial view of `RE::hkpWorldRayCastOutput` exposing
/// `rootCollidable`.
#[repr(C)]
pub struct hkpWorldRayCastOutputCollisionView {
    pub pad00: [u8; 0x50],                                  // 00
    pub root_collidable: *const hkpCollidableCollisionView, // 50
    pub pad58: u64,                                         // 58
}

const _: () = assert!(core::mem::size_of::<hkpWorldRayCastOutputCollisionView>() == 0x60);
const _: () =
    assert!(core::mem::offset_of!(hkpWorldRayCastOutputCollisionView, root_collidable) == 0x50);

/// Source-backed partial layout of `RE::bhkPickData` for projectile validation.
#[repr(C)]
pub struct bhkPickData {
    pub pad00: [u8; 0x30],                              // 00
    pub ray_output: hkpWorldRayCastOutputCollisionView, // 30
    pub pad90: [u8; 0x30],                              // 90
    pub pick_failed: bool,                              // C0
    pub padc1: u8,                                      // C1
    pub padc2: u16,                                     // C2
    pub padc4: u32,                                     // C4
    pub padc8: u32,                                     // C8
    pub padcc: u32,                                     // CC
}

const _: () = assert!(core::mem::size_of::<bhkPickData>() == 0xD0);
const _: () = assert!(core::mem::offset_of!(bhkPickData, ray_output) == 0x30);
const _: () = assert!(core::mem::offset_of!(bhkPickData, pick_failed) == 0xC0);
