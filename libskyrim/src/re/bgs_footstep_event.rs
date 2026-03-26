use core_util::inherit;

use crate::re::{BGSActorEvent, BSFixedString};

/// C++ `RE::BGSFootstepEvent`
#[repr(C)]
pub struct BGSFootstepEvent {
    pub base: BGSActorEvent, // 00
    pub pad04: u32,          // 04
    pub tag: BSFixedString,  // 08
}

const _: () = assert!(core::mem::size_of::<BGSFootstepEvent>() == 0x10);
const _: () = assert!(core::mem::offset_of!(BGSFootstepEvent, base) == 0x00);
const _: () = assert!(core::mem::offset_of!(BGSFootstepEvent, tag) == 0x08);

inherit!(BGSFootstepEvent : BGSActorEvent, base);
