#![allow(non_camel_case_types)]

use core_util::inherit;

use crate::offsets::offsets_rtti::RTTI_MouseMoveEvent;
use crate::offsets::offsets_vtable::VTABLE_MouseMoveEvent;
use crate::re::{BSFixedString, IDEvent};
use crate::relocation::{RttiType, VariantID};

/// C++ `RE::MouseMoveEvent`
#[repr(C)]
pub struct MouseMoveEvent {
    pub base: IDEvent,      // 00
    pub mouse_input_x: i32, // 28
    pub mouse_input_y: i32, // 2C
}

const _: () = assert!(core::mem::size_of::<MouseMoveEvent>() == 0x30);
const _: () = assert!(core::mem::offset_of!(MouseMoveEvent, base) == 0x00);
const _: () = assert!(core::mem::offset_of!(MouseMoveEvent, mouse_input_x) == 0x28);
const _: () = assert!(core::mem::offset_of!(MouseMoveEvent, mouse_input_y) == 0x2C);

impl RttiType for MouseMoveEvent {
    const RTTI: VariantID = RTTI_MouseMoveEvent;
}

inherit!(MouseMoveEvent : IDEvent, base);

impl MouseMoveEvent {
    pub const RTTI: VariantID = RTTI_MouseMoveEvent;
    pub const VTABLE: &'static [VariantID] = &VTABLE_MouseMoveEvent;

    #[inline(always)]
    pub fn init(&mut self, mouse_input_x: i32, mouse_input_y: i32) {
        self.mouse_input_x = mouse_input_x;
        self.mouse_input_y = mouse_input_y;
    }

    #[inline(always)]
    pub fn init_with_user_event(
        &mut self,
        mouse_input_x: i32,
        mouse_input_y: i32,
        user_event: BSFixedString,
    ) {
        self.mouse_input_x = mouse_input_x;
        self.mouse_input_y = mouse_input_y;
        self.base.user_event = user_event;
    }
}
