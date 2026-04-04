use super::{
    TaskHandoff, TaskHandoffOrigin, TaskQueueKind, gameplay_event_handoff, gameplay_handoff,
    papyrus_handoff, ui_event_handoff,
};
use crate::re::INPUT_CONTEXT_ID;
use crate::sdk::core::RuntimePhaseBlocker;
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

fn gameplay_phase_snapshot() -> crate::sdk::core::RuntimePhaseSnapshot {
    crate::sdk::core::RuntimePhaseSnapshot {
        ui: gameplay_ui_snapshot(),
        pause_menu_disabled: false,
        saving_allowed: true,
        cursor_hidden_when_topmost: false,
        custom_rendering_active: false,
    }
}

#[test]
fn handoff_constructors_pick_expected_queue_origin_and_policy() {
    assert_eq!(TaskQueueKind::Background.as_str(), "background");
    assert_eq!(TaskQueueKind::Ui.as_str(), "ui");
    assert_eq!(TaskHandoffOrigin::Event.as_str(), "event");
    assert_eq!(TaskHandoffOrigin::Papyrus.as_str(), "papyrus");

    assert_eq!(
        papyrus_handoff(),
        TaskHandoff {
            queue: TaskQueueKind::Background,
            origin: TaskHandoffOrigin::Papyrus,
            require_safe_gameplay_phase: false,
        }
    );
    assert_eq!(
        ui_event_handoff(),
        TaskHandoff {
            queue: TaskQueueKind::Ui,
            origin: TaskHandoffOrigin::Event,
            require_safe_gameplay_phase: false,
        }
    );
    assert_eq!(
        gameplay_event_handoff(),
        TaskHandoff {
            queue: TaskQueueKind::Background,
            origin: TaskHandoffOrigin::Event,
            require_safe_gameplay_phase: true,
        }
    );
}

#[test]
fn gameplay_handoff_is_gated_by_phase_snapshot() {
    let safe = gameplay_phase_snapshot();
    assert!(gameplay_handoff().can_run(safe));
    assert_eq!(gameplay_handoff().phase_blocker(safe), None);

    let mut blocked = gameplay_phase_snapshot();
    blocked.ui.loading_open = true;
    assert!(!gameplay_handoff().can_run(blocked));
    assert_eq!(
        gameplay_handoff().phase_blocker(blocked),
        Some(RuntimePhaseBlocker::LoadingScreen)
    );
}

#[test]
fn plain_handoff_ignores_gameplay_phase_gate() {
    let mut blocked = gameplay_phase_snapshot();
    blocked.ui.player_input_blocked = true;

    let handoff = TaskHandoff::event();
    assert!(handoff.can_run(blocked));
    assert_eq!(handoff.phase_blocker(blocked), None);
}
