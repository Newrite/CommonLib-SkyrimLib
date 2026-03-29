use core_util::inherit;

use crate::offsets::offsets_rtti::RTTI_TESFormUIData;
use crate::offsets::offsets_vtable::VTABLE_TESFormUIData;
use crate::re::{IUIMessageData, TESForm};
use crate::relocation::{RttiType, VariantID};

/// C++ `RE::TESFormUIData`
#[repr(C)]
pub struct TESFormUIData {
    pub base: IUIMessageData, // 00
    pub data: *mut TESForm,   // 10
}

const _: () = assert!(core::mem::size_of::<TESFormUIData>() == 0x18);
const _: () = assert!(core::mem::offset_of!(TESFormUIData, base) == 0x00);
const _: () = assert!(core::mem::offset_of!(TESFormUIData, data) == 0x10);

inherit!(TESFormUIData : IUIMessageData);

impl RttiType for TESFormUIData {
    const RTTI: VariantID = RTTI_TESFormUIData;
}

impl TESFormUIData {
    pub const RTTI: VariantID = RTTI_TESFormUIData;
    pub const VTABLE: &'static [VariantID] = &VTABLE_TESFormUIData;
    pub const CLASS_NAME: &'static str = "TESFormUIData";

    // override (IUIMessageData)
    // ~TESFormUIData() override; // 00
}
