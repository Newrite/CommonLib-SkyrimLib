#![allow(non_camel_case_types)]

/// C++ `RE::GStatGroups::GStatGroup`
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GStatGroup {
    kGStatGroup_Default = 0,

    kGStatGroup_Core = 16,
    kGStatGroup_Renderer = 1 << 6,
    kGStatGroup_RenderGen = 2 << 6,

    kGStatGroup_GFxFontCache = 3 << 6,
    kGStatGroup_GFxMovieData = 4 << 6,
    kGStatGroup_GFxMovieView = 5 << 6,
    kGStatGroup_GFxRenderCache = 6 << 6,
    kGStatGroup_GFxPlayer = 7 << 6,
    kGStatGroup_GFxIME = 8 << 6,

    kGStat_Mem = Self::kGStatGroup_Default as u32 + 1,
    kGStat_Default_Mem,
    kGStat_Image_Mem,
    kGStat_Sound_Mem,
    kGStat_String_Mem,
    kGStat_Video_Mem,

    kGStat_Debug_Mem,
    kGStat_DebugHUD_Mem,
    kGStat_DebugTracker_Mem,
    kGStat_StatBag_Mem,

    kGStat_MaxId = 64 << 6,
}

impl GStatGroup {
    pub const GROUP_DEFAULT: Self = Self::kGStatGroup_Default;
    pub const GROUP_CORE: Self = Self::kGStatGroup_Core;
    pub const GROUP_RENDERER: Self = Self::kGStatGroup_Renderer;
    pub const GROUP_RENDER_GEN: Self = Self::kGStatGroup_RenderGen;
    pub const GROUP_GFX_FONT_CACHE: Self = Self::kGStatGroup_GFxFontCache;
    pub const GROUP_GFX_MOVIE_DATA: Self = Self::kGStatGroup_GFxMovieData;
    pub const GROUP_GFX_MOVIE_VIEW: Self = Self::kGStatGroup_GFxMovieView;
    pub const GROUP_GFX_RENDER_CACHE: Self = Self::kGStatGroup_GFxRenderCache;
    pub const GROUP_GFX_PLAYER: Self = Self::kGStatGroup_GFxPlayer;
    pub const GROUP_GFX_IME: Self = Self::kGStatGroup_GFxIME;
    pub const MEM: Self = Self::kGStat_Mem;
    pub const DEFAULT_MEM: Self = Self::kGStat_Default_Mem;
    pub const IMAGE_MEM: Self = Self::kGStat_Image_Mem;
    pub const SOUND_MEM: Self = Self::kGStat_Sound_Mem;
    pub const STRING_MEM: Self = Self::kGStat_String_Mem;
    pub const VIDEO_MEM: Self = Self::kGStat_Video_Mem;
    pub const DEBUG_MEM: Self = Self::kGStat_Debug_Mem;
    pub const DEBUG_HUD_MEM: Self = Self::kGStat_DebugHUD_Mem;
    pub const DEBUG_TRACKER_MEM: Self = Self::kGStat_DebugTracker_Mem;
    pub const STAT_BAG_MEM: Self = Self::kGStat_StatBag_Mem;
    pub const HEAP_START: Self = Self::kGStatGroup_Core;
    pub const MAX_ID: Self = Self::kGStat_MaxId;
    pub const ENTRY_COUNT: u32 = 512;
}

/// C++ `RE::GStatGroups`
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct GStatGroups;

impl GStatGroups {
    pub const GROUP_DEFAULT: GStatGroup = GStatGroup::GROUP_DEFAULT;
    pub const GROUP_CORE: GStatGroup = GStatGroup::GROUP_CORE;
    pub const GROUP_RENDERER: GStatGroup = GStatGroup::GROUP_RENDERER;
    pub const GROUP_RENDER_GEN: GStatGroup = GStatGroup::GROUP_RENDER_GEN;
    pub const GROUP_GFX_FONT_CACHE: GStatGroup = GStatGroup::GROUP_GFX_FONT_CACHE;
    pub const GROUP_GFX_MOVIE_DATA: GStatGroup = GStatGroup::GROUP_GFX_MOVIE_DATA;
    pub const GROUP_GFX_MOVIE_VIEW: GStatGroup = GStatGroup::GROUP_GFX_MOVIE_VIEW;
    pub const GROUP_GFX_RENDER_CACHE: GStatGroup = GStatGroup::GROUP_GFX_RENDER_CACHE;
    pub const GROUP_GFX_PLAYER: GStatGroup = GStatGroup::GROUP_GFX_PLAYER;
    pub const GROUP_GFX_IME: GStatGroup = GStatGroup::GROUP_GFX_IME;
    pub const MEM: GStatGroup = GStatGroup::MEM;
    pub const DEFAULT_MEM: GStatGroup = GStatGroup::DEFAULT_MEM;
    pub const IMAGE_MEM: GStatGroup = GStatGroup::IMAGE_MEM;
    pub const SOUND_MEM: GStatGroup = GStatGroup::SOUND_MEM;
    pub const STRING_MEM: GStatGroup = GStatGroup::STRING_MEM;
    pub const VIDEO_MEM: GStatGroup = GStatGroup::VIDEO_MEM;
    pub const DEBUG_MEM: GStatGroup = GStatGroup::DEBUG_MEM;
    pub const DEBUG_HUD_MEM: GStatGroup = GStatGroup::DEBUG_HUD_MEM;
    pub const DEBUG_TRACKER_MEM: GStatGroup = GStatGroup::DEBUG_TRACKER_MEM;
    pub const STAT_BAG_MEM: GStatGroup = GStatGroup::STAT_BAG_MEM;
    pub const HEAP_START: GStatGroup = GStatGroup::HEAP_START;
    pub const MAX_ID: GStatGroup = GStatGroup::MAX_ID;
    pub const ENTRY_COUNT: u32 = GStatGroup::ENTRY_COUNT;
}

/// C++ `RE::GStatRenderers::GStatRenderer`
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GStatRenderer {
    kDefault = GStatGroup::kGStatGroup_Renderer as u32,

    kMem,
    kVMem,
    kTextureVMem,
    kBufferVMem,

    kCounters,
    kTextureUploadCnt,
    kTextureUpdateCnt,
    kDPCnt,
    kDPLineCnt,
    kDPTriangleCnt,
    kTriangleCnt,
    kLineCnt,
    kMaskCnt,
    kFilterCnt,
}

impl GStatRenderer {
    pub const DEFAULT: Self = Self::kDefault;
    pub const MEM: Self = Self::kMem;
    pub const VMEM: Self = Self::kVMem;
    pub const TEXTURE_VMEM: Self = Self::kTextureVMem;
    pub const BUFFER_VMEM: Self = Self::kBufferVMem;
    pub const COUNTERS: Self = Self::kCounters;
    pub const TEXTURE_UPLOAD_CNT: Self = Self::kTextureUploadCnt;
    pub const TEXTURE_UPDATE_CNT: Self = Self::kTextureUpdateCnt;
    pub const DP_CNT: Self = Self::kDPCnt;
    pub const DP_LINE_CNT: Self = Self::kDPLineCnt;
    pub const DP_TRIANGLE_CNT: Self = Self::kDPTriangleCnt;
    pub const TRIANGLE_CNT: Self = Self::kTriangleCnt;
    pub const LINE_CNT: Self = Self::kLineCnt;
    pub const MASK_CNT: Self = Self::kMaskCnt;
    pub const FILTER_CNT: Self = Self::kFilterCnt;
}

/// C++ `RE::GStatRenderers`
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct GStatRenderers;

impl GStatRenderers {
    pub const DEFAULT: GStatRenderer = GStatRenderer::DEFAULT;
    pub const MEM: GStatRenderer = GStatRenderer::MEM;
    pub const VMEM: GStatRenderer = GStatRenderer::VMEM;
    pub const TEXTURE_VMEM: GStatRenderer = GStatRenderer::TEXTURE_VMEM;
    pub const BUFFER_VMEM: GStatRenderer = GStatRenderer::BUFFER_VMEM;
    pub const COUNTERS: GStatRenderer = GStatRenderer::COUNTERS;
    pub const TEXTURE_UPLOAD_CNT: GStatRenderer = GStatRenderer::TEXTURE_UPLOAD_CNT;
    pub const TEXTURE_UPDATE_CNT: GStatRenderer = GStatRenderer::TEXTURE_UPDATE_CNT;
    pub const DP_CNT: GStatRenderer = GStatRenderer::DP_CNT;
    pub const DP_LINE_CNT: GStatRenderer = GStatRenderer::DP_LINE_CNT;
    pub const DP_TRIANGLE_CNT: GStatRenderer = GStatRenderer::DP_TRIANGLE_CNT;
    pub const TRIANGLE_CNT: GStatRenderer = GStatRenderer::TRIANGLE_CNT;
    pub const LINE_CNT: GStatRenderer = GStatRenderer::LINE_CNT;
    pub const MASK_CNT: GStatRenderer = GStatRenderer::MASK_CNT;
    pub const FILTER_CNT: GStatRenderer = GStatRenderer::FILTER_CNT;
}

/// C++ `RE::GHeapID`
#[repr(usize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GHeapID {
    kReserved = 0,
    kGlobal = 1,
    kMovieDef = 2,
    kMovieView = 3,
    kMovieData = 4,
    kMeshCache = 5,
    kFontCache = 6,
    kImages = 7,
    kOtherHeaps = 8,
    kHUDHeaps = 9,
}

const _: () = assert!(core::mem::size_of::<GStatGroup>() == 0x4);
const _: () = assert!(core::mem::size_of::<GStatRenderer>() == 0x4);
const _: () = assert!(core::mem::size_of::<GHeapID>() == 0x8);
