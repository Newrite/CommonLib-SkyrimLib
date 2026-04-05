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
    /// The quest this snapshot was taken from.
    pub quest: GamePtr<TESQuest>,
    /// Cached quest form ID for log/config/UI use.
    pub form_id: u32,
    /// Cached editor ID.
    pub editor_id: String,
    /// Cached display name.
    pub display_name: String,
    /// Current quest stage reported by the game.
    pub current_stage_id: u16,
    /// Classified quest type when available.
    pub quest_type: Option<QuestType>,
    /// Source-backed quest flags.
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
    /// Whether the quest is in a start/stop transition.
    #[inline(always)]
    pub const fn is_in_transition(&self) -> bool {
        self.starting || self.stopping
    }
}

/// Snapshot of one quest alias plus its currently resolved reference state.
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
    /// Whether the alias currently resolves to a reference handle.
    #[inline(always)]
    pub const fn has_reference(&self) -> bool {
        self.reference_handle.has_value()
    }

    /// Resolve the current reference handle into a live object reference.
    #[inline(always)]
    pub fn resolved_reference(&self) -> Option<Resolved<TESObjectREFR>> {
        if !self.reference_handle.has_value() {
            None
        } else {
            Resolved::from_handle(self.reference_handle)
        }
    }

    /// Return the current reference target as a nullable game pointer.
    #[inline(always)]
    pub fn reference(&self) -> GamePtr<TESObjectREFR> {
        self.resolved_reference()
            .map_or(GamePtr::null(), |resolved| resolved.as_game_ptr())
    }

    /// Return the current alias reference cast to `Actor`.
    #[inline(always)]
    pub fn actor_reference(&self) -> GamePtr<Actor> {
        self.reference().try_cast::<Actor>()
    }

    /// Whether the alias is a reference alias rather than another alias kind.
    #[inline(always)]
    pub const fn is_ref_alias(&self) -> bool {
        self.vm_type_id == BGSRefAlias::VM_TYPE_ID
    }
}

/// Snapshot of one objective inside a quest.
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
    /// Whether the objective is displayed in any displayed state.
    #[inline(always)]
    pub const fn is_displayed(&self) -> bool {
        matches!(
            self.state,
            Some(QuestObjectiveState::Displayed)
                | Some(QuestObjectiveState::CompletedDisplayed)
                | Some(QuestObjectiveState::FailedDisplayed)
        )
    }

    /// Whether the objective is completed.
    #[inline(always)]
    pub const fn is_completed(&self) -> bool {
        matches!(
            self.state,
            Some(QuestObjectiveState::Completed) | Some(QuestObjectiveState::CompletedDisplayed)
        )
    }

    /// Whether the objective is failed.
    #[inline(always)]
    pub const fn is_failed(&self) -> bool {
        matches!(
            self.state,
            Some(QuestObjectiveState::Failed) | Some(QuestObjectiveState::FailedDisplayed)
        )
    }
}

/// Snapshot of one executed or waiting quest stage.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct QuestStageSnapshot {
    pub index: u16,
    pub flags: EnumSet<QuestStageFlag, u8>,
}

impl QuestStageSnapshot {
    /// Whether the stage is flagged as the quest's startup stage.
    #[inline(always)]
    pub fn is_start_up_stage(&self) -> bool {
        self.flags.all(QuestStageFlag::StartUpStage)
    }

    /// Whether the stage is flagged as the quest's shutdown stage.
    #[inline(always)]
    pub fn is_shut_down_stage(&self) -> bool {
        self.flags.all(QuestStageFlag::ShutDownStage)
    }

    /// Whether the stage keeps quest instance data alive from that point on.
    #[inline(always)]
    pub fn keeps_instance_data(&self) -> bool {
        self.flags.all(QuestStageFlag::KeepInstanceDataFromHereOn)
    }
}

/// Snapshot of one objective target entry.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct QuestTargetSnapshot {
    pub target: GamePtr<TESQuestTarget>,
    pub alias_id: u32,
    pub flags: EnumSet<TESQuestTargetFlag, u8>,
    pub target_handle: ObjectRefHandle,
    pub tracking_handle: ObjectRefHandle,
}

impl QuestTargetSnapshot {
    /// Whether the target's compass marker ignores locks.
    #[inline(always)]
    pub fn ignores_locks(&self) -> bool {
        self.flags
            .all(TESQuestTargetFlag::CompassMarkerIgnoresLocks)
    }

    /// Whether the target currently resolves to an object reference handle.
    #[inline(always)]
    pub const fn has_target_ref(&self) -> bool {
        self.target_handle.has_value()
    }

    /// Whether the tracking handle currently resolves to an object reference.
    #[inline(always)]
    pub const fn has_tracking_ref(&self) -> bool {
        self.tracking_handle.has_value()
    }

    /// Resolve the target reference handle.
    #[inline(always)]
    pub fn resolved_target_ref(&self) -> Option<Resolved<TESObjectREFR>> {
        if !self.target_handle.has_value() {
            None
        } else {
            Resolved::from_handle(self.target_handle)
        }
    }

    /// Return the target reference as a nullable pointer.
    #[inline(always)]
    pub fn target_ref(&self) -> GamePtr<TESObjectREFR> {
        self.resolved_target_ref()
            .map_or(GamePtr::null(), |resolved| resolved.as_game_ptr())
    }

    /// Resolve the tracking reference handle.
    #[inline(always)]
    pub fn resolved_tracking_ref(&self) -> Option<Resolved<TESObjectREFR>> {
        if !self.tracking_handle.has_value() {
            None
        } else {
            Resolved::from_handle(self.tracking_handle)
        }
    }

    /// Return the tracking reference as a nullable pointer.
    #[inline(always)]
    pub fn tracking_ref(&self) -> GamePtr<TESObjectREFR> {
        self.resolved_tracking_ref()
            .map_or(GamePtr::null(), |resolved| resolved.as_game_ptr())
    }
}
