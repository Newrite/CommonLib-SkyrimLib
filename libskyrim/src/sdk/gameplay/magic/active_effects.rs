use alloc::vec::Vec;
use core::ops::ControlFlow;

use crate::re::{
    ActiveEffect, BGSKeyword, BSContainerForEachResult, EffectArchetypeId, EffectSetting,
    MagicTarget,
};
use crate::sdk::core::GamePtr;

use super::shared::{
    active_effect_matches_archetype, active_effect_matches_keyword,
    active_effect_matches_keyword_with_editor_id,
};
use super::types::ActiveEffectSnapshot;

#[inline(always)]
fn flow_to_engine_result(flow: ControlFlow<()>) -> BSContainerForEachResult {
    match flow {
        ControlFlow::Continue(()) => BSContainerForEachResult::Continue,
        ControlFlow::Break(()) => BSContainerForEachResult::Stop,
    }
}

/// Iterates every loaded active effect on a magic target until the visitor
/// breaks.
pub fn for_each_active_effect<T>(
    target: &T,
    mut visit: impl FnMut(&ActiveEffect) -> ControlFlow<()>,
) -> ControlFlow<()>
where
    T: AsRef<MagicTarget> + ?Sized,
{
    let mut flow = ControlFlow::Continue(());
    target.as_ref().visit_active_effects(|effect| {
        let Some(effect) = (unsafe { effect.as_ref() }) else {
            return BSContainerForEachResult::Continue;
        };

        flow = visit(effect);
        flow_to_engine_result(flow)
    });
    flow
}

/// Collects all loaded active effects from a magic target.
pub fn collect_active_effects<T>(target: &T) -> Vec<GamePtr<ActiveEffect>>
where
    T: AsRef<MagicTarget> + ?Sized,
{
    collect_active_effects_matching(target, |_| true)
}

/// Collects all loaded active effects matching the predicate.
pub fn collect_active_effects_matching<T>(
    target: &T,
    mut predicate: impl FnMut(&ActiveEffect) -> bool,
) -> Vec<GamePtr<ActiveEffect>>
where
    T: AsRef<MagicTarget> + ?Sized,
{
    let mut effects = Vec::new();
    target.as_ref().visit_active_effects(|effect| {
        let Some(effect_ref) = (unsafe { effect.as_ref() }) else {
            return BSContainerForEachResult::Continue;
        };

        if predicate(effect_ref) {
            effects.push(unsafe { GamePtr::from_raw(effect) });
        }

        BSContainerForEachResult::Continue
    });
    effects
}

/// Dispels all active effects matching the predicate and returns the count.
pub fn dispel_active_effects_matching<T>(
    target: &T,
    mut predicate: impl FnMut(&ActiveEffect) -> bool,
    force: bool,
) -> usize
where
    T: AsRef<MagicTarget> + ?Sized,
{
    let queued = collect_active_effects_matching(target, |effect| predicate(effect));
    let mut dispelled = 0usize;
    for effect in queued {
        if unsafe { effect.with_mut_unchecked(|effect| effect.dispel(force)) }.is_some() {
            dispelled += 1;
        }
    }
    dispelled
}

/// Captures read-mostly snapshots for every loaded active effect.
pub fn snapshot_active_effects<T>(target: &T) -> Vec<ActiveEffectSnapshot>
where
    T: AsRef<MagicTarget> + ?Sized,
{
    collect_active_effects_matching(target, |_| true)
        .into_iter()
        .filter_map(|effect| {
            effect.as_ref().map(|effect_ref| ActiveEffectSnapshot {
                effect,
                spell: unsafe { GamePtr::from_raw(effect_ref.spell) },
                base: unsafe { GamePtr::from_raw(effect_ref.get_base_object()) },
                magnitude: effect_ref.get_magnitude(),
                duration: effect_ref.duration,
                elapsed_seconds: effect_ref.elapsed_seconds,
                casting_source: effect_ref.casting_source,
            })
        })
        .collect()
}

/// Returns `true` when the target currently has the given effect setting.
#[inline(always)]
pub fn has_active_effect<T>(target: &T, effect: &EffectSetting) -> bool
where
    T: AsRef<MagicTarget> + ?Sized,
{
    target
        .as_ref()
        .has_magic_effect(effect as *const EffectSetting as *mut EffectSetting)
}

/// Returns `true` when the target currently has an active effect with the given keyword.
#[inline(always)]
pub fn has_active_effect_with_keyword<T>(target: &T, keyword: &BGSKeyword) -> bool
where
    T: AsRef<MagicTarget> + ?Sized,
{
    target.as_ref().has_magic_effect_with_keyword(
        keyword as *const BGSKeyword as *mut BGSKeyword,
        core::ptr::null_mut(),
    )
}

/// Returns `true` when the target currently has an active effect with the given
/// keyword editor ID.
pub fn has_active_effect_with_keyword_with_editor_id<T>(target: &T, editor_id: &str) -> bool
where
    T: AsRef<MagicTarget> + ?Sized,
{
    for_each_active_effect(target, |effect| {
        if active_effect_matches_keyword_with_editor_id(effect, editor_id) {
            ControlFlow::Break(())
        } else {
            ControlFlow::Continue(())
        }
    })
    .is_break()
}

/// Collects active effects matching a keyword editor ID.
pub fn collect_active_effects_with_keyword_with_editor_id<T>(
    target: &T,
    editor_id: &str,
) -> Vec<GamePtr<ActiveEffect>>
where
    T: AsRef<MagicTarget> + ?Sized,
{
    collect_active_effects_matching(target, |effect| {
        active_effect_matches_keyword_with_editor_id(effect, editor_id)
    })
}

/// Collects active effects matching a keyword.
pub fn collect_active_effects_with_keyword<T>(
    target: &T,
    keyword: &BGSKeyword,
) -> Vec<GamePtr<ActiveEffect>>
where
    T: AsRef<MagicTarget> + ?Sized,
{
    collect_active_effects_matching(target, |effect| {
        active_effect_matches_keyword(effect, keyword)
    })
}

/// Collects active effects matching an archetype.
pub fn collect_active_effects_with_archetype<T>(
    target: &T,
    archetype: EffectArchetypeId,
) -> Vec<GamePtr<ActiveEffect>>
where
    T: AsRef<MagicTarget> + ?Sized,
{
    collect_active_effects_matching(target, |effect| {
        active_effect_matches_archetype(effect, archetype)
    })
}

/// Dispels active effects matching a keyword and returns the count.
pub fn dispel_active_effects_with_keyword<T>(target: &T, keyword: &BGSKeyword, force: bool) -> usize
where
    T: AsRef<MagicTarget> + ?Sized,
{
    dispel_active_effects_matching(
        target,
        |effect| active_effect_matches_keyword(effect, keyword),
        force,
    )
}

/// Dispels active effects matching a keyword editor ID and returns the count.
pub fn dispel_active_effects_with_keyword_with_editor_id<T>(
    target: &T,
    editor_id: &str,
    force: bool,
) -> usize
where
    T: AsRef<MagicTarget> + ?Sized,
{
    dispel_active_effects_matching(
        target,
        |effect| active_effect_matches_keyword_with_editor_id(effect, editor_id),
        force,
    )
}

/// Dispels active effects matching an archetype and returns the count.
pub fn dispel_active_effects_with_archetype<T>(
    target: &T,
    archetype: EffectArchetypeId,
    force: bool,
) -> usize
where
    T: AsRef<MagicTarget> + ?Sized,
{
    dispel_active_effects_matching(
        target,
        |effect| active_effect_matches_archetype(effect, archetype),
        force,
    )
}
