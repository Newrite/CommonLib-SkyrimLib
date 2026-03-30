use core_util::{EnumSet, inherit};

use crate::re::BGSActorEvent;
use crate::re::bs_core_types::FormID;

/// C++ `RE::BGSActorCellEvent::CellFlag`
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BGSActorCellEventCellFlag {
    Enter = 0,
    Leave = 1,
}

core_util::impl_enumset_type!(BGSActorCellEventCellFlag => u32);

/// C++ `RE::BGSActorCellEvent`
#[repr(C)]
pub struct BGSActorCellEvent {
    pub base: BGSActorEvent,                            // 00
    pub cell_id: FormID,                                // 04
    pub flags: EnumSet<BGSActorCellEventCellFlag, u32>, // 08
}

const _: () = assert!(core::mem::size_of::<BGSActorCellEvent>() == 0x0C);
const _: () = assert!(core::mem::offset_of!(BGSActorCellEvent, base) == 0x00);
const _: () = assert!(core::mem::offset_of!(BGSActorCellEvent, cell_id) == 0x04);
const _: () = assert!(core::mem::offset_of!(BGSActorCellEvent, flags) == 0x08);

inherit!(BGSActorCellEvent : BGSActorEvent, base);
