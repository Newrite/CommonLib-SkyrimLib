use crate::re::bs_core_types::FormID;

/// C++ `RE::TESFormDeleteEvent`
#[repr(C)]
pub struct TESFormDeleteEvent {
    pub form_id: FormID, // 00
    pub pad04: u32,      // 04
}

const _: () = assert!(core::mem::size_of::<TESFormDeleteEvent>() == 0x08);
