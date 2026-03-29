#![allow(non_camel_case_types)]

use core_util::EnumSet;
use core_util::inherit;

use crate::re::{
    GASGlobalContext, GColor, GFxKeyboardState, GFxMovieDef, GFxMovieDefMemoryContextImpl,
    GFxMovieView, GFxSprite, GMatrix3D, GMemoryHeap, GPoint3F, GPointF, GRectF, GStatBag,
    GViewport,
};

/// C++ `RE::GFxActionPriority`
#[repr(C)]
pub struct GFxActionPriority {
    pub unk00: u8,
}

const _: () = assert!(core::mem::size_of::<GFxActionPriority>() == 0x1);

/// C++ `RE::GFxMovieRoot::Flag`
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GFxMovieRootFlag {
    kNone = 0,
    kDirty = 1 << 10,
    kMovieFocused = 1 << 18,
    kPaused = 1 << 20,
}

core_util::impl_enumset_type!(GFxMovieRootFlag => u32);

/// C++ `RE::GFxMovieRoot`
#[repr(C)]
pub struct GFxMovieRoot {
    pub base: GFxMovieView,                                    // 0000
    pub action_priority: GFxActionPriority,                    // 0018
    pub pad19: [u8; 7],                                        // 0019
    pub unk0020: u64,                                          // 0020
    pub memory_context: *mut GFxMovieDefMemoryContextImpl,     // 0028
    pub unk0030: u64,                                          // 0030
    pub heap: *mut GMemoryHeap,                                // 0038
    pub unk0040: u64,                                          // 0040
    pub unk0048: u64,                                          // 0048
    pub unk0050: u64,                                          // 0050
    pub timeline: *mut GFxSprite,                              // 0058
    pub movie_def: *mut GFxMovieDef,                           // 0060
    pub unk0068: u64,                                          // 0068
    pub unk0070: u64,                                          // 0070
    pub viewport: GViewport,                                   // 0078
    pub unk00b0: u64,                                          // 00B0
    pub unk00b8: u64,                                          // 00B8
    pub view_scale_mode: crate::re::GFxMovieViewScaleModeType, // 00C0
    pub view_alignment: crate::re::GFxMovieViewAlignType,      // 00C4
    pub visible_frame_rect: GRectF,                            // 00C8
    pub unk00d8: u64,                                          // 00D8
    pub safe_rect: GRectF,                                     // 00E0
    pub unk00f0: u64,                                          // 00F0
    pub unk00f8: u64,                                          // 00F8
    pub perspective3d: *mut GMatrix3D,                         // 0100
    pub unk0108: u64,                                          // 0108
    pub unk0110: u64,                                          // 0110
    pub unk0118: [u64; (0x09A0 - 0x0118) / 8],                 // 0118
    pub background_color: GColor,                              // 09A0
    pub unk09a4: u32,                                          // 09A4
    pub unk09a8: [u64; (0x0A68 - 0x09A8) / 8],                 // 09A8
    pub mouse_cursor_count: u32,                               // 0A68
    pub controller_count: u32,                                 // 0A6C
    pub user_data: *mut core::ffi::c_void,                     // 0A70
    pub unk0a78: u64,                                          // 0A78
    pub keyboard_state: GFxKeyboardState,                      // 0A80
    pub unk1108: [u64; (0x24A0 - 0x1108) / 8],                 // 1108
    pub global_context: *mut GASGlobalContext,                 // 24A0
    pub unk24a8: [u64; (0x25E0 - 0x24A8) / 8],                 // 24A8
    pub flags: EnumSet<GFxMovieRootFlag, u32>,                 // 25E0
    pub unk25e4: u32,                                          // 25E4
    pub unk25e8: [u64; (0x2B48 - 0x25E8) / 8],                 // 25E8
    pub focus_group: u32,                                      // 2B48
    pub controller_groups: [u8; 16],                           // 2B4C
    // TODO: `GFxMovieRoot.h`'s tail comments currently overlap:
    // `controllerGroups[16]` at `0x2B4C` would extend through `0x2B5B`, but
    // the same header also labels an `unk2B54` field and an `unk2B58` array.
    // Keep the source-backed `controller_groups` surface plus the previously
    // validated opaque tail blob until an upstream source/cpp proves the real
    // split of this overlapping region.
    pub unk2b5c: [u8; 0x2BF0 - 0x2B5C], // 2B5C
}

const _: () = assert!(core::mem::size_of::<GFxMovieRoot>() == 0x2BF0);
const _: () = assert!(core::mem::offset_of!(GFxMovieRoot, base) == 0x0);
const _: () = assert!(core::mem::offset_of!(GFxMovieRoot, action_priority) == 0x18);
const _: () = assert!(core::mem::offset_of!(GFxMovieRoot, keyboard_state) == 0x0A80);
const _: () = assert!(core::mem::offset_of!(GFxMovieRoot, global_context) == 0x24A0);
const _: () = assert!(core::mem::offset_of!(GFxMovieRoot, flags) == 0x25E0);

inherit!(GFxMovieRoot : GFxMovieView, base);
inherit!(GFxMovieRoot => GFxActionPriority, action_priority);

impl GFxMovieRoot {
    crate::virtual_method! { pub const VFUNC_UNK_46: usize = 0x46; pub fn unk_46() }
    crate::virtual_method! { pub const VFUNC_UNK_47: usize = 0x47; pub fn unk_47() }
    crate::virtual_method! { pub const VFUNC_UNK_48: usize = 0x48; pub fn unk_48() }
}
