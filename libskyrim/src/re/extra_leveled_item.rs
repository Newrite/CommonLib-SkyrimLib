use core_util::inherit;

use crate::offsets::offsets_rtti::RTTI_ExtraLeveledItem;
use crate::offsets::offsets_vtable::VTABLE_ExtraLeveledItem;
use crate::re::bs_core_types::FormID;
use crate::re::{BSExtraData, ExtraDataType, ExtraDataTyped};
use crate::relocation::{RttiType, VariantID};

/// C++ `RE::ExtraLeveledItem`
#[repr(C)]
pub struct ExtraLeveledItem {
    pub base: BSExtraData, // 00
    pub lev_item: FormID,  // 10
    pub pad14: u32,        // 14
}

const _: () = assert!(core::mem::size_of::<ExtraLeveledItem>() == 0x18);
const _: () = assert!(core::mem::offset_of!(ExtraLeveledItem, base) == 0x00);
const _: () = assert!(core::mem::offset_of!(ExtraLeveledItem, lev_item) == 0x10);
const _: () = assert!(core::mem::offset_of!(ExtraLeveledItem, pad14) == 0x14);

impl RttiType for ExtraLeveledItem {
    const RTTI: VariantID = RTTI_ExtraLeveledItem;
}

impl ExtraDataTyped for ExtraLeveledItem {
    const EXTRADATATYPE: ExtraDataType = ExtraDataType::LeveledItem;
}

inherit!(ExtraLeveledItem : BSExtraData);

impl ExtraLeveledItem {
    pub const RTTI: VariantID = RTTI_ExtraLeveledItem;
    pub const VTABLE: &'static [VariantID] = &VTABLE_ExtraLeveledItem;
    pub const EXTRADATATYPE: ExtraDataType = ExtraDataType::LeveledItem;

    // override (BSExtraData)
    // ExtraDataType GetType() const override;  // 01 - { return kLeveledItem; }

    #[inline(always)]
    pub fn new(lev_item: FormID) -> Self {
        Self {
            base: BSExtraData {
                vtable: Self::VTABLE[0].address() as *const usize,
                next: core::ptr::null_mut(),
            },
            lev_item,
            pad14: 0,
        }
    }
}
