use core_util::inherit;

use crate::offsets::offsets_rtti::RTTI_ExtraSayToTopicInfo;
use crate::offsets::offsets_vtable::VTABLE_ExtraSayToTopicInfo;
use crate::re::{
    BGSDialogueBranch, BSExtraData, BSSoundHandle, DialogueItem, ExtraDataType, ExtraDataTyped,
    TESTopic,
};
use crate::relocation::{RttiType, VariantID};

/// C++ `RE::ExtraSayToTopicInfo`
#[repr(C)]
pub struct ExtraSayToTopicInfo {
    pub base: BSExtraData,                        // 00
    pub topic: *mut TESTopic,                     // 10
    pub voice_paused: bool,                       // 18
    pub pad19: u8,                                // 19
    pub pad1a: u16,                               // 1A
    pub subtitle_speech_delay: f32,               // 1C
    pub exclusive_branch: *mut BGSDialogueBranch, // 20
    pub sound: BSSoundHandle,                     // 28
    pub pad34: u32,                               // 34
    pub item: *mut DialogueItem,                  // 38
}

const _: () = assert!(core::mem::size_of::<ExtraSayToTopicInfo>() == 0x40);
const _: () = assert!(core::mem::offset_of!(ExtraSayToTopicInfo, topic) == 0x10);
const _: () = assert!(core::mem::offset_of!(ExtraSayToTopicInfo, voice_paused) == 0x18);
const _: () = assert!(core::mem::offset_of!(ExtraSayToTopicInfo, subtitle_speech_delay) == 0x1C);
const _: () = assert!(core::mem::offset_of!(ExtraSayToTopicInfo, exclusive_branch) == 0x20);
const _: () = assert!(core::mem::offset_of!(ExtraSayToTopicInfo, sound) == 0x28);
const _: () = assert!(core::mem::offset_of!(ExtraSayToTopicInfo, item) == 0x38);

impl RttiType for ExtraSayToTopicInfo {
    const RTTI: VariantID = RTTI_ExtraSayToTopicInfo;
}

impl ExtraDataTyped for ExtraSayToTopicInfo {
    const EXTRADATATYPE: ExtraDataType = ExtraDataType::SayTopicInfo;
}

inherit!(ExtraSayToTopicInfo : BSExtraData);

impl ExtraSayToTopicInfo {
    pub const RTTI: VariantID = RTTI_ExtraSayToTopicInfo;
    pub const VTABLE: &'static [VariantID] = &VTABLE_ExtraSayToTopicInfo;
    pub const EXTRADATATYPE: ExtraDataType = ExtraDataType::SayTopicInfo;
}
