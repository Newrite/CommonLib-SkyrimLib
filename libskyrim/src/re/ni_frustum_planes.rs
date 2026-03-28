use core_util::EnumSet;

use crate::re::NiPlane;

pub struct NiFrustumPlanesPlanes;

impl NiFrustumPlanesPlanes {
    pub const NEAR: usize = 0;
    pub const FAR: usize = 1;
    pub const LEFT: usize = 2;
    pub const RIGHT: usize = 3;
    pub const TOP: usize = 4;
    pub const BOTTOM: usize = 5;
    pub const TOTAL: usize = 6;
}

/// C++ `RE::NiFrustumPlanes::ActivePlane`
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum NiFrustumPlanesActivePlane {
    Near = 1 << 0,
    Far = 1 << 1,
    Left = 1 << 2,
    Right = 1 << 3,
    Top = 1 << 4,
    Bottom = 1 << 5,
}

core_util::impl_enumset_type!(NiFrustumPlanesActivePlane => u32);

/// C++ `RE::NiFrustumPlanes`
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct NiFrustumPlanes {
    pub culling_planes: [NiPlane; NiFrustumPlanesPlanes::TOTAL], // 00
    pub active_planes: EnumSet<NiFrustumPlanesActivePlane, u32>, // 60
    pub base_plane_states: u32,                                  // 64
    pub unk68: u32,                                              // 68
    pub unk6c: u32,                                              // 6C
}

const _: () = assert!(core::mem::size_of::<NiFrustumPlanes>() == 0x70);
const _: () = assert!(core::mem::offset_of!(NiFrustumPlanes, culling_planes) == 0x00);
const _: () = assert!(core::mem::offset_of!(NiFrustumPlanes, active_planes) == 0x60);
const _: () = assert!(core::mem::offset_of!(NiFrustumPlanes, base_plane_states) == 0x64);
