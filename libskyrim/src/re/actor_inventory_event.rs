use crate::re::{BIPED_OBJECT, InventoryEvent};

/// C++ `RE::ActorInventoryEvent`
#[repr(C)]
pub struct ActorInventoryEvent {
    pub event: InventoryEvent,     // 00
    pub equip_index: BIPED_OBJECT, // 04
}

const _: () = assert!(core::mem::size_of::<ActorInventoryEvent>() == 0x08);
const _: () = assert!(core::mem::offset_of!(ActorInventoryEvent, event) == 0x00);
const _: () = assert!(core::mem::offset_of!(ActorInventoryEvent, equip_index) == 0x04);
