use core::ffi::c_void;

use crate::re::{IPostAnimationChannelUpdateFunctor, NiPoint3, TESObjectREFR};

/// C++ `RE::BSAnimationUpdateData`
#[repr(C)]
pub struct BSAnimationUpdateData {
    pub delta_time: f32,                                         // 00
    pub pad04: u32,                                              // 04
    pub unk_function_ptr: *mut c_void,                           // 08
    pub refr: *mut TESObjectREFR,                                // 10
    pub optional_eye_position: *mut NiPoint3,                    // 18
    pub update_functor: *mut IPostAnimationChannelUpdateFunctor, // 20
    pub flags: u16,                                              // 28
    pub unk2a: bool,                                             // 2A
    pub unk2b: bool,                                             // 2B
    pub unk2c: bool,                                             // 2C
    pub unk2d: bool,                                             // 2D
    pub unk2e: bool,                                             // 2E
    pub unk2f: bool,                                             // 2F
}

const _: () = assert!(core::mem::size_of::<BSAnimationUpdateData>() == 0x30);
const _: () = assert!(core::mem::offset_of!(BSAnimationUpdateData, delta_time) == 0x00);
const _: () = assert!(core::mem::offset_of!(BSAnimationUpdateData, refr) == 0x10);
const _: () = assert!(core::mem::offset_of!(BSAnimationUpdateData, optional_eye_position) == 0x18);
const _: () = assert!(core::mem::offset_of!(BSAnimationUpdateData, update_functor) == 0x20);
const _: () = assert!(core::mem::offset_of!(BSAnimationUpdateData, flags) == 0x28);
