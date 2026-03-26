#![allow(non_camel_case_types)]

use core_util::inherit;

use crate::offsets::offsets_rtti::RTTI_CharEvent;
use crate::offsets::offsets_vtable::VTABLE_CharEvent;
use crate::re::InputEvent;
use crate::relocation::{RttiType, VariantID};

/// C++ `RE::CharEvent`
#[repr(C)]
pub struct CharEvent {
    pub base: InputEvent, // 00
    pub key_code: u32,    // 18
    pub pad1c: u32,       // 1C
}

const _: () = assert!(core::mem::size_of::<CharEvent>() == 0x20);
const _: () = assert!(core::mem::offset_of!(CharEvent, base) == 0x00);
const _: () = assert!(core::mem::offset_of!(CharEvent, key_code) == 0x18);

impl RttiType for CharEvent {
    const RTTI: VariantID = RTTI_CharEvent;
}

inherit!(CharEvent : InputEvent, base);

impl CharEvent {
    pub const RTTI: VariantID = RTTI_CharEvent;
    pub const VTABLE: &'static [VariantID] = &VTABLE_CharEvent;

    #[inline(always)]
    pub fn init(&mut self, key_code: u32) {
        self.key_code = key_code;
    }
}
