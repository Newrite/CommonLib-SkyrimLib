use core_util::inherit;

use crate::offsets::offsets_rtti::RTTI_ExtraDroppedItemList;
use crate::offsets::offsets_vtable::VTABLE_ExtraDroppedItemList;
use crate::re::{BSExtraData, BSSimpleList, ExtraDataType, ExtraDataTyped, ObjectRefHandle};
use crate::relocation::{RttiType, VariantID};

/// C++ `RE::ExtraDroppedItemList`
#[repr(C)]
pub struct ExtraDroppedItemList {
    pub base: BSExtraData,                                // 00
    pub dropped_item_list: BSSimpleList<ObjectRefHandle>, // 10
}

const _: () = assert!(core::mem::size_of::<ExtraDroppedItemList>() == 0x20);
const _: () = assert!(core::mem::offset_of!(ExtraDroppedItemList, dropped_item_list) == 0x10);

impl RttiType for ExtraDroppedItemList {
    const RTTI: VariantID = RTTI_ExtraDroppedItemList;
}

impl ExtraDataTyped for ExtraDroppedItemList {
    const EXTRADATATYPE: ExtraDataType = ExtraDataType::DroppedItemList;
}

inherit!(ExtraDroppedItemList : BSExtraData);

impl ExtraDroppedItemList {
    pub const RTTI: VariantID = RTTI_ExtraDroppedItemList;
    pub const VTABLE: &'static [VariantID] = &VTABLE_ExtraDroppedItemList;
    pub const EXTRADATATYPE: ExtraDataType = ExtraDataType::DroppedItemList;

    // override (BSExtraData)
    // ExtraDataType GetType() const override;  // 01 - { return kDroppedItemList; }
}
