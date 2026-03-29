use core_util::inherit;

use crate::offsets::offsets_rtti::RTTI_InventoryUpdateData;
use crate::offsets::offsets_vtable::VTABLE_InventoryUpdateData;
use crate::re::{IUIMessageData, RefHandle, TESBoundObject};
use crate::relocation::{RttiType, VariantID};

/// C++ `RE::InventoryUpdateData`
#[repr(C)]
pub struct InventoryUpdateData {
    pub base: IUIMessageData,            // 00
    pub inventory_ref: RefHandle,        // 10
    pub pad14: u32,                      // 14
    pub update_obj: *mut TESBoundObject, // 18
}

const _: () = assert!(core::mem::size_of::<InventoryUpdateData>() == 0x20);
const _: () = assert!(core::mem::offset_of!(InventoryUpdateData, base) == 0x00);
const _: () = assert!(core::mem::offset_of!(InventoryUpdateData, inventory_ref) == 0x10);
const _: () = assert!(core::mem::offset_of!(InventoryUpdateData, update_obj) == 0x18);

inherit!(InventoryUpdateData : IUIMessageData);

impl RttiType for InventoryUpdateData {
    const RTTI: VariantID = RTTI_InventoryUpdateData;
}

impl InventoryUpdateData {
    pub const RTTI: VariantID = RTTI_InventoryUpdateData;
    pub const VTABLE: &'static [VariantID] = &VTABLE_InventoryUpdateData;
    pub const CLASS_NAME: &'static str = "InventoryUpdateData";

    // override (IUIMessageData)
    // ~InventoryUpdateData() override; // 00
}
