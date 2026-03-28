use core_util::EnumSet;

use crate::re::{BSFixedString, IUIMessageData};

/// C++ `RE::UI_MESSAGE_TYPE`
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum UI_MESSAGE_TYPE {
    kUpdate = 0,
    kShow = 1,
    kReshow = 2,
    kHide = 3,
    kForceHide = 4,
    kScaleformEvent = 6,
    kUserEvent = 7,
    kInventoryUpdate = 8,
    kUserProfileChange = 9,
    kMUStatusChange = 10,
    kResumeCaching = 11,
    kUpdateController = 12,
    kChatterEvent = 13,
}

core_util::impl_enumset_type!(UI_MESSAGE_TYPE => u32);

/// C++ `RE::UIMessage`
#[repr(C)]
pub struct UIMessage {
    pub menu: BSFixedString,                  // 00
    pub type_: EnumSet<UI_MESSAGE_TYPE, u32>, // 08
    pub pad0c: u32,                           // 0C
    pub data: *mut IUIMessageData,            // 10
    pub is_pooled: bool,                      // 18
    pub pad19: u8,                            // 19
    pub pad1a: u16,                           // 1A
    pub pad1c: u32,                           // 1C
}

const _: () = assert!(core::mem::size_of::<UIMessage>() == 0x20);
const _: () = assert!(core::mem::offset_of!(UIMessage, menu) == 0x0);
const _: () = assert!(core::mem::offset_of!(UIMessage, type_) == 0x8);
const _: () = assert!(core::mem::offset_of!(UIMessage, data) == 0x10);
const _: () = assert!(core::mem::offset_of!(UIMessage, is_pooled) == 0x18);
