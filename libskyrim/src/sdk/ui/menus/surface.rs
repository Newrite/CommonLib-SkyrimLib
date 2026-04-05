use crate::re::{FaderMenu, GFxMovieView, GPtr, IMenu, UI, UICreateFn, UIMessageQueue};
use crate::sdk::core::{DynamicCastExt, GameRef};

use super::NamedMenu;

/// Default search depth used by top-most menu helpers.
pub const DEFAULT_TOP_MOST_MENU_DEPTH_LIMIT: u32 = 15;

/// Returns the live `UI` singleton.
#[inline(always)]
pub fn ui() -> GameRef<UI> {
    unsafe { GameRef::from_raw(UI::get_singleton()) }
}

/// Returns the live `UIMessageQueue` singleton.
#[inline(always)]
pub fn message_queue() -> GameRef<UIMessageQueue> {
    unsafe { GameRef::from_raw(UIMessageQueue::get_singleton()) }
}

/// Shows or hides menus globally.
#[inline(always)]
pub fn show_menus(show: bool) {
    unsafe { ui().with_mut_unchecked(|ui| ui.show_menus(show)) };
}

/// Returns `true` when menus are globally visible.
#[inline(always)]
pub fn are_menus_visible() -> bool {
    ui().is_showing_menus()
}

/// Returns `true` when the game is currently paused by UI state.
#[inline(always)]
pub fn game_is_paused() -> bool {
    ui().game_is_paused()
}

/// Returns `true` when an application-style menu is open.
#[inline(always)]
pub fn is_application_menu_open() -> bool {
    ui().is_application_menu_open()
}

/// Returns `true` when an item-style menu is open.
#[inline(always)]
pub fn is_item_menu_open() -> bool {
    ui().is_item_menu_open()
}

/// Returns `true` when a modal menu is open.
#[inline(always)]
pub fn is_modal_menu_open() -> bool {
    ui().is_modal_menu_open()
}

/// Returns `true` when the pause menu is disabled.
#[inline(always)]
pub fn is_pause_menu_disabled() -> bool {
    ui().is_pause_menu_disabled()
}

/// Returns `true` when saving is currently allowed.
#[inline(always)]
pub fn is_saving_allowed() -> bool {
    ui().is_saving_allowed()
}

/// Returns `true` when the cursor should be hidden for the top-most menu.
#[inline(always)]
pub fn is_cursor_hidden_when_topmost() -> bool {
    ui().is_cursor_hidden_when_topmost()
}

/// Returns `true` when UI custom rendering is active.
#[inline(always)]
pub fn is_custom_rendering_active() -> bool {
    ui().is_using_custom_rendering()
}

/// Returns the named menu if it is currently available.
#[inline(always)]
pub fn menu(menu_name: &str) -> GPtr<IMenu> {
    ui().get_menu(menu_name)
}

/// Typed variant of [`menu`].
#[inline(always)]
pub fn named_menu<M>() -> GPtr<IMenu>
where
    M: NamedMenu,
{
    menu(M::MENU_NAME)
}

/// Returns the named menu's `GFxMovieView`, if any.
#[inline(always)]
pub fn movie_view(menu_name: &str) -> GPtr<GFxMovieView> {
    ui().get_movie_view(menu_name)
}

/// Typed variant of [`movie_view`].
#[inline(always)]
pub fn named_movie_view<M>() -> GPtr<GFxMovieView>
where
    M: NamedMenu,
{
    movie_view(M::MENU_NAME)
}

/// Returns the current top-most menu up to the given depth limit.
pub fn top_most_menu(depth_limit: u32) -> GPtr<IMenu> {
    let mut result = core::ptr::null_mut();
    unsafe {
        ui().with_mut_unchecked(|ui| ui.get_top_most_menu(&mut result, depth_limit));
        GPtr::new(result)
    }
}

/// Returns the current top-most menu using the default depth limit.
#[inline(always)]
pub fn top_most_menu_default() -> GPtr<IMenu> {
    top_most_menu(DEFAULT_TOP_MOST_MENU_DEPTH_LIMIT)
}

/// Returns `true` when the named menu is open.
#[inline(always)]
pub fn is_menu_open(menu_name: &str) -> bool {
    ui().is_menu_open(menu_name)
}

/// Typed variant of [`is_menu_open`].
#[inline(always)]
pub fn is_named_menu_open<M>() -> bool
where
    M: NamedMenu,
{
    is_menu_open(M::MENU_NAME)
}

/// Returns `true` when the `Fader Menu` is open.
#[inline(always)]
pub fn is_fader_open() -> bool {
    is_named_menu_open::<FaderMenu>()
}

/// Returns `true` when the `Fader Menu` reports itself as active.
pub fn is_fader_active() -> bool {
    let menu = named_menu::<FaderMenu>();
    if menu.is_null() {
        return false;
    }

    menu.try_cast_ref::<FaderMenu>()
        .map(|menu| menu.runtime_data().is_active)
        .unwrap_or(false)
}

/// Registers or replaces a named menu create function.
#[inline(always)]
pub fn register_menu(menu_name: &str, creator: Option<UICreateFn>) {
    unsafe { ui().with_mut_unchecked(|ui| ui.register(menu_name, creator)) };
}

/// Typed variant of [`register_menu`].
#[inline(always)]
pub fn register_named_menu<M>(creator: Option<UICreateFn>)
where
    M: NamedMenu,
{
    register_menu(M::MENU_NAME, creator)
}
