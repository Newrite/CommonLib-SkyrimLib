use alloc::borrow::ToOwned;

use crate::re::{
    BGSBaseAlias, BGSQuestObjective, ObjectRefHandle, TESObjectREFR, TESQuest, TESQuestStage,
    TESQuestTarget,
};
use crate::sdk::core::GamePtr;

use super::types::{
    QuestAliasSnapshot, QuestObjectiveSnapshot, QuestStageSnapshot, QuestTargetSnapshot,
};

#[inline(always)]
pub(crate) fn game_ptr<T>(raw: *mut T) -> GamePtr<T> {
    unsafe { GamePtr::from_raw(raw) }
}

#[inline(always)]
pub(crate) fn handle_to_ptr(handle: ObjectRefHandle) -> GamePtr<TESObjectREFR> {
    if !handle.has_value() {
        return GamePtr::null();
    }

    game_ptr(handle.get().get())
}

#[inline(always)]
pub(crate) fn stage_snapshot_from(stage: &TESQuestStage) -> QuestStageSnapshot {
    QuestStageSnapshot {
        index: stage.data.index,
        flags: stage.data.flags,
    }
}

#[inline(always)]
pub(crate) fn objective_snapshot_from(objective: &BGSQuestObjective) -> QuestObjectiveSnapshot {
    QuestObjectiveSnapshot {
        objective: game_ptr(objective as *const BGSQuestObjective as *mut BGSQuestObjective),
        owner_quest: game_ptr(objective.owner_quest),
        index: objective.index,
        display_text: objective.display_text.as_str().to_owned(),
        initialized: objective.initialized,
        state: objective.state.get(),
        flags: objective.flags,
        target_count: objective.num_targets,
    }
}

#[inline(always)]
pub(crate) fn alias_snapshot_from(alias: &BGSBaseAlias) -> QuestAliasSnapshot {
    let owner_quest = game_ptr(alias.owning_quest);
    let reference_handle = owner_quest.map_or(ObjectRefHandle::new(), |quest| {
        quest.get_aliased_ref(alias.alias_id)
    });

    QuestAliasSnapshot {
        alias: game_ptr(alias as *const BGSBaseAlias as *mut BGSBaseAlias),
        owner_quest,
        alias_id: alias.alias_id,
        alias_name: alias.alias_name.as_str().to_owned(),
        type_name: alias.q_type().as_str().to_owned(),
        vm_type_id: alias.get_vm_type_id(),
        essential: alias.is_essential(),
        protected: alias.is_protected(),
        quest_object: alias.is_quest_object(),
        reference_handle,
    }
}

#[inline(always)]
pub(crate) fn target_snapshot_from(
    target: &TESQuestTarget,
    quest: &TESQuest,
    allow_pick_up_actor: bool,
) -> QuestTargetSnapshot {
    let mut target_handle = ObjectRefHandle::new();
    let mut tracking_handle = ObjectRefHandle::new();
    target.get_target_ref(
        &mut target_handle,
        allow_pick_up_actor,
        quest as *const TESQuest,
    );
    target.get_tracking_ref(&mut tracking_handle, quest as *const TESQuest);

    QuestTargetSnapshot {
        target: game_ptr(target as *const TESQuestTarget as *mut TESQuestTarget),
        alias_id: target.alias,
        flags: target.flags,
        target_handle,
        tracking_handle,
    }
}
