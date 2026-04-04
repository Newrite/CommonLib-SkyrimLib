//! UI/control helpers layered over menu and input state.
//!
//! This module intentionally complements `sdk::gameplay::input` instead of
//! replacing it. The lower input layer remains the place for raw context-stack
//! and control-map work; this UI layer answers the higher-level questions that
//! widget/menu code repeatedly needs:
//!
//! - is the UI currently capturing gameplay input?
//! - should HUD-like widgets be visible?
//! - are we in a loading/fader transition?
//! - which common menu surfaces are open right now?

use crate::re::{
    Console, INPUT_CONTEXT_ID, InventoryMenu, JournalMenu, LoadingMenu, MapMenu, USER_EVENT_FLAG,
};
use crate::sdk::gameplay::input;
use crate::sdk::ui::menus;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UiControlSnapshot {
    pub top_context: Option<INPUT_CONTEXT_ID>,
    pub menus_visible: bool,
    pub game_paused: bool,
    pub application_menu_open: bool,
    pub item_menu_open: bool,
    pub modal_menu_open: bool,
    pub console_open: bool,
    pub inventory_open: bool,
    pub map_open: bool,
    pub journal_open: bool,
    pub loading_open: bool,
    pub fader_open: bool,
    pub fader_active: bool,
    pub menu_like_context: bool,
    pub text_input_active: bool,
    pub keyboard_mouse_ignored: bool,
    pub activate_disabled_events_ignored: bool,
    pub player_input_blocked: bool,
    pub gameplay_controls_enabled: bool,
    pub menu_controls_enabled: bool,
    pub console_controls_enabled: bool,
}

impl UiControlSnapshot {
    #[inline(always)]
    pub const fn has_major_menu_open(self) -> bool {
        self.application_menu_open
            || self.item_menu_open
            || self.modal_menu_open
            || self.console_open
            || self.inventory_open
            || self.map_open
            || self.journal_open
    }

    #[inline(always)]
    pub const fn has_loading_transition(self) -> bool {
        self.loading_open || self.fader_open || self.fader_active
    }

    #[inline(always)]
    pub const fn is_ui_capturing_input(self) -> bool {
        self.text_input_active
            || self.menu_like_context
            || self.has_major_menu_open()
            || self.has_loading_transition()
    }

    #[inline(always)]
    pub const fn is_gameplay_input_available(self) -> bool {
        !self.is_ui_capturing_input()
            && !self.keyboard_mouse_ignored
            && !self.player_input_blocked
            && self.gameplay_controls_enabled
    }

    #[inline(always)]
    pub const fn is_gameplay_input_suppressed(self) -> bool {
        !self.is_gameplay_input_available()
    }

    #[inline(always)]
    pub const fn should_show_hud_widgets(self) -> bool {
        self.menus_visible
            && !self.has_major_menu_open()
            && !self.has_loading_transition()
            && !self.menu_like_context
    }
}

#[inline(always)]
fn are_control_flags_enabled(flags: &[USER_EVENT_FLAG]) -> bool {
    flags.iter().all(|&flag| input::are_controls_enabled(flag))
}

#[inline(always)]
pub fn top_input_context() -> Option<INPUT_CONTEXT_ID> {
    input::top_context()
}

#[inline(always)]
pub fn is_text_input_active() -> bool {
    input::has_text_input_requests()
}

#[inline(always)]
pub fn is_menu_like_input_context() -> bool {
    input::has_menu_like_context()
}

#[inline(always)]
pub fn is_ignoring_keyboard_mouse() -> bool {
    input::ignores_keyboard_mouse()
}

#[inline(always)]
pub fn is_ignoring_activate_disabled_events() -> bool {
    input::ignores_activate_disabled_events()
}

#[inline(always)]
pub fn is_player_input_blocked() -> bool {
    input::is_player_input_blocked()
}

#[inline(always)]
pub fn are_gameplay_controls_enabled() -> bool {
    are_control_flags_enabled(&input::GAMEPLAY_CONTROL_FLAGS)
}

#[inline(always)]
pub fn are_menu_controls_enabled() -> bool {
    are_control_flags_enabled(&input::MENU_CONTROL_FLAGS)
}

#[inline(always)]
pub fn are_console_controls_enabled() -> bool {
    input::are_controls_enabled(USER_EVENT_FLAG::kConsole)
}

#[inline(always)]
pub fn is_console_open() -> bool {
    menus::is_named_menu_open::<Console>()
}

#[inline(always)]
pub fn is_inventory_open() -> bool {
    menus::is_named_menu_open::<InventoryMenu>()
}

#[inline(always)]
pub fn is_map_open() -> bool {
    menus::is_named_menu_open::<MapMenu>()
}

#[inline(always)]
pub fn is_journal_open() -> bool {
    menus::is_named_menu_open::<JournalMenu>()
}

#[inline(always)]
pub fn is_loading_open() -> bool {
    menus::is_named_menu_open::<LoadingMenu>()
}

#[inline(always)]
pub fn is_loading_transition_active() -> bool {
    is_loading_open() || menus::is_fader_open() || menus::is_fader_active()
}

#[inline(always)]
pub fn control_snapshot() -> UiControlSnapshot {
    UiControlSnapshot {
        top_context: input::top_context(),
        menus_visible: menus::are_menus_visible(),
        game_paused: menus::game_is_paused(),
        application_menu_open: menus::is_application_menu_open(),
        item_menu_open: menus::is_item_menu_open(),
        modal_menu_open: menus::is_modal_menu_open(),
        console_open: is_console_open(),
        inventory_open: is_inventory_open(),
        map_open: is_map_open(),
        journal_open: is_journal_open(),
        loading_open: is_loading_open(),
        fader_open: menus::is_fader_open(),
        fader_active: menus::is_fader_active(),
        menu_like_context: input::has_menu_like_context(),
        text_input_active: input::has_text_input_requests(),
        keyboard_mouse_ignored: input::ignores_keyboard_mouse(),
        activate_disabled_events_ignored: input::ignores_activate_disabled_events(),
        player_input_blocked: input::is_player_input_blocked(),
        gameplay_controls_enabled: are_gameplay_controls_enabled(),
        menu_controls_enabled: are_menu_controls_enabled(),
        console_controls_enabled: are_console_controls_enabled(),
    }
}

#[inline(always)]
pub fn is_ui_capturing_input() -> bool {
    control_snapshot().is_ui_capturing_input()
}

#[inline(always)]
pub fn is_gameplay_input_available() -> bool {
    control_snapshot().is_gameplay_input_available()
}

#[inline(always)]
pub fn is_gameplay_input_suppressed() -> bool {
    control_snapshot().is_gameplay_input_suppressed()
}

#[inline(always)]
pub fn should_show_hud_widgets() -> bool {
    control_snapshot().should_show_hud_widgets()
}

#[inline(always)]
pub fn enable_ui_text_input_scoped() -> input::TextInputGuard {
    input::allow_text_input_scoped()
}

#[inline(always)]
pub fn push_menu_mode_context_scoped() -> input::ContextGuard {
    input::push_context_scoped(INPUT_CONTEXT_ID::kMenuMode)
}

#[inline(always)]
pub fn push_cursor_context_scoped() -> input::ContextGuard {
    input::push_context_scoped(INPUT_CONTEXT_ID::kCursor)
}

#[inline(always)]
pub fn push_console_context_scoped() -> input::ContextGuard {
    input::push_context_scoped(INPUT_CONTEXT_ID::kConsole)
}

#[inline(always)]
pub fn suppress_gameplay_input_for_ui_scoped() -> input::InputStateGuard {
    input::scoped_gameplay_input_suppressed()
}

#[cfg(test)]
mod tests {
    use super::UiControlSnapshot;
    use crate::re::INPUT_CONTEXT_ID;

    fn gameplay_snapshot() -> UiControlSnapshot {
        UiControlSnapshot {
            top_context: Some(INPUT_CONTEXT_ID::kGameplay),
            menus_visible: true,
            game_paused: false,
            application_menu_open: false,
            item_menu_open: false,
            modal_menu_open: false,
            console_open: false,
            inventory_open: false,
            map_open: false,
            journal_open: false,
            loading_open: false,
            fader_open: false,
            fader_active: false,
            menu_like_context: false,
            text_input_active: false,
            keyboard_mouse_ignored: false,
            activate_disabled_events_ignored: false,
            player_input_blocked: false,
            gameplay_controls_enabled: true,
            menu_controls_enabled: true,
            console_controls_enabled: true,
        }
    }

    #[test]
    fn gameplay_snapshot_reports_available_input_and_visible_hud() {
        let snapshot = gameplay_snapshot();

        assert!(!snapshot.is_ui_capturing_input());
        assert!(snapshot.is_gameplay_input_available());
        assert!(!snapshot.is_gameplay_input_suppressed());
        assert!(snapshot.should_show_hud_widgets());
    }

    #[test]
    fn menu_like_snapshot_captures_input_and_hides_hud() {
        let mut snapshot = gameplay_snapshot();
        snapshot.top_context = Some(INPUT_CONTEXT_ID::kInventory);
        snapshot.item_menu_open = true;
        snapshot.inventory_open = true;
        snapshot.menu_like_context = true;

        assert!(snapshot.has_major_menu_open());
        assert!(snapshot.is_ui_capturing_input());
        assert!(!snapshot.is_gameplay_input_available());
        assert!(snapshot.is_gameplay_input_suppressed());
        assert!(!snapshot.should_show_hud_widgets());
    }

    #[test]
    fn loading_transition_counts_as_ui_capture() {
        let mut snapshot = gameplay_snapshot();
        snapshot.loading_open = true;

        assert!(snapshot.has_loading_transition());
        assert!(snapshot.is_ui_capturing_input());
        assert!(!snapshot.should_show_hud_widgets());
    }

    #[test]
    fn ignored_keyboard_mouse_disables_gameplay_without_marking_ui_capture() {
        let mut snapshot = gameplay_snapshot();
        snapshot.keyboard_mouse_ignored = true;

        assert!(!snapshot.is_ui_capturing_input());
        assert!(!snapshot.is_gameplay_input_available());
        assert!(snapshot.is_gameplay_input_suppressed());
    }
}
