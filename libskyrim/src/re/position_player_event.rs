use core_util::EnumSet;

/// C++ `RE::PositionPlayerEvent::EVENT_TYPE`
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PositionPlayerEventType {
    Pre = 0,
    PreUpdatePackages = 1,
    PostUpdatePackages = 2,
    Post = 3,
    Finish = 4,
}

core_util::impl_enumset_type!(PositionPlayerEventType => u32);

/// C++ `RE::PositionPlayerEvent`
#[repr(C)]
pub struct PositionPlayerEvent {
    pub type_: EnumSet<PositionPlayerEventType, u32>, // 00
}

const _: () = assert!(core::mem::size_of::<PositionPlayerEvent>() == 0x04);
const _: () = assert!(core::mem::offset_of!(PositionPlayerEvent, type_) == 0x00);
