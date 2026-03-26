use crate::re::ObjectRefHandle;
use crate::re::bs_core_types::FormID;

/// C++ `RE::TESContainerChangedEvent`
#[repr(C)]
pub struct TESContainerChangedEvent {
    pub old_container: FormID,      // 00
    pub new_container: FormID,      // 04
    pub base_obj: FormID,           // 08
    pub item_count: i32,            // 0C
    pub reference: ObjectRefHandle, // 10
    pub unique_id: u16,             // 14
    pub pad16: u16,                 // 16
}

const _: () = assert!(core::mem::size_of::<TESContainerChangedEvent>() == 0x18);
