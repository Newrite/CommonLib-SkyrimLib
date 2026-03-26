#![allow(non_camel_case_types)]

/// C++ `RE::hkpPropertyValue`
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct hkpPropertyValue {
    pub data: u64, // 00
}

const _: () = assert!(core::mem::size_of::<hkpPropertyValue>() == 0x8);

/// C++ `RE::hkpProperty`
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct hkpProperty {
    pub key: u32,                // 00
    pub alignment_padding: u32,  // 04
    pub value: hkpPropertyValue, // 08
}

const _: () = assert!(core::mem::size_of::<hkpProperty>() == 0x10);
