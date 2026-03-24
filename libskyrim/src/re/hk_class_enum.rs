#![allow(non_camel_case_types)]

use crate::core_util::EnumSet;
use crate::re::hkCustomAttributes;
use core::ffi::c_char;

/// C++ `RE::hkClassEnum::Item`
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct hkClassEnumItem {
    pub value: i32,          // 00
    pub name: *const c_char, // 08
}

const _: () = assert!(core::mem::size_of::<hkClassEnumItem>() == 0x10);
const _: () = assert!(core::mem::offset_of!(hkClassEnumItem, value) == 0x00);
const _: () = assert!(core::mem::offset_of!(hkClassEnumItem, name) == 0x08);

impl hkClassEnumItem {
    #[inline(always)]
    pub const fn new(value: i32, name: *const c_char) -> Self {
        Self { value, name }
    }

    #[inline(always)]
    pub const fn get_name(&self) -> *const c_char {
        self.name
    }

    #[inline(always)]
    pub const fn get_value(&self) -> i32 {
        self.value
    }
}

/// C++ `RE::hkClassEnum::FlagValues`
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum hkClassEnumFlagValues {
    None = 0,
}

core_util::impl_enumset_type!(hkClassEnumFlagValues => u32);

/// C++ `RE::hkClassEnum`
#[repr(C)]
pub struct hkClassEnum {
    pub name: *const c_char,                        // 00
    pub items: *const hkClassEnumItem,              // 08
    pub num_items: i32,                             // 10
    pub pad14: i32,                                 // 14
    pub attributes: *mut hkCustomAttributes,        // 18
    pub flags: EnumSet<hkClassEnumFlagValues, u32>, // 20
}

const _: () = assert!(core::mem::size_of::<hkClassEnum>() == 0x28);
const _: () = assert!(core::mem::offset_of!(hkClassEnum, name) == 0x00);
const _: () = assert!(core::mem::offset_of!(hkClassEnum, items) == 0x08);
const _: () = assert!(core::mem::offset_of!(hkClassEnum, num_items) == 0x10);
const _: () = assert!(core::mem::offset_of!(hkClassEnum, attributes) == 0x18);
const _: () = assert!(core::mem::offset_of!(hkClassEnum, flags) == 0x20);
