#![allow(non_camel_case_types)]

use crate::re::GStatGroup;

/// C++ `RE::GFxStatMovieData::GFxStatMovieDatum`
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GFxStatMovieDatum {
    kGFxStatMD_Default = GStatGroup::kGStatGroup_GFxMovieData as u32,

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
    pub const kGFxStatMD_Default: GFxStatMovieDatum = GFxStatMovieDatum::kGFxStatMD_Default;
    pub const kGFxStatMD_Mem: GFxStatMovieDatum = GFxStatMovieDatum::kGFxStatMD_Mem;
    pub const kGFxStatMD_CharDefs_Mem: GFxStatMovieDatum =
        GFxStatMovieDatum::kGFxStatMD_CharDefs_Mem;
    pub const kGFxStatMD_ShapeData_Mem: GFxStatMovieDatum =
        GFxStatMovieDatum::kGFxStatMD_ShapeData_Mem;
    pub const kGFxStatMD_Tags_Mem: GFxStatMovieDatum = GFxStatMovieDatum::kGFxStatMD_Tags_Mem;
    pub const kGFxStatMD_Fonts_Mem: GFxStatMovieDatum = GFxStatMovieDatum::kGFxStatMD_Fonts_Mem;
    pub const kGFxStatMD_Images_Mem: GFxStatMovieDatum = GFxStatMovieDatum::kGFxStatMD_Images_Mem;
    pub const kGFxStatMD_Sounds_Mem: GFxStatMovieDatum = GFxStatMovieDatum::kGFxStatMD_Sounds_Mem;
    pub const kGFxStatMD_ActionOps_Mem: GFxStatMovieDatum =
        GFxStatMovieDatum::kGFxStatMD_ActionOps_Mem;
    pub const kGFxStatMD_Other_Mem: GFxStatMovieDatum = GFxStatMovieDatum::kGFxStatMD_Other_Mem;
    pub const kGFxStatMD_Time: GFxStatMovieDatum = GFxStatMovieDatum::kGFxStatMD_Time;
    pub const kGFxStatMD_Load_Tks: GFxStatMovieDatum = GFxStatMovieDatum::kGFxStatMD_Load_Tks;
    pub const kGFxStatMD_Bind_Tks: GFxStatMovieDatum = GFxStatMovieDatum::kGFxStatMD_Bind_Tks;
}

/// C++ `RE::GFxStatMovieViews::GFxStatMovieView`
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GFxStatMovieView {
    kGFxStatMV_Default = GStatGroup::kGStatGroup_GFxMovieView as u32,

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
    pub const kGFxStatMV_Default: GFxStatMovieView = GFxStatMovieView::kGFxStatMV_Default;
    pub const kGFxStatMV_Mem: GFxStatMovieView = GFxStatMovieView::kGFxStatMV_Mem;
    pub const kGFxStatMV_MovieClip_Mem: GFxStatMovieView =
        GFxStatMovieView::kGFxStatMV_MovieClip_Mem;
    pub const kGFxStatMV_ActionScript_Mem: GFxStatMovieView =
        GFxStatMovieView::kGFxStatMV_ActionScript_Mem;
    pub const kGFxStatMV_Text_Mem: GFxStatMovieView = GFxStatMovieView::kGFxStatMV_Text_Mem;
    pub const kGFxStatMV_XML_Mem: GFxStatMovieView = GFxStatMovieView::kGFxStatMV_XML_Mem;
    pub const kGFxStatMV_Other_Mem: GFxStatMovieView = GFxStatMovieView::kGFxStatMV_Other_Mem;
    pub const kGFxStatMV_Tks: GFxStatMovieView = GFxStatMovieView::kGFxStatMV_Tks;
    pub const kGFxStatMV_Advance_Tks: GFxStatMovieView = GFxStatMovieView::kGFxStatMV_Advance_Tks;
    pub const kGFxStatMV_Action_Tks: GFxStatMovieView = GFxStatMovieView::kGFxStatMV_Action_Tks;
    pub const kGFxStatMV_Timeline_Tks: GFxStatMovieView = GFxStatMovieView::kGFxStatMV_Timeline_Tks;
    pub const kGFxStatMV_Input_Tks: GFxStatMovieView = GFxStatMovieView::kGFxStatMV_Input_Tks;
    pub const kGFxStatMV_Mouse_Tks: GFxStatMovieView = GFxStatMovieView::kGFxStatMV_Mouse_Tks;
    pub const kGFxStatMV_ScriptCommunication_Tks: GFxStatMovieView =
        GFxStatMovieView::kGFxStatMV_ScriptCommunication_Tks;
    pub const kGFxStatMV_GetVariable_Tks: GFxStatMovieView =
        GFxStatMovieView::kGFxStatMV_GetVariable_Tks;
    pub const kGFxStatMV_SetVariable_Tks: GFxStatMovieView =
        GFxStatMovieView::kGFxStatMV_SetVariable_Tks;
    pub const kGFxStatMV_Invoke_Tks: GFxStatMovieView = GFxStatMovieView::kGFxStatMV_Invoke_Tks;
    pub const kGFxStatMV_Display_Tks: GFxStatMovieView = GFxStatMovieView::kGFxStatMV_Display_Tks;
    pub const kGFxStatMV_Tessellate_Tks: GFxStatMovieView =
        GFxStatMovieView::kGFxStatMV_Tessellate_Tks;
    pub const kGFxStatMV_GradientGen_Tks: GFxStatMovieView =
        GFxStatMovieView::kGFxStatMV_GradientGen_Tks;
}

/// C++ `RE::GFxStatIMEs::GFxStatIME`
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GFxStatIME {
    kGFxStatIME_Default = GStatGroup::kGStatGroup_GFxIME as u32,
    kGFxStatIME_Mem,
}

/// C++ `RE::GFxStatIMEs`
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct GFxStatIMEs;

impl GFxStatIMEs {
    pub const kGFxStatIME_Default: GFxStatIME = GFxStatIME::kGFxStatIME_Default;
    pub const kGFxStatIME_Mem: GFxStatIME = GFxStatIME::kGFxStatIME_Mem;
}

/// C++ `RE::GFxStatFontCaches::GFxStatFontCache`
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GFxStatFontCache {
    kGFxStatFC_Default = GStatGroup::kGStatGroup_GFxFontCache as u32,

    kGFxStatFC_Mem,
    kGFxStatFC_Batch_Mem,
    kGFxStatFC_GlyphCache_Mem,
    kGFxStatFC_Other_Mem,
}

/// C++ `RE::GFxStatFontCaches`
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct GFxStatFontCaches;

impl GFxStatFontCaches {
    pub const kGFxStatFC_Default: GFxStatFontCache = GFxStatFontCache::kGFxStatFC_Default;
    pub const kGFxStatFC_Mem: GFxStatFontCache = GFxStatFontCache::kGFxStatFC_Mem;
    pub const kGFxStatFC_Batch_Mem: GFxStatFontCache = GFxStatFontCache::kGFxStatFC_Batch_Mem;
    pub const kGFxStatFC_GlyphCache_Mem: GFxStatFontCache =
        GFxStatFontCache::kGFxStatFC_GlyphCache_Mem;
    pub const kGFxStatFC_Other_Mem: GFxStatFontCache = GFxStatFontCache::kGFxStatFC_Other_Mem;
}

const _: () = assert!(core::mem::size_of::<GFxStatMovieDatum>() == 0x4);
const _: () = assert!(core::mem::size_of::<GFxStatMovieView>() == 0x4);
const _: () = assert!(core::mem::size_of::<GFxStatIME>() == 0x4);
const _: () = assert!(core::mem::size_of::<GFxStatFontCache>() == 0x4);
