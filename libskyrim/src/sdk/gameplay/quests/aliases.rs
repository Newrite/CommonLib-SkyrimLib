use alloc::vec::Vec;
use core::ops::ControlFlow;

use crate::re::{BGSBaseAlias, BGSRefAlias, TESQuest};
use crate::sdk::core::GamePtr;

use super::shared::{alias_snapshot_from, game_ptr};
use super::types::QuestAliasSnapshot;

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

pub fn collect_alias_snapshots(quest: &TESQuest) -> Vec<QuestAliasSnapshot> {
    let mut aliases = Vec::new();
    let _ = for_each_alias(quest, |alias| {
        aliases.push(alias_snapshot_from(alias));
        ControlFlow::Continue(())
    });
    aliases
}
