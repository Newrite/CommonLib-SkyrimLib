use super::{RuntimePhaseBlocker, RuntimePhaseSnapshot};
use crate::re::INPUT_CONTEXT_ID;
use crate::sdk::ui::controls::UiControlSnapshot;

fn gameplay_ui_snapshot() -> UiControlSnapshot {
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

fn gameplay_phase_snapshot() -> RuntimePhaseSnapshot {
    RuntimePhaseSnapshot {
        ui: gameplay_ui_snapshot(),
        pause_menu_disabled: false,
        saving_allowed: true,
        cursor_hidden_when_topmost: false,
        custom_rendering_active: false,
    }
}

#[test]
fn gameplay_snapshot_reports_safe_phase_and_visible_hud() {
    let snapshot = gameplay_phase_snapshot();

    assert!(!snapshot.is_loading_or_fading());
    assert_eq!(snapshot.gameplay_blocker(), None);
    assert!(snapshot.is_safe_for_gameplay());
    assert!(!snapshot.is_unsafe_for_gameplay());
    assert!(!snapshot.should_defer_gameplay_work());
    assert_eq!(snapshot.hud_widget_blocker(), None);
    assert!(snapshot.allows_hud_widgets());
}

#[test]
fn loading_and_fader_take_priority_as_phase_blockers() {
    let mut loading = gameplay_phase_snapshot();
    loading.ui.loading_open = true;
    assert_eq!(
        loading.gameplay_blocker(),
        Some(RuntimePhaseBlocker::LoadingScreen)
    );
    assert_eq!(
        loading.hud_widget_blocker(),
        Some(RuntimePhaseBlocker::LoadingScreen)
    );

    let mut fader = gameplay_phase_snapshot();
    fader.ui.fader_active = true;
    assert_eq!(fader.gameplay_blocker(), Some(RuntimePhaseBlocker::Fader));
    assert_eq!(fader.hud_widget_blocker(), Some(RuntimePhaseBlocker::Fader));
}

#[test]
fn paused_and_menu_states_block_gameplay() {
    let mut paused = gameplay_phase_snapshot();
    paused.ui.game_paused = true;
    assert_eq!(
        paused.gameplay_blocker(),
        Some(RuntimePhaseBlocker::GamePaused)
    );

    let mut menu = gameplay_phase_snapshot();
    menu.ui.inventory_open = true;
    menu.ui.item_menu_open = true;
    assert_eq!(
        menu.gameplay_blocker(),
        Some(RuntimePhaseBlocker::MajorMenuOpen)
    );
    assert_eq!(
        menu.hud_widget_blocker(),
        Some(RuntimePhaseBlocker::MajorMenuOpen)
    );
}

#[test]
fn input_level_blockers_fire_after_ui_phase_checks() {
    let mut snapshot = gameplay_phase_snapshot();
    snapshot.ui.text_input_active = true;
    assert_eq!(
        snapshot.gameplay_blocker(),
        Some(RuntimePhaseBlocker::TextInput)
    );

    snapshot.ui.text_input_active = false;
    snapshot.ui.player_input_blocked = true;
    assert_eq!(
        snapshot.gameplay_blocker(),
        Some(RuntimePhaseBlocker::PlayerInputBlocked)
    );

    snapshot.ui.player_input_blocked = false;
    snapshot.ui.gameplay_controls_enabled = false;
    assert_eq!(
        snapshot.gameplay_blocker(),
        Some(RuntimePhaseBlocker::GameplayControlsDisabled)
    );
}

#[test]
fn hidden_menus_only_block_hud_widgets() {
    let mut snapshot = gameplay_phase_snapshot();
    snapshot.ui.menus_visible = false;

    assert_eq!(snapshot.gameplay_blocker(), None);
    assert_eq!(
        snapshot.hud_widget_blocker(),
        Some(RuntimePhaseBlocker::MenusHidden)
    );
    assert!(!snapshot.allows_hud_widgets());
}
