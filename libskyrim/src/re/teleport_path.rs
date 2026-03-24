use crate::re::NiPoint3;
use crate::re::TESObjectCELL;
use crate::re::TESObjectREFR;
use crate::re::TESWorldSpace;
use crate::re::bst_array::BSTArray;

/// C++ `RE::TeleportPath::ParentSpaceNode`
#[repr(C)]
pub struct ParentSpaceNode {
    pub is_worldspace: bool,               // 00
    pub pad01: [u8; 7],                    // 01
    pub worldspace: *mut TESWorldSpace,    // 08
    pub interior_cell: *mut TESObjectCELL, // 10
}

const _: () = assert!(core::mem::size_of::<ParentSpaceNode>() == 0x18);
const _: () = assert!(core::mem::offset_of!(ParentSpaceNode, is_worldspace) == 0x00);
const _: () = assert!(core::mem::offset_of!(ParentSpaceNode, worldspace) == 0x08);
const _: () = assert!(core::mem::offset_of!(ParentSpaceNode, interior_cell) == 0x10);

/// C++ `RE::TeleportPath::TeleportLink`
#[repr(C)]
pub struct TeleportLink {
    pub refr: *mut TESObjectREFR,    // 00
    pub teleport_location: NiPoint3, // 08
}

const _: () = assert!(core::mem::size_of::<TeleportLink>() == 0x18);
const _: () = assert!(core::mem::offset_of!(TeleportLink, refr) == 0x00);
const _: () = assert!(core::mem::offset_of!(TeleportLink, teleport_location) == 0x08);

/// C++ `RE::TeleportPath`
#[repr(C)]
pub struct TeleportPath {
    pub spaces: BSTArray<ParentSpaceNode>,     // 00
    pub teleport_refs: BSTArray<TeleportLink>, // 18
    pub start: NiPoint3,                       // 30
    pub end: NiPoint3,                         // 3C
}

const _: () = assert!(core::mem::size_of::<TeleportPath>() == 0x48);
const _: () = assert!(core::mem::offset_of!(TeleportPath, spaces) == 0x00);
const _: () = assert!(core::mem::offset_of!(TeleportPath, teleport_refs) == 0x18);
const _: () = assert!(core::mem::offset_of!(TeleportPath, start) == 0x30);
const _: () = assert!(core::mem::offset_of!(TeleportPath, end) == 0x3C);
