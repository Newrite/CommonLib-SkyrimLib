use core_util::inherit;

use crate::offsets::offsets_rtti::RTTI_FaderData;
use crate::offsets::offsets_vtable::VTABLE_FaderData;
use crate::re::IUIMessageData;
use crate::relocation::{RttiType, VariantID};

/// C++ `RE::FaderData`
#[repr(C)]
pub struct FaderData {
    pub base: IUIMessageData, // 00
    pub unk10: u64,           // 10
    pub unk18: u8,            // 18
    pub pad19: u8,            // 19
    pub pad1a: u16,           // 1A
    pub min_duration: f32,    // 1C
    pub fade_duration: f32,   // 20
    pub is_fading_out: bool,  // 24
    pub is_black: bool,       // 25
    pub unk26: bool,          // 26
    pub pauses_game: bool,    // 27
}

const _: () = assert!(core::mem::size_of::<FaderData>() == 0x28);
const _: () = assert!(core::mem::offset_of!(FaderData, base) == 0x00);
const _: () = assert!(core::mem::offset_of!(FaderData, unk10) == 0x10);
const _: () = assert!(core::mem::offset_of!(FaderData, min_duration) == 0x1C);
const _: () = assert!(core::mem::offset_of!(FaderData, fade_duration) == 0x20);
const _: () = assert!(core::mem::offset_of!(FaderData, is_fading_out) == 0x24);

inherit!(FaderData : IUIMessageData);

impl RttiType for FaderData {
    const RTTI: VariantID = RTTI_FaderData;
}

impl FaderData {
    pub const RTTI: VariantID = RTTI_FaderData;
    pub const VTABLE: &'static [VariantID] = &VTABLE_FaderData;
    pub const CLASS_NAME: &'static str = "FaderData";

    // override (IUIMessageData)
    // ~FaderData() override; // 00
}
