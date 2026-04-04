use crate::re::{QuestObjectiveState, QuestStageFlag};
use crate::sdk::core::GamePtr;
use core_util::EnumSet;

use super::{QuestObjectiveSnapshot, QuestStageSnapshot, QuestStateSnapshot};

#[test]
fn objective_snapshot_state_helpers_cover_display_and_terminal_states() {
    let displayed = QuestObjectiveSnapshot {
        objective: GamePtr::null(),
        owner_quest: GamePtr::null(),
        index: 10,
        display_text: "Recover the Horn".into(),
        initialized: true,
        state: Some(QuestObjectiveState::CompletedDisplayed),
        flags: EnumSet::default(),
        target_count: 1,
    };
    let failed = QuestObjectiveSnapshot {
        state: Some(QuestObjectiveState::Failed),
        ..displayed.clone()
    };

    assert!(displayed.is_displayed());
    assert!(displayed.is_completed());
    assert!(!displayed.is_failed());
    assert!(failed.is_failed());
    assert!(!failed.is_completed());
}

#[test]
fn stage_snapshot_helpers_report_role_flags() {
    let flags = EnumSet::from_underlying(
        (QuestStageFlag::StartUpStage as u8) | (QuestStageFlag::KeepInstanceDataFromHereOn as u8),
    );
    let stage = QuestStageSnapshot { index: 20, flags };

    assert!(stage.is_start_up_stage());
    assert!(!stage.is_shut_down_stage());
    assert!(stage.keeps_instance_data());
}

#[test]
fn quest_state_snapshot_transition_helper_tracks_start_stop() {
    let snapshot = QuestStateSnapshot {
        quest: GamePtr::null(),
        form_id: 0,
        editor_id: "MQ101".into(),
        display_name: "Unbound".into(),
        current_stage_id: 5,
        quest_type: None,
        flags: EnumSet::default(),
        priority: 50,
        active: false,
        completed: false,
        enabled: true,
        running: false,
        starting: true,
        stopped: false,
        stopping: false,
        starts_enabled: true,
        already_run: false,
    };

    assert!(snapshot.is_in_transition());
}
