use crate::re::BSFixedString;

/// C++ `RE::MenuOpenCloseEvent`
#[repr(C)]
pub struct MenuOpenCloseEvent {
    pub menu_name: BSFixedString, // 00
    pub opening: bool,            // 08
    pub pad09: u8,                // 09
    pub pad0a: u16,               // 0A
    pub pad0c: u32,               // 0C
}

const _: () = assert!(core::mem::size_of::<MenuOpenCloseEvent>() == 0x10);
const _: () = assert!(core::mem::offset_of!(MenuOpenCloseEvent, menu_name) == 0x00);
const _: () = assert!(core::mem::offset_of!(MenuOpenCloseEvent, opening) == 0x08);
