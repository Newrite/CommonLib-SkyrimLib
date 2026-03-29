#![allow(non_camel_case_types)]

use core_util::inherit;

use crate::re::{
    GArray, GFxLoadUpdateSync, GFxMovieDataDef, GFxMovieDef, GFxMovieDefBindStates, GFxResource,
    GFxStateBagImpl, GLock, GMemoryHeap, GPtr, GPtrTarget, GRefCountBase, GStatGroups,
};

/// C++ `RE::GFxMovieDefImpl::ImportedResource`
#[repr(C)]
pub struct GFxMovieDefImplImportedResource {
    pub resource: GPtr<GFxResource>,                 // 00
    pub import_data: *mut GFxMovieDefImplImportData, // 08
}

const _: () = assert!(core::mem::size_of::<GFxMovieDefImplImportedResource>() == 0x10);
const _: () = assert!(core::mem::offset_of!(GFxMovieDefImplImportedResource, resource) == 0x0);
const _: () = assert!(core::mem::offset_of!(GFxMovieDefImplImportedResource, import_data) == 0x8);

/// C++ `RE::GFxMovieDefImpl::ImportData`
#[repr(C)]
pub struct GFxMovieDefImplImportData {
    pub heap: *mut GMemoryHeap,                               // 00
    pub import_count: u32,                                    // 08
    pub pad0c: u32,                                           // 0C
    pub resource_array: *mut GFxMovieDefImplImportedResource, // 10
    pub lock: GLock,                                          // 18
    pub has_imports: bool,                                    // 40
    pub pad41: u8,                                            // 41
    pub pad42: u16,                                           // 42
    pub pad44: u32,                                           // 44
    pub movie_def: *mut GFxMovieDef,                          // 48
}

const _: () = assert!(core::mem::size_of::<GFxMovieDefImplImportData>() == 0x50);
const _: () = assert!(core::mem::offset_of!(GFxMovieDefImplImportData, heap) == 0x0);
const _: () = assert!(core::mem::offset_of!(GFxMovieDefImplImportData, lock) == 0x18);
const _: () = assert!(core::mem::offset_of!(GFxMovieDefImplImportData, movie_def) == 0x48);

/// C++ `RE::GFxMovieDefImpl::BindTaskData`
#[repr(C)]
pub struct GFxMovieDefImplBindTaskData {
    pub base:
        GRefCountBase<GFxMovieDefImplBindTaskData, { GStatGroups::kGStat_Default_Mem as u32 }>, // 00
    pub bind_data_heap: *mut GMemoryHeap,          // 10
    pub movie_data_resource: *mut GFxMovieDataDef, // 18
    pub unk20: *mut GFxMovieDefImpl,               // 20
    pub unk28: u32,                                // 28
    pub pad2c: u32,                                // 2C
    pub import_data: GFxMovieDefImplImportData,    // 30
    pub imported_movies: GArray<*mut GFxMovieDef>, // 80
    pub lock: GLock,                               // 98
    pub unkc0: u64,                                // C0
    pub unkc8: u64,                                // C8
    pub unkd0: u64,                                // D0
    pub task_state: u32,                           // D8
    pub paddc: u32,                                // DC
    pub update_sync: GPtr<GFxLoadUpdateSync>,      // E0
    pub loading_frame: u32,                        // E8
    pub bytes_loaded: u32,                         // EC
    pub has_error: bool,                           // F0
    pub padf1: u8,                                 // F1
    pub padf2: u16,                                // F2
    pub padf4: u32,                                // F4
}

const _: () = assert!(core::mem::size_of::<GFxMovieDefImplBindTaskData>() == 0xF8);
const _: () = assert!(core::mem::offset_of!(GFxMovieDefImplBindTaskData, base) == 0x0);
const _: () = assert!(core::mem::offset_of!(GFxMovieDefImplBindTaskData, import_data) == 0x30);
const _: () = assert!(core::mem::offset_of!(GFxMovieDefImplBindTaskData, imported_movies) == 0x80);
const _: () = assert!(core::mem::offset_of!(GFxMovieDefImplBindTaskData, update_sync) == 0xE0);

inherit!(GFxMovieDefImplBindTaskData : GRefCountBase<GFxMovieDefImplBindTaskData, { GStatGroups::kGStat_Default_Mem as u32 }>, base);

impl GFxMovieDefImplBindTaskData {
    crate::virtual_method! { pub const VFUNC_DTOR: usize = 0x00; pub fn dtor() }
}

impl GPtrTarget for GFxMovieDefImplBindTaskData {
    #[inline(always)]
    fn gptr_add_ref(&self) {
        unsafe {
            (*(core::ptr::from_ref(self).cast_mut())).base.add_ref();
        }
    }

    #[inline(always)]
    fn gptr_release(&self) {
        unsafe {
            (*(core::ptr::from_ref(self).cast_mut())).base.release();
        }
    }
}

/// C++ `RE::GFxMovieDefImpl`
#[repr(C)]
pub struct GFxMovieDefImpl {
    pub base: GFxMovieDef,                                 // 00
    pub state_bag_impl: GPtr<GFxStateBagImpl>,             // 20
    pub unk28: *mut core::ffi::c_void,                     // 28
    pub movie_bind_states: GPtr<GFxMovieDefBindStates>,    // 30
    pub bind_task_data: GPtr<GFxMovieDefImplBindTaskData>, // 38
}

const _: () = assert!(core::mem::size_of::<GFxMovieDefImpl>() == 0x40);
const _: () = assert!(core::mem::offset_of!(GFxMovieDefImpl, base) == 0x0);
const _: () = assert!(core::mem::offset_of!(GFxMovieDefImpl, state_bag_impl) == 0x20);
const _: () = assert!(core::mem::offset_of!(GFxMovieDefImpl, movie_bind_states) == 0x30);
const _: () = assert!(core::mem::offset_of!(GFxMovieDefImpl, bind_task_data) == 0x38);

inherit!(GFxMovieDefImpl : GFxMovieDef, base);

impl GFxMovieDefImpl {
    crate::virtual_method! { pub const VFUNC_DTOR: usize = 0x00; pub fn dtor() }
    crate::virtual_method! { pub const VFUNC_GET_FONT: usize = 0x1C; pub fn get_font(name: *const i8, font_flags: u32, arg3: *mut i32) -> *mut GFxResource }
}
