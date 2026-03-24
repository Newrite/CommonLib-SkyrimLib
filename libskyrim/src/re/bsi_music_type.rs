use core::ffi::c_void;

use crate::offsets::offsets_rtti::RTTI_BSIMusicType;
use crate::offsets::offsets_vtable::VTABLE_BSIMusicType;
use crate::re::bst_array::BSTArray;
use crate::relocation::{RttiType, VariantID};
use crate::virtual_method;
use core_util::EnumSet;

/// C++ `RE::BSIMusicType::MST`
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MST {
    None = 0,
    PlaysOnce = 1 << 0,
    AbruptTransition = 1 << 1,
    CycleTracks = 1 << 2,
    UseTrackOrder = 1 << 3,
    RemovalQueued = 1 << 4,
    PlaysOver = 1 << 5,
    DoesntQueue = 1 << 6,
}

/// C++ `RE::BSIMusicType::MUSIC_STATUS`
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MusicStatus {
    Inactive = 0,
    Playing = 1,
    Paused = 2,
    Finishing = 3,
    Finished = 4,
}

core_util::impl_enumset_type!(MST => u32);
core_util::impl_enumset_type!(MusicStatus => u32);

/// C++ `RE::BSIMusicType`
#[repr(C)]
pub struct BSIMusicType {
    pub vtable: *const usize,                   // 00
    pub flags: EnumSet<MST, u32>,               // 08 - FNAM
    pub priority: u8,                           // 0C
    pub padding: u8,                            // 0D
    pub ducks_other_music_by: u16,              // 0E
    pub fade_time: f32,                         // 10 - WNAM
    pub current_track_index: u32,               // 14
    pub track_history: BSTArray<u32>,           // 18
    pub tracks: BSTArray<*mut c_void>,          // 30 - TNAM - BSIMusicTrack*
    pub type_status: EnumSet<MusicStatus, u32>, // 48
    pub pad4c: u32,                             // 4C
}

const _: () = assert!(core::mem::size_of::<BSIMusicType>() == 0x50);
const _: () = assert!(core::mem::offset_of!(BSIMusicType, flags) == 0x08);
const _: () = assert!(core::mem::offset_of!(BSIMusicType, track_history) == 0x18);
const _: () = assert!(core::mem::offset_of!(BSIMusicType, tracks) == 0x30);

impl RttiType for BSIMusicType {
    const RTTI: VariantID = RTTI_BSIMusicType;
}

impl BSIMusicType {
    pub const RTTI: VariantID = RTTI_BSIMusicType;
    pub const VTABLE: &'static [VariantID] = &VTABLE_BSIMusicType;

    virtual_method! {
        pub const VFUNC_DO_UPDATE: usize = 0x00;
        pub fn do_update()
    }

    virtual_method! {
        pub const VFUNC_DO_PLAY: usize = 0x01;
        pub fn do_play()
    }

    virtual_method! {
        pub const VFUNC_DO_PAUSE: usize = 0x02;
        pub fn do_pause()
    }

    virtual_method! {
        pub const VFUNC_DO_FINISH: usize = 0x03;
        pub fn do_finish(a_arg1: bool)
    }

    virtual_method! {
        pub const VFUNC_DO_APPLY_DUCKING_ATTENUATION: usize = 0x04;
        pub fn do_apply_ducking_attenuation(a_ducking: u16)
    }

    virtual_method! {
        pub const VFUNC_DO_CLEAR_DUCKING: usize = 0x05;
        pub fn do_clear_ducking()
    }

    virtual_method! {
        pub const VFUNC_DO_PREPARE: usize = 0x06;
        pub fn do_prepare()
    }
}
