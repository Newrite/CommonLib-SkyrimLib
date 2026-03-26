#![allow(non_camel_case_types)]

use crate::core_util::inherit;
use crate::offsets::offsets_rtti::RTTI_hkLocalFrameGroup;
use crate::offsets::offsets_vtable::VTABLE_hkLocalFrameGroup;
use crate::re::{hkReferencedObject, hkStringPtr};
use crate::relocation::{RttiType, VariantID};

/// C++ `RE::hkLocalFrameGroup`
#[repr(C)]
pub struct hkLocalFrameGroup {
    pub base: hkReferencedObject, // 00
    pub name: hkStringPtr,        // 10
}

const _: () = assert!(core::mem::size_of::<hkLocalFrameGroup>() == 0x18);
const _: () = assert!(core::mem::offset_of!(hkLocalFrameGroup, base) == 0x00);
const _: () = assert!(core::mem::offset_of!(hkLocalFrameGroup, name) == 0x10);

impl RttiType for hkLocalFrameGroup {
    const RTTI: VariantID = RTTI_hkLocalFrameGroup;
}

inherit!(hkLocalFrameGroup : hkReferencedObject, base);

impl hkLocalFrameGroup {
    pub const RTTI: VariantID = RTTI_hkLocalFrameGroup;
    pub const VTABLE: &'static [VariantID] = &VTABLE_hkLocalFrameGroup;
}
