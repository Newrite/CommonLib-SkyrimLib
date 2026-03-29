#![allow(non_camel_case_types)]

/// C++ `RE::GFxLogConstants`
#[repr(C)]
pub struct GFxLogConstants {
    pub _pad: u8,
}

impl GFxLogConstants {
    pub const kChannel_General: u32 = 0x10;
    pub const kChannel_Script: u32 = 0x20;
    pub const kChannel_Parse: u32 = 0x30;
    pub const kChannel_Action: u32 = 0x40;
    pub const kChannel_Debug: u32 = 0x50;
    pub const kChannel_Mask: u32 = 0xF0;
    pub const kMessageType_Error: u32 = 0;
    pub const kMessageType_Warning: u32 = 1;
    pub const kMessageType_Message: u32 = 2;
    pub const kError: u32 = Self::kChannel_General | Self::kMessageType_Error;
    pub const kWarning: u32 = Self::kChannel_General | Self::kMessageType_Warning;
    pub const kMessage: u32 = Self::kChannel_General | Self::kMessageType_Message;
    pub const kScriptError: u32 = Self::kChannel_Script | Self::kMessageType_Error;
    pub const kScriptWarning: u32 = Self::kChannel_Script | Self::kMessageType_Warning;
    pub const kScriptMessage: u32 = Self::kChannel_Script | Self::kMessageType_Message;
    pub const kParse: u32 = Self::kChannel_Parse;
    pub const kParseShape: u32 = Self::kChannel_Parse | 1;
    pub const kParseMorphShape: u32 = Self::kChannel_Parse | 2;
    pub const kParseAction: u32 = Self::kChannel_Parse | 3;
    pub const kAction: u32 = Self::kChannel_Action;
}
