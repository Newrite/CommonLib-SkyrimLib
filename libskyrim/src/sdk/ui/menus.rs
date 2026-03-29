//! Menu-oriented high-level helpers.

use alloc::borrow::ToOwned;
use alloc::ffi::CString;
use core::ffi::c_void;
use core::ptr::NonNull;

use crate::re::{
    BGSLocation, BSFixedString, BSUIMessageData, BSUIMessageDataData, BSUIScaleformData, FaderData,
    FaderMenu, GFxEvent, GFxMovieView, GPtr, IMenu, IUIMessageData, LoadingMenu, LoadingMenuData,
    MenuOpenCloseEvent, UI, UI_MESSAGE_TYPE, UICreate_t, UIMessageQueue,
};
use crate::sdk::core::{DynamicCastExt, GamePtr, GameRef};
use crate::sdk::events::source::{EventFlow, EventInstallError, EventSubscription, IntoEventFlow};
use crate::sdk::events::ui as ui_events;

pub const DEFAULT_TOP_MOST_MENU_DEPTH_LIMIT: u32 = 15;

/// Small trait for menu-like types with a stable UI registration name.
///
/// Users can implement this for their own custom menu markers even before the
/// concrete `RE::*Menu` type exists in `libskyrim`.
pub trait NamedMenu {
    const MENU_NAME: &'static str;
}

/// Marker trait for menu message-data types constructible through the engine's
/// `UIMessageDataFactory`.
pub trait TypedMenuMessageData {
    const CLASS_NAME: &'static str;
}

macro_rules! impl_named_menu {
    ($($ty:path),+ $(,)?) => {
        $(
            impl NamedMenu for $ty {
                const MENU_NAME: &'static str = <$ty>::MENU_NAME;
            }
        )+
    };
}

impl_named_menu!(
    crate::re::BarterMenu,
    crate::re::BookMenu,
    crate::re::Console,
    crate::re::ConsoleNativeUIMenu,
    crate::re::ContainerMenu,
    crate::re::CraftingMenu,
    crate::re::CreationClubMenu,
    crate::re::CreditsMenu,
    crate::re::CursorMenu,
    crate::re::DialogueMenu,
    crate::re::FavoritesMenu,
    crate::re::FaderMenu,
    crate::re::GiftMenu,
    crate::re::HUDMenu,
    crate::re::InventoryMenu,
    crate::re::JournalMenu,
    crate::re::KinectMenu,
    crate::re::LevelUpMenu,
    crate::re::LoadingMenu,
    crate::re::LoadWaitSpinner,
    crate::re::LockpickingMenu,
    crate::re::MagicMenu,
    crate::re::MainMenu,
    crate::re::MapMenu,
    crate::re::MessageBoxMenu,
    crate::re::MistMenu,
    crate::re::ModManagerMenu,
    crate::re::RaceSexMenu,
    crate::re::SafeZoneMenu,
    crate::re::SleepWaitMenu,
    crate::re::StatsMenu,
    crate::re::TitleSequenceMenu,
    crate::re::TrainingMenu,
    crate::re::TutorialMenu,
    crate::re::TweenMenu,
    crate::re::WSActivateRollover,
);

macro_rules! impl_typed_menu_message_data {
    ($($ty:path),+ $(,)?) => {
        $(
            impl TypedMenuMessageData for $ty {
                const CLASS_NAME: &'static str = <$ty>::CLASS_NAME;
            }
        )+
    };
}

impl_typed_menu_message_data!(
    BSUIMessageData,
    BSUIScaleformData,
    FaderData,
    LoadingMenuData,
);

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FadeRequest {
    pub min_duration: f32,
    pub fade_duration: f32,
    pub message_type: UI_MESSAGE_TYPE,
    pub is_fading_out: bool,
    pub is_black: bool,
    pub pauses_game: bool,
}

impl FadeRequest {
    #[inline(always)]
    pub const fn to_black(min_duration: f32, fade_duration: f32, pauses_game: bool) -> Self {
        Self {
            min_duration,
            fade_duration,
            message_type: UI_MESSAGE_TYPE::kShow,
            is_fading_out: true,
            is_black: true,
            pauses_game,
        }
    }

    #[inline(always)]
    pub const fn from_black(fade_duration: f32, pauses_game: bool) -> Self {
        Self {
            min_duration: 0.0,
            fade_duration,
            message_type: UI_MESSAGE_TYPE::kShow,
            is_fading_out: false,
            is_black: true,
            pauses_game,
        }
    }
}

impl Default for FadeRequest {
    #[inline(always)]
    fn default() -> Self {
        Self::to_black(0.0, 0.25, false)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LoadingMenuRequest {
    pub current_location: GamePtr<BGSLocation>,
    pub message_type: UI_MESSAGE_TYPE,
    pub show_loading_text: bool,
}

impl LoadingMenuRequest {
    #[inline(always)]
    pub const fn show(current_location: GamePtr<BGSLocation>, show_loading_text: bool) -> Self {
        Self {
            current_location,
            message_type: UI_MESSAGE_TYPE::kShow,
            show_loading_text,
        }
    }
}

impl Default for LoadingMenuRequest {
    #[inline(always)]
    fn default() -> Self {
        Self::show(GamePtr::null(), false)
    }
}

#[inline(always)]
pub fn ui() -> GameRef<UI> {
    unsafe { GameRef::from_raw(UI::get_singleton()) }
}

#[inline(always)]
pub fn message_queue() -> GameRef<UIMessageQueue> {
    unsafe { GameRef::from_raw(UIMessageQueue::get_singleton()) }
}

#[inline(always)]
pub fn show_menus(show: bool) {
    unsafe { ui().with_mut_unchecked(|ui| ui.show_menus(show)) };
}

#[inline(always)]
pub fn are_menus_visible() -> bool {
    ui().is_showing_menus()
}

#[inline(always)]
pub fn game_is_paused() -> bool {
    ui().game_is_paused()
}

#[inline(always)]
pub fn is_application_menu_open() -> bool {
    ui().is_application_menu_open()
}

#[inline(always)]
pub fn is_item_menu_open() -> bool {
    ui().is_item_menu_open()
}

#[inline(always)]
pub fn is_modal_menu_open() -> bool {
    ui().is_modal_menu_open()
}

#[inline(always)]
pub fn is_pause_menu_disabled() -> bool {
    ui().is_pause_menu_disabled()
}

#[inline(always)]
pub fn is_saving_allowed() -> bool {
    ui().is_saving_allowed()
}

#[inline(always)]
pub fn is_cursor_hidden_when_topmost() -> bool {
    ui().is_cursor_hidden_when_topmost()
}

#[inline(always)]
pub fn is_custom_rendering_active() -> bool {
    ui().is_using_custom_rendering()
}

#[inline(always)]
pub fn menu(menu_name: &str) -> GPtr<IMenu> {
    ui().get_menu(menu_name)
}

#[inline(always)]
pub fn named_menu<M>() -> GPtr<IMenu>
where
    M: NamedMenu,
{
    menu(M::MENU_NAME)
}

#[inline(always)]
pub fn movie_view(menu_name: &str) -> GPtr<GFxMovieView> {
    ui().get_movie_view(menu_name)
}

#[inline(always)]
pub fn named_movie_view<M>() -> GPtr<GFxMovieView>
where
    M: NamedMenu,
{
    movie_view(M::MENU_NAME)
}

pub fn top_most_menu(depth_limit: u32) -> GPtr<IMenu> {
    let mut result = core::ptr::null_mut();
    unsafe {
        ui().with_mut_unchecked(|ui| ui.get_top_most_menu(&mut result, depth_limit));
        GPtr::new(result)
    }
}

#[inline(always)]
pub fn top_most_menu_default() -> GPtr<IMenu> {
    top_most_menu(DEFAULT_TOP_MOST_MENU_DEPTH_LIMIT)
}

#[inline(always)]
pub fn is_menu_open(menu_name: &str) -> bool {
    ui().is_menu_open(menu_name)
}

#[inline(always)]
pub fn is_named_menu_open<M>() -> bool
where
    M: NamedMenu,
{
    is_menu_open(M::MENU_NAME)
}

#[inline(always)]
pub fn is_fader_open() -> bool {
    is_named_menu_open::<FaderMenu>()
}

pub fn is_fader_active() -> bool {
    let menu = named_menu::<FaderMenu>();
    if menu.is_null() {
        return false;
    }

    menu.try_cast_ref::<FaderMenu>()
        .map(|menu| menu.runtime_data().is_active)
        .unwrap_or(false)
}

#[inline(always)]
pub fn register_menu(menu_name: &str, creator: Option<UICreate_t>) {
    unsafe { ui().with_mut_unchecked(|ui| ui.register(menu_name, creator)) };
}

#[inline(always)]
pub fn register_named_menu<M>(creator: Option<UICreate_t>)
where
    M: NamedMenu,
{
    register_menu(M::MENU_NAME, creator)
}

#[inline(always)]
pub fn process_commands() {
    unsafe { message_queue().with_mut_unchecked(UIMessageQueue::process_commands) };
}

#[inline(always)]
pub fn queue_message(menu_name: &str, message_type: UI_MESSAGE_TYPE) {
    unsafe { queue_message_data_unchecked(menu_name, message_type, core::ptr::null_mut()) };
}

/// # Safety
/// `data` must either be null or point to a live `IUIMessageData` instance
/// suitable for the target menu and owned according to the engine's UI message
/// contract.
pub unsafe fn queue_message_data_unchecked(
    menu_name: &str,
    message_type: UI_MESSAGE_TYPE,
    data: *mut IUIMessageData,
) {
    unsafe {
        message_queue().with_mut_unchecked(|queue| {
            let menu_name = BSFixedString::from_str(menu_name);
            queue.add_message(&menu_name, message_type, data);
        })
    };
}

#[inline(always)]
fn create_message_data<T>() -> Option<NonNull<T>>
where
    T: TypedMenuMessageData,
{
    let class_name = CString::new(T::CLASS_NAME).ok()?;
    NonNull::new(unsafe {
        crate::ffi::commonlib_create_ui_message_data(class_name.as_ptr()).cast()
    })
}

pub fn queue_message_with<T>(
    menu_name: &str,
    message_type: UI_MESSAGE_TYPE,
    init: impl FnOnce(&mut T),
) -> bool
where
    T: TypedMenuMessageData,
{
    unsafe {
        message_queue().with_mut_unchecked(|queue| {
            let Some(mut data) = create_message_data::<T>() else {
                return false;
            };

            init(data.as_mut());

            let menu_name = BSFixedString::from_str(menu_name);
            queue.add_message(&menu_name, message_type, data.as_ptr().cast());
            true
        })
    }
}

#[inline(always)]
pub fn queue_named_message_with<M, T>(
    message_type: UI_MESSAGE_TYPE,
    init: impl FnOnce(&mut T),
) -> bool
where
    M: NamedMenu,
    T: TypedMenuMessageData,
{
    queue_message_with::<T>(M::MENU_NAME, message_type, init)
}

#[inline(always)]
pub fn open_menu(menu_name: &str) {
    queue_message(menu_name, UI_MESSAGE_TYPE::kShow);
}

#[inline(always)]
pub fn close_menu(menu_name: &str) {
    queue_message(menu_name, UI_MESSAGE_TYPE::kHide);
}

#[inline(always)]
pub fn force_close_menu(menu_name: &str) {
    queue_message(menu_name, UI_MESSAGE_TYPE::kForceHide);
}

#[inline(always)]
pub fn reshow_menu(menu_name: &str) {
    queue_message(menu_name, UI_MESSAGE_TYPE::kReshow);
}

#[inline(always)]
pub fn toggle_menu(menu_name: &str, open: bool) {
    if open {
        open_menu(menu_name);
    } else {
        close_menu(menu_name);
    }
}

#[inline(always)]
pub fn open_named_menu<M>()
where
    M: NamedMenu,
{
    open_menu(M::MENU_NAME)
}

#[inline(always)]
pub fn close_named_menu<M>()
where
    M: NamedMenu,
{
    close_menu(M::MENU_NAME)
}

#[inline(always)]
pub fn force_close_named_menu<M>()
where
    M: NamedMenu,
{
    force_close_menu(M::MENU_NAME)
}

#[inline(always)]
pub fn toggle_named_menu<M>(open: bool)
where
    M: NamedMenu,
{
    toggle_menu(M::MENU_NAME, open)
}

pub fn subscribe_open_close<F, R>(
    mut callback: F,
) -> Result<EventSubscription<'static, MenuOpenCloseEvent>, EventInstallError>
where
    F: FnMut(&MenuOpenCloseEvent) -> R + 'static,
    R: IntoEventFlow,
{
    ui_events::subscribe(move |event| match event {
        Some(event) => callback(event).into_event_flow(),
        None => EventFlow::Continue,
    })
}

pub fn subscribe_menu_open_close<F, R>(
    menu_name: &str,
    mut callback: F,
) -> Result<EventSubscription<'static, MenuOpenCloseEvent>, EventInstallError>
where
    F: FnMut(&MenuOpenCloseEvent) -> R + 'static,
    R: IntoEventFlow,
{
    let menu_name = menu_name.to_owned();
    subscribe_open_close(move |event| {
        if event.menu_name.as_str() == menu_name.as_str() {
            callback(event).into_event_flow()
        } else {
            EventFlow::Continue
        }
    })
}

pub fn subscribe_menu_open<F, R>(
    menu_name: &str,
    mut callback: F,
) -> Result<EventSubscription<'static, MenuOpenCloseEvent>, EventInstallError>
where
    F: FnMut(&MenuOpenCloseEvent) -> R + 'static,
    R: IntoEventFlow,
{
    let menu_name = menu_name.to_owned();
    subscribe_open_close(move |event| {
        if event.opening && event.menu_name.as_str() == menu_name.as_str() {
            callback(event).into_event_flow()
        } else {
            EventFlow::Continue
        }
    })
}

pub fn subscribe_menu_close<F, R>(
    menu_name: &str,
    mut callback: F,
) -> Result<EventSubscription<'static, MenuOpenCloseEvent>, EventInstallError>
where
    F: FnMut(&MenuOpenCloseEvent) -> R + 'static,
    R: IntoEventFlow,
{
    let menu_name = menu_name.to_owned();
    subscribe_open_close(move |event| {
        if !event.opening && event.menu_name.as_str() == menu_name.as_str() {
            callback(event).into_event_flow()
        } else {
            EventFlow::Continue
        }
    })
}

#[inline(always)]
pub fn subscribe_named_open_close<M, F, R>(
    callback: F,
) -> Result<EventSubscription<'static, MenuOpenCloseEvent>, EventInstallError>
where
    M: NamedMenu,
    F: FnMut(&MenuOpenCloseEvent) -> R + 'static,
    R: IntoEventFlow,
{
    subscribe_menu_open_close(M::MENU_NAME, callback)
}

#[inline(always)]
pub fn subscribe_named_open<M, F, R>(
    callback: F,
) -> Result<EventSubscription<'static, MenuOpenCloseEvent>, EventInstallError>
where
    M: NamedMenu,
    F: FnMut(&MenuOpenCloseEvent) -> R + 'static,
    R: IntoEventFlow,
{
    subscribe_menu_open(M::MENU_NAME, callback)
}

#[inline(always)]
pub fn subscribe_named_close<M, F, R>(
    callback: F,
) -> Result<EventSubscription<'static, MenuOpenCloseEvent>, EventInstallError>
where
    M: NamedMenu,
    F: FnMut(&MenuOpenCloseEvent) -> R + 'static,
    R: IntoEventFlow,
{
    subscribe_menu_close(M::MENU_NAME, callback)
}

#[inline(always)]
pub fn queue_fade(request: FadeRequest) -> bool {
    queue_named_message_with::<FaderMenu, FaderData>(request.message_type, |data| {
        data.min_duration = request.min_duration;
        data.fade_duration = request.fade_duration;
        data.is_fading_out = request.is_fading_out;
        data.is_black = request.is_black;
        data.pauses_game = request.pauses_game;
    })
}

#[inline(always)]
pub fn fade_to_black(min_duration: f32, fade_duration: f32, pauses_game: bool) -> bool {
    queue_fade(FadeRequest::to_black(
        min_duration,
        fade_duration,
        pauses_game,
    ))
}

#[inline(always)]
pub fn fade_from_black(fade_duration: f32, pauses_game: bool) -> bool {
    queue_fade(FadeRequest::from_black(fade_duration, pauses_game))
}

#[inline(always)]
pub fn queue_loading_menu(request: LoadingMenuRequest) -> bool {
    queue_named_message_with::<LoadingMenu, LoadingMenuData>(request.message_type, |data| {
        data.current_location = request.current_location.as_ptr();
        data.unk18 = request.show_loading_text;
    })
}

#[inline(always)]
pub fn show_loading_menu(current_location: GamePtr<BGSLocation>, show_loading_text: bool) -> bool {
    queue_loading_menu(LoadingMenuRequest::show(
        current_location,
        show_loading_text,
    ))
}

#[inline(always)]
pub fn queue_bsui_bool_message(menu_name: &str, message_type: UI_MESSAGE_TYPE, data: bool) -> bool {
    queue_message_with::<BSUIMessageData>(menu_name, message_type, |msg_data| {
        msg_data.data = BSUIMessageDataData { b: data };
    })
}

#[inline(always)]
pub fn queue_bsui_uint_message(menu_name: &str, message_type: UI_MESSAGE_TYPE, data: u32) -> bool {
    queue_message_with::<BSUIMessageData>(menu_name, message_type, |msg_data| {
        msg_data.data = BSUIMessageDataData { u: data };
    })
}

#[inline(always)]
pub fn queue_bsui_ptr_message(
    menu_name: &str,
    message_type: UI_MESSAGE_TYPE,
    data: *mut c_void,
) -> bool {
    queue_message_with::<BSUIMessageData>(menu_name, message_type, |msg_data| {
        msg_data.data = BSUIMessageDataData { p: data };
    })
}

#[inline(always)]
pub fn queue_bsui_string_message(
    menu_name: &str,
    message_type: UI_MESSAGE_TYPE,
    value: &str,
) -> bool {
    queue_message_with::<BSUIMessageData>(menu_name, message_type, |msg_data| {
        msg_data.fixed_str = BSFixedString::from_str(value);
    })
}

#[inline(always)]
pub fn queue_bsui_string_bool_message(
    menu_name: &str,
    message_type: UI_MESSAGE_TYPE,
    value: &str,
    data: bool,
) -> bool {
    queue_message_with::<BSUIMessageData>(menu_name, message_type, |msg_data| {
        msg_data.fixed_str = BSFixedString::from_str(value);
        msg_data.data = BSUIMessageDataData { b: data };
    })
}

#[inline(always)]
pub fn queue_bsui_string_float_message(
    menu_name: &str,
    message_type: UI_MESSAGE_TYPE,
    value: &str,
    data: f32,
) -> bool {
    queue_message_with::<BSUIMessageData>(menu_name, message_type, |msg_data| {
        msg_data.fixed_str = BSFixedString::from_str(value);
        msg_data.data = BSUIMessageDataData { f: data };
    })
}

#[inline(always)]
pub fn queue_bsui_string_uint_message(
    menu_name: &str,
    message_type: UI_MESSAGE_TYPE,
    value: &str,
    data: u32,
) -> bool {
    queue_message_with::<BSUIMessageData>(menu_name, message_type, |msg_data| {
        msg_data.fixed_str = BSFixedString::from_str(value);
        msg_data.data = BSUIMessageDataData { u: data };
    })
}

#[inline(always)]
pub fn queue_scaleform_event_message(
    menu_name: &str,
    message_type: UI_MESSAGE_TYPE,
    scaleform_event: *mut GFxEvent,
) -> bool {
    queue_message_with::<BSUIScaleformData>(menu_name, message_type, |msg_data| {
        msg_data.scaleform_event = scaleform_event;
    })
}
