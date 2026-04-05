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

/// Snapshot of the current UI-facing control and menu state.
///
/// This is the right starting point when widget or menu runtime code wants to
/// answer several related questions at once without repeatedly querying the UI
/// and input singletons. Reach for the free helper predicates in this module
/// when only one answer is needed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UiControlSnapshot {
    /// Top input context from the control stack, if one is currently available.
    pub top_context: Option<INPUT_CONTEXT_ID>,
    /// Whether the engine reports menus as globally visible.
    pub menus_visible: bool,
    /// Whether the game is currently paused by UI state.
    pub game_paused: bool,
    /// Whether an application-style menu is open.
    pub application_menu_open: bool,
    /// Whether an item-style menu is open.
    pub item_menu_open: bool,
    /// Whether a modal menu is open.
    pub modal_menu_open: bool,
    /// Whether the console is open.
    pub console_open: bool,
    /// Whether the inventory menu is open.
    pub inventory_open: bool,
    /// Whether the map menu is open.
    pub map_open: bool,
    /// Whether the journal menu is open.
    pub journal_open: bool,
    /// Whether the loading menu is open.
    pub loading_open: bool,
    /// Whether the fader menu is currently open.
    pub fader_open: bool,
    /// Whether the fader reports itself as active.
    pub fader_active: bool,
    /// Whether the current input context behaves like menu capture.
    pub menu_like_context: bool,
    /// Whether text input requests are active.
    pub text_input_active: bool,
    /// Whether keyboard/mouse gameplay input is currently ignored.
    pub keyboard_mouse_ignored: bool,
    /// Whether activate-disabled events are currently ignored.
    pub activate_disabled_events_ignored: bool,
    /// Whether player input is blocked at the input layer.
    pub player_input_blocked: bool,
    /// Whether gameplay control flags are enabled.
    pub gameplay_controls_enabled: bool,
    /// Whether menu control flags are enabled.
    pub menu_controls_enabled: bool,
    /// Whether console control flags are enabled.
    pub console_controls_enabled: bool,
}

impl UiControlSnapshot {
    /// Returns `true` when a major menu surface is open.
    ///
    /// This is a broader notion than "menu-like input context": it reflects
    /// visible UI surfaces that typically suppress HUD widgets and gameplay
    /// controls.
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

    /// Returns `true` when a loading or fade transition is active.
    #[inline(always)]
    pub const fn has_loading_transition(self) -> bool {
        self.loading_open || self.fader_open || self.fader_active
    }

    /// Returns `true` when the UI should be treated as the current input owner.
    ///
    /// This is the main predicate for "should gameplay stop reacting to direct
    /// input right now?" logic in menu- or widget-driven plugins.
    #[inline(always)]
    pub const fn is_ui_capturing_input(self) -> bool {
        self.text_input_active
            || self.menu_like_context
            || self.has_major_menu_open()
            || self.has_loading_transition()
    }

    /// Returns `true` when gameplay input is still available.
    ///
    /// This is stricter than just `!is_ui_capturing_input()`: it also checks
    /// keyboard/mouse ignore flags, explicit player-input blocking, and the
    /// gameplay control-map flags.
    #[inline(always)]
    pub const fn is_gameplay_input_available(self) -> bool {
        !self.is_ui_capturing_input()
            && !self.keyboard_mouse_ignored
            && !self.player_input_blocked
            && self.gameplay_controls_enabled
    }

    /// Convenience inverse of [`Self::is_gameplay_input_available`].
    #[inline(always)]
    pub const fn is_gameplay_input_suppressed(self) -> bool {
        !self.is_gameplay_input_available()
    }

    /// Returns `true` when HUD-style widgets should remain visible.
    ///
    /// This is intentionally conservative: it hides HUD widgets during major
    /// menu surfaces, loading/fader transitions, and menu-like input capture.
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

/// Returns the current top-most input context, if one is available.
#[inline(always)]
pub fn top_input_context() -> Option<INPUT_CONTEXT_ID> {
    input::top_context()
}

/// Returns `true` when UI text input requests are active.
#[inline(always)]
pub fn is_text_input_active() -> bool {
    input::has_text_input_requests()
}

/// Returns `true` when the current input context behaves like menu capture.
#[inline(always)]
pub fn is_menu_like_input_context() -> bool {
    input::has_menu_like_context()
}

/// Returns `true` when gameplay keyboard/mouse input is being ignored.
#[inline(always)]
pub fn is_ignoring_keyboard_mouse() -> bool {
    input::ignores_keyboard_mouse()
}

/// Returns `true` when activate-disabled events are being ignored.
#[inline(always)]
pub fn is_ignoring_activate_disabled_events() -> bool {
    input::ignores_activate_disabled_events()
}

/// Returns `true` when player input is blocked at the input layer.
#[inline(always)]
pub fn is_player_input_blocked() -> bool {
    input::is_player_input_blocked()
}

/// Returns `true` when gameplay controls are currently enabled.
#[inline(always)]
pub fn are_gameplay_controls_enabled() -> bool {
    are_control_flags_enabled(&input::GAMEPLAY_CONTROL_FLAGS)
}

/// Returns `true` when menu controls are currently enabled.
#[inline(always)]
pub fn are_menu_controls_enabled() -> bool {
    are_control_flags_enabled(&input::MENU_CONTROL_FLAGS)
}

/// Returns `true` when console controls are currently enabled.
#[inline(always)]
pub fn are_console_controls_enabled() -> bool {
    input::are_controls_enabled(USER_EVENT_FLAG::kConsole)
}

/// Returns `true` when the console menu is open.
#[inline(always)]
pub fn is_console_open() -> bool {
    menus::is_named_menu_open::<Console>()
}

/// Returns `true` when the inventory menu is open.
#[inline(always)]
pub fn is_inventory_open() -> bool {
    menus::is_named_menu_open::<InventoryMenu>()
}

/// Returns `true` when the world map menu is open.
#[inline(always)]
pub fn is_map_open() -> bool {
    menus::is_named_menu_open::<MapMenu>()
}

/// Returns `true` when the journal menu is open.
#[inline(always)]
pub fn is_journal_open() -> bool {
    menus::is_named_menu_open::<JournalMenu>()
}

/// Returns `true` when the loading menu is open.
#[inline(always)]
pub fn is_loading_open() -> bool {
    menus::is_named_menu_open::<LoadingMenu>()
}

/// Returns `true` when a loading or fader transition is active.
#[inline(always)]
pub fn is_loading_transition_active() -> bool {
    is_loading_open() || menus::is_fader_open() || menus::is_fader_active()
}

/// Captures a [`UiControlSnapshot`] from the current menu and input singletons.
///
/// This is the most useful entrypoint for code that wants to cache one coherent
/// UI gating decision and reuse it for several checks in the same frame or
/// callback.
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

/// Returns `true` when the UI is currently capturing input.
#[inline(always)]
pub fn is_ui_capturing_input() -> bool {
    control_snapshot().is_ui_capturing_input()
}

/// Returns `true` when gameplay input is currently available.
#[inline(always)]
pub fn is_gameplay_input_available() -> bool {
    control_snapshot().is_gameplay_input_available()
}

/// Returns `true` when gameplay input is currently suppressed.
#[inline(always)]
pub fn is_gameplay_input_suppressed() -> bool {
    control_snapshot().is_gameplay_input_suppressed()
}

/// Returns `true` when HUD-style widgets should currently be visible.
#[inline(always)]
pub fn should_show_hud_widgets() -> bool {
    control_snapshot().should_show_hud_widgets()
}

/// Enables UI text-input mode for the scope of the returned guard.
#[inline(always)]
pub fn enable_ui_text_input_scoped() -> input::TextInputGuard {
    input::allow_text_input_scoped()
}

/// Pushes the menu-mode input context for the scope of the returned guard.
#[inline(always)]
pub fn push_menu_mode_context_scoped() -> input::ContextGuard {
    input::push_context_scoped(INPUT_CONTEXT_ID::kMenuMode)
}

/// Pushes the cursor input context for the scope of the returned guard.
#[inline(always)]
pub fn push_cursor_context_scoped() -> input::ContextGuard {
    input::push_context_scoped(INPUT_CONTEXT_ID::kCursor)
}

/// Pushes the console input context for the scope of the returned guard.
#[inline(always)]
pub fn push_console_context_scoped() -> input::ContextGuard {
    input::push_context_scoped(INPUT_CONTEXT_ID::kConsole)
}

/// Suppresses gameplay input for the scope of the returned guard.
///
/// This is the direct UI-side companion to the lower-level scoped gameplay
/// input suppression helpers in `sdk::gameplay::input`.
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
