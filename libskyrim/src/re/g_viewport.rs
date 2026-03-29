#![allow(non_camel_case_types)]

use core_util::EnumSet;

/// C++ `RE::GViewport::Flag`
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GViewportFlag {
    kNone = 0,
    kIsRenderTexture = 1,
    kAlphaComposite = 2,
    kUseScissorRect = 4,
    kNoSetState = 8,
    kRenderTextureAlpha = (Self::kIsRenderTexture as u32) | (Self::kAlphaComposite as u32),
}

core_util::impl_enumset_type!(GViewportFlag => u32);

/// C++ `RE::GViewport`
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct GViewport {
    pub buffer_width: i32,                  // 00
    pub buffer_height: i32,                 // 04
    pub left: i32,                          // 08
    pub top: i32,                           // 0C
    pub width: i32,                         // 10
    pub height: i32,                        // 14
    pub scissor_left: i32,                  // 18
    pub scissor_top: i32,                   // 1C
    pub scissor_width: i32,                 // 20
    pub scissor_height: i32,                // 24
    pub scale: f32,                         // 28
    pub aspect_ratio: f32,                  // 2C
    pub flags: EnumSet<GViewportFlag, u32>, // 30
    pub pad34: u32,                         // 34
}

const _: () = assert!(core::mem::size_of::<GViewport>() == 0x38);
const _: () = assert!(core::mem::offset_of!(GViewport, buffer_width) == 0x0);
const _: () = assert!(core::mem::offset_of!(GViewport, buffer_height) == 0x4);
const _: () = assert!(core::mem::offset_of!(GViewport, left) == 0x8);
const _: () = assert!(core::mem::offset_of!(GViewport, top) == 0xC);
const _: () = assert!(core::mem::offset_of!(GViewport, width) == 0x10);
const _: () = assert!(core::mem::offset_of!(GViewport, height) == 0x14);
const _: () = assert!(core::mem::offset_of!(GViewport, scissor_left) == 0x18);
const _: () = assert!(core::mem::offset_of!(GViewport, scissor_top) == 0x1C);
const _: () = assert!(core::mem::offset_of!(GViewport, scissor_width) == 0x20);
const _: () = assert!(core::mem::offset_of!(GViewport, scissor_height) == 0x24);
const _: () = assert!(core::mem::offset_of!(GViewport, scale) == 0x28);
const _: () = assert!(core::mem::offset_of!(GViewport, aspect_ratio) == 0x2C);
const _: () = assert!(core::mem::offset_of!(GViewport, flags) == 0x30);

impl GViewport {
    #[inline(always)]
    pub fn new() -> Self {
        Self {
            buffer_width: 0,
            buffer_height: 0,
            left: 0,
            top: 0,
            width: 1,
            height: 1,
            scissor_left: 0,
            scissor_top: 0,
            scissor_width: 0,
            scissor_height: 0,
            scale: 1.0,
            aspect_ratio: 1.0,
            flags: EnumSet::from_underlying(GViewportFlag::kNone as u32),
            pad34: 0,
        }
    }

    #[inline(always)]
    pub fn with_viewport(
        buffer_width: i32,
        buffer_height: i32,
        left: i32,
        top: i32,
        width: i32,
        height: i32,
        flags: GViewportFlag,
    ) -> Self {
        Self {
            buffer_width,
            buffer_height,
            left,
            top,
            width,
            height,
            scissor_left: 0,
            scissor_top: 0,
            scissor_width: 0,
            scissor_height: 0,
            scale: 1.0,
            aspect_ratio: 1.0,
            flags: EnumSet::from_underlying(flags as u32),
            pad34: 0,
        }
    }

    #[inline(always)]
    pub fn with_scissor(
        buffer_width: i32,
        buffer_height: i32,
        left: i32,
        top: i32,
        width: i32,
        height: i32,
        scissor_left: i32,
        scissor_top: i32,
        scissor_width: i32,
        scissor_height: i32,
        flags: GViewportFlag,
    ) -> Self {
        Self {
            buffer_width,
            buffer_height,
            left,
            top,
            width,
            height,
            scissor_left,
            scissor_top,
            scissor_width,
            scissor_height,
            scale: 1.0,
            aspect_ratio: 1.0,
            flags: EnumSet::from_underlying(flags as u32),
            pad34: 0,
        }
    }

    #[inline(always)]
    pub fn with_scissor_scale_ratio(
        buffer_width: i32,
        buffer_height: i32,
        left: i32,
        top: i32,
        width: i32,
        height: i32,
        scissor_left: i32,
        scissor_top: i32,
        scissor_width: i32,
        scissor_height: i32,
        scale: f32,
        aspect_ratio: f32,
        flags: GViewportFlag,
    ) -> Self {
        Self {
            buffer_width,
            buffer_height,
            left,
            top,
            width,
            height,
            scissor_left,
            scissor_top,
            scissor_width,
            scissor_height,
            scale,
            aspect_ratio,
            flags: EnumSet::from_underlying(
                (flags as u32) | (GViewportFlag::kUseScissorRect as u32),
            ),
            pad34: 0,
        }
    }

    #[inline(always)]
    pub fn set_viewport(
        &mut self,
        buffer_width: i32,
        buffer_height: i32,
        left: i32,
        top: i32,
        width: i32,
        height: i32,
        flags: GViewportFlag,
    ) {
        self.buffer_width = buffer_width;
        self.buffer_height = buffer_height;
        self.left = left;
        self.top = top;
        self.width = width;
        self.height = height;
        self.flags = EnumSet::from_underlying(flags as u32);
        self.scissor_left = 0;
        self.scissor_top = 0;
        self.scissor_width = 0;
        self.scissor_height = 0;
        self.scale = 1.0;
        self.aspect_ratio = 1.0;
    }

    #[inline(always)]
    pub fn set_scissor_rect(
        &mut self,
        scissor_left: i32,
        scissor_top: i32,
        scissor_width: i32,
        scissor_height: i32,
    ) {
        self.scissor_left = scissor_left;
        self.scissor_top = scissor_top;
        self.scissor_width = scissor_width;
        self.scissor_height = scissor_height;
        self.flags = EnumSet::from_underlying(
            self.flags.underlying() | (GViewportFlag::kUseScissorRect as u32),
        );
    }
}

impl Default for GViewport {
    #[inline(always)]
    fn default() -> Self {
        Self::new()
    }
}
