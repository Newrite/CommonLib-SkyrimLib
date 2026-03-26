use crate::re::{BGSLocation, NiPointer, TESObjectREFR};

/// C++ `RE::TESActorLocationChangeEvent`
#[repr(C)]
pub struct TESActorLocationChangeEvent {
    pub actor: NiPointer<TESObjectREFR>, // 00
    pub old_loc: *mut BGSLocation,       // 08
    pub new_loc: *mut BGSLocation,       // 10
}

const _: () = assert!(core::mem::size_of::<TESActorLocationChangeEvent>() == 0x18);
