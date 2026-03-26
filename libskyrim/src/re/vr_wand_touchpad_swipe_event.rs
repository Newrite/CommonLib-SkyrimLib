#![allow(non_camel_case_types)]

use core_util::inherit;

use crate::re::VRWandEvent;

/// C++ `RE::VrWandTouchpadSwipeEvent`
#[repr(C)]
pub struct VrWandTouchpadSwipeEvent {
    pub base: VRWandEvent, // 00
    pub unk30: u64,        // 30
    pub unk38: u64,        // 38
}

const _: () = assert!(core::mem::size_of::<VrWandTouchpadSwipeEvent>() == 0x40);
const _: () = assert!(core::mem::offset_of!(VrWandTouchpadSwipeEvent, base) == 0x00);
const _: () = assert!(core::mem::offset_of!(VrWandTouchpadSwipeEvent, unk30) == 0x30);
const _: () = assert!(core::mem::offset_of!(VrWandTouchpadSwipeEvent, unk38) == 0x38);

inherit!(VrWandTouchpadSwipeEvent : VRWandEvent, base);

// TODO: CommonLibVR offsets headers include RTTI/VTABLE for
// `VrWandTouchpadSwipeEvent`, but the current generated Rust offsets set does
// not expose them. Add `RttiType` / constants once the generator carries those
// VR-only entries through.
