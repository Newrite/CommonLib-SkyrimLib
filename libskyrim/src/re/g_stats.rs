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
    pub const kGStatHeap_Start: Self = Self::kGStatGroup_Core;
    pub const kGStat_EntryCount: u32 = 512;
}

/// C++ `RE::GStatGroups`
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct GStatGroups;

impl GStatGroups {
    pub const kGStatGroup_Default: GStatGroup = GStatGroup::kGStatGroup_Default;
    pub const kGStatGroup_Core: GStatGroup = GStatGroup::kGStatGroup_Core;
    pub const kGStatGroup_Renderer: GStatGroup = GStatGroup::kGStatGroup_Renderer;
    pub const kGStatGroup_RenderGen: GStatGroup = GStatGroup::kGStatGroup_RenderGen;
    pub const kGStatGroup_GFxFontCache: GStatGroup = GStatGroup::kGStatGroup_GFxFontCache;
    pub const kGStatGroup_GFxMovieData: GStatGroup = GStatGroup::kGStatGroup_GFxMovieData;
    pub const kGStatGroup_GFxMovieView: GStatGroup = GStatGroup::kGStatGroup_GFxMovieView;
    pub const kGStatGroup_GFxRenderCache: GStatGroup = GStatGroup::kGStatGroup_GFxRenderCache;
    pub const kGStatGroup_GFxPlayer: GStatGroup = GStatGroup::kGStatGroup_GFxPlayer;
    pub const kGStatGroup_GFxIME: GStatGroup = GStatGroup::kGStatGroup_GFxIME;
    pub const kGStat_Mem: GStatGroup = GStatGroup::kGStat_Mem;
    pub const kGStat_Default_Mem: GStatGroup = GStatGroup::kGStat_Default_Mem;
    pub const kGStat_Image_Mem: GStatGroup = GStatGroup::kGStat_Image_Mem;
    pub const kGStat_Sound_Mem: GStatGroup = GStatGroup::kGStat_Sound_Mem;
    pub const kGStat_String_Mem: GStatGroup = GStatGroup::kGStat_String_Mem;
    pub const kGStat_Video_Mem: GStatGroup = GStatGroup::kGStat_Video_Mem;
    pub const kGStat_Debug_Mem: GStatGroup = GStatGroup::kGStat_Debug_Mem;
    pub const kGStat_DebugHUD_Mem: GStatGroup = GStatGroup::kGStat_DebugHUD_Mem;
    pub const kGStat_DebugTracker_Mem: GStatGroup = GStatGroup::kGStat_DebugTracker_Mem;
    pub const kGStat_StatBag_Mem: GStatGroup = GStatGroup::kGStat_StatBag_Mem;
    pub const kGStatHeap_Start: GStatGroup = GStatGroup::kGStatHeap_Start;
    pub const kGStat_MaxId: GStatGroup = GStatGroup::kGStat_MaxId;
    pub const kGStat_EntryCount: u32 = GStatGroup::kGStat_EntryCount;
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

/// C++ `RE::GStatRenderers`
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct GStatRenderers;

impl GStatRenderers {
    pub const kDefault: GStatRenderer = GStatRenderer::kDefault;
    pub const kMem: GStatRenderer = GStatRenderer::kMem;
    pub const kVMem: GStatRenderer = GStatRenderer::kVMem;
    pub const kTextureVMem: GStatRenderer = GStatRenderer::kTextureVMem;
    pub const kBufferVMem: GStatRenderer = GStatRenderer::kBufferVMem;
    pub const kCounters: GStatRenderer = GStatRenderer::kCounters;
    pub const kTextureUploadCnt: GStatRenderer = GStatRenderer::kTextureUploadCnt;
    pub const kTextureUpdateCnt: GStatRenderer = GStatRenderer::kTextureUpdateCnt;
    pub const kDPCnt: GStatRenderer = GStatRenderer::kDPCnt;
    pub const kDPLineCnt: GStatRenderer = GStatRenderer::kDPLineCnt;
    pub const kDPTriangleCnt: GStatRenderer = GStatRenderer::kDPTriangleCnt;
    pub const kTriangleCnt: GStatRenderer = GStatRenderer::kTriangleCnt;
    pub const kLineCnt: GStatRenderer = GStatRenderer::kLineCnt;
    pub const kMaskCnt: GStatRenderer = GStatRenderer::kMaskCnt;
    pub const kFilterCnt: GStatRenderer = GStatRenderer::kFilterCnt;
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
