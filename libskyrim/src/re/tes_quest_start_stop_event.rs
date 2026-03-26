use crate::re::bs_core_types::FormID;

/// C++ `RE::TESQuestStartStopEvent`
#[repr(C)]
pub struct TESQuestStartStopEvent {
    pub form_id: FormID, // 00
    pub started: bool,   // 04
    pub failed: bool,    // 05
    pub pad06: u16,      // 06
}

const _: () = assert!(core::mem::size_of::<TESQuestStartStopEvent>() == 0x08);
