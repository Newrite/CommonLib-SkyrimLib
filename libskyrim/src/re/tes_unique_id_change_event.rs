use crate::re::bs_core_types::FormID;

/// C++ `RE::TESUniqueIDChangeEvent`
#[repr(C)]
pub struct TESUniqueIDChangeEvent {
    pub old_base_id: FormID, // 00
    pub new_base_id: FormID, // 04
    pub object_id: FormID,   // 08
    pub old_unique_id: u16,  // 0C
    pub new_unique_id: u16,  // 0E
}

const _: () = assert!(core::mem::size_of::<TESUniqueIDChangeEvent>() == 0x10);
