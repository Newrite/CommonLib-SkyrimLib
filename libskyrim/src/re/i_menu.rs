#![allow(non_camel_case_types)]

use core_util::Enum;
use core_util::{EnumSet, inherit};

use crate::ffi::{commonlib_imenu_add_ref, commonlib_imenu_release};
use crate::offsets::offsets_rtti::RTTI_IMenu;
use crate::offsets::offsets_vtable::VTABLE_IMenu;
use crate::re::{
    BSFixedString, FxDelegate, FxDelegateHandler, FxDelegateHandlerCallbackProcessor, GFxMovieView,
    GPtr, GPtrTarget, INPUT_CONTEXT_ID, UIMessage,
};
use crate::relocation::{RttiType, VariantID, VariantOffset};
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
#[libskyrim_macros::open_enum]
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

impl crate::rex::EnumSetType<u32> for UI_MENU_Unk09 {
    #[inline(always)]
    fn to_underlying(self) -> u32 {
        self as i32 as u32
    }
}

impl TryFrom<u32> for UI_MENU_Unk09 {
    type Error = ();

    #[inline(always)]
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            0xFFFF_FFFF => Ok(Self::kNone),
            _ => Err(()),
        }
    }
}

pub type IMenuContext = INPUT_CONTEXT_ID;
pub type IMenuFlag = UI_MENU_FLAGS;

/// C++ `RE::IMenu::VR_RUNTIME_DATA`
#[repr(C)]
pub struct IMenuVRRuntimeData {
    pub unk30: EnumSet<UI_MENU_Unk09, u32>, // 00
    pub unk34: u8,                          // 04
    pub pad35: [u8; 3],                     // 05
    pub menu_name: BSFixedString,           // 08
}

const _: () = assert!(core::mem::size_of::<IMenuVRRuntimeData>() == 0x10);
const _: () = assert!(core::mem::offset_of!(IMenuVRRuntimeData, unk30) == 0x0);
const _: () = assert!(core::mem::offset_of!(IMenuVRRuntimeData, unk34) == 0x4);
const _: () = assert!(core::mem::offset_of!(IMenuVRRuntimeData, menu_name) == 0x8);

/// Honest common prefix of C++ `RE::IMenu`.
#[repr(C)]
pub struct IMenu {
    pub base: FxDelegateHandler,                       // 00
    pub ui_movie: GPtr<GFxMovieView>,                  // 10
    pub depth_priority: i8,                            // 18
    pub pad19: u8,                                     // 19
    pub pad1a: u16,                                    // 1A
    pub menu_flags: EnumSet<UI_MENU_FLAGS, u32>,       // 1C
    pub input_context: EnumSet<INPUT_CONTEXT_ID, u32>, // 20
    pub pad24: u32,                                    // 24
    pub fx_delegate: GPtr<FxDelegate>,                 // 28
}

const _: () = assert!(core::mem::size_of::<IMenu>() == 0x30);
const _: () = assert!(core::mem::offset_of!(IMenu, base) == 0x0);
const _: () = assert!(core::mem::offset_of!(IMenu, ui_movie) == 0x10);
const _: () = assert!(core::mem::offset_of!(IMenu, depth_priority) == 0x18);
const _: () = assert!(core::mem::offset_of!(IMenu, menu_flags) == 0x1C);
const _: () = assert!(core::mem::offset_of!(IMenu, input_context) == 0x20);
const _: () = assert!(core::mem::offset_of!(IMenu, fx_delegate) == 0x28);

inherit!(IMenu : FxDelegateHandler);

impl RttiType for IMenu {
    const RTTI: VariantID = RTTI_IMenu;
}

impl IMenu {
    pub const RTTI: VariantID = RTTI_IMenu;
    pub const VTABLE: &'static [VariantID] = &VTABLE_IMenu;
    pub const VR_RUNTIME_DATA_OFFSET: usize = 0x30;
    pub const VFUNC_UNK_09: VariantOffset = VariantOffset::new(0, 0, 0x09);
    pub const VFUNC_UNK_0A: VariantOffset = VariantOffset::new(0, 0, 0x0A);

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

    // override (FxDelegateHandler)
    virtual_method! { pub const VFUNC_DTOR: usize = 0x0; pub fn dtor() }
    virtual_method! { pub const VFUNC_ACCEPT: usize = 0x1; pub fn accept(processor: *mut FxDelegateHandlerCallbackProcessor) }

    // add
    virtual_method! { pub const VFUNC_POST_CREATE: usize = 0x2; pub fn post_create() }
    virtual_method! { pub const VFUNC_UNK_03: usize = 0x3; pub fn unk_03() }
    virtual_method! { pub const VFUNC_PROCESS_MESSAGE: usize = 0x4; pub fn process_message_raw(message: &mut UIMessage) -> i32 }
    virtual_method! { pub const VFUNC_ADVANCE_MOVIE: usize = 0x5; pub fn advance_movie(interval: f32, current_time: u32) }
    virtual_method! { pub const VFUNC_POST_DISPLAY: usize = 0x6; pub fn post_display() }
    virtual_method! { pub const VFUNC_PRE_DISPLAY: usize = 0x7; pub fn pre_display() }
    virtual_method! { pub const VFUNC_REFRESH_PLATFORM: usize = 0x8; pub fn refresh_platform() }

    #[inline(always)]
    pub fn unk_09(&mut self, unk: UI_MENU_Unk09) {
        crate::runtime::require_vr("IMenu::unk_09");
        crate::relocate_virtual!(
            extern "C" fn(*mut Self, UI_MENU_Unk09),
            self as *mut Self,
            Self::VFUNC_UNK_09,
            unk
        )
    }

    #[inline(always)]
    pub fn unk_0a(&mut self) {
        crate::runtime::require_vr("IMenu::unk_0a");
        crate::relocate_virtual!(
            extern "C" fn(*mut Self),
            self as *mut Self,
            Self::VFUNC_UNK_0A
        )
    }

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
    pub fn process_message_storage(
        &mut self,
        message: &mut UIMessage,
    ) -> Enum<UI_MESSAGE_RESULTS, i32> {
        Enum::from_underlying(self.process_message_raw(message))
    }

    #[inline(always)]
    pub fn try_process_message(&mut self, message: &mut UIMessage) -> Option<UI_MESSAGE_RESULTS> {
        self.process_message_storage(message).get()
    }

    #[inline(always)]
    pub fn process_message(&mut self, message: &mut UIMessage) -> UI_MESSAGE_RESULTS {
        self.try_process_message(message)
            .unwrap_or(UI_MESSAGE_RESULTS::kIgnore)
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
    fn unk_09(&mut self, unk: UI_MENU_Unk09);
    fn unk_0a(&mut self);
    fn process_message(&mut self, message: &mut UIMessage) -> UI_MESSAGE_RESULTS;
    fn advance_movie(&mut self, interval: f32, current_time: u32);
    fn post_display(&mut self);
    fn pre_display(&mut self);
    fn refresh_platform(&mut self);
    fn advances_under_pause_menu(&self) -> bool;
    fn allow_saving(&self) -> bool;
    fn always_open(&self) -> bool;
    fn application_menu(&self) -> bool;
    fn assign_cursor_to_renderer(&self) -> bool;
    fn custom_rendering(&self) -> bool;
    fn companion_app_allowed(&self) -> bool;
    fn disable_pause_menu(&self) -> bool;
    fn dont_hide_cursor_when_topmost(&self) -> bool;
    fn freeze_frame_background(&self) -> bool;
    fn freeze_frame_pause(&self) -> bool;
    fn has_button_bar(&self) -> bool;
    fn inventory_item_menu(&self) -> bool;
    fn is_top_button_bar(&self) -> bool;
    fn large_scaleform_render_cache_mode(&self) -> bool;
    fn on_stack(&self) -> bool;
    fn pauses_game(&self) -> bool;
    fn renders_offscreen_targets(&self) -> bool;
    fn renders_under_pause_menu(&self) -> bool;
    fn requires_update(&self) -> bool;
    fn skip_render_during_freeze_frame_screenshot(&self) -> bool;
    fn topmost_rendered_menu(&self) -> bool;
    fn uses_cursor(&self) -> bool;
    fn update_uses_cursor(&self) -> bool;
    fn uses_blurred_background(&self) -> bool;
    fn uses_menu_context(&self) -> bool;
    fn uses_movement_to_direction(&self) -> bool;
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
    fn unk_09(&mut self, unk: UI_MENU_Unk09) {
        self.as_mut().unk_09(unk)
    }

    #[inline(always)]
    fn unk_0a(&mut self) {
        self.as_mut().unk_0a()
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
    fn advances_under_pause_menu(&self) -> bool {
        self.as_ref().advances_under_pause_menu()
    }

    #[inline(always)]
    fn allow_saving(&self) -> bool {
        self.as_ref().allow_saving()
    }

    #[inline(always)]
    fn always_open(&self) -> bool {
        self.as_ref().always_open()
    }

    #[inline(always)]
    fn application_menu(&self) -> bool {
        self.as_ref().application_menu()
    }

    #[inline(always)]
    fn assign_cursor_to_renderer(&self) -> bool {
        self.as_ref().assign_cursor_to_renderer()
    }

    #[inline(always)]
    fn custom_rendering(&self) -> bool {
        self.as_ref().custom_rendering()
    }

    #[inline(always)]
    fn companion_app_allowed(&self) -> bool {
        self.as_ref().companion_app_allowed()
    }

    #[inline(always)]
    fn disable_pause_menu(&self) -> bool {
        self.as_ref().disable_pause_menu()
    }

    #[inline(always)]
    fn dont_hide_cursor_when_topmost(&self) -> bool {
        self.as_ref().dont_hide_cursor_when_topmost()
    }

    #[inline(always)]
    fn freeze_frame_background(&self) -> bool {
        self.as_ref().freeze_frame_background()
    }

    #[inline(always)]
    fn freeze_frame_pause(&self) -> bool {
        self.as_ref().freeze_frame_pause()
    }

    #[inline(always)]
    fn has_button_bar(&self) -> bool {
        self.as_ref().has_button_bar()
    }

    #[inline(always)]
    fn inventory_item_menu(&self) -> bool {
        self.as_ref().inventory_item_menu()
    }

    #[inline(always)]
    fn is_top_button_bar(&self) -> bool {
        self.as_ref().is_top_button_bar()
    }

    #[inline(always)]
    fn large_scaleform_render_cache_mode(&self) -> bool {
        self.as_ref().large_scaleform_render_cache_mode()
    }

    #[inline(always)]
    fn on_stack(&self) -> bool {
        self.as_ref().on_stack()
    }

    #[inline(always)]
    fn pauses_game(&self) -> bool {
        self.as_ref().pauses_game()
    }

    #[inline(always)]
    fn renders_offscreen_targets(&self) -> bool {
        self.as_ref().renders_offscreen_targets()
    }

    #[inline(always)]
    fn renders_under_pause_menu(&self) -> bool {
        self.as_ref().renders_under_pause_menu()
    }

    #[inline(always)]
    fn requires_update(&self) -> bool {
        self.as_ref().requires_update()
    }

    #[inline(always)]
    fn skip_render_during_freeze_frame_screenshot(&self) -> bool {
        self.as_ref().skip_render_during_freeze_frame_screenshot()
    }

    #[inline(always)]
    fn topmost_rendered_menu(&self) -> bool {
        self.as_ref().topmost_rendered_menu()
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
    fn uses_blurred_background(&self) -> bool {
        self.as_ref().uses_blurred_background()
    }

    #[inline(always)]
    fn uses_menu_context(&self) -> bool {
        self.as_ref().uses_menu_context()
    }

    #[inline(always)]
    fn uses_movement_to_direction(&self) -> bool {
        self.as_ref().uses_movement_to_direction()
    }

    #[inline(always)]
    fn modal(&self) -> bool {
        self.as_ref().modal()
    }
}
