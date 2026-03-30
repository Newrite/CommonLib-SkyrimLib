#![allow(non_camel_case_types)]

use crate::re::GStatGroup;

/// C++ `RE::GFxStatMovieData::GFxStatMovieDatum`
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GFxStatMovieDatum {
    kGFxStatMD_Default = GStatGroup::GROUP_GFX_MOVIE_DATA as u32,

    kGFxStatMD_Mem,
    kGFxStatMD_CharDefs_Mem,
    kGFxStatMD_ShapeData_Mem,
    kGFxStatMD_Tags_Mem,
    kGFxStatMD_Fonts_Mem,
    kGFxStatMD_Images_Mem,
    kGFxStatMD_Sounds_Mem,
    kGFxStatMD_ActionOps_Mem,
    kGFxStatMD_Other_Mem,

    kGFxStatMD_Time,
    kGFxStatMD_Load_Tks,
    kGFxStatMD_Bind_Tks,
}

/// C++ `RE::GFxStatMovieData`
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct GFxStatMovieData;

impl GFxStatMovieData {
    pub const DEFAULT: GFxStatMovieDatum = GFxStatMovieDatum::kGFxStatMD_Default;
    pub const MEM: GFxStatMovieDatum = GFxStatMovieDatum::kGFxStatMD_Mem;
    pub const CHAR_DEFS_MEM: GFxStatMovieDatum = GFxStatMovieDatum::kGFxStatMD_CharDefs_Mem;
    pub const SHAPE_DATA_MEM: GFxStatMovieDatum = GFxStatMovieDatum::kGFxStatMD_ShapeData_Mem;
    pub const TAGS_MEM: GFxStatMovieDatum = GFxStatMovieDatum::kGFxStatMD_Tags_Mem;
    pub const FONTS_MEM: GFxStatMovieDatum = GFxStatMovieDatum::kGFxStatMD_Fonts_Mem;
    pub const IMAGES_MEM: GFxStatMovieDatum = GFxStatMovieDatum::kGFxStatMD_Images_Mem;
    pub const SOUNDS_MEM: GFxStatMovieDatum = GFxStatMovieDatum::kGFxStatMD_Sounds_Mem;
    pub const ACTION_OPS_MEM: GFxStatMovieDatum = GFxStatMovieDatum::kGFxStatMD_ActionOps_Mem;
    pub const OTHER_MEM: GFxStatMovieDatum = GFxStatMovieDatum::kGFxStatMD_Other_Mem;
    pub const TIME: GFxStatMovieDatum = GFxStatMovieDatum::kGFxStatMD_Time;
    pub const LOAD_TKS: GFxStatMovieDatum = GFxStatMovieDatum::kGFxStatMD_Load_Tks;
    pub const BIND_TKS: GFxStatMovieDatum = GFxStatMovieDatum::kGFxStatMD_Bind_Tks;
}

/// C++ `RE::GFxStatMovieViews::GFxStatMovieView`
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GFxStatMovieView {
    kGFxStatMV_Default = GStatGroup::GROUP_GFX_MOVIE_VIEW as u32,

    kGFxStatMV_Mem,
    kGFxStatMV_MovieClip_Mem,
    kGFxStatMV_ActionScript_Mem,
    kGFxStatMV_Text_Mem,
    kGFxStatMV_XML_Mem,
    kGFxStatMV_Other_Mem,

    kGFxStatMV_Tks,
    kGFxStatMV_Advance_Tks,
    kGFxStatMV_Action_Tks,
    kGFxStatMV_Timeline_Tks,
    kGFxStatMV_Input_Tks,
    kGFxStatMV_Mouse_Tks,
    kGFxStatMV_ScriptCommunication_Tks,
    kGFxStatMV_GetVariable_Tks,
    kGFxStatMV_SetVariable_Tks,
    kGFxStatMV_Invoke_Tks,
    kGFxStatMV_Display_Tks,
    kGFxStatMV_Tessellate_Tks,
    kGFxStatMV_GradientGen_Tks,
}

/// C++ `RE::GFxStatMovieViews`
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct GFxStatMovieViews;

impl GFxStatMovieViews {
    pub const DEFAULT: GFxStatMovieView = GFxStatMovieView::kGFxStatMV_Default;
    pub const MEM: GFxStatMovieView = GFxStatMovieView::kGFxStatMV_Mem;
    pub const MOVIE_CLIP_MEM: GFxStatMovieView = GFxStatMovieView::kGFxStatMV_MovieClip_Mem;
    pub const ACTION_SCRIPT_MEM: GFxStatMovieView = GFxStatMovieView::kGFxStatMV_ActionScript_Mem;
    pub const TEXT_MEM: GFxStatMovieView = GFxStatMovieView::kGFxStatMV_Text_Mem;
    pub const XML_MEM: GFxStatMovieView = GFxStatMovieView::kGFxStatMV_XML_Mem;
    pub const OTHER_MEM: GFxStatMovieView = GFxStatMovieView::kGFxStatMV_Other_Mem;
    pub const TKS: GFxStatMovieView = GFxStatMovieView::kGFxStatMV_Tks;
    pub const ADVANCE_TKS: GFxStatMovieView = GFxStatMovieView::kGFxStatMV_Advance_Tks;
    pub const ACTION_TKS: GFxStatMovieView = GFxStatMovieView::kGFxStatMV_Action_Tks;
    pub const TIMELINE_TKS: GFxStatMovieView = GFxStatMovieView::kGFxStatMV_Timeline_Tks;
    pub const INPUT_TKS: GFxStatMovieView = GFxStatMovieView::kGFxStatMV_Input_Tks;
    pub const MOUSE_TKS: GFxStatMovieView = GFxStatMovieView::kGFxStatMV_Mouse_Tks;
    pub const SCRIPT_COMMUNICATION_TKS: GFxStatMovieView =
        GFxStatMovieView::kGFxStatMV_ScriptCommunication_Tks;
    pub const GET_VARIABLE_TKS: GFxStatMovieView = GFxStatMovieView::kGFxStatMV_GetVariable_Tks;
    pub const SET_VARIABLE_TKS: GFxStatMovieView = GFxStatMovieView::kGFxStatMV_SetVariable_Tks;
    pub const INVOKE_TKS: GFxStatMovieView = GFxStatMovieView::kGFxStatMV_Invoke_Tks;
    pub const DISPLAY_TKS: GFxStatMovieView = GFxStatMovieView::kGFxStatMV_Display_Tks;
    pub const TESSELLATE_TKS: GFxStatMovieView = GFxStatMovieView::kGFxStatMV_Tessellate_Tks;
    pub const GRADIENT_GEN_TKS: GFxStatMovieView = GFxStatMovieView::kGFxStatMV_GradientGen_Tks;
}

/// C++ `RE::GFxStatIMEs::GFxStatIME`
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GFxStatIME {
    kGFxStatIME_Default = GStatGroup::GROUP_GFX_IME as u32,
    kGFxStatIME_Mem,
}

/// C++ `RE::GFxStatIMEs`
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct GFxStatIMEs;

impl GFxStatIMEs {
    pub const DEFAULT: GFxStatIME = GFxStatIME::kGFxStatIME_Default;
    pub const MEM: GFxStatIME = GFxStatIME::kGFxStatIME_Mem;
}

/// C++ `RE::GFxStatFontCaches::GFxStatFontCache`
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GFxStatFontCache {
    kGFxStatFC_Default = GStatGroup::GROUP_GFX_FONT_CACHE as u32,

    kGFxStatFC_Mem,
    kGFxStatFC_Batch_Mem,
    kGFxStatFC_GlyphCache_Mem,
    kGFxStatFC_Other_Mem,
}

/// C++ `RE::GFxStatFontCaches`
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct GFxStatFontCaches;

impl GFxStatFontCaches {
    pub const DEFAULT: GFxStatFontCache = GFxStatFontCache::kGFxStatFC_Default;
    pub const MEM: GFxStatFontCache = GFxStatFontCache::kGFxStatFC_Mem;
    pub const BATCH_MEM: GFxStatFontCache = GFxStatFontCache::kGFxStatFC_Batch_Mem;
    pub const GLYPH_CACHE_MEM: GFxStatFontCache = GFxStatFontCache::kGFxStatFC_GlyphCache_Mem;
    pub const OTHER_MEM: GFxStatFontCache = GFxStatFontCache::kGFxStatFC_Other_Mem;
}

const _: () = assert!(core::mem::size_of::<GFxStatMovieDatum>() == 0x4);
const _: () = assert!(core::mem::size_of::<GFxStatMovieView>() == 0x4);
const _: () = assert!(core::mem::size_of::<GFxStatIME>() == 0x4);
const _: () = assert!(core::mem::size_of::<GFxStatFontCache>() == 0x4);
