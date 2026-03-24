#![allow(non_camel_case_types)]

use crate::core_util::EnumSet;
use crate::re::{hkClassEnum, hkClassMember, hkCustomAttributes};
use core::ffi::c_char;

/// C++ `RE::hkClass::FlagValues`
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum hkClassFlagValues {
    None = 0,
    NotSerializable = 1,
}

core_util::impl_enumset_type!(hkClassFlagValues => u32);

/// C++ `RE::hkClass`
#[repr(C)]
pub struct hkClass {
    pub name: *const c_char,                    // 00
    pub parent: *const hkClass,                 // 08
    pub object_size: i32,                       // 10
    pub num_implemented_interfaces: i32,        // 14
    pub declared_enums: *const hkClassEnum,     // 18
    pub num_declared_enums: i32,                // 20
    pub pad24: u32,                             // 24
    pub declared_members: *const hkClassMember, // 28
    pub num_declared_members: i32,              // 30
    pub pad34: u32,                             // 34
    pub defaults: *const core::ffi::c_void,     // 38
    pub attributes: *const hkCustomAttributes,  // 40
    pub flags: EnumSet<hkClassFlagValues, u32>, // 48
    pub described_version: i32,                 // 4C
}

const _: () = assert!(core::mem::size_of::<hkClass>() == 0x50);
const _: () = assert!(core::mem::offset_of!(hkClass, name) == 0x00);
const _: () = assert!(core::mem::offset_of!(hkClass, parent) == 0x08);
const _: () = assert!(core::mem::offset_of!(hkClass, object_size) == 0x10);
const _: () = assert!(core::mem::offset_of!(hkClass, num_implemented_interfaces) == 0x14);
const _: () = assert!(core::mem::offset_of!(hkClass, declared_enums) == 0x18);
const _: () = assert!(core::mem::offset_of!(hkClass, num_declared_enums) == 0x20);
const _: () = assert!(core::mem::offset_of!(hkClass, declared_members) == 0x28);
const _: () = assert!(core::mem::offset_of!(hkClass, num_declared_members) == 0x30);
const _: () = assert!(core::mem::offset_of!(hkClass, defaults) == 0x38);
const _: () = assert!(core::mem::offset_of!(hkClass, attributes) == 0x40);
const _: () = assert!(core::mem::offset_of!(hkClass, flags) == 0x48);
const _: () = assert!(core::mem::offset_of!(hkClass, described_version) == 0x4C);
