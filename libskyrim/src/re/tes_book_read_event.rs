use crate::re::bs_core_types::FormID;
use crate::re::{NiPointer, TESObjectREFR};

/// C++ `RE::TESBookReadEvent`
#[repr(C)]
pub struct TESBookReadEvent {
    pub ref_: NiPointer<TESObjectREFR>, // 00
    pub base_form_id: FormID,           // 08
    pub unique_id: u16,                 // 0C
    pub pad0e: u16,                     // 0E
}

const _: () = assert!(core::mem::size_of::<TESBookReadEvent>() == 0x10);
