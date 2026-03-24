#![allow(non_camel_case_types)]

use crate::core_util::EnumSet;
use crate::re::{hkClass, hkClassEnum, hkCustomAttributes};
use core::ffi::c_char;

/// C++ `RE::hkClassMember::Type`
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum hkClassMemberType {
    Void = 0,
    Bool = 1,
    Char = 2,
    Int8 = 3,
    Uint8 = 4,
    Int16 = 5,
    Uint16 = 6,
    Int32 = 7,
    Uint32 = 8,
    Int64 = 9,
    Uint64 = 10,
    Real = 11,
    Vector4 = 12,
    Quaternion = 13,
    Matrix3 = 14,
    Rotation = 15,
    QsTransform = 16,
    Matrix4 = 17,
    Transform = 18,
    Zero = 19,
    Pointer = 20,
    FunctionPointer = 21,
    Array = 22,
    InplaceArray = 23,
    Enum = 24,
    Struct = 25,
    SimpleArray = 26,
    HomogeneousArray = 27,
    Variant = 28,
    CString = 29,
    Ulong = 30,
    Flags = 31,
    Half = 32,
    StringPtr = 33,
    Max = 34,
}

core_util::impl_enumset_type!(hkClassMemberType => u8);

/// C++ `RE::hkClassMember::FlagValues`
#[repr(u16)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum hkClassMemberFlagValues {
    None = 0,
    Align8 = 128,
    Align16 = 256,
    NotOwned = 512,
    SerializeIgnored = 1024,
}

core_util::impl_enumset_type!(hkClassMemberFlagValues => u16);

/// C++ `RE::hkClassMember::DeprecatedFlagValues`
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum hkClassMemberDeprecatedFlagValues {
    Size8 = 8,
    Size16 = 16,
    Size32 = 32,
}

impl hkClassMemberDeprecatedFlagValues {
    pub const ENUM8: Self = Self::Size8;
    pub const ENUM16: Self = Self::Size16;
    pub const ENUM32: Self = Self::Size32;
}

/// C++ `RE::hkClassMember::TypeProperties`
#[repr(C)]
pub struct hkClassMemberTypeProperties {
    pub ty: EnumSet<hkClassMemberType, u8>, // 00
    pub name: *const c_char,                // 08
    pub size: i16,                          // 10
    pub align: i16,                         // 12
}

const _: () = assert!(core::mem::size_of::<hkClassMemberTypeProperties>() == 0x18);
const _: () = assert!(core::mem::offset_of!(hkClassMemberTypeProperties, ty) == 0x00);
const _: () = assert!(core::mem::offset_of!(hkClassMemberTypeProperties, name) == 0x08);
const _: () = assert!(core::mem::offset_of!(hkClassMemberTypeProperties, size) == 0x10);
const _: () = assert!(core::mem::offset_of!(hkClassMemberTypeProperties, align) == 0x12);

/// C++ `RE::hkClassMember`
#[repr(C)]
pub struct hkClassMember {
    pub name: *const c_char,                          // 00
    pub class: *const hkClass,                        // 08
    pub enum_type: *const hkClassEnum,                // 10
    pub ty: EnumSet<hkClassMemberType, u8>,           // 18
    pub subtype: EnumSet<hkClassMemberType, u8>,      // 19
    pub c_array_size: i16,                            // 1A
    pub flags: EnumSet<hkClassMemberFlagValues, u16>, // 1C
    pub offset: u16,                                  // 1E
    pub attributes: *const hkCustomAttributes,        // 20
}

const _: () = assert!(core::mem::size_of::<hkClassMember>() == 0x28);
const _: () = assert!(core::mem::offset_of!(hkClassMember, name) == 0x00);
const _: () = assert!(core::mem::offset_of!(hkClassMember, class) == 0x08);
const _: () = assert!(core::mem::offset_of!(hkClassMember, enum_type) == 0x10);
const _: () = assert!(core::mem::offset_of!(hkClassMember, ty) == 0x18);
const _: () = assert!(core::mem::offset_of!(hkClassMember, subtype) == 0x19);
const _: () = assert!(core::mem::offset_of!(hkClassMember, c_array_size) == 0x1A);
const _: () = assert!(core::mem::offset_of!(hkClassMember, flags) == 0x1C);
const _: () = assert!(core::mem::offset_of!(hkClassMember, offset) == 0x1E);
const _: () = assert!(core::mem::offset_of!(hkClassMember, attributes) == 0x20);
