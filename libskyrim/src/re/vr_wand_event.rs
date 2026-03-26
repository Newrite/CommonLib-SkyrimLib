#![allow(non_camel_case_types)]

use core_util::inherit;

use crate::re::IDEvent;

/// C++ `RE::VRWandEvent`
#[repr(C)]
pub struct VRWandEvent {
    pub base: IDEvent, // 00
    pub unk_vr28: i32, // 28
    pub pad_vr2c: u32, // 2C
}

const _: () = assert!(core::mem::size_of::<VRWandEvent>() == 0x30);
const _: () = assert!(core::mem::offset_of!(VRWandEvent, base) == 0x00);
const _: () = assert!(core::mem::offset_of!(VRWandEvent, unk_vr28) == 0x28);
const _: () = assert!(core::mem::offset_of!(VRWandEvent, pad_vr2c) == 0x2C);

inherit!(VRWandEvent : IDEvent, base);

impl VRWandEvent {
    // TODO: CommonLibVR publishes RTTI/VTABLE entries for `VRWandEvent` in its
    // offsets headers, but the current generated Rust offsets layer does not
    // expose matching `RTTI_VRWandEvent` / `VTABLE_VRWandEvent` constants yet.
    // Add `RttiType` / constants here once the generator captures those VR-only
    // offsets instead of hard-coding them locally.

    #[inline(always)]
    pub fn as_vr_wand_event(&self) -> Option<&VRWandEvent> {
        crate::runtime::is_vr().then_some(self)
    }

    #[inline(always)]
    pub fn as_vr_wand_event_mut(&mut self) -> Option<&mut VRWandEvent> {
        crate::runtime::is_vr().then_some(self)
    }
}

pub trait VRWandEventExt {
    fn as_vr_wand_event(&self) -> Option<&VRWandEvent>;
    fn as_vr_wand_event_mut(&mut self) -> Option<&mut VRWandEvent>;
}

impl<T> VRWandEventExt for T
where
    T: AsRef<VRWandEvent> + AsMut<VRWandEvent>,
{
    #[inline(always)]
    fn as_vr_wand_event(&self) -> Option<&VRWandEvent> {
        VRWandEvent::as_vr_wand_event(self.as_ref())
    }

    #[inline(always)]
    fn as_vr_wand_event_mut(&mut self) -> Option<&mut VRWandEvent> {
        VRWandEvent::as_vr_wand_event_mut(self.as_mut())
    }
}
