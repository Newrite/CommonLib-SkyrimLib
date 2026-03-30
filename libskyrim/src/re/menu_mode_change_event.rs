use core_util::EnumSet;

use crate::re::BSFixedString;

/// C++ `RE::MenuModeChangeEvent::Mode`
#[repr(i8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MenuMode {
    None = -1,
    Hidden = 0,
    Displayed = 1,
}

core_util::impl_enumset_type!(MenuMode => u8);

/// C++ `RE::MenuModeChangeEvent`
#[repr(C)]
pub struct MenuModeChangeEvent {
    pub menu: BSFixedString,         // 00
    pub mode: EnumSet<MenuMode, u8>, // 08
}

const _: () = assert!(core::mem::size_of::<MenuModeChangeEvent>() == 0x10);
const _: () = assert!(core::mem::offset_of!(MenuModeChangeEvent, menu) == 0x00);
const _: () = assert!(core::mem::offset_of!(MenuModeChangeEvent, mode) == 0x08);
