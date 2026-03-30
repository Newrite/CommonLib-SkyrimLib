#![allow(non_camel_case_types)]

use crate::offsets::offsets_rtti::RTTI_GFxState;
use crate::offsets::offsets_vtable::VTABLE_GFxState;
use crate::re::{GPtrTarget, GRefCountBase, GStatGroups};
use crate::relocation::{RttiType, VariantID};

/// C++ `RE::GFxState::StateType`
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GFxStateStateType {
    kNone = 0,
    kRenderConfig = 1,
    kRenderStats = 2,
    kTranslator = 3,
    kLog = 4,
    kImageLoader = 5,
    kActionControl = 6,
    kUserEventHandler = 7,
    kFSCommandHandler = 8,
    kExternalInterface = 9,
    kFileOpener = 10,
    kURLBuilder = 11,
    kImageCreator = 12,
    kParseControl = 13,
    kProgressHandler = 14,
    kImportVisitor = 15,
    kMeshCacheManager = 16,
    kFontPackParams = 17,
    kFontCacheManager = 18,
    kFontLib = 19,
    kFontProvider = 20,
    kFontMap = 21,
    kGradientParams = 22,
    kTaskManager = 23,
    kClipboard = 24,
    kTextKeyMap = 25,
    kPreprocessParams = 26,
    kIMEManager = 27,
    kXMLSupport = 28,
    kJpegSupport = 29,
    kZlibSupport = 30,
    kFontCompactorParams = 31,
    kImagePackerParams = 32,
    kPNGSupport = 33,
    kAudio = 34,
    kVideo = 35,
    kTestStream = 36,
    kSharedObject = 37,
    kLocSupport = 38,
}

/// C++ `RE::GFxState`
#[repr(C)]
pub struct GFxState {
    pub base: GRefCountBase<GFxState, { GStatGroups::DEFAULT_MEM as u32 }>, // 00
    pub state_type: GFxStateStateType,                                      // 10
    pub pad14: u32,                                                         // 14
}

const _: () = assert!(core::mem::size_of::<GFxState>() == 0x18);
const _: () = assert!(core::mem::offset_of!(GFxState, base) == 0x0);
const _: () = assert!(core::mem::offset_of!(GFxState, state_type) == 0x10);

core_util::inherit!(GFxState : GRefCountBase<GFxState, { GStatGroups::DEFAULT_MEM as u32 }>, base);

impl RttiType for GFxState {
    const RTTI: VariantID = RTTI_GFxState;
}

impl GFxState {
    pub const RTTI: VariantID = RTTI_GFxState;
    pub const VTABLE: &'static [VariantID] = &VTABLE_GFxState;

    crate::virtual_method! {
        pub const VFUNC_DTOR: usize = 0x00;
        pub fn dtor()
    }

    #[inline(always)]
    pub const fn get_state_type(&self) -> GFxStateStateType {
        self.state_type
    }
}

impl GPtrTarget for GFxState {
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
