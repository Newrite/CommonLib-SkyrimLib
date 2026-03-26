#![allow(non_camel_case_types)]

/// C++ `RE::hkpBroadPhaseHandle`
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct hkpBroadPhaseHandle {
    pub id: u32, // 00
}

const _: () = assert!(core::mem::size_of::<hkpBroadPhaseHandle>() == 0x4);
