use alloc::string::String;

use core_util::EnumSet;

use crate::re::{
    Actor, BGSBaseAlias, BGSQuestObjective, BGSRefAlias, ObjectRefHandle, QuestFlag,
    QuestObjectiveFlag, QuestObjectiveState, QuestStageFlag, QuestType, TESObjectREFR, TESQuest,
    TESQuestTarget, TESQuestTargetFlag, VMTypeID,
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
