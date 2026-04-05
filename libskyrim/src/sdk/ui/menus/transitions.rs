use crate::re::{BGSLocation, FaderData, FaderMenu, LoadingMenu, LoadingMenuData};
use crate::relocation::RelocationID;
use crate::sdk::core::GamePtr;

use super::{FadeRequest, LoadingMenuRequest, queue_named_message_with};

crate::relocation_func! {
    fn fade_out_game_raw(
        fading_out: bool,
        black_fade: bool,
        fade_duration: f32,
        arg4: bool,
        secs_before_fade: f32,
    ) => RelocationID::new(51909, 52847)
}

/// Queues a typed fade request through the `Fader Menu`.
#[inline(always)]
pub fn queue_fade(request: FadeRequest) -> bool {
    let queued = queue_named_message_with::<FaderMenu, FaderData>(request.message_type, |data| {
        data.min_duration = request.min_duration;
        data.fade_duration = request.fade_duration;
        data.is_fading_out = request.is_fading_out;
        data.is_black = request.is_black;
        data.pauses_game = request.pauses_game;
    });

    if !queued {
        crate::defensive_sdk_warn!(
            "sdk::ui::menus::queue_fade() failed to queue Fader Menu request"
        );
    }

    queued
}

/// Queues a fade-to-black request.
#[inline(always)]
pub fn fade_to_black(min_duration: f32, fade_duration: f32, pauses_game: bool) -> bool {
    queue_fade(FadeRequest::to_black(
        min_duration,
        fade_duration,
        pauses_game,
    ))
}

/// Queues a fade-from-black request.
#[inline(always)]
pub fn fade_from_black(fade_duration: f32, pauses_game: bool) -> bool {
    queue_fade(FadeRequest::from_black(fade_duration, pauses_game))
}

/// Direct engine fade helper backed by the engine's `FadeOutGame` relocation.
///
/// IDA verification on SE `0x1408D5530` shows that this path still constructs
/// `FaderData` through `MessageDataFactoryManager` and dispatches it through
/// the `Fader Menu` path. In practice this bypasses our typed
/// `queue_named_message_with::<FaderMenu, FaderData>(...)` wrapper, but it
/// does not bypass the engine's `Fader Menu` / fader-message subsystem.
///
/// The fourth boolean parameter is still named `arg4` in public community
/// references, so `pauses_game` in the convenience wrappers below remains an
/// inference from `FaderData`, not a fully proven engine symbol name.
#[inline(always)]
pub fn fade_out_game_direct(
    fading_out: bool,
    black_fade: bool,
    fade_duration: f32,
    arg4: bool,
    secs_before_fade: f32,
) {
    fade_out_game_raw(
        fading_out,
        black_fade,
        fade_duration,
        arg4,
        secs_before_fade,
    );
}

#[inline(always)]
pub fn fade_to_black_direct(min_duration: f32, fade_duration: f32, pauses_game: bool) {
    fade_out_game_direct(true, true, fade_duration, pauses_game, min_duration);
}

/// Direct-engine variant of [`fade_from_black`].
#[inline(always)]
pub fn fade_from_black_direct(fade_duration: f32, pauses_game: bool) {
    fade_out_game_direct(false, true, fade_duration, pauses_game, 0.0);
}

/// Queues a typed loading-menu request.
#[inline(always)]
pub fn queue_loading_menu(request: LoadingMenuRequest) -> bool {
    let queued =
        queue_named_message_with::<LoadingMenu, LoadingMenuData>(request.message_type, |data| {
            data.current_location = request.current_location.as_ptr();
            data.unk18 = request.show_loading_text;
        });

    if !queued {
        crate::defensive_sdk_warn!(
            "sdk::ui::menus::queue_loading_menu() failed to queue Loading Menu request"
        );
    }

    queued
}

/// Queues a loading-menu show request.
#[inline(always)]
pub fn show_loading_menu(current_location: GamePtr<BGSLocation>, show_loading_text: bool) -> bool {
    queue_loading_menu(LoadingMenuRequest::show(
        current_location,
        show_loading_text,
    ))
}
