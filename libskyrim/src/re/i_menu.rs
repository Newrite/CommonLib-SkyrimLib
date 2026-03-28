#![allow(non_camel_case_types)]

use core_util::{EnumSet, inherit};

use crate::ffi::{commonlib_imenu_add_ref, commonlib_imenu_release};
use crate::offsets::offsets_rtti::RTTI_IMenu;
use crate::offsets::offsets_vtable::VTABLE_IMenu;
use crate::re::{
    BSFixedString, FxDelegate, FxDelegateHandler, FxDelegateHandlerCallbackProcessor, GFxMovieView,
    GPtr, GPtrTarget, INPUT_CONTEXT_ID, UIMessage,
};
use crate::relocation::{RttiType, VariantID};
use crate::virtual_method;

/// C++ `RE::UI_MENU_FLAGS`
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum UI_MENU_FLAGS {
    kNone = 0,
    kPausesGame = 1 << 0,
    kAlwaysOpen = 1 << 1,
    kUsesCursor = 1 << 2,
    kUsesMenuContext = 1 << 3,
    kModal = 1 << 4,
    kFreezeFrameBackground = 1 << 5,
    kOnStack = 1 << 6,
    kDisablePauseMenu = 1 << 7,
    kRequiresUpdate = 1 << 8,
    kTopmostRenderedMenu = 1 << 9,
    kUpdateUsesCursor = 1 << 10,
    kAllowSaving = 1 << 11,
    kRendersOffscreenTargets = 1 << 12,
    kInventoryItemMenu = 1 << 13,
    kDontHideCursorWhenTopmost = 1 << 14,
    kCustomRendering = 1 << 15,
    kAssignCursorToRenderer = 1 << 16,
    kApplicationMenu = 1 << 17,
    kHasButtonBar = 1 << 18,
    kIsTopButtonBar = 1 << 19,
    kAdvancesUnderPauseMenu = 1 << 20,
    kRendersUnderPauseMenu = 1 << 21,
    kUsesBlurredBackground = 1 << 22,
    kCompanionAppAllowed = 1 << 23,
    kFreezeFramePause = 1 << 24,
    kSkipRenderDuringFreezeFrameScreenshot = 1 << 25,
    kLargeScaleformRenderCacheMode = 1 << 26,
    kUsesMovementToDirection = 1 << 27,
}

core_util::impl_enumset_type!(UI_MENU_FLAGS => u32);

/// C++ `RE::UI_MESSAGE_RESULTS`
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum UI_MESSAGE_RESULTS {
    kHandled = 0,
    kIgnore = 1,
    kPassOn = 2,
}

/// C++ `RE::UI_MENU_Unk09`
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum UI_MENU_Unk09 {
    kNone = -1,
}

/// C++ `RE::IMenu::VR_RUNTIME_DATA`
#[repr(C)]
pub struct IMenuVRRuntimeData {
    pub unk30: u32,               // 00
    pub unk34: u8,                // 04
    pub pad35: [u8; 3],           // 05
    pub menu_name: BSFixedString, // 08
}

const _: () = assert!(core::mem::size_of::<IMenuVRRuntimeData>() == 0x10);
const _: () = assert!(core::mem::offset_of!(IMenuVRRuntimeData, unk30) == 0x0);
const _: () = assert!(core::mem::offset_of!(IMenuVRRuntimeData, unk34) == 0x4);
const _: () = assert!(core::mem::offset_of!(IMenuVRRuntimeData, menu_name) == 0x8);

/// Honest common prefix of C++ `RE::IMenu`.
#[repr(C)]
pub struct IMenu {
    pub base: FxDelegateHandler,                 // 00
    pub ui_movie: GPtr<GFxMovieView>,            // 10
    pub depth_priority: i8,                      // 18
    pub pad19: u8,                               // 19
    pub pad1a: u16,                              // 1A
    pub menu_flags: EnumSet<UI_MENU_FLAGS, u32>, // 1C
    // TODO: `REX::EnumSet<UserEvents::INPUT_CONTEXT_ID, u32>` needs a shared
    // runtime-aware bitset layer for `INPUT_CONTEXT_ID`. Keep the exact storage
    // as raw bits until that support exists.
    pub input_context_bits: u32,       // 20
    pub pad24: u32,                    // 24
    pub fx_delegate: GPtr<FxDelegate>, // 28
}

const _: () = assert!(core::mem::size_of::<IMenu>() == 0x30);
const _: () = assert!(core::mem::offset_of!(IMenu, base) == 0x0);
const _: () = assert!(core::mem::offset_of!(IMenu, ui_movie) == 0x10);
const _: () = assert!(core::mem::offset_of!(IMenu, depth_priority) == 0x18);
const _: () = assert!(core::mem::offset_of!(IMenu, menu_flags) == 0x1C);
const _: () = assert!(core::mem::offset_of!(IMenu, input_context_bits) == 0x20);
const _: () = assert!(core::mem::offset_of!(IMenu, fx_delegate) == 0x28);

inherit!(IMenu : FxDelegateHandler);

impl RttiType for IMenu {
    const RTTI: VariantID = RTTI_IMenu;
}

impl IMenu {
    pub const RTTI: VariantID = RTTI_IMenu;
    pub const VTABLE: &'static [VariantID] = &VTABLE_IMenu;
    pub const VR_RUNTIME_DATA_OFFSET: usize = 0x30;

    crate::runtime_optional_data_accessor! {
        pub fn get_vr_runtime_data() -> IMenuVRRuntimeData {
            se_ae: 0,
            vr: Self::VR_RUNTIME_DATA_OFFSET,
        }
    }

    crate::runtime_optional_data_mut_accessor! {
        pub fn get_vr_runtime_data_mut() -> IMenuVRRuntimeData {
            se_ae: 0,
            vr: Self::VR_RUNTIME_DATA_OFFSET,
        }
    }

    virtual_method! {
        pub const VFUNC_DTOR: usize = 0x0;
        pub fn dtor()
    }

    virtual_method! {
        pub const VFUNC_ACCEPT: usize = 0x1;
        pub fn accept(processor: *mut FxDelegateHandlerCallbackProcessor)
    }

    virtual_method! {
        pub const VFUNC_POST_CREATE: usize = 0x2;
        pub fn post_create()
    }

    virtual_method! {
        pub const VFUNC_UNK_03: usize = 0x3;
        pub fn unk_03()
    }

    virtual_method! {
        pub const VFUNC_PROCESS_MESSAGE: usize = 0x4;
        pub fn process_message(message: &mut UIMessage) -> UI_MESSAGE_RESULTS
    }

    virtual_method! {
        pub const VFUNC_ADVANCE_MOVIE: usize = 0x5;
        pub fn advance_movie(interval: f32, current_time: u32)
    }

    virtual_method! {
        pub const VFUNC_POST_DISPLAY: usize = 0x6;
        pub fn post_display()
    }

    virtual_method! {
        pub const VFUNC_PRE_DISPLAY: usize = 0x7;
        pub fn pre_display()
    }

    virtual_method! {
        pub const VFUNC_REFRESH_PLATFORM: usize = 0x8;
        pub fn refresh_platform()
    }

    // TODO: `IMenu.cpp` carries richer default implementations for
    // `ProcessMessage`, `AdvanceMovie`, `PostDisplay`, and `RefreshPlatform`.
    // Keep the honest virtual surface and layout here until the Scaleform/UI
    // dependency chain (`BSUIScaleformData`, `GFxValue`, `UIMessageQueue`,
    // `InterfaceStrings`) is translated source-backed.

    #[inline(always)]
    pub fn advances_under_pause_menu(&self) -> bool {
        self.menu_flags.all(UI_MENU_FLAGS::kAdvancesUnderPauseMenu)
    }

    #[inline(always)]
    pub fn allow_saving(&self) -> bool {
        self.menu_flags.all(UI_MENU_FLAGS::kAllowSaving)
    }

    #[inline(always)]
    pub fn always_open(&self) -> bool {
        self.menu_flags.all(UI_MENU_FLAGS::kAlwaysOpen)
    }

    #[inline(always)]
    pub fn application_menu(&self) -> bool {
        self.menu_flags.all(UI_MENU_FLAGS::kApplicationMenu)
    }

    #[inline(always)]
    pub fn assign_cursor_to_renderer(&self) -> bool {
        self.menu_flags.all(UI_MENU_FLAGS::kAssignCursorToRenderer)
    }

    #[inline(always)]
    pub fn custom_rendering(&self) -> bool {
        self.menu_flags.all(UI_MENU_FLAGS::kCustomRendering)
    }

    #[inline(always)]
    pub fn companion_app_allowed(&self) -> bool {
        self.menu_flags.all(UI_MENU_FLAGS::kCompanionAppAllowed)
    }

    #[inline(always)]
    pub fn disable_pause_menu(&self) -> bool {
        self.menu_flags.all(UI_MENU_FLAGS::kDisablePauseMenu)
    }

    #[inline(always)]
    pub fn dont_hide_cursor_when_topmost(&self) -> bool {
        self.menu_flags
            .all(UI_MENU_FLAGS::kDontHideCursorWhenTopmost)
    }

    #[inline(always)]
    pub fn freeze_frame_background(&self) -> bool {
        self.menu_flags.all(UI_MENU_FLAGS::kFreezeFrameBackground)
    }

    #[inline(always)]
    pub fn freeze_frame_pause(&self) -> bool {
        self.menu_flags.all(UI_MENU_FLAGS::kFreezeFramePause)
    }

    #[inline(always)]
    pub fn has_button_bar(&self) -> bool {
        self.menu_flags.all(UI_MENU_FLAGS::kHasButtonBar)
    }

    #[inline(always)]
    pub fn inventory_item_menu(&self) -> bool {
        self.menu_flags.all(UI_MENU_FLAGS::kInventoryItemMenu)
    }

    #[inline(always)]
    pub fn is_top_button_bar(&self) -> bool {
        self.menu_flags.all(UI_MENU_FLAGS::kIsTopButtonBar)
    }

    #[inline(always)]
    pub fn large_scaleform_render_cache_mode(&self) -> bool {
        self.menu_flags
            .all(UI_MENU_FLAGS::kLargeScaleformRenderCacheMode)
    }

    #[inline(always)]
    pub fn modal(&self) -> bool {
        self.menu_flags.all(UI_MENU_FLAGS::kModal)
    }

    #[inline(always)]
    pub fn on_stack(&self) -> bool {
        self.menu_flags.all(UI_MENU_FLAGS::kOnStack)
    }

    #[inline(always)]
    pub fn pauses_game(&self) -> bool {
        self.menu_flags.all(UI_MENU_FLAGS::kPausesGame)
    }

    #[inline(always)]
    pub fn renders_offscreen_targets(&self) -> bool {
        self.menu_flags.all(UI_MENU_FLAGS::kRendersOffscreenTargets)
    }

    #[inline(always)]
    pub fn renders_under_pause_menu(&self) -> bool {
        self.menu_flags.all(UI_MENU_FLAGS::kRendersUnderPauseMenu)
    }

    #[inline(always)]
    pub fn requires_update(&self) -> bool {
        self.menu_flags.all(UI_MENU_FLAGS::kRequiresUpdate)
    }

    #[inline(always)]
    pub fn skip_render_during_freeze_frame_screenshot(&self) -> bool {
        self.menu_flags
            .all(UI_MENU_FLAGS::kSkipRenderDuringFreezeFrameScreenshot)
    }

    #[inline(always)]
    pub fn topmost_rendered_menu(&self) -> bool {
        self.menu_flags.all(UI_MENU_FLAGS::kTopmostRenderedMenu)
    }

    #[inline(always)]
    pub fn update_uses_cursor(&self) -> bool {
        self.menu_flags.all(UI_MENU_FLAGS::kUpdateUsesCursor)
    }

    #[inline(always)]
    pub fn uses_blurred_background(&self) -> bool {
        self.menu_flags.all(UI_MENU_FLAGS::kUsesBlurredBackground)
    }

    #[inline(always)]
    pub fn uses_cursor(&self) -> bool {
        self.menu_flags.all(UI_MENU_FLAGS::kUsesCursor)
    }

    #[inline(always)]
    pub fn uses_menu_context(&self) -> bool {
        self.menu_flags.all(UI_MENU_FLAGS::kUsesMenuContext)
    }

    #[inline(always)]
    pub fn uses_movement_to_direction(&self) -> bool {
        self.menu_flags.all(UI_MENU_FLAGS::kUsesMovementToDirection)
    }

    #[inline(always)]
    pub fn has_input_context(&self, context: INPUT_CONTEXT_ID) -> bool {
        let bit = context.get();
        bit < u32::BITS && (self.input_context_bits & (1_u32 << bit)) != 0
    }
}

impl GPtrTarget for IMenu {
    #[inline(always)]
    fn gptr_add_ref(&self) {
        unsafe {
            commonlib_imenu_add_ref(core::ptr::from_ref(self).cast_mut().cast());
        }
    }

    #[inline(always)]
    fn gptr_release(&self) {
        unsafe {
            commonlib_imenu_release(core::ptr::from_ref(self).cast_mut().cast());
        }
    }
}

pub trait IMenuExt {
    fn dtor(&mut self);
    fn accept(&mut self, processor: *mut FxDelegateHandlerCallbackProcessor);
    fn post_create(&mut self);
    fn unk_03(&mut self);
    fn process_message(&mut self, message: &mut UIMessage) -> UI_MESSAGE_RESULTS;
    fn advance_movie(&mut self, interval: f32, current_time: u32);
    fn post_display(&mut self);
    fn pre_display(&mut self);
    fn refresh_platform(&mut self);
    fn on_stack(&self) -> bool;
    fn uses_cursor(&self) -> bool;
    fn update_uses_cursor(&self) -> bool;
    fn modal(&self) -> bool;
}

impl<T> IMenuExt for T
where
    T: AsRef<IMenu> + AsMut<IMenu>,
{
    #[inline(always)]
    fn dtor(&mut self) {
        self.as_mut().dtor()
    }

    #[inline(always)]
    fn accept(&mut self, processor: *mut FxDelegateHandlerCallbackProcessor) {
        self.as_mut().accept(processor)
    }

    #[inline(always)]
    fn post_create(&mut self) {
        self.as_mut().post_create()
    }

    #[inline(always)]
    fn unk_03(&mut self) {
        self.as_mut().unk_03()
    }

    #[inline(always)]
    fn process_message(&mut self, message: &mut UIMessage) -> UI_MESSAGE_RESULTS {
        self.as_mut().process_message(message)
    }

    #[inline(always)]
    fn advance_movie(&mut self, interval: f32, current_time: u32) {
        self.as_mut().advance_movie(interval, current_time)
    }

    #[inline(always)]
    fn post_display(&mut self) {
        self.as_mut().post_display()
    }

    #[inline(always)]
    fn pre_display(&mut self) {
        self.as_mut().pre_display()
    }

    #[inline(always)]
    fn refresh_platform(&mut self) {
        self.as_mut().refresh_platform()
    }

    #[inline(always)]
    fn on_stack(&self) -> bool {
        self.as_ref().on_stack()
    }

    #[inline(always)]
    fn uses_cursor(&self) -> bool {
        self.as_ref().uses_cursor()
    }

    #[inline(always)]
    fn update_uses_cursor(&self) -> bool {
        self.as_ref().update_uses_cursor()
    }

    #[inline(always)]
    fn modal(&self) -> bool {
        self.as_ref().modal()
    }
}
