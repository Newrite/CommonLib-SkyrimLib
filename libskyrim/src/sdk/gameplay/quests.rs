//! Quest and quest-stage gameplay helpers.
//!
//! This module keeps quest objects themselves source-backed while adding a
//! higher-level Rust-first layer around the common plugin workflows:
//! quest state inspection, alias/objective/stage traversal, and small safe
//! wrappers around the most common lifecycle operations.

use alloc::borrow::ToOwned;
use alloc::string::String;
use alloc::vec::Vec;
use core::ops::ControlFlow;

use core_util::EnumSet;

use crate::re::{
    Actor, BGSBaseAlias, BGSQuestObjective, BGSRefAlias, ObjectRefHandle, QuestFlag,
    QuestObjectiveFlag, QuestObjectiveState, QuestStageFlag, QuestType, TESObjectREFR, TESQuest,
    TESQuestStage, TESQuestTarget, TESQuestTargetFlag, VMTypeID,
};
use crate::sdk::core::{GamePtr, Resolved};

#[derive(Debug, Clone)]
pub struct QuestStateSnapshot {
    pub quest: GamePtr<TESQuest>,
    pub form_id: u32,
    pub editor_id: String,
    pub display_name: String,
    pub current_stage_id: u16,
    pub quest_type: Option<QuestType>,
    pub flags: EnumSet<QuestFlag, u16>,
    pub priority: i8,
    pub active: bool,
    pub completed: bool,
    pub enabled: bool,
    pub running: bool,
    pub starting: bool,
    pub stopped: bool,
    pub stopping: bool,
    pub starts_enabled: bool,
    pub already_run: bool,
}

impl QuestStateSnapshot {
    #[inline(always)]
    pub const fn is_in_transition(&self) -> bool {
        self.starting || self.stopping
    }
}

#[derive(Debug, Clone)]
pub struct QuestAliasSnapshot {
    pub alias: GamePtr<BGSBaseAlias>,
    pub owner_quest: GamePtr<TESQuest>,
    pub alias_id: u32,
    pub alias_name: String,
    pub type_name: String,
    pub vm_type_id: VMTypeID,
    pub essential: bool,
    pub protected: bool,
    pub quest_object: bool,
    pub reference_handle: ObjectRefHandle,
}

impl QuestAliasSnapshot {
    #[inline(always)]
    pub const fn has_reference(&self) -> bool {
        self.reference_handle.has_value()
    }

    #[inline(always)]
    pub fn resolved_reference(&self) -> Option<Resolved<TESObjectREFR>> {
        if !self.reference_handle.has_value() {
            None
        } else {
            Resolved::from_handle(self.reference_handle)
        }
    }

    #[inline(always)]
    pub fn reference(&self) -> GamePtr<TESObjectREFR> {
        self.resolved_reference()
            .map_or(GamePtr::null(), |resolved| resolved.as_game_ptr())
    }

    #[inline(always)]
    pub fn actor_reference(&self) -> GamePtr<Actor> {
        self.reference().try_cast::<Actor>()
    }

    #[inline(always)]
    pub const fn is_ref_alias(&self) -> bool {
        self.vm_type_id == BGSRefAlias::VM_TYPE_ID
    }
}

#[derive(Debug, Clone)]
pub struct QuestObjectiveSnapshot {
    pub objective: GamePtr<BGSQuestObjective>,
    pub owner_quest: GamePtr<TESQuest>,
    pub index: u16,
    pub display_text: String,
    pub initialized: bool,
    pub state: Option<QuestObjectiveState>,
    pub flags: EnumSet<QuestObjectiveFlag, u32>,
    pub target_count: u32,
}

impl QuestObjectiveSnapshot {
    #[inline(always)]
    pub const fn is_displayed(&self) -> bool {
        matches!(
            self.state,
            Some(QuestObjectiveState::Displayed)
                | Some(QuestObjectiveState::CompletedDisplayed)
                | Some(QuestObjectiveState::FailedDisplayed)
        )
    }

    #[inline(always)]
    pub const fn is_completed(&self) -> bool {
        matches!(
            self.state,
            Some(QuestObjectiveState::Completed) | Some(QuestObjectiveState::CompletedDisplayed)
        )
    }

    #[inline(always)]
    pub const fn is_failed(&self) -> bool {
        matches!(
            self.state,
            Some(QuestObjectiveState::Failed) | Some(QuestObjectiveState::FailedDisplayed)
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct QuestStageSnapshot {
    pub index: u16,
    pub flags: EnumSet<QuestStageFlag, u8>,
}

impl QuestStageSnapshot {
    #[inline(always)]
    pub fn is_start_up_stage(&self) -> bool {
        self.flags.all(QuestStageFlag::StartUpStage)
    }

    #[inline(always)]
    pub fn is_shut_down_stage(&self) -> bool {
        self.flags.all(QuestStageFlag::ShutDownStage)
    }

    #[inline(always)]
    pub fn keeps_instance_data(&self) -> bool {
        self.flags.all(QuestStageFlag::KeepInstanceDataFromHereOn)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct QuestTargetSnapshot {
    pub target: GamePtr<TESQuestTarget>,
    pub alias_id: u32,
    pub flags: EnumSet<TESQuestTargetFlag, u8>,
    pub target_handle: ObjectRefHandle,
    pub tracking_handle: ObjectRefHandle,
}

impl QuestTargetSnapshot {
    #[inline(always)]
    pub fn ignores_locks(&self) -> bool {
        self.flags
            .all(TESQuestTargetFlag::CompassMarkerIgnoresLocks)
    }

    #[inline(always)]
    pub const fn has_target_ref(&self) -> bool {
        self.target_handle.has_value()
    }

    #[inline(always)]
    pub const fn has_tracking_ref(&self) -> bool {
        self.tracking_handle.has_value()
    }

    #[inline(always)]
    pub fn resolved_target_ref(&self) -> Option<Resolved<TESObjectREFR>> {
        if !self.target_handle.has_value() {
            None
        } else {
            Resolved::from_handle(self.target_handle)
        }
    }

    #[inline(always)]
    pub fn target_ref(&self) -> GamePtr<TESObjectREFR> {
        self.resolved_target_ref()
            .map_or(GamePtr::null(), |resolved| resolved.as_game_ptr())
    }

    #[inline(always)]
    pub fn resolved_tracking_ref(&self) -> Option<Resolved<TESObjectREFR>> {
        if !self.tracking_handle.has_value() {
            None
        } else {
            Resolved::from_handle(self.tracking_handle)
        }
    }

    #[inline(always)]
    pub fn tracking_ref(&self) -> GamePtr<TESObjectREFR> {
        self.resolved_tracking_ref()
            .map_or(GamePtr::null(), |resolved| resolved.as_game_ptr())
    }
}

#[inline(always)]
fn game_ptr<T>(raw: *mut T) -> GamePtr<T> {
    unsafe { GamePtr::from_raw(raw) }
}

#[inline(always)]
fn handle_to_ptr(handle: ObjectRefHandle) -> GamePtr<TESObjectREFR> {
    if !handle.has_value() {
        return GamePtr::null();
    }

    game_ptr(handle.get().get())
}

#[inline(always)]
fn stage_snapshot_from(stage: &TESQuestStage) -> QuestStageSnapshot {
    QuestStageSnapshot {
        index: stage.data.index,
        flags: stage.data.flags,
    }
}

#[inline(always)]
fn objective_snapshot_from(objective: &BGSQuestObjective) -> QuestObjectiveSnapshot {
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
fn alias_snapshot_from(alias: &BGSBaseAlias) -> QuestAliasSnapshot {
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
fn target_snapshot_from(
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

#[inline(always)]
pub fn quest_editor_id(quest: &TESQuest) -> &str {
    quest.get_form_editor_id_as_str()
}

#[inline(always)]
pub fn quest_display_name(quest: &TESQuest) -> &str {
    quest.full_name.get_name_as_str()
}

#[inline(always)]
pub const fn current_stage_id(quest: &TESQuest) -> u16 {
    quest.get_current_stage_id()
}

pub fn snapshot_quest_state(quest: &TESQuest) -> QuestStateSnapshot {
    QuestStateSnapshot {
        quest: game_ptr(quest as *const TESQuest as *mut TESQuest),
        form_id: quest.get_form_id(),
        editor_id: quest_editor_id(quest).to_owned(),
        display_name: quest_display_name(quest).to_owned(),
        current_stage_id: quest.get_current_stage_id(),
        quest_type: quest.get_type(),
        flags: quest.data.flags,
        priority: quest.data.priority,
        active: quest.is_active(),
        completed: quest.is_completed(),
        enabled: quest.is_enabled(),
        running: quest.is_running(),
        starting: quest.is_starting(),
        stopped: quest.is_stopped(),
        stopping: quest.is_stopping(),
        starts_enabled: quest.starts_enabled(),
        already_run: quest.already_run,
    }
}

pub fn for_each_alias(
    quest: &TESQuest,
    mut visit: impl FnMut(&BGSBaseAlias) -> ControlFlow<()>,
) -> ControlFlow<()> {
    for alias in unsafe { quest.aliases.as_slice() }.iter().copied() {
        let Some(alias) = (unsafe { alias.as_ref() }) else {
            continue;
        };

        let flow = visit(alias);
        if flow.is_break() {
            return flow;
        }
    }

    ControlFlow::Continue(())
}

pub fn for_each_ref_alias(
    quest: &TESQuest,
    mut visit: impl FnMut(&BGSRefAlias) -> ControlFlow<()>,
) -> ControlFlow<()> {
    for_each_alias(quest, |alias| {
        let ref_alias =
            game_ptr(alias as *const BGSBaseAlias as *mut BGSBaseAlias).try_cast::<BGSRefAlias>();
        let Some(ref_alias) = ref_alias.into_option() else {
            return ControlFlow::Continue(());
        };
        visit(ref_alias.as_ref())
    })
}

pub fn collect_aliases(quest: &TESQuest) -> Vec<GamePtr<BGSBaseAlias>> {
    let mut aliases = Vec::new();
    let _ = for_each_alias(quest, |alias| {
        aliases.push(game_ptr(alias as *const BGSBaseAlias as *mut BGSBaseAlias));
        ControlFlow::Continue(())
    });
    aliases
}

pub fn collect_ref_aliases(quest: &TESQuest) -> Vec<GamePtr<BGSRefAlias>> {
    let mut aliases = Vec::new();
    let _ = for_each_ref_alias(quest, |alias| {
        aliases.push(game_ptr(alias as *const BGSRefAlias as *mut BGSRefAlias));
        ControlFlow::Continue(())
    });
    aliases
}

pub fn find_alias_matching(
    quest: &TESQuest,
    mut predicate: impl FnMut(&BGSBaseAlias) -> bool,
) -> GamePtr<BGSBaseAlias> {
    let mut found = GamePtr::null();
    let _ = for_each_alias(quest, |alias| {
        if predicate(alias) {
            found = game_ptr(alias as *const BGSBaseAlias as *mut BGSBaseAlias);
            ControlFlow::Break(())
        } else {
            ControlFlow::Continue(())
        }
    });
    found
}

#[inline(always)]
pub fn find_alias_by_id(quest: &TESQuest, alias_id: u32) -> GamePtr<BGSBaseAlias> {
    find_alias_matching(quest, |alias| alias.alias_id == alias_id)
}

#[inline(always)]
pub fn find_alias_by_name(quest: &TESQuest, alias_name: &str) -> GamePtr<BGSBaseAlias> {
    find_alias_matching(quest, |alias| alias.alias_name.as_str() == alias_name)
}

pub fn find_ref_alias_matching(
    quest: &TESQuest,
    mut predicate: impl FnMut(&BGSRefAlias) -> bool,
) -> GamePtr<BGSRefAlias> {
    let mut found = GamePtr::null();
    let _ = for_each_ref_alias(quest, |alias| {
        if predicate(alias) {
            found = game_ptr(alias as *const BGSRefAlias as *mut BGSRefAlias);
            ControlFlow::Break(())
        } else {
            ControlFlow::Continue(())
        }
    });
    found
}

#[inline(always)]
pub fn find_ref_alias_by_id(quest: &TESQuest, alias_id: u32) -> GamePtr<BGSRefAlias> {
    find_ref_alias_matching(quest, |alias| alias.base.alias_id == alias_id)
}

pub fn snapshot_aliases(quest: &TESQuest) -> Vec<QuestAliasSnapshot> {
    let mut aliases = Vec::new();
    let _ = for_each_alias(quest, |alias| {
        aliases.push(alias_snapshot_from(alias));
        ControlFlow::Continue(())
    });
    aliases
}

#[inline(always)]
pub fn aliased_ref_handle(quest: &TESQuest, alias_id: u32) -> ObjectRefHandle {
    quest.get_aliased_ref(alias_id)
}

#[inline(always)]
pub fn aliased_ref(quest: &TESQuest, alias_id: u32) -> GamePtr<TESObjectREFR> {
    handle_to_ptr(aliased_ref_handle(quest, alias_id))
}

#[inline(always)]
pub fn aliased_actor(quest: &TESQuest, alias_id: u32) -> GamePtr<Actor> {
    aliased_ref(quest, alias_id).try_cast::<Actor>()
}

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

pub fn snapshot_objectives(quest: &TESQuest) -> Vec<QuestObjectiveSnapshot> {
    let mut objectives = Vec::new();
    let _ = for_each_objective(quest, |objective| {
        objectives.push(objective_snapshot_from(objective));
        ControlFlow::Continue(())
    });
    objectives
}

pub fn snapshot_objective_targets_with(
    objective: &BGSQuestObjective,
    quest: &TESQuest,
    allow_pick_up_actor: bool,
) -> Vec<QuestTargetSnapshot> {
    if objective.num_targets == 0 {
        return Vec::new();
    }

    if objective.targets.is_null() {
        crate::defensive_sdk_warn!(
            "sdk::gameplay::quests::snapshot_objective_targets_with() observed null target array with num_targets={}",
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

pub fn snapshot_objective_targets(objective: &BGSQuestObjective) -> Vec<QuestTargetSnapshot> {
    let Some(owner_quest) = (unsafe { objective.owner_quest.as_ref() }) else {
        crate::defensive_sdk_warn!(
            "sdk::gameplay::quests::snapshot_objective_targets() received objective {} without an owning quest",
            objective.index
        );
        return Vec::new();
    };

    snapshot_objective_targets_with(objective, owner_quest, false)
}

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

#[inline(always)]
pub fn set_quest_enabled(quest: &mut TESQuest, enabled: bool) {
    quest.set_enabled(enabled);
}

#[inline(always)]
pub fn start_quest(quest: &mut TESQuest) -> bool {
    quest.start()
}

#[inline(always)]
pub fn ensure_quest_started(quest: &mut TESQuest, start_now: bool) -> bool {
    let mut result = false;
    quest.ensure_quest_started(&mut result, start_now)
}

#[inline(always)]
pub fn stop_quest(quest: &mut TESQuest) {
    quest.stop();
}

#[inline(always)]
pub fn reset_quest(quest: &mut TESQuest) {
    quest.reset();
}

#[inline(always)]
pub fn reset_and_update_quest(quest: &mut TESQuest) {
    quest.reset_and_update();
}

pub fn force_alias_reference(
    quest: &mut TESQuest,
    alias_id: u32,
    reference: GamePtr<TESObjectREFR>,
) -> bool {
    let Some(reference) = reference.into_option() else {
        crate::defensive_sdk_warn!(
            "sdk::gameplay::quests::force_alias_reference() ignored null reference for alias_id={}",
            alias_id
        );
        return false;
    };

    quest.force_ref_into_alias(alias_id, reference.as_ptr());
    true
}

pub fn force_ref_alias_reference(
    alias: &mut BGSRefAlias,
    reference: GamePtr<TESObjectREFR>,
) -> bool {
    let Some(reference) = reference.into_option() else {
        crate::defensive_sdk_warn!(
            "sdk::gameplay::quests::force_ref_alias_reference() ignored null reference for alias_id={}",
            alias.base.alias_id
        );
        return false;
    };

    alias.force_ref_to(reference.as_ptr());
    true
}

#[cfg(test)]
mod tests {
    use super::{QuestObjectiveSnapshot, QuestStageSnapshot, QuestStateSnapshot};
    use crate::re::{QuestObjectiveState, QuestStageFlag};
    use crate::sdk::core::GamePtr;
    use core_util::EnumSet;

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
            (QuestStageFlag::StartUpStage as u8)
                | (QuestStageFlag::KeepInstanceDataFromHereOn as u8),
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
}
