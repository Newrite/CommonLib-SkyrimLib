use core_util::inherit;

use crate::offsets::offsets_rtti::RTTI_LoadingMenuData;
use crate::offsets::offsets_vtable::VTABLE_LoadingMenuData;
use crate::re::{BGSLocation, IUIMessageData};
use crate::relocation::{RttiType, VariantID};

/// C++ `RE::LoadingMenuData`
#[repr(C)]
pub struct LoadingMenuData {
    pub base: IUIMessageData,               // 00
    pub current_location: *mut BGSLocation, // 10
    pub unk18: bool,                        // 18
    pub pad19: u8,                          // 19
    pub pad1a: u16,                         // 1A
}

const _: () = assert!(core::mem::size_of::<LoadingMenuData>() == 0x20);
const _: () = assert!(core::mem::offset_of!(LoadingMenuData, base) == 0x00);
const _: () = assert!(core::mem::offset_of!(LoadingMenuData, current_location) == 0x10);
const _: () = assert!(core::mem::offset_of!(LoadingMenuData, unk18) == 0x18);

inherit!(LoadingMenuData : IUIMessageData);

impl RttiType for LoadingMenuData {
    const RTTI: VariantID = RTTI_LoadingMenuData;
}

impl LoadingMenuData {
    pub const RTTI: VariantID = RTTI_LoadingMenuData;
    pub const VTABLE: &'static [VariantID] = &VTABLE_LoadingMenuData;
    pub const CLASS_NAME: &'static str = "LoadingMenuData";

    // override (IUIMessageData)
    // ~LoadingMenuData() override; // 00
}
