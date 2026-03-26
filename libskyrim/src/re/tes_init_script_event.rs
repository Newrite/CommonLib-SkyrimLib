use crate::re::{NiPointer, TESObjectREFR};

/// C++ `RE::TESInitScriptEvent`
#[repr(C)]
pub struct TESInitScriptEvent {
    pub object_initialized: NiPointer<TESObjectREFR>, // 00
}

const _: () = assert!(core::mem::size_of::<TESInitScriptEvent>() == 0x08);
