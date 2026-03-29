use core_util::inherit;

use crate::offsets::offsets_rtti::RTTI_RefHandleUIData;
use crate::offsets::offsets_vtable::VTABLE_RefHandleUIData;
use crate::re::{IUIMessageData, RefHandle};
use crate::relocation::{RttiType, VariantID};

/// C++ `RE::RefHandleUIData`
#[repr(C)]
pub struct RefHandleUIData {
    pub base: IUIMessageData, // 00
    pub data: RefHandle,      // 10
    pub pad14: u32,           // 14
}

const _: () = assert!(core::mem::size_of::<RefHandleUIData>() == 0x18);
const _: () = assert!(core::mem::offset_of!(RefHandleUIData, base) == 0x00);
const _: () = assert!(core::mem::offset_of!(RefHandleUIData, data) == 0x10);

inherit!(RefHandleUIData : IUIMessageData);

impl RttiType for RefHandleUIData {
    const RTTI: VariantID = RTTI_RefHandleUIData;
}

impl RefHandleUIData {
    pub const RTTI: VariantID = RTTI_RefHandleUIData;
    pub const VTABLE: &'static [VariantID] = &VTABLE_RefHandleUIData;
    pub const CLASS_NAME: &'static str = "RefHandleUIData";

    // override (IUIMessageData)
    // ~RefHandleUIData() override; // 00
}
