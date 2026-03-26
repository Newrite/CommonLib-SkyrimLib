use crate::re::{BGSLocation, ObjectRefHandle, TESForm, TESObjectREFR};
use crate::relocation::RelocationID;

/// C++ `RE::BGSCraftItemEvent`
#[repr(C)]
pub struct BGSCraftItemEvent {
    pub workbench: ObjectRefHandle,       // 00
    pub pad04: u32,                       // 04
    pub bench_location: *mut BGSLocation, // 08
    pub created_object: *mut TESForm,     // 10
}

const _: () = assert!(core::mem::size_of::<BGSCraftItemEvent>() == 0x18);
const _: () = assert!(core::mem::offset_of!(BGSCraftItemEvent, workbench) == 0x00);
const _: () = assert!(core::mem::offset_of!(BGSCraftItemEvent, bench_location) == 0x08);
const _: () = assert!(core::mem::offset_of!(BGSCraftItemEvent, created_object) == 0x10);

impl BGSCraftItemEvent {
    crate::relocation_variable! {
        pub fn index() -> &'static mut i32 => RelocationID::new(508414, 380076)
    }

    // TODO: CommonLib constructs `workbench` from `BSPointerHandleManagerInterface<TESObjectREFR>::GetHandle`.
    // This translation keeps the event layout and relocated index, but does not yet expose a source-backed constructor
    // until the repo has a matching public handle-construction bridge for arbitrary `TESObjectREFR*`.
    #[allow(dead_code)]
    fn _workbench_type_marker(_: *mut TESObjectREFR) {}
}
