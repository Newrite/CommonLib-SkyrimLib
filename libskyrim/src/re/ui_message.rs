use core_util::{Enum, EnumSet};

use crate::re::{BSFixedString, IUIMessageData};

/// C++ `RE::UI_MESSAGE_TYPE`
#[libskyrim_macros::open_enum]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum UIMessageType {
    Update = 0,
    Show = 1,
    Reshow = 2,
    Hide = 3,
    ForceHide = 4,
    ScaleformEvent = 6,
    UserEvent = 7,
    InventoryUpdate = 8,
    UserProfileChange = 9,
    MuStatusChange = 10,
    ResumeCaching = 11,
    UpdateController = 12,
    ChatterEvent = 13,
}

core_util::impl_enumset_type!(UIMessageType => u32);

/// C++ `RE::UIMessage`
#[repr(C)]
pub struct UIMessage {
    pub menu: BSFixedString,                // 00
    pub type_: EnumSet<UIMessageType, u32>, // 08
    pub pad0c: u32,                         // 0C
    pub data: *mut IUIMessageData,          // 10
    pub is_pooled: bool,                    // 18
    pub pad19: u8,                          // 19
    pub pad1a: u16,                         // 1A
    pub pad1c: u32,                         // 1C
}

const _: () = assert!(core::mem::size_of::<UIMessage>() == 0x20);
const _: () = assert!(core::mem::offset_of!(UIMessage, menu) == 0x0);
const _: () = assert!(core::mem::offset_of!(UIMessage, type_) == 0x8);
const _: () = assert!(core::mem::offset_of!(UIMessage, data) == 0x10);
const _: () = assert!(core::mem::offset_of!(UIMessage, is_pooled) == 0x18);

impl UIMessage {
    #[inline(always)]
    pub fn message_type_storage(&self) -> Enum<UIMessageType, i32> {
        Enum::from_underlying(self.type_.underlying() as i32)
    }

    #[inline(always)]
    pub fn try_get_message_type(&self) -> Option<UIMessageType> {
        self.message_type_storage().get()
    }

    #[inline(always)]
    pub fn get_message_type(&self) -> UIMessageType {
        self.try_get_message_type().unwrap_or(UIMessageType::Update)
    }
}
