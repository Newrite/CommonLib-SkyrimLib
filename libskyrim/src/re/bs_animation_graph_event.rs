use crate::re::{BSFixedString, TESObjectREFR};

/// C++ `RE::BSAnimationGraphEvent`
#[repr(C)]
pub struct BSAnimationGraphEvent {
    pub tag: BSFixedString,         // 00
    pub holder: *mut TESObjectREFR, // 08
    pub payload: BSFixedString,     // 10
}

const _: () = assert!(core::mem::size_of::<BSAnimationGraphEvent>() == 0x18);
const _: () = assert!(core::mem::offset_of!(BSAnimationGraphEvent, tag) == 0x00);
const _: () = assert!(core::mem::offset_of!(BSAnimationGraphEvent, holder) == 0x08);
const _: () = assert!(core::mem::offset_of!(BSAnimationGraphEvent, payload) == 0x10);

impl BSAnimationGraphEvent {
    #[inline(always)]
    pub fn new(tag: BSFixedString, holder: *mut TESObjectREFR, payload: BSFixedString) -> Self {
        Self {
            tag,
            holder,
            payload,
        }
    }
}
