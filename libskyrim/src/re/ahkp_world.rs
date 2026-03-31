#![allow(non_camel_case_types)]

use crate::core_util::inherit;
use crate::offsets::offsets_rtti::RTTI_ahkpWorld;
use crate::offsets::offsets_vtable::VTABLE_ahkpWorld;
use crate::re::{bhkWorld, hkpWorld};
use crate::relocation::{RttiType, VariantID};

/// C++ `RE::ahkpWorld`
#[repr(C)]
pub struct ahkpWorld {
    pub base: hkpWorld,           // 000
    pub user_data: *mut bhkWorld, // 430
    pub unk438: u64,              // 438
}

const _: () = assert!(core::mem::size_of::<ahkpWorld>() == 0x440);
const _: () = assert!(core::mem::offset_of!(ahkpWorld, base) == 0x000);
const _: () = assert!(core::mem::offset_of!(ahkpWorld, user_data) == 0x430);
const _: () = assert!(core::mem::offset_of!(ahkpWorld, unk438) == 0x438);

impl RttiType for ahkpWorld {
    const RTTI: VariantID = RTTI_ahkpWorld;
}

inherit!(ahkpWorld : hkpWorld, base);

impl ahkpWorld {
    pub const RTTI: VariantID = RTTI_ahkpWorld;
    pub const VTABLE: &'static [VariantID] = &VTABLE_ahkpWorld;
}
