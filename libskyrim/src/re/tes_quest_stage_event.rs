use crate::re::bs_core_types::FormID;

/// C++ `RE::TESQuestStageEvent`
#[repr(C)]
pub struct TESQuestStageEvent {
    pub finished_callback: *mut core::ffi::c_void, // 00
    pub form_id: FormID,                           // 08
    pub stage: u16,                                // 0C
    pub item_index: u8,                            // 0E
    pub pad0f: u8,                                 // 0F
}

const _: () = assert!(core::mem::size_of::<TESQuestStageEvent>() == 0x10);
