#![allow(non_camel_case_types)]

use core_util::inherit;

use crate::offsets::offsets_rtti::RTTI_IDEvent;
use crate::offsets::offsets_vtable::VTABLE_IDEvent;
use crate::re::{BSFixedString, InputEvent};
use crate::relocation::{RttiType, VariantID};

/// C++ `RE::IDEvent`
#[repr(C)]
pub struct IDEvent {
    pub base: InputEvent,          // 00
    pub user_event: BSFixedString, // 18
    pub id_code: u32,              // 20
    pub pad24: u32,                // 24
}

const _: () = assert!(core::mem::size_of::<IDEvent>() == 0x28);
const _: () = assert!(core::mem::offset_of!(IDEvent, base) == 0x00);
const _: () = assert!(core::mem::offset_of!(IDEvent, user_event) == 0x18);
const _: () = assert!(core::mem::offset_of!(IDEvent, id_code) == 0x20);

impl RttiType for IDEvent {
    const RTTI: VariantID = RTTI_IDEvent;
}

inherit!(IDEvent : InputEvent, base);

impl IDEvent {
    pub const RTTI: VariantID = RTTI_IDEvent;
    pub const VTABLE: &'static [VariantID] = &VTABLE_IDEvent;

    crate::virtual_method! {
        pub const VFUNC_HAS_ID_CODE: usize = 0x01;
        pub fn has_id_code() -> bool
    }

    crate::virtual_method! {
        pub const VFUNC_Q_USER_EVENT: usize = 0x02;
        pub fn q_user_event() -> *const BSFixedString
    }

    #[inline(always)]
    pub const fn get_id_code(&self) -> u32 {
        self.id_code
    }

    #[inline(always)]
    pub const fn get_user_event(&self) -> &BSFixedString {
        &self.user_event
    }
}

pub trait IDEventExt {
    fn has_id_code(&self) -> bool;
    fn q_user_event(&self) -> *const BSFixedString;
    fn get_id_code(&self) -> u32;
    fn get_user_event(&self) -> &BSFixedString;
}

impl<T: AsRef<IDEvent> + AsMut<IDEvent>> IDEventExt for T {
    #[inline(always)]
    fn has_id_code(&self) -> bool {
        IDEvent::has_id_code(self.as_ref())
    }

    #[inline(always)]
    fn q_user_event(&self) -> *const BSFixedString {
        IDEvent::q_user_event(self.as_ref())
    }

    #[inline(always)]
    fn get_id_code(&self) -> u32 {
        IDEvent::get_id_code(self.as_ref())
    }

    #[inline(always)]
    fn get_user_event(&self) -> &BSFixedString {
        IDEvent::get_user_event(self.as_ref())
    }
}
