use crate::re::{NiPointer, TESObjectREFR};

/// C++ `RE::TESLockChangedEvent`
#[repr(C)]
pub struct TESLockChangedEvent {
    pub locked_object: NiPointer<TESObjectREFR>, // 00
}

const _: () = assert!(core::mem::size_of::<TESLockChangedEvent>() == 0x08);
