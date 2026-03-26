#![allow(non_camel_case_types)]

use core_util::inherit;

use crate::offsets::offsets_rtti::RTTI_KinectEvent;
use crate::offsets::offsets_vtable::VTABLE_KinectEvent;
use crate::re::{BSFixedString, IDEvent};
use crate::relocation::{RttiType, VariantID};

/// C++ `RE::KinectEvent`
#[repr(C)]
pub struct KinectEvent {
    pub base: IDEvent,        // 00
    pub heard: BSFixedString, // 28
}

const _: () = assert!(core::mem::size_of::<KinectEvent>() == 0x30);
const _: () = assert!(core::mem::offset_of!(KinectEvent, base) == 0x00);
const _: () = assert!(core::mem::offset_of!(KinectEvent, heard) == 0x28);

impl RttiType for KinectEvent {
    const RTTI: VariantID = RTTI_KinectEvent;
}

inherit!(KinectEvent : IDEvent, base);

impl KinectEvent {
    pub const RTTI: VariantID = RTTI_KinectEvent;
    pub const VTABLE: &'static [VariantID] = &VTABLE_KinectEvent;

    #[inline(always)]
    pub fn init(&mut self, user_event: BSFixedString, heard: BSFixedString) {
        self.base.user_event = user_event;
        self.heard = heard;
    }
}
