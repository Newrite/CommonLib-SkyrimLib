use core_util::EnumSet;

use crate::re::{BGSLocation, ObjectRefHandle, TESForm};
use crate::relocation::RelocationID;

/// C++ `RE::AQUIRE_TYPE`
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AQUIRE_TYPE {
    kNone = 0,
    kSteal = 1,
    kBuy = 2,
    kPickPocket = 3,
    kPickup = 4,
    kContainer = 5,
    kDeadBody = 6,
}

core_util::impl_enumset_type!(AQUIRE_TYPE => u32);

/// C++ `RE::BGSAddToPlayerInventoryEvent`
#[repr(C)]
pub struct BGSAddToPlayerInventoryEvent {
    pub owner_ref: ObjectRefHandle,              // 00
    pub container_ref: ObjectRefHandle,          // 04
    pub location: *mut BGSLocation,              // 08
    pub item_base: *mut TESForm,                 // 10
    pub acquire_type: EnumSet<AQUIRE_TYPE, u32>, // 18
    pub pad1c: u32,                              // 1C
}

const _: () = assert!(core::mem::size_of::<BGSAddToPlayerInventoryEvent>() == 0x20);
const _: () = assert!(core::mem::offset_of!(BGSAddToPlayerInventoryEvent, owner_ref) == 0x00);
const _: () = assert!(core::mem::offset_of!(BGSAddToPlayerInventoryEvent, item_base) == 0x10);
const _: () = assert!(core::mem::offset_of!(BGSAddToPlayerInventoryEvent, acquire_type) == 0x18);

impl BGSAddToPlayerInventoryEvent {
    crate::relocation_variable! {
        pub fn index() -> &'static mut u32 => RelocationID::new(508412, 380074)
    }
}
