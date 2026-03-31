use core_util::inherit;

use crate::offsets::offsets_rtti::RTTI_ExtraNavMeshPortal;
use crate::offsets::offsets_vtable::VTABLE_ExtraNavMeshPortal;
use crate::re::{BSExtraData, ExtraDataType, ExtraDataTyped, FormID, NavMesh};
use crate::relocation::{RttiType, VariantID};

/// C++ `RE::NAVMESH_PORTAL::Nav`
#[repr(C)]
#[derive(Clone, Copy)]
pub union NAVMESH_PORTALNav {
    pub nav_mesh_id: FormID,    // 00
    pub nav_mesh: *mut NavMesh, // 00
}

const _: () = assert!(core::mem::size_of::<NAVMESH_PORTALNav>() == 0x8);

/// C++ `RE::NAVMESH_PORTAL`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct NAVMESH_PORTAL {
    pub nav: NAVMESH_PORTALNav, // 00
    pub tri_index: u16,         // 08
    pub pad0a: u16,             // 0A
    pub pad0c: u32,             // 0C
}

const _: () = assert!(core::mem::size_of::<NAVMESH_PORTAL>() == 0x10);
const _: () = assert!(core::mem::offset_of!(NAVMESH_PORTAL, nav) == 0x00);
const _: () = assert!(core::mem::offset_of!(NAVMESH_PORTAL, tri_index) == 0x08);
const _: () = assert!(core::mem::offset_of!(NAVMESH_PORTAL, pad0a) == 0x0A);
const _: () = assert!(core::mem::offset_of!(NAVMESH_PORTAL, pad0c) == 0x0C);

/// C++ `RE::ExtraNavMeshPortal`
#[repr(C)]
pub struct ExtraNavMeshPortal {
    pub base: BSExtraData,      // 00
    pub portal: NAVMESH_PORTAL, // 10
}

const _: () = assert!(core::mem::size_of::<ExtraNavMeshPortal>() == 0x20);
const _: () = assert!(core::mem::offset_of!(ExtraNavMeshPortal, base) == 0x00);
const _: () = assert!(core::mem::offset_of!(ExtraNavMeshPortal, portal) == 0x10);

impl RttiType for ExtraNavMeshPortal {
    const RTTI: VariantID = RTTI_ExtraNavMeshPortal;
}

impl ExtraDataTyped for ExtraNavMeshPortal {
    const EXTRADATATYPE: ExtraDataType = ExtraDataType::NavMeshPortal;
}

inherit!(ExtraNavMeshPortal : BSExtraData, base);

impl ExtraNavMeshPortal {
    pub const RTTI: VariantID = RTTI_ExtraNavMeshPortal;
    pub const VTABLE: &'static [VariantID] = &VTABLE_ExtraNavMeshPortal;
    pub const EXTRADATATYPE: ExtraDataType = ExtraDataType::NavMeshPortal;

    // override (BSExtraData)
    // ExtraDataType GetType() const override;  // 01 - { return kNavMeshPortal; }
}
