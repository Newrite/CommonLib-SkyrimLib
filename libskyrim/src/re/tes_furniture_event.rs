use core_util::EnumSet;

use crate::re::{NiPointer, TESObjectREFR};

/// C++ `RE::TESFurnitureEvent::FurnitureEventType`
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FurnitureEventType {
    kEnter = 0,
    kExit = 1,
}

core_util::impl_enumset_type!(FurnitureEventType => u32);

/// C++ `RE::TESFurnitureEvent`
#[repr(C)]
pub struct TESFurnitureEvent {
    pub actor: NiPointer<TESObjectREFR>,            // 00
    pub target_furniture: NiPointer<TESObjectREFR>, // 08
    pub type_: EnumSet<FurnitureEventType, u32>,    // 10
    pub pad14: u32,                                 // 14
}

const _: () = assert!(core::mem::size_of::<TESFurnitureEvent>() == 0x18);
