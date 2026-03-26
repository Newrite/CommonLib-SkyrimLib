#![allow(non_camel_case_types)]

use core_util::inherit;

use crate::offsets::offsets_rtti::RTTI_hkbEventPayload;
use crate::offsets::offsets_vtable::VTABLE_hkbEventPayload;
use crate::re::{hkRefPtr, hkReferencedObject};
use crate::relocation::{RttiType, VariantID};

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum hkbEventBaseSystemEventIDs {
    kNull = u32::MAX,
}

core_util::impl_enumset_type!(hkbEventBaseSystemEventIDs => u32);

/// C++ `RE::hkbEventPayload`
#[repr(C)]
pub struct hkbEventPayload {
    pub base: hkReferencedObject, // 00
}

const _: () = assert!(core::mem::size_of::<hkbEventPayload>() == 0x10);

impl RttiType for hkbEventPayload {
    const RTTI: VariantID = RTTI_hkbEventPayload;
}

inherit!(hkbEventPayload : hkReferencedObject, base);

impl hkbEventPayload {
    pub const RTTI: VariantID = RTTI_hkbEventPayload;
    pub const VTABLE: &'static [VariantID] = &VTABLE_hkbEventPayload;
}

/// C++ `RE::hkbEventBase`
#[repr(C)]
pub struct hkbEventBase {
    pub id: hkbEventBaseSystemEventIDs,     // 00
    pub pad04: u32,                         // 04
    pub payload: hkRefPtr<hkbEventPayload>, // 08
}

const _: () = assert!(core::mem::size_of::<hkbEventBase>() == 0x10);
