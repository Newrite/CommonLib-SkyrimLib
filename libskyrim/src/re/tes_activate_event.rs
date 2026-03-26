use crate::re::{NiPointer, TESObjectREFR};

/// C++ `RE::TESActivateEvent`
#[repr(C)]
pub struct TESActivateEvent {
    pub object_activated: NiPointer<TESObjectREFR>, // 00
    pub action_ref: NiPointer<TESObjectREFR>,       // 08
}

const _: () = assert!(core::mem::size_of::<TESActivateEvent>() == 0x10);
