use alloc::borrow::ToOwned;

use crate::re::{ObjectRefHandle, TESObjectREFR, TESQuest};
use crate::sdk::core::GamePtr;

use super::shared::{game_ptr, handle_to_ptr};
use super::types::QuestStateSnapshot;

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

pub fn quest_state_snapshot(quest: &TESQuest) -> QuestStateSnapshot {
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

#[inline(always)]
pub fn aliased_reference_handle(quest: &TESQuest, alias_id: u32) -> ObjectRefHandle {
    quest.get_aliased_ref(alias_id)
}

#[inline(always)]
pub fn aliased_reference(quest: &TESQuest, alias_id: u32) -> GamePtr<TESObjectREFR> {
    handle_to_ptr(aliased_reference_handle(quest, alias_id))
}

#[inline(always)]
pub fn aliased_actor(quest: &TESQuest, alias_id: u32) -> GamePtr<crate::re::Actor> {
    aliased_reference(quest, alias_id).try_cast::<crate::re::Actor>()
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
    alias: &mut crate::re::BGSRefAlias,
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
