use core_util::EnumSet;

use crate::re::BSIMusicType;

/// C++ `RE::BSMusicEvent::MUSIC_MESSAGE_TYPE`
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BSMusicMessageType {
    kAdd = 0,
    kRemove = 1,
    kRemoveImmediate = 2,
    kPause = 3,
    kUnpause = 4,
}

core_util::impl_enumset_type!(BSMusicMessageType => i32);

/// C++ `RE::BSMusicEvent`
#[repr(C)]
pub struct BSMusicEvent {
    pub music_type: *mut BSIMusicType,              // 00
    pub msg_type: EnumSet<BSMusicMessageType, i32>, // 08
    pub pad0c: u32,                                 // 0C
}

const _: () = assert!(core::mem::size_of::<BSMusicEvent>() == 0x10);
const _: () = assert!(core::mem::offset_of!(BSMusicEvent, music_type) == 0x00);
const _: () = assert!(core::mem::offset_of!(BSMusicEvent, msg_type) == 0x08);
