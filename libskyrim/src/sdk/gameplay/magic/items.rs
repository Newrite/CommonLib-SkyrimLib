use alloc::vec::Vec;
use core::ops::ControlFlow;

use crate::re::{BGSKeyword, EffectArchetypeId, EffectSetting, MagicItem};
use crate::sdk::core::GamePtr;

use super::shared::{
    effect_base_matches_keyword, effect_base_matches_keyword_with_editor_id, magic_item_effect_base,
};

pub fn for_each_item_effect_base<T>(
    item: &T,
    mut visit: impl FnMut(&EffectSetting) -> ControlFlow<()>,
) -> ControlFlow<()>
where
    T: AsRef<MagicItem> + ?Sized,
{
    let mut flow = ControlFlow::Continue(());
    for &effect in unsafe { item.as_ref().effects.as_slice() } {
        let base = magic_item_effect_base(effect);
        let Some(base) = (unsafe { base.as_ref() }) else {
            continue;
        };

        flow = visit(base);
        if flow.is_break() {
            break;
        }
    }
    flow
}

pub fn collect_item_effect_bases<T>(item: &T) -> Vec<GamePtr<EffectSetting>>
where
    T: AsRef<MagicItem> + ?Sized,
{
    collect_item_effect_bases_matching(item, |_| true)
}

pub fn collect_item_effect_bases_matching<T>(
    item: &T,
    mut predicate: impl FnMut(&EffectSetting) -> bool,
) -> Vec<GamePtr<EffectSetting>>
where
    T: AsRef<MagicItem> + ?Sized,
{
    let mut bases = Vec::new();
    for &effect in unsafe { item.as_ref().effects.as_slice() } {
        let base = magic_item_effect_base(effect);
        let Some(base_ref) = (unsafe { base.as_ref() }) else {
            continue;
        };

        if predicate(base_ref) {
            bases.push(unsafe { GamePtr::from_raw(base) });
        }
    }
    bases
}

pub fn has_item_effect_with_archetype<T>(item: &T, archetype: EffectArchetypeId) -> bool
where
    T: AsRef<MagicItem> + ?Sized,
{
    for_each_item_effect_base(item, |base| {
        if base.has_archetype(archetype) {
            ControlFlow::Break(())
        } else {
            ControlFlow::Continue(())
        }
    })
    .is_break()
}

pub fn has_item_effect_with_keyword<T>(item: &T, keyword: &BGSKeyword) -> bool
where
    T: AsRef<MagicItem> + ?Sized,
{
    for_each_item_effect_base(item, |base| {
        if effect_base_matches_keyword(base, keyword) {
            ControlFlow::Break(())
        } else {
            ControlFlow::Continue(())
        }
    })
    .is_break()
}

pub fn has_item_effect_with_keyword_with_editor_id<T>(item: &T, editor_id: &str) -> bool
where
    T: AsRef<MagicItem> + ?Sized,
{
    for_each_item_effect_base(item, |base| {
        if effect_base_matches_keyword_with_editor_id(base, editor_id) {
            ControlFlow::Break(())
        } else {
            ControlFlow::Continue(())
        }
    })
    .is_break()
}

pub fn collect_item_effect_bases_with_keyword<T>(
    item: &T,
    keyword: &BGSKeyword,
) -> Vec<GamePtr<EffectSetting>>
where
    T: AsRef<MagicItem> + ?Sized,
{
    collect_item_effect_bases_matching(item, |base| effect_base_matches_keyword(base, keyword))
}

pub fn collect_item_effect_bases_with_keyword_with_editor_id<T>(
    item: &T,
    editor_id: &str,
) -> Vec<GamePtr<EffectSetting>>
where
    T: AsRef<MagicItem> + ?Sized,
{
    collect_item_effect_bases_matching(item, |base| {
        effect_base_matches_keyword_with_editor_id(base, editor_id)
    })
}

pub fn collect_item_effect_bases_with_archetype<T>(
    item: &T,
    archetype: EffectArchetypeId,
) -> Vec<GamePtr<EffectSetting>>
where
    T: AsRef<MagicItem> + ?Sized,
{
    collect_item_effect_bases_matching(item, |base| base.has_archetype(archetype))
}
