#![allow(non_camel_case_types)]

/// C++ `RE::GFxLogConstants`
#[repr(C)]
pub struct GFxLogConstants {
    pub _pad: u8,
}

impl GFxLogConstants {
    pub const CHANNEL_GENERAL: u32 = 0x10;
    pub const CHANNEL_SCRIPT: u32 = 0x20;
    pub const CHANNEL_PARSE: u32 = 0x30;
    pub const CHANNEL_ACTION: u32 = 0x40;
    pub const CHANNEL_DEBUG: u32 = 0x50;
    pub const CHANNEL_MASK: u32 = 0xF0;
    pub const MESSAGE_TYPE_ERROR: u32 = 0;
    pub const MESSAGE_TYPE_WARNING: u32 = 1;
    pub const MESSAGE_TYPE_MESSAGE: u32 = 2;
    pub const ERROR: u32 = Self::CHANNEL_GENERAL | Self::MESSAGE_TYPE_ERROR;
    pub const WARNING: u32 = Self::CHANNEL_GENERAL | Self::MESSAGE_TYPE_WARNING;
    pub const MESSAGE: u32 = Self::CHANNEL_GENERAL | Self::MESSAGE_TYPE_MESSAGE;
    pub const SCRIPT_ERROR: u32 = Self::CHANNEL_SCRIPT | Self::MESSAGE_TYPE_ERROR;
    pub const SCRIPT_WARNING: u32 = Self::CHANNEL_SCRIPT | Self::MESSAGE_TYPE_WARNING;
    pub const SCRIPT_MESSAGE: u32 = Self::CHANNEL_SCRIPT | Self::MESSAGE_TYPE_MESSAGE;
    pub const PARSE: u32 = Self::CHANNEL_PARSE;
    pub const PARSE_SHAPE: u32 = Self::CHANNEL_PARSE | 1;
    pub const PARSE_MORPH_SHAPE: u32 = Self::CHANNEL_PARSE | 2;
    pub const PARSE_ACTION: u32 = Self::CHANNEL_PARSE | 3;
    pub const ACTION: u32 = Self::CHANNEL_ACTION;
}
