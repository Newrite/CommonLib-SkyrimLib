use core_util::inherit;

use crate::offsets::offsets_rtti::RTTI_ExtraContainerChanges;
use crate::offsets::offsets_vtable::VTABLE_ExtraContainerChanges;
use crate::re::{BSExtraData, ExtraDataType, ExtraDataTyped, InventoryChanges};
use crate::relocation::{RttiType, VariantID};

/// C++ `RE::ExtraContainerChanges`
#[repr(C)]
pub struct ExtraContainerChanges {
    pub base: BSExtraData,              // 00
    pub changes: *mut InventoryChanges, // 10
}

const _: () = assert!(core::mem::size_of::<ExtraContainerChanges>() == 0x18);
const _: () = assert!(core::mem::offset_of!(ExtraContainerChanges, base) == 0x00);
const _: () = assert!(core::mem::offset_of!(ExtraContainerChanges, changes) == 0x10);

impl RttiType for ExtraContainerChanges {
    const RTTI: VariantID = RTTI_ExtraContainerChanges;
}

impl ExtraDataTyped for ExtraContainerChanges {
    const EXTRADATATYPE: ExtraDataType = ExtraDataType::ContainerChanges;
}

inherit!(ExtraContainerChanges : BSExtraData);

impl ExtraContainerChanges {
    pub const RTTI: VariantID = RTTI_ExtraContainerChanges;
    pub const VTABLE: &'static [VariantID] = &VTABLE_ExtraContainerChanges;
    pub const EXTRADATATYPE: ExtraDataType = ExtraDataType::ContainerChanges;
}
