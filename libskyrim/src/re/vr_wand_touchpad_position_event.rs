#![allow(non_camel_case_types)]

use core_util::inherit;

use crate::re::VRWandEvent;

/// C++ `RE::VrWandTouchpadPositionEvent`
#[repr(C)]
pub struct VrWandTouchpadPositionEvent {
    pub base: VRWandEvent, // 00
    pub unk30: u64,        // 30
    pub unk38: u64,        // 38
    pub unk40: u64,        // 40
}

const _: () = assert!(core::mem::size_of::<VrWandTouchpadPositionEvent>() == 0x48);
const _: () = assert!(core::mem::offset_of!(VrWandTouchpadPositionEvent, base) == 0x00);
const _: () = assert!(core::mem::offset_of!(VrWandTouchpadPositionEvent, unk30) == 0x30);
const _: () = assert!(core::mem::offset_of!(VrWandTouchpadPositionEvent, unk38) == 0x38);
const _: () = assert!(core::mem::offset_of!(VrWandTouchpadPositionEvent, unk40) == 0x40);

inherit!(VrWandTouchpadPositionEvent : VRWandEvent, base);

// TODO: CommonLibVR offsets headers include RTTI/VTABLE for
// `VrWandTouchpadPositionEvent`, but the current generated Rust offsets set
// does not expose them. Add `RttiType` / constants once the generator carries
// those VR-only entries through.
