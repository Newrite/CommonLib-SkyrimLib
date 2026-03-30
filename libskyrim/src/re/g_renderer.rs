#![allow(non_camel_case_types)]

use core::ffi::c_void;

use core_util::{EnumSet, inherit};

use crate::re::{
    GColor, GList, GMatrix2D, GMatrix3D, GPointF, GPtrTarget, GRectF, GRefCountBase, GRenderTarget,
    GRendererEventHandler, GStatBag, GStatRenderer, GTexture, GViewport,
};

/// C++ `RE::GRenderer::BlendType`
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GRendererBlendType {
    kNone = 0,
    kNormal = 1,
    kLayer = 2,
    kMultiply = 3,
    kScreen = 4,
    kLighten = 5,
    kDarken = 6,
    kDifference = 7,
    kAdd = 8,
    kSubtract = 9,
    kInvert = 10,
    kAlpha = 11,
    kErase = 12,
    kOverlay = 13,
    kHardLight = 14,
}

/// C++ `RE::GRenderer::ResizeImageType`
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GRendererResizeImageType {
    kRGBToRGB = 0,
    kRGBAToRGBA = 1,
    kRGBToRGBA = 2,
    kGray = 3,
}

/// C++ `RE::GRenderer::VertexFormat`
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GRendererVertexFormat {
    kNone = 0,
    kXY16i = 1 << 1,
    kXY32f = 1 << 2,
    kXY16iC32 = 1 << 3,
    kXY16iCF32 = 1 << 4,
}

/// C++ `RE::GRenderer::IndexFormat`
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GRendererIndexFormat {
    kNone = 0,
    k16 = 1,
    k32 = 2,
}

/// C++ `RE::GRenderer::RenderCapBits`
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GRendererRenderCapBits {
    kNone = 0,
    kCacheDataUse = 1 << 0,
    kIndex16 = 1 << 2,
    kIndex32 = 1 << 3,
    kRenderStats = 1 << 4,
    kRenderTargets = 1 << 5,
    kRenderTargetPrePass = 1 << 6,
    kRenderTargetNonPow2 = 1 << 7,
    kFillGouraud = 1 << 8,
    kFillGouraudTex = 1 << 9,
    kCxformAdd = 1 << 12,
    kNestedMasks = 1 << 13,
    kTexNonPower2 = 1 << 14,
    kTexNonPower2Wrap = 1 << 15,
    kCanLoseData = 1 << 16,
    kKeepVertexData = 1 << 17,
    kNoTexOverwrite = 1 << 18,
    kTexNonPower2Mip = 1 << 19,
    kThreadedTextureCreation = 1 << 20,
    kRenderTargetMip = 1 << 21,
    kFilter_Blurs = 1 << 22,
    kFilter_ColorMatrix = 1 << 23,
}

core_util::impl_enumset_type!(GRendererRenderCapBits => u32);

/// C++ `RE::GRenderer::StereoDisplay`
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GRendererStereoDisplay {
    kStereoCenter = 0,
    kStereoLeft = 1,
    kStereoRight = 2,
}

core_util::impl_enumset_type!(GRendererStereoDisplay => u32);

/// C++ `RE::GRenderer::CachedDataType`
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GRendererCachedDataType {
    kVertex = 1,
    kIndex = 2,
    kBitmapList = 3,
}

/// C++ `RE::GRenderer::UserDataPropertyFlag`
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GRendererUserDataPropertyFlag {
    kNone = 0,
    kHasString = 1,
    kHasFloat = 2,
    kHasMatrix = 3,
}

core_util::impl_enumset_type!(GRendererUserDataPropertyFlag => u8);

/// C++ `RE::GRenderer::BitmapWrapMode`
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GRendererBitmapWrapMode {
    kRepeat = 0,
    kClamp = 1,
}

core_util::impl_enumset_type!(GRendererBitmapWrapMode => u32);

/// C++ `RE::GRenderer::BitmapSampleMode`
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GRendererBitmapSampleMode {
    kPoint = 0,
    kLinear = 1,
}

core_util::impl_enumset_type!(GRendererBitmapSampleMode => u32);

/// C++ `RE::GRenderer::GouraudFillType`
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GRendererGouraudFillType {
    kColor = 0,
    k1Texture = 1,
    k1TextureColor = 2,
    k2Texture = 3,
    k2TextureColor = 4,
    k3Texture = 5,
}

/// C++ `RE::GRenderer::SubmitMaskMode`
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GRendererSubmitMaskMode {
    kClear = 0,
    kIncrement = 1,
    kDecrement = 2,
}

/// C++ `RE::GRenderer::FilterModes`
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GRendererFilterModes {
    kBlur = 1 << 0,
    kShadow = 1 << 1,
    kHighlight = 1 << 2,
    kKnockout = 1 << 8,
    kInner = 1 << 9,
    kHideObject = 1 << 10,
    kUserModes = 0xFFFF,
    kSkipLastPass = 1 << 16,
    kLastPassOnly = 1 << 17,
}

core_util::impl_enumset_type!(GRendererFilterModes => u32);

/// C++ `RE::GRenderer::FilterSupport`
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GRendererFilterSupport {
    kNone = 0,
    kOk = 1 << 0,
    kMultipass = 1 << 1,
    kSlow = 1 << 2,
}

core_util::impl_enumset_type!(GRendererFilterSupport => u32);

/// C++ `RE::GRenderer::Stats`
#[repr(C)]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct GRendererStats {
    pub triangles: u32,  // 00
    pub lines: u32,      // 04
    pub primitives: u32, // 08
    pub masks: u32,      // 0C
    pub filters: u32,    // 10
}

const _: () = assert!(core::mem::size_of::<GRendererStats>() == 0x14);

/// C++ `RE::GRenderer::CachedData`
#[repr(C)]
pub struct GRendererCachedData {
    pub renderer: *mut GRenderer, // 00
    pub data: *mut c_void,        // 08
}

const _: () = assert!(core::mem::size_of::<GRendererCachedData>() == 0x10);

/// C++ `RE::GRenderer::CacheProvider`
#[repr(C)]
pub struct GRendererCacheProvider {
    pub data: *mut GRendererCachedData, // 00
    pub discard_shared_data: bool,      // 08
    pub pad09: u8,                      // 09
    pub pad0a: u16,                     // 0A
    pub pad0c: u32,                     // 0C
}

const _: () = assert!(core::mem::size_of::<GRendererCacheProvider>() == 0x10);

/// C++ `RE::GRenderer::Cxform`
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GRendererCxform {
    pub matrix: [[f32; 2]; 4], // 00
}

const _: () = assert!(core::mem::size_of::<GRendererCxform>() == 0x20);

/// C++ `RE::GRenderer::StereoParams`
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GRendererStereoParams {
    pub display_width_cm: f32,     // 00
    pub distortion: f32,           // 04
    pub display_diag_inches: f32,  // 08
    pub display_aspect_ratio: f32, // 0C
    pub eye_separation_cm: f32,    // 10
}

const _: () = assert!(core::mem::size_of::<GRendererStereoParams>() == 0x14);

/// C++ `RE::GRenderer::RenderCaps`
#[repr(C)]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct GRendererRenderCaps {
    pub cap_bits: EnumSet<GRendererRenderCapBits, u32>, // 00
    pub vertex_formats: u32,                            // 04
    pub blend_modes: u32,                               // 08
    pub max_texture_size: u32,                          // 0C
}

const _: () = assert!(core::mem::size_of::<GRendererRenderCaps>() == 0x10);

/// C++ `RE::GRenderer::UserData`
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct GRendererUserData {
    pub string: *const i8,                                      // 00
    pub flt: *mut f32,                                          // 08
    pub matrix: *mut f32,                                       // 10
    pub matrix_size: u32,                                       // 18
    pub prop_flags: EnumSet<GRendererUserDataPropertyFlag, u8>, // 1C
    pub pad1d: u8,                                              // 1D
    pub pad1e: u16,                                             // 1E
}

const _: () = assert!(core::mem::size_of::<GRendererUserData>() == 0x20);

/// C++ `RE::GRenderer::FillTexture`
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct GRendererFillTexture {
    pub texture: *mut GTexture,                               // 00
    pub texture_matrix: GMatrix2D,                            // 08
    pub wrap_mode: EnumSet<GRendererBitmapWrapMode, u32>,     // 20
    pub sample_mode: EnumSet<GRendererBitmapSampleMode, u32>, // 24
}

const _: () = assert!(core::mem::size_of::<GRendererFillTexture>() == 0x28);

/// C++ `RE::GRenderer::VertexXY16i`
#[repr(C)]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct GRendererVertexXY16i {
    pub x: i16,
    pub y: i16,
}

const _: () = assert!(core::mem::size_of::<GRendererVertexXY16i>() == 0x4);

/// C++ `RE::GRenderer::VertexXY16iC32`
#[repr(C)]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct GRendererVertexXY16iC32 {
    pub x: i16,
    pub y: i16,
    pub color: u32,
}

const _: () = assert!(core::mem::size_of::<GRendererVertexXY16iC32>() == 0x8);

/// C++ `RE::GRenderer::VertexXY16iCF32`
#[repr(C)]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct GRendererVertexXY16iCF32 {
    pub x: i16,
    pub y: i16,
    pub color: u32,
    pub factors: u32,
}

const _: () = assert!(core::mem::size_of::<GRendererVertexXY16iCF32>() == 0xC);

/// C++ `RE::GRenderer::BitmapDesc`
#[repr(C)]
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct GRendererBitmapDesc {
    pub coords: GRectF,
    pub texture_coords: GRectF,
    pub color: GColor,
}

const _: () = assert!(core::mem::size_of::<GRendererBitmapDesc>() == 0x24);

/// C++ `RE::GRenderer::DistanceFieldParams`
#[repr(C)]
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct GRendererDistanceFieldParams {
    pub width: f32,
    pub shadow_width: f32,
    pub shadow_color: GColor,
    pub shadow_offset: GPointF,
    pub glow_color: GColor,
    pub glow_size: [f32; 2],
}

const _: () = assert!(core::mem::size_of::<GRendererDistanceFieldParams>() == 0x20);

/// C++ `RE::GRenderer::BlurFilterParams`
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GRendererBlurFilterParams {
    pub mode: u32,
    pub blur_x: f32,
    pub blur_y: f32,
    pub passes: u32,
    pub offset: GPointF,
    pub color: GColor,
    pub color2: GColor,
    pub strength: f32,
    pub cxform: GRendererCxform,
}

const _: () = assert!(core::mem::size_of::<GRendererBlurFilterParams>() == 0x44);

/// C++ `RE::GRenderer`
#[repr(C)]
pub struct GRenderer {
    pub base: GRefCountBase<GRenderer, { GStatRenderer::MEM as u32 }>, // 00
    pub handlers: GList<GRendererEventHandler>,                        // 10
    pub s3d_params: GRendererStereoParams,                             // 20
    pub s3d_display: EnumSet<GRendererStereoDisplay, u32>,             // 34
}

const _: () = assert!(core::mem::size_of::<GRenderer>() == 0x38);
const _: () = assert!(core::mem::offset_of!(GRenderer, base) == 0x0);
const _: () = assert!(core::mem::offset_of!(GRenderer, handlers) == 0x10);
const _: () = assert!(core::mem::offset_of!(GRenderer, s3d_params) == 0x20);
const _: () = assert!(core::mem::offset_of!(GRenderer, s3d_display) == 0x34);

inherit!(GRenderer : GRefCountBase<GRenderer, { GStatRenderer::MEM as u32 }>, base);

impl GRendererStats {
    #[inline(always)]
    pub fn clear(&mut self) {
        *self = Self::default();
    }
}

impl GRendererCachedData {
    #[inline(always)]
    pub fn get_renderer(&self) -> *mut GRenderer {
        self.renderer
    }

    #[inline(always)]
    pub fn get_renderer_data(&self) -> *mut c_void {
        self.data
    }

    #[inline(always)]
    pub fn set_renderer_data(&mut self, handle: *mut c_void) {
        self.data = handle;
    }

    #[inline(always)]
    pub fn release_data(&mut self, ty: GRendererCachedDataType) {
        if !self.renderer.is_null() {
            unsafe {
                (*self.renderer).release_cached_data(self, ty);
            }
        }
        self.release_data_by_renderer();
    }

    #[inline(always)]
    pub fn release_data_by_renderer(&mut self) {
        self.renderer = core::ptr::null_mut();
        self.data = core::ptr::null_mut();
    }
}

impl Drop for GRendererCachedData {
    #[inline(always)]
    fn drop(&mut self) {
        debug_assert!(self.renderer.is_null());
    }
}

impl Default for GRendererCacheProvider {
    #[inline(always)]
    fn default() -> Self {
        Self {
            data: core::ptr::null_mut(),
            discard_shared_data: false,
            pad09: 0,
            pad0a: 0,
            pad0c: 0,
        }
    }
}

impl GRendererCacheProvider {
    #[inline(always)]
    pub fn new(cache: *mut GRendererCachedData) -> Self {
        debug_assert!(!cache.is_null());
        Self {
            data: cache,
            ..Self::default()
        }
    }

    #[inline(always)]
    pub fn get_cached_data(&self, renderer: *mut GRenderer) -> *mut GRendererCachedData {
        debug_assert!(!self.data.is_null());
        if renderer.is_null() || self.data.is_null() {
            core::ptr::null_mut()
        } else if unsafe { (*self.data).renderer == renderer } {
            self.data
        } else {
            core::ptr::null_mut()
        }
    }

    #[inline(always)]
    pub fn create_cached_data(
        &mut self,
        ty: GRendererCachedDataType,
        renderer: *mut GRenderer,
        keep_shared_data: bool,
    ) -> *mut GRendererCachedData {
        debug_assert!(!renderer.is_null());
        debug_assert!(!self.data.is_null());
        if self.data.is_null() {
            return core::ptr::null_mut();
        }
        if !self.data.is_null() && unsafe { (*self.data).renderer != renderer } {
            unsafe {
                (*self.data).release_data(ty);
            }
        }
        unsafe {
            (*self.data).renderer = renderer;
        }
        self.discard_shared_data = !keep_shared_data;
        self.data
    }

    #[inline(always)]
    pub fn can_discard_data(&self) -> bool {
        self.discard_shared_data
    }
}

impl Default for GRendererCxform {
    #[inline(always)]
    fn default() -> Self {
        Self::IDENTITY
    }
}

impl GRendererCxform {
    pub const R: usize = 0;
    pub const G: usize = 1;
    pub const B: usize = 2;
    pub const A: usize = 3;
    pub const RGBA: usize = 4;
    pub const MULT: usize = 0;
    pub const ADD: usize = 1;
    pub const MULT_ADD: usize = 2;
    pub const IDENTITY: Self = Self {
        matrix: [[1.0, 0.0], [1.0, 0.0], [1.0, 0.0], [1.0, 0.0]],
    };

    #[inline(always)]
    pub fn set_identity(&mut self) {
        *self = Self::IDENTITY;
    }

    #[inline(always)]
    pub fn is_identity(&self) -> bool {
        *self == Self::IDENTITY
    }
}

impl Default for GRendererStereoParams {
    #[inline(always)]
    fn default() -> Self {
        Self {
            display_width_cm: 0.0,
            distortion: 0.75,
            display_diag_inches: 52.0,
            display_aspect_ratio: 9.0 / 16.0,
            eye_separation_cm: 6.4,
        }
    }
}

impl Default for GRendererUserData {
    #[inline(always)]
    fn default() -> Self {
        Self {
            string: core::ptr::null(),
            flt: core::ptr::null_mut(),
            matrix: core::ptr::null_mut(),
            matrix_size: 0,
            prop_flags: EnumSet::from_underlying(GRendererUserDataPropertyFlag::kNone as u8),
            pad1d: 0,
            pad1e: 0,
        }
    }
}

impl GRendererVertexXY16iC32 {
    pub const VERTEX_FORMAT: u32 = GRendererVertexFormat::kXY16iC32 as u32;
}

impl GRendererVertexXY16iCF32 {
    pub const VERTEX_FORMAT: u32 = GRendererVertexFormat::kXY16iCF32 as u32;
}

impl GRenderer {
    // override (GRefCountBase<GRenderer, { GStatRenderer::MEM as u32 }>)
    crate::virtual_method! { pub const VFUNC_DTOR: usize = 0x0; pub fn dtor() }
    crate::virtual_method! { pub const VFUNC_GET_RENDER_CAPS: usize = 0x1; pub fn get_render_caps(caps: *mut GRendererRenderCaps) -> bool }
    crate::virtual_method! { pub const VFUNC_CREATE_TEXTURE: usize = 0x2; pub fn create_texture() -> *mut GTexture }
    crate::virtual_method! { pub const VFUNC_CREATE_TEXTURE_YUV: usize = 0x3; pub fn create_texture_yuv() -> *mut GTexture }
    crate::virtual_method! { pub const VFUNC_BEGIN_FRAME: usize = 0x4; pub fn begin_frame() }
    crate::virtual_method! { pub const VFUNC_END_FRAME: usize = 0x5; pub fn end_frame() }
    crate::virtual_method! { pub const VFUNC_CREATE_RENDER_TARGET: usize = 0x6; pub fn create_render_target() -> *mut GRenderTarget }
    crate::virtual_method! { pub const VFUNC_SET_DISPLAY_RENDER_TARGET: usize = 0x7; pub fn set_display_render_target(render_target: *mut GRenderTarget, set_state: bool) }
    crate::virtual_method! { pub const VFUNC_PUSH_RENDER_TARGET: usize = 0x8; pub fn push_render_target(frame_rect: &GRectF, render_target: *mut GRenderTarget) }
    crate::virtual_method! { pub const VFUNC_POP_RENDER_TARGET: usize = 0x9; pub fn pop_render_target() }
    crate::virtual_method! { pub const VFUNC_PUSH_TEMP_RENDER_TARGET: usize = 0xA; pub fn push_temp_render_target(frame_rect: &GRectF, target_w: u32, target_h: u32) -> *mut GTexture }
    crate::virtual_method! { pub const VFUNC_RELEASE_TEMP_RENDER_TARGETS: usize = 0xB; pub fn release_temp_render_targets(keep_area: u32) }
    crate::virtual_method! { pub const VFUNC_BEGIN_DISPLAY: usize = 0xC; pub fn begin_display(background_color: GColor, viewport: &GViewport, x0: f32, x1: f32, y0: f32, y1: f32) }
    crate::virtual_method! { pub const VFUNC_END_DISPLAY: usize = 0xD; pub fn end_display() }
    crate::virtual_method! { pub const VFUNC_SET_MATRIX: usize = 0xE; pub fn set_matrix(matrix: &GMatrix2D) }
    crate::virtual_method! { pub const VFUNC_SET_USER_MATRIX: usize = 0xF; pub fn set_user_matrix(matrix: &GMatrix2D) }
    crate::virtual_method! { pub const VFUNC_SET_CXFORM: usize = 0x10; pub fn set_cxform(cx_form: &GRendererCxform) }
    crate::virtual_method! { pub const VFUNC_PUSH_BLEND_MODE: usize = 0x11; pub fn push_blend_mode(mode: GRendererBlendType) }
    crate::virtual_method! { pub const VFUNC_POP_BLEND_MODE: usize = 0x12; pub fn pop_blend_mode() }
    crate::virtual_method! { pub const VFUNC_PUSH_USER_DATA: usize = 0x13; pub fn push_user_data(data: *mut GRendererUserData) -> bool }
    crate::virtual_method! { pub const VFUNC_POP_USER_DATA: usize = 0x14; pub fn pop_user_data() }
    crate::virtual_method! { pub const VFUNC_SET_PERSPECTIVE_3D: usize = 0x15; pub fn set_perspective_3d(proj_mat: &GMatrix3D) }
    crate::virtual_method! { pub const VFUNC_SET_VIEW_3D: usize = 0x16; pub fn set_view_3d(view_mat: &GMatrix3D) }
    crate::virtual_method! { pub const VFUNC_SET_WORLD_3D: usize = 0x17; pub fn set_world_3d(world_mat: *const GMatrix3D) }
    crate::virtual_method! { pub const VFUNC_MAKE_VIEW_AND_PERSP_3D: usize = 0x18; pub fn make_view_and_persp_3d(vis_frame_rect: &GRectF, mat_view: *mut GMatrix3D, mat_persp: *mut GMatrix3D, persp_fov: f32, invert_y: bool) }
    crate::virtual_method! { pub const VFUNC_SET_STEREO_PARAMS: usize = 0x19; pub fn set_stereo_params(params: GRendererStereoParams) }
    crate::virtual_method! { pub const VFUNC_SET_STEREO_DISPLAY: usize = 0x1A; pub fn set_stereo_display(display: GRendererStereoDisplay, set_state: bool) }
    crate::virtual_method! { pub const VFUNC_SET_VERTEX_DATA: usize = 0x1B; pub fn set_vertex_data(vertices: *const c_void, num_vertices: i32, vtx_fmt: GRendererVertexFormat, cache: *mut GRendererCacheProvider) }
    crate::virtual_method! { pub const VFUNC_SET_INDEX_DATA: usize = 0x1C; pub fn set_index_data(indices: *const c_void, num_indices: i32, idx_fmt: GRendererIndexFormat, cache: *mut GRendererCacheProvider) }
    crate::virtual_method! { pub const VFUNC_RELEASE_CACHED_DATA: usize = 0x1D; pub fn release_cached_data(data: *mut GRendererCachedData, ty: GRendererCachedDataType) }
    crate::virtual_method! { pub const VFUNC_DRAW_INDEXED_TRI_LIST: usize = 0x1E; pub fn draw_indexed_tri_list(base_vertex_index: i32, min_vertex_index: i32, num_vertices: i32, start_index: i32, triangle_count: i32) }
    crate::virtual_method! { pub const VFUNC_DRAW_LINE_STRIP: usize = 0x1F; pub fn draw_line_strip(base_vertex_index: i32, line_count: i32) }
    crate::virtual_method! { pub const VFUNC_LINE_STYLE_DISABLE: usize = 0x20; pub fn line_style_disable() }
    crate::virtual_method! { pub const VFUNC_LINE_STYLE_COLOR: usize = 0x21; pub fn line_style_color(color: GColor) }
    crate::virtual_method! { pub const VFUNC_FILL_STYLE_DISABLE: usize = 0x22; pub fn fill_style_disable() }
    crate::virtual_method! { pub const VFUNC_FILL_STYLE_COLOR: usize = 0x23; pub fn fill_style_color(color: GColor) }
    crate::virtual_method! { pub const VFUNC_FILL_STYLE_BITMAP: usize = 0x24; pub fn fill_style_bitmap(fill: *const GRendererFillTexture) }
    crate::virtual_method! { pub const VFUNC_FILL_STYLE_GOURAUD: usize = 0x25; pub fn fill_style_gouraud(fill_type: GRendererGouraudFillType, texture0: *const GRendererFillTexture, texture1: *const GRendererFillTexture, texture2: *const GRendererFillTexture) }
    crate::virtual_method! { pub const VFUNC_DRAW_BITMAPS: usize = 0x26; pub fn draw_bitmaps(bitmap_list: *mut GRendererBitmapDesc, list_size: i32, start_index: i32, count: i32, texture: *const GTexture, matrix: &GMatrix2D, cache: *mut GRendererCacheProvider) }
    crate::virtual_method! { pub const VFUNC_DRAW_DISTANCE_FIELD_BITMAPS: usize = 0x27; pub fn draw_distance_field_bitmaps(bitmap_list: *mut GRendererBitmapDesc, list_size: i32, start_index: i32, count: i32, texture: *const GTexture, matrix: &GMatrix2D, params: &GRendererDistanceFieldParams, cache: *mut GRendererCacheProvider) }
    crate::virtual_method! { pub const VFUNC_BEGIN_SUBMIT_MASK: usize = 0x28; pub fn begin_submit_mask(mask_mode: GRendererSubmitMaskMode) }
    crate::virtual_method! { pub const VFUNC_END_SUBMIT_MASK: usize = 0x29; pub fn end_submit_mask() }
    crate::virtual_method! { pub const VFUNC_DISABLE_MASK: usize = 0x2A; pub fn disable_mask() }
    crate::virtual_method! { pub const VFUNC_CHECK_FILTER_SUPPORT: usize = 0x2B; pub fn check_filter_support(params: &GRendererBlurFilterParams) -> u32 }
    crate::virtual_method! { pub const VFUNC_DRAW_BLUR_RECT: usize = 0x2C; pub fn draw_blur_rect(src: *mut GTexture, in_src_rect: &GRectF, in_dst_rect: &GRectF, params: &GRendererBlurFilterParams) }
    crate::virtual_method! { pub const VFUNC_DRAW_COLOR_MATRIX_RECT: usize = 0x2D; pub fn draw_color_matrix_rect(src: *mut GTexture, in_src_rect: &GRectF, dst_rect: &GRectF, matrix: *const f32) }
    crate::virtual_method! { pub const VFUNC_GET_RENDER_STATS: usize = 0x2E; pub fn get_render_stats(stats: *mut GRendererStats, reset_stats: bool) }
    crate::virtual_method! { pub const VFUNC_GET_STATS: usize = 0x2F; pub fn get_stats(bag: *mut GStatBag, reset: bool) }
    crate::virtual_method! { pub const VFUNC_RELEASE_RESOURCES: usize = 0x30; pub fn release_resources() }
    crate::virtual_method! { pub const VFUNC_ADD_EVENT_HANDLER: usize = 0x31; pub fn add_event_handler(handler: *mut GRendererEventHandler) -> bool }
    crate::virtual_method! { pub const VFUNC_REMOVE_EVENT_HANDLER: usize = 0x32; pub fn remove_event_handler(handler: *mut GRendererEventHandler) }

    #[inline(always)]
    pub fn begin_frame_base_impl(&mut self) {}

    #[inline(always)]
    pub fn release_temp_render_targets_base_impl(&mut self, _: u32) {}

    #[inline(always)]
    pub fn push_user_data_base_impl(&mut self, _: *mut GRendererUserData) -> bool {
        false
    }

    #[inline(always)]
    pub fn pop_user_data_base_impl(&mut self) {}

    #[inline(always)]
    pub fn set_stereo_params_base_impl(&mut self, params: GRendererStereoParams) {
        self.s3d_params = params;
        if self.s3d_params.display_width_cm == 0.0 {
            self.s3d_params.display_width_cm = params.display_diag_inches
                / sqrtf32(
                    1.0 + 1.0 / params.display_aspect_ratio * 1.0 / params.display_aspect_ratio,
                )
                * 2.54;
        }
    }

    #[inline(always)]
    pub fn set_stereo_display_base_impl(&mut self, _: GRendererStereoDisplay, _: bool) {}

    #[inline(always)]
    pub fn draw_distance_field_bitmaps_base_impl(
        &mut self,
        _: *mut GRendererBitmapDesc,
        _: i32,
        _: i32,
        _: i32,
        _: *const GTexture,
        _: &GMatrix2D,
        _: &GRendererDistanceFieldParams,
        _: *mut GRendererCacheProvider,
    ) {
    }

    #[inline(always)]
    pub fn fill_style_bitmap_components(
        &mut self,
        texture: *mut GTexture,
        matrix: &GMatrix2D,
        wrap_mode: GRendererBitmapWrapMode,
        sample_mode: GRendererBitmapSampleMode,
    ) {
        let fill = GRendererFillTexture {
            texture,
            texture_matrix: *matrix,
            wrap_mode: EnumSet::from_underlying(wrap_mode as u32),
            sample_mode: EnumSet::from_underlying(sample_mode as u32),
        };
        self.fill_style_bitmap(&fill);
    }
}

impl GPtrTarget for GRenderer {
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

#[inline(always)]
fn sqrtf32(x: f32) -> f32 {
    unsafe { sqrtf(x) }
}

unsafe extern "C" {
    fn sqrtf(x: f32) -> f32;
}
