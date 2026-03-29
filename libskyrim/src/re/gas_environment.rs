#![allow(non_camel_case_types)]

use core_util::inherit;

use crate::re::{GASGlobalContext, GFxLogBase, GFxSprite, GFxStatMovieViews, GNewOverrideBase};

/// C++ `RE::GASEnvironment`
#[repr(C)]
pub struct GASEnvironment {
    pub log_base: GFxLogBase<GASEnvironment>, // 000
    pub new_override_base:
        GNewOverrideBase<{ GFxStatMovieViews::kGFxStatMV_ActionScript_Mem as u32 }>, // 008
    pub pad008: u64,                          // 008
    pub unk010: *mut core::ffi::c_void,       // 010
    pub unk018: u64,                          // 018
    pub unk020: *mut core::ffi::c_void,       // 020
    pub unk028: *mut core::ffi::c_void,       // 028
    pub unk030: u64,                          // 030
    pub unk038: *mut core::ffi::c_void,       // 038
    pub unk040: *mut core::ffi::c_void,       // 040
    pub unk048: *mut core::ffi::c_void,       // 048
    pub unk050: [u64; 19],                    // 050
    pub sprite: *mut GFxSprite,               // 0E8
    pub global_context: *mut GASGlobalContext, // 0F0
    pub version: u8,                          // 0F8
    pub pad0f9: [u8; 7],                      // 0F9
    pub pad100: [u64; 20],                    // 100
}

const _: () = assert!(core::mem::size_of::<GASEnvironment>() == 0x1A0);
const _: () = assert!(core::mem::offset_of!(GASEnvironment, log_base) == 0x0);
const _: () = assert!(core::mem::offset_of!(GASEnvironment, sprite) == 0xE8);
const _: () = assert!(core::mem::offset_of!(GASEnvironment, global_context) == 0xF0);

inherit!(GASEnvironment : GFxLogBase<GASEnvironment>, log_base);
inherit!(GASEnvironment => GNewOverrideBase<{ GFxStatMovieViews::kGFxStatMV_ActionScript_Mem as u32 }>, new_override_base);

impl GASEnvironment {
    crate::virtual_method! { pub const VFUNC_DTOR: usize = 0x00; pub fn dtor() }

    crate::virtual_method! {
        pub const VFUNC_IS_VERBOSE_ACTION_ERRORS: usize = 0x01;
        pub fn is_verbose_action_errors() -> bool
    }
}
