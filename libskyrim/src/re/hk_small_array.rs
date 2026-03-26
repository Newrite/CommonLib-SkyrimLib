#![allow(non_camel_case_types)]

use core_util::EnumSet;

/// C++ `RE::hkSmallArray::CapacityAndFlags`
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum hkSmallArrayCapacityAndFlags {
    CapacityMask = 0x3FFF,
    FlagMask = 0xC000,
    NoDealloc = 0x8000,
    Locked = 0x4000,
    ForceSigned = -1,
}

core_util::impl_enumset_type!(hkSmallArrayCapacityAndFlags => u16);

/// C++ `RE::hkSmallArray<T>`
#[repr(C)]
pub struct hkSmallArray<T> {
    pub data: *mut T,                                                   // 00
    pub size: u16,                                                      // 08
    pub capacity_and_flags: EnumSet<hkSmallArrayCapacityAndFlags, u16>, // 0A
    pub pad0c: u32,                                                     // 0C
}

const _: () = assert!(core::mem::size_of::<hkSmallArray<*mut core::ffi::c_void>>() == 0x10);
