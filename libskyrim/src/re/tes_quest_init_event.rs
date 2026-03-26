use crate::re::bs_core_types::FormID;

/// C++ `RE::TESQuestInitEvent`
#[repr(C)]
pub struct TESQuestInitEvent {
    pub form_id: FormID, // 00
}

const _: () = assert!(core::mem::size_of::<TESQuestInitEvent>() == 0x04);
