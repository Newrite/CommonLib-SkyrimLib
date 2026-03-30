#![allow(non_camel_case_types)]

use crate::re::{
    GAtomicInt, GImageBase, GImageImageFormat, GNewOverrideBase, GPoint, GRect, GRenderer,
    GStatRenderer,
};

/// C++ `RE::GTexture::MapFlags`
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GTextureMapFlags {
    kNone = 0,
    kKeepOld = 1,
}

/// C++ `RE::GTexture::ImageTexUsage`
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GTextureImageTexUsage {
    kWrap = 1 << 0,
    kUpdate = 1 << 4,
    kMap = 1 << 5,
    kRenderTarget = 1 << 6,
}

/// C++ `RE::GTexture::UpdateRect`
#[repr(C)]
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct GTextureUpdateRect {
    pub dest: GPoint<i32>, // 00
    pub src: GRect<i32>,   // 08
}

const _: () = assert!(core::mem::size_of::<GTextureUpdateRect>() == 0x18);

/// C++ `RE::GTexture::MapRect`
#[repr(C)]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct GTextureMapRect {
    pub width: u32,    // 00
    pub height: u32,   // 04
    pub data: *mut u8, // 08
    pub pitch: u32,    // 10
    pub pad14: u32,    // 14
}

const _: () = assert!(core::mem::size_of::<GTextureMapRect>() == 0x18);

/// C++ `RE::GTexture::ChangeHandler::EventType`
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GTextureChangeHandlerEventType {
    kDataChange = 0,
    kDataLost = 1,
    kRendererReleased = 2,
}

/// C++ `RE::GTexture::ChangeHandler`
#[repr(C)]
pub struct GTextureChangeHandler {
    pub vtable: *const usize, // 00
}

const _: () = assert!(core::mem::size_of::<GTextureChangeHandler>() == 0x8);

impl GTextureChangeHandler {
    crate::virtual_method! {
        pub const VFUNC_DTOR: usize = 0x0;
        pub fn dtor()
    }

    crate::virtual_method! {
        pub const VFUNC_ON_CHANGE: usize = 0x1;
        pub fn on_change(renderer: *mut GRenderer, change_type: GTextureChangeHandlerEventType)
    }

    crate::virtual_method! {
        pub const VFUNC_RECREATE: usize = 0x2;
        pub fn recreate(renderer: *mut GRenderer) -> bool
    }
}

/// C++ `RE::GTexture`
#[repr(C)]
pub struct GTexture {
    pub vtable: *const usize,       // 00
    pub ref_count: GAtomicInt<i32>, // 08
}

const _: () = assert!(core::mem::size_of::<GTexture>() == 0x10);
const _: () = assert!(core::mem::offset_of!(GTexture, vtable) == 0x0);
const _: () = assert!(core::mem::offset_of!(GTexture, ref_count) == 0x8);

impl GTexture {
    pub const STAT_TYPE: u32 = GNewOverrideBase::<{ GStatRenderer::MEM as u32 }>::STAT_TYPE;

    crate::virtual_method! {
        pub const VFUNC_DTOR: usize = 0x0;
        pub fn dtor()
    }

    crate::virtual_method! {
        pub const VFUNC_INIT_TEXTURE: usize = 0x1;
        pub fn init_texture(image: *mut GImageBase, usage: GTextureImageTexUsage) -> bool
    }

    crate::virtual_method! {
        pub const VFUNC_INIT_DYNAMIC_TEXTURE: usize = 0x2;
        pub fn init_dynamic_texture(
            width: i32,
            height: i32,
            format: GImageImageFormat,
            mipmaps: i32,
            usage: GTextureImageTexUsage
        ) -> bool
    }

    crate::virtual_method! {
        pub const VFUNC_UPDATE: usize = 0x3;
        pub fn update(level: i32, num: i32, rects: *const GTextureUpdateRect, image: *const GImageBase)
    }

    crate::virtual_method! {
        pub const VFUNC_MAP: usize = 0x4;
        pub fn map(level: i32, num: i32, maps: *mut GTextureMapRect, flags: GTextureMapFlags) -> i32
    }

    crate::virtual_method! {
        pub const VFUNC_UNMAP: usize = 0x5;
        pub fn unmap(level: i32, num: i32, maps: *mut GTextureMapRect, flags: GTextureMapFlags) -> bool
    }

    crate::virtual_method! {
        pub const VFUNC_GET_RENDERER: usize = 0x6;
        pub fn get_renderer() -> *mut GRenderer
    }

    crate::virtual_method! {
        pub const VFUNC_IS_DATA_VALID: usize = 0x7;
        pub fn is_data_valid() -> bool
    }

    crate::virtual_method! {
        pub const VFUNC_GET_USER_DATA: usize = 0x8;
        pub fn get_user_data() -> *mut core::ffi::c_void
    }

    crate::virtual_method! {
        pub const VFUNC_SET_USER_DATA: usize = 0x9;
        pub fn set_user_data(data: *mut core::ffi::c_void)
    }

    crate::virtual_method! {
        pub const VFUNC_ADD_CHANGE_HANDLER: usize = 0xA;
        pub fn add_change_handler(handler: *mut GTextureChangeHandler)
    }

    crate::virtual_method! {
        pub const VFUNC_REMOVE_CHANGE_HANDLER: usize = 0xB;
        pub fn remove_change_handler(handler: *mut GTextureChangeHandler)
    }
}
