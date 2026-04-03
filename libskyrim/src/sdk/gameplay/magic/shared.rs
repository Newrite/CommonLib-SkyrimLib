use crate::re::{
    ActiveEffect, Actor, BGSKeyword, Effect, EffectArchetypeId, EffectSetting, SpellItem,
    magic_system::CastingSource,
};

#[inline(always)]
pub(super) fn effect_base_matches_keyword(
    base: &EffectSetting,
    keyword: *const BGSKeyword,
) -> bool {
    !keyword.is_null() && base.has_keyword(keyword)
}

#[inline(always)]
pub(super) fn effect_base_matches_keyword_editor_id(base: &EffectSetting, editor_id: &str) -> bool {
    !editor_id.is_empty() && base.has_keyword_string(editor_id)
}

#[inline(always)]
pub(super) fn magic_item_effect_base(effect: *mut Effect) -> *mut EffectSetting {
    unsafe { effect.as_ref() }
        .map(|effect| effect.base_effect)
        .unwrap_or(core::ptr::null_mut())
}

#[inline(always)]
pub(super) fn active_effect_base(effect: &ActiveEffect) -> Option<&EffectSetting> {
    unsafe { effect.get_base_object().as_ref() }
}

#[inline(always)]
pub(super) fn active_effect_matches_keyword(effect: &ActiveEffect, keyword: &BGSKeyword) -> bool {
    active_effect_base(effect).is_some_and(|base| effect_base_matches_keyword(base, keyword))
}

#[inline(always)]
pub(super) fn active_effect_matches_keyword_editor_id(
    effect: &ActiveEffect,
    editor_id: &str,
) -> bool {
    active_effect_base(effect)
        .is_some_and(|base| effect_base_matches_keyword_editor_id(base, editor_id))
}

#[inline(always)]
pub(super) fn active_effect_matches_archetype(
    effect: &ActiveEffect,
    archetype: EffectArchetypeId,
) -> bool {
    active_effect_base(effect).is_some_and(|base| base.has_archetype(archetype))
}

#[inline(always)]
pub(super) fn casting_source_slot(source: CastingSource) -> Option<usize> {
    match source {
        CastingSource::LeftHand => Some(0),
        CastingSource::RightHand => Some(1),
        CastingSource::Other => Some(2),
        CastingSource::Instant => Some(3),
        CastingSource::None => None,
    }
}

#[inline(always)]
pub(super) fn invalid_casting_source(_caller: &'static str, source: CastingSource) -> bool {
    if casting_source_slot(source).is_none() {
        crate::defensive_sdk_warn!(
            "sdk::gameplay::magic ignored unsupported casting source {:?}",
            source
        );
        true
    } else {
        false
    }
}

pub(super) fn with_magic_caster_mut<R>(
    actor: &mut Actor,
    source: CastingSource,
    caller: &'static str,
    f: impl FnOnce(&mut crate::re::MagicCaster) -> R,
) -> Option<R> {
    if invalid_casting_source(caller, source) {
        return None;
    }

    let caster = unsafe { actor.base.get_magic_caster(source).as_mut() };
    let Some(caster) = caster else {
        crate::defensive_sdk_warn!(
            "{} skipped because get_magic_caster({:?}) returned null",
            caller,
            source
        );
        return None;
    };

    Some(f(caster))
}

#[inline(always)]
pub(super) fn actor_spell_has_keyword(spell: &SpellItem, keyword: &BGSKeyword) -> bool {
    spell.base.keyword_form.has_keyword(keyword)
}

#[inline(always)]
pub(super) fn actor_spell_has_keyword_editor_id(spell: &SpellItem, editor_id: &str) -> bool {
    spell.base.keyword_form.has_keyword_string(editor_id)
}
