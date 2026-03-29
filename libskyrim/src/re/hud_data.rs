use core_util::inherit;

use crate::offsets::offsets_rtti::RTTI_HUDData;
use crate::offsets::offsets_vtable::VTABLE_HUDData;
use crate::re::{
    BSString, HUD_MESSAGE_TYPE, HUDMessageTypeSet, IUIMessageData, MARKER_TYPE, ObjectRefHandle,
    TESQuest, TESWordOfPower,
};
use crate::relocation::{RttiType, VariantID};
use crate::rex::EnumSet;

/// C++ `RE::HUDData`
#[repr(C)]
pub struct HUDData {
    pub base: IUIMessageData,                 // 00
    pub type_: HUDMessageTypeSet,             // 10
    pub pad14: u32,                           // 14
    pub text: BSString,                       // 18
    pub crosshair_ref: ObjectRefHandle,       // 28
    pub pad2c: u32,                           // 2C
    pub quest: *mut TESQuest,                 // 30
    pub word_of_power: *mut TESWordOfPower,   // 38
    pub show: bool,                           // 40
    pub pad41: u8,                            // 41
    pub pad42: u16,                           // 42
    pub discovery: EnumSet<MARKER_TYPE, u32>, // 44
}

const _: () = assert!(core::mem::size_of::<HUDData>() == 0x48);
const _: () = assert!(core::mem::offset_of!(HUDData, base) == 0x00);
const _: () = assert!(core::mem::offset_of!(HUDData, type_) == 0x10);
const _: () = assert!(core::mem::offset_of!(HUDData, text) == 0x18);
const _: () = assert!(core::mem::offset_of!(HUDData, crosshair_ref) == 0x28);
const _: () = assert!(core::mem::offset_of!(HUDData, quest) == 0x30);
const _: () = assert!(core::mem::offset_of!(HUDData, word_of_power) == 0x38);
const _: () = assert!(core::mem::offset_of!(HUDData, show) == 0x40);
const _: () = assert!(core::mem::offset_of!(HUDData, discovery) == 0x44);

inherit!(HUDData : IUIMessageData);

impl RttiType for HUDData {
    const RTTI: VariantID = RTTI_HUDData;
}

impl HUDData {
    pub const RTTI: VariantID = RTTI_HUDData;
    pub const VTABLE: &'static [VariantID] = &VTABLE_HUDData;
    pub const CLASS_NAME: &'static str = "HUDData";

    #[inline(always)]
    pub fn get_type(&self) -> Option<HUD_MESSAGE_TYPE> {
        self.type_.get()
    }

    #[inline(always)]
    pub fn set_type(&mut self, type_: HUD_MESSAGE_TYPE) {
        self.type_ = HUD_MESSAGE_TYPE::get_hud_message_type(type_);
    }

    // override (IUIMessageData)
    // ~HUDData() override; // 00
}
