use crate::re::bs_core_types::FormID;

/// C++ `RE::TESObjectLoadedEvent`
#[repr(C)]
pub struct TESObjectLoadedEvent {
    pub form_id: FormID, // 00
    pub loaded: bool,    // 04
    pub pad5: u8,        // 05
    pub pad6: u16,       // 06
}

const _: () = assert!(core::mem::size_of::<TESObjectLoadedEvent>() == 0x08);
