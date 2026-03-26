use crate::re::bs_core_types::FormID;

/// C++ `RE::TESResolveNPCTemplatesEvent`
#[repr(C)]
pub struct TESResolveNPCTemplatesEvent {
    pub template_id: FormID, // 00
    pub pad04: u32,          // 04
}

const _: () = assert!(core::mem::size_of::<TESResolveNPCTemplatesEvent>() == 0x08);
