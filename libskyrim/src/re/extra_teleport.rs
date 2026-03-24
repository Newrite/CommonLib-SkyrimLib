use core_util::inherit;

use crate::offsets::offsets_rtti::RTTI_ExtraTeleport;
use crate::offsets::offsets_vtable::VTABLE_ExtraTeleport;
use crate::re::{BSExtraData, ExtraDataType, ExtraDataTyped, NiPoint3, ObjectRefHandle};
use crate::relocation::{RttiType, VariantID};

/// C++ `RE::DoorTeleportData`
#[repr(C)]
pub struct DoorTeleportData {
    pub linked_door: ObjectRefHandle, // 00
    pub position: NiPoint3,           // 04
    pub rotation: NiPoint3,           // 10
    pub flags: i8,                    // 1C
    pub pad1d: u8,                    // 1D
    pub pad1e: u16,                   // 1E
}

const _: () = assert!(core::mem::size_of::<DoorTeleportData>() == 0x20);

/// C++ `RE::ExtraTeleport`
#[repr(C)]
pub struct ExtraTeleport {
    pub base: BSExtraData,                    // 00
    pub teleport_data: *mut DoorTeleportData, // 10
}

const _: () = assert!(core::mem::size_of::<ExtraTeleport>() == 0x18);
const _: () = assert!(core::mem::offset_of!(ExtraTeleport, teleport_data) == 0x10);

impl RttiType for ExtraTeleport {
    const RTTI: VariantID = RTTI_ExtraTeleport;
}

impl ExtraDataTyped for ExtraTeleport {
    const EXTRADATATYPE: ExtraDataType = ExtraDataType::Teleport;
}

inherit!(ExtraTeleport : BSExtraData);

impl ExtraTeleport {
    pub const RTTI: VariantID = RTTI_ExtraTeleport;
    pub const VTABLE: &'static [VariantID] = &VTABLE_ExtraTeleport;
    pub const EXTRADATATYPE: ExtraDataType = ExtraDataType::Teleport;
}
