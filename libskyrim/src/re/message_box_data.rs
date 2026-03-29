use core_util::inherit;

use crate::offsets::offsets_rtti::RTTI_MessageBoxData;
use crate::offsets::offsets_vtable::VTABLE_MessageBoxData;
use crate::re::{BSString, BSTArray, BSTSmartPointer, IMessageBoxCallback, IUIMessageData};
use crate::relocation::{RttiType, VariantID};

/// C++ `RE::MessageBoxData`
#[repr(C)]
pub struct MessageBoxData {
    pub base: IUIMessageData,                           // 00
    pub body_text: BSString,                            // 10
    pub button_text: BSTArray<BSString>,                // 20
    pub type_: u32,                                     // 38
    pub cancel_option_index: i32,                       // 3C
    pub callback: BSTSmartPointer<IMessageBoxCallback>, // 40
    pub menu_depth: i32,                                // 48
    pub option_index_offset: u8,                        // 4C
    pub use_html: bool,                                 // 4D
    pub vertical_buttons: bool,                         // 4E
    pub is_cancellable: bool,                           // 4F
}

const _: () = assert!(core::mem::size_of::<MessageBoxData>() == 0x50);
const _: () = assert!(core::mem::offset_of!(MessageBoxData, base) == 0x00);
const _: () = assert!(core::mem::offset_of!(MessageBoxData, body_text) == 0x10);
const _: () = assert!(core::mem::offset_of!(MessageBoxData, button_text) == 0x20);
const _: () = assert!(core::mem::offset_of!(MessageBoxData, type_) == 0x38);
const _: () = assert!(core::mem::offset_of!(MessageBoxData, callback) == 0x40);
const _: () = assert!(core::mem::offset_of!(MessageBoxData, menu_depth) == 0x48);
const _: () = assert!(core::mem::offset_of!(MessageBoxData, option_index_offset) == 0x4C);

inherit!(MessageBoxData : IUIMessageData);

impl RttiType for MessageBoxData {
    const RTTI: VariantID = RTTI_MessageBoxData;
}

impl MessageBoxData {
    pub const RTTI: VariantID = RTTI_MessageBoxData;
    pub const VTABLE: &'static [VariantID] = &VTABLE_MessageBoxData;
    pub const CLASS_NAME: &'static str = "MessageBoxData";

    // override (IUIMessageData)
    // ~MessageBoxData() override; // 00
}
