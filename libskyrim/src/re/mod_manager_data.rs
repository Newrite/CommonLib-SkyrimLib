use core_util::inherit;

use crate::offsets::offsets_rtti::RTTI_ModManagerData;
use crate::offsets::offsets_vtable::VTABLE_ModManagerData;
use crate::re::IUIMessageData;
use crate::relocation::{RttiType, VariantID};

/// C++ `RE::ModManagerData`
#[repr(C)]
pub struct ModManagerData {
    pub base: IUIMessageData,             // 00
    pub use_transparent_background: bool, // 10
    pub pad11: u8,                        // 11
    pub pad12: u16,                       // 12
    pub pad14: u32,                       // 14
}

const _: () = assert!(core::mem::size_of::<ModManagerData>() == 0x18);
const _: () = assert!(core::mem::offset_of!(ModManagerData, base) == 0x00);
const _: () = assert!(core::mem::offset_of!(ModManagerData, use_transparent_background) == 0x10);

inherit!(ModManagerData : IUIMessageData);

impl RttiType for ModManagerData {
    const RTTI: VariantID = RTTI_ModManagerData;
}

impl ModManagerData {
    pub const RTTI: VariantID = RTTI_ModManagerData;
    pub const VTABLE: &'static [VariantID] = &VTABLE_ModManagerData;
    pub const CLASS_NAME: &'static str = "ModManagerData";

    // override (IUIMessageData)
    // ~ModManagerData() override; // 00
}
