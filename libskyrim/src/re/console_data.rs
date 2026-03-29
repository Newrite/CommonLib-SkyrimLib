use core_util::inherit;

use crate::offsets::offsets_rtti::RTTI_ConsoleData;
use crate::offsets::offsets_vtable::VTABLE_ConsoleData;
use crate::re::{BSString, IUIMessageData, NiBinaryStream, ObjectRefHandle};
use crate::relocation::{RttiType, VariantID};

/// Storage-backed C++ `RE::ConsoleData::DataType`.
///
/// `ConsoleData.h` declares this nested enum without enumerators, so Rust keeps
/// the exact 32-bit storage layer instead of inventing values.
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct ConsoleDataDataType(pub u32);

/// C++ `RE::ConsoleData`
#[repr(C)]
pub struct ConsoleData {
    pub base: IUIMessageData,      // 00
    pub str_: *mut BSString,       // 10
    pub pick_ref: ObjectRefHandle, // 18
    pub pad1c: u32,                // 1C
    pub file: *mut NiBinaryStream, // 20
    // TODO: `ConsoleData.h` stores this as `REX::EnumSet<DataType, u32>`, but
    // the nested `DataType` enum is declared with no enumerators and Rust does
    // not support a zero-variant `repr(u32)` enum that could back the shared
    // `EnumSet` helper. End state: switch this field to the real
    // `EnumSet<DataType, u32>` once the shared ABI layer can represent
    // zero-variant storage enums honestly.
    pub type_: ConsoleDataDataType, // 28
    pub pad2c: u32,                 // 2C
}

const _: () = assert!(core::mem::size_of::<ConsoleData>() == 0x30);
const _: () = assert!(core::mem::offset_of!(ConsoleData, base) == 0x00);
const _: () = assert!(core::mem::offset_of!(ConsoleData, str_) == 0x10);
const _: () = assert!(core::mem::offset_of!(ConsoleData, pick_ref) == 0x18);
const _: () = assert!(core::mem::offset_of!(ConsoleData, file) == 0x20);
const _: () = assert!(core::mem::offset_of!(ConsoleData, type_) == 0x28);

inherit!(ConsoleData : IUIMessageData);

impl RttiType for ConsoleData {
    const RTTI: VariantID = RTTI_ConsoleData;
}

impl ConsoleData {
    pub const RTTI: VariantID = RTTI_ConsoleData;
    pub const VTABLE: &'static [VariantID] = &VTABLE_ConsoleData;
    pub const CLASS_NAME: &'static str = "ConsoleData";

    // override (IUIMessageData)
    // ~ConsoleData() override; // 00
}
