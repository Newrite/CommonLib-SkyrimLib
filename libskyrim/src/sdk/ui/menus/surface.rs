use crate::re::{FaderMenu, GFxMovieView, GPtr, IMenu, UI, UICreateFn, UIMessageQueue};
use crate::sdk::core::{DynamicCastExt, GameRef};

use super::NamedMenu;

pub const DEFAULT_TOP_MOST_MENU_DEPTH_LIMIT: u32 = 15;

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
pub fn register_menu(menu_name: &str, creator: Option<UICreateFn>) {
    unsafe { ui().with_mut_unchecked(|ui| ui.register(menu_name, creator)) };
}

#[inline(always)]
pub fn register_named_menu<M>(creator: Option<UICreateFn>)
where
    M: NamedMenu,
{
    register_menu(M::MENU_NAME, creator)
}
