use crate::re::TESObjectCELL;

/// C++ `RE::TESCellFullyLoadedEvent`
#[repr(C)]
pub struct TESCellFullyLoadedEvent {
    pub cell: *mut TESObjectCELL, // 00
}

const _: () = assert!(core::mem::size_of::<TESCellFullyLoadedEvent>() == 0x08);
