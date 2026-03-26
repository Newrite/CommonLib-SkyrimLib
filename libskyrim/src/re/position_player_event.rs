use core_util::EnumSet;

/// C++ `RE::PositionPlayerEvent::EVENT_TYPE`
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PositionPlayerEventType {
    kPre = 0,
    kPreUpdatePackages = 1,
    kPostUpdatePackages = 2,
    kPost = 3,
    kFinish = 4,
}

core_util::impl_enumset_type!(PositionPlayerEventType => u32);

/// C++ `RE::PositionPlayerEvent`
#[repr(C)]
pub struct PositionPlayerEvent {
    pub type_: EnumSet<PositionPlayerEventType, u32>, // 00
}

const _: () = assert!(core::mem::size_of::<PositionPlayerEvent>() == 0x04);
const _: () = assert!(core::mem::offset_of!(PositionPlayerEvent, type_) == 0x00);
