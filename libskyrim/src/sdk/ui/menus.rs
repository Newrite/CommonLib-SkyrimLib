//! Menu-oriented high-level helpers.

use crate::re::{BSFixedString, FaderData, FaderMenu, UI, UI_MESSAGE_TYPE, UIMessageQueue};
use crate::sdk::core::GameRef;

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
pub fn is_menu_open(menu_name: &str) -> bool {
    ui().is_menu_open(menu_name)
}

#[inline(always)]
pub fn is_fader_open() -> bool {
    is_menu_open(FaderMenu::MENU_NAME)
}

#[inline(always)]
pub fn process_commands() {
    unsafe { message_queue().with_mut_unchecked(UIMessageQueue::process_commands) };
}

#[inline(always)]
pub fn queue_fade(request: FadeRequest) -> bool {
    unsafe {
        message_queue().with_mut_unchecked(|queue| {
            let class_name = BSFixedString::from_str(FaderData::CLASS_NAME);
            let data = queue.create_ui_message_data(&class_name);
            if data.is_null() {
                return false;
            }

            let data = data.cast::<FaderData>();
            (*data).min_duration = request.min_duration;
            (*data).fade_duration = request.fade_duration;
            (*data).is_fading_out = request.is_fading_out;
            (*data).is_black = request.is_black;
            (*data).pauses_game = request.pauses_game;

            let menu_name = BSFixedString::from_str(FaderMenu::MENU_NAME);
            queue.add_message(&menu_name, request.message_type, data.cast());
            true
        })
    }
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
