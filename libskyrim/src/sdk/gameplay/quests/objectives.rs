use alloc::vec::Vec;
use core::ops::ControlFlow;

use crate::re::{BGSQuestObjective, QuestObjectiveState, TESQuest};
use crate::sdk::core::GamePtr;

use super::shared::{game_ptr, objective_snapshot_from, target_snapshot_from};
use super::types::{QuestObjectiveSnapshot, QuestTargetSnapshot};

pub fn for_each_objective(
    quest: &TESQuest,
    mut visit: impl FnMut(&BGSQuestObjective) -> ControlFlow<()>,
) -> ControlFlow<()> {
    for objective in quest.objectives.iter().copied() {
        let Some(objective) = (unsafe { objective.as_ref() }) else {
            continue;
        };

        let flow = visit(objective);
        if flow.is_break() {
            return flow;
        }
    }

    ControlFlow::Continue(())
}

pub fn collect_objectives(quest: &TESQuest) -> Vec<GamePtr<BGSQuestObjective>> {
    let mut objectives = Vec::new();
    let _ = for_each_objective(quest, |objective| {
        objectives.push(game_ptr(
            objective as *const BGSQuestObjective as *mut BGSQuestObjective,
        ));
        ControlFlow::Continue(())
    });
    objectives
}

pub fn find_objective_matching(
    quest: &TESQuest,
    mut predicate: impl FnMut(&BGSQuestObjective) -> bool,
) -> GamePtr<BGSQuestObjective> {
    let mut found = GamePtr::null();
    let _ = for_each_objective(quest, |objective| {
        if predicate(objective) {
            found = game_ptr(objective as *const BGSQuestObjective as *mut BGSQuestObjective);
            ControlFlow::Break(())
        } else {
            ControlFlow::Continue(())
        }
    });
    found
}

#[inline(always)]
pub fn find_objective_by_index(quest: &TESQuest, index: u16) -> GamePtr<BGSQuestObjective> {
    find_objective_matching(quest, |objective| objective.index == index)
}

pub fn collect_objectives_with_state(
    quest: &TESQuest,
    state: QuestObjectiveState,
) -> Vec<GamePtr<BGSQuestObjective>> {
    let mut objectives = Vec::new();
    let _ = for_each_objective(quest, |objective| {
        if objective.state.get() == Some(state) {
            objectives.push(game_ptr(
                objective as *const BGSQuestObjective as *mut BGSQuestObjective,
            ));
        }
        ControlFlow::Continue(())
    });
    objectives
}

#[inline(always)]
pub fn has_objective_state(quest: &TESQuest, state: QuestObjectiveState) -> bool {
    find_objective_matching(quest, |objective| objective.state.get() == Some(state)).is_some()
}

pub fn collect_objective_snapshots(quest: &TESQuest) -> Vec<QuestObjectiveSnapshot> {
    let mut objectives = Vec::new();
    let _ = for_each_objective(quest, |objective| {
        objectives.push(objective_snapshot_from(objective));
        ControlFlow::Continue(())
    });
    objectives
}

pub fn collect_objective_target_snapshots_with(
    objective: &BGSQuestObjective,
    quest: &TESQuest,
    allow_pick_up_actor: bool,
) -> Vec<QuestTargetSnapshot> {
    if objective.num_targets == 0 {
        return Vec::new();
    }

    if objective.targets.is_null() {
        crate::defensive_sdk_warn!(
            "sdk::gameplay::quests::collect_objective_target_snapshots_with() observed null target array with num_targets={}",
            objective.num_targets
        );
        return Vec::new();
    }

    let targets =
        unsafe { core::slice::from_raw_parts(objective.targets, objective.num_targets as usize) };
    let mut snapshots = Vec::with_capacity(targets.len());
    for target in targets.iter().copied() {
        let Some(target) = (unsafe { target.as_ref() }) else {
            continue;
        };
        snapshots.push(target_snapshot_from(target, quest, allow_pick_up_actor));
    }
    snapshots
}

pub fn collect_objective_target_snapshots(
    objective: &BGSQuestObjective,
) -> Vec<QuestTargetSnapshot> {
    let Some(owner_quest) = (unsafe { objective.owner_quest.as_ref() }) else {
        crate::defensive_sdk_warn!(
            "sdk::gameplay::quests::collect_objective_target_snapshots() received objective {} without an owning quest",
            objective.index
        );
        return Vec::new();
    };

    collect_objective_target_snapshots_with(objective, owner_quest, false)
}
