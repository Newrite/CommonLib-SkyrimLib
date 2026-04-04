use alloc::vec::Vec;
use core::ops::ControlFlow;

use crate::re::{TESQuest, TESQuestStage};
use crate::sdk::core::GamePtr;

use super::shared::{game_ptr, stage_snapshot_from};
use super::types::QuestStageSnapshot;

pub fn for_each_executed_stage(
    quest: &TESQuest,
    mut visit: impl FnMut(&TESQuestStage) -> ControlFlow<()>,
) -> ControlFlow<()> {
    let Some(stages) = (unsafe { quest.executed_stages.as_ref() }) else {
        return ControlFlow::Continue(());
    };

    for stage in stages.iter() {
        let flow = visit(stage);
        if flow.is_break() {
            return flow;
        }
    }

    ControlFlow::Continue(())
}

pub fn collect_executed_stage_snapshots(quest: &TESQuest) -> Vec<QuestStageSnapshot> {
    let mut stages = Vec::new();
    let _ = for_each_executed_stage(quest, |stage| {
        stages.push(stage_snapshot_from(stage));
        ControlFlow::Continue(())
    });
    stages
}

#[inline(always)]
pub fn has_executed_stage(quest: &TESQuest, stage_id: u16) -> bool {
    let mut found = false;
    let _ = for_each_executed_stage(quest, |stage| {
        if stage.data.index == stage_id {
            found = true;
            ControlFlow::Break(())
        } else {
            ControlFlow::Continue(())
        }
    });
    found
}

pub fn for_each_waiting_stage(
    quest: &TESQuest,
    mut visit: impl FnMut(&TESQuestStage) -> ControlFlow<()>,
) -> ControlFlow<()> {
    let Some(stages) = (unsafe { quest.waiting_stages.as_ref() }) else {
        return ControlFlow::Continue(());
    };

    for stage in stages.iter().copied() {
        let Some(stage) = (unsafe { stage.as_ref() }) else {
            continue;
        };

        let flow = visit(stage);
        if flow.is_break() {
            return flow;
        }
    }

    ControlFlow::Continue(())
}

pub fn collect_waiting_stages(quest: &TESQuest) -> Vec<GamePtr<TESQuestStage>> {
    let Some(stages) = (unsafe { quest.waiting_stages.as_ref() }) else {
        return Vec::new();
    };

    stages
        .iter()
        .copied()
        .filter(|stage| !stage.is_null())
        .map(game_ptr)
        .collect()
}

pub fn collect_waiting_stage_snapshots(quest: &TESQuest) -> Vec<QuestStageSnapshot> {
    let mut stages = Vec::new();
    let _ = for_each_waiting_stage(quest, |stage| {
        stages.push(stage_snapshot_from(stage));
        ControlFlow::Continue(())
    });
    stages
}

#[inline(always)]
pub fn has_waiting_stage(quest: &TESQuest, stage_id: u16) -> bool {
    let mut found = false;
    let _ = for_each_waiting_stage(quest, |stage| {
        if stage.data.index == stage_id {
            found = true;
            ControlFlow::Break(())
        } else {
            ControlFlow::Continue(())
        }
    });
    found
}
