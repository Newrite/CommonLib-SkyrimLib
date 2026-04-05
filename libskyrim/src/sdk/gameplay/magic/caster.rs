use crate::re::{
    Actor, MagicItem, TESObjectREFR,
    magic_system::{CannotCastReason, CastingSource},
};
use crate::sdk::core::GamePtr;

use super::shared::{invalid_casting_source, with_magic_caster_mut};
use super::spells::selected_actor_spell;
use super::types::{CastCheck, CastingSourceSnapshot, ImmediateCastOptions};

/// Returns the actor's `MagicCaster` for one casting source.
///
/// This is the lowest-level caster-side entrypoint in this module. Most plugin
/// code will prefer the higher-level snapshot/check/cast helpers unless it
/// truly needs the raw caster object.
#[inline(always)]
pub fn magic_caster_for_source(
    actor: &mut Actor,
    source: CastingSource,
) -> GamePtr<crate::re::MagicCaster> {
    with_magic_caster_mut(
        actor,
        source,
        "sdk::gameplay::magic::magic_caster_for_source()",
        |caster| unsafe { GamePtr::from_raw(caster as *mut crate::re::MagicCaster) },
    )
    .unwrap_or(GamePtr::null())
}

/// Returns the currently active spell being cast from one source.
#[inline(always)]
pub fn current_spell_for_source(actor: &mut Actor, source: CastingSource) -> GamePtr<MagicItem> {
    magic_caster_for_source(actor, source)
        .as_ref()
        .map_or(GamePtr::null(), |caster| unsafe {
            GamePtr::from_raw(caster.current_spell)
        })
}

/// Returns `true` when the source is currently casting a spell.
#[inline(always)]
pub fn is_casting_for_source(actor: &mut Actor, source: CastingSource) -> bool {
    current_spell_for_source(actor, source).is_some()
}

/// Captures a snapshot of one actor casting source.
pub fn snapshot_casting_source(actor: &mut Actor, source: CastingSource) -> CastingSourceSnapshot {
    let selected_spell = selected_actor_spell(actor, source);
    let caster = magic_caster_for_source(actor, source);
    let current_spell = caster.as_ref().map_or(GamePtr::null(), |caster| unsafe {
        GamePtr::from_raw(caster.current_spell)
    });
    let current_spell_cost = caster
        .as_ref()
        .map_or(0.0, |caster| caster.current_spell_cost);
    let dual_casting = caster
        .as_ref()
        .is_some_and(crate::re::MagicCaster::get_is_dual_casting);

    CastingSourceSnapshot {
        source,
        selected_spell,
        current_spell,
        current_spell_cost,
        dual_casting,
        has_magic_caster: caster.is_some(),
    }
}

/// Checks whether a spell can be cast from the selected source.
///
/// This is the main diagnostic preflight helper before immediate cast flows.
pub fn check_spell_cast(
    actor: &mut Actor,
    source: CastingSource,
    spell: &MagicItem,
    dual_cast: bool,
    use_base_value_for_cost: bool,
) -> CastCheck {
    with_magic_caster_mut(
        actor,
        source,
        "sdk::gameplay::magic::check_spell_cast()",
        |caster| {
            let mut effect_strength = 1.0f32;
            let mut reason = CannotCastReason::OK;
            let can_cast = caster.check_cast(
                spell as *const MagicItem as *mut MagicItem,
                dual_cast,
                &mut effect_strength,
                &mut reason,
                use_base_value_for_cost,
            );
            CastCheck {
                can_cast,
                reason,
                effect_strength,
            }
        },
    )
    .unwrap_or(CastCheck {
        can_cast: false,
        reason: CannotCastReason::CustomReasonNoStart,
        effect_strength: 0.0,
    })
}

/// Returns `true` when the spell can be cast from the selected source.
#[inline(always)]
pub fn can_cast_spell(
    actor: &mut Actor,
    source: CastingSource,
    spell: &MagicItem,
    dual_cast: bool,
) -> bool {
    check_spell_cast(actor, source, spell, dual_cast, false).can_cast
}

/// Returns the current spell cost for one casting source.
#[inline(always)]
pub fn current_spell_cost_for_source(actor: &mut Actor, source: CastingSource) -> f32 {
    with_magic_caster_mut(
        actor,
        source,
        "sdk::gameplay::magic::current_spell_cost_for_source()",
        |caster| caster.get_current_spell_cost(),
    )
    .unwrap_or(0.0)
}

/// Returns `true` when the casting source is currently dual-casting.
#[inline(always)]
pub fn is_dual_casting_for_source(actor: &mut Actor, source: CastingSource) -> bool {
    magic_caster_for_source(actor, source)
        .as_ref()
        .is_some_and(crate::re::MagicCaster::get_is_dual_casting)
}

/// Enables or disables dual-casting for one source.
pub fn set_dual_casting_for_source(actor: &mut Actor, source: CastingSource, set: bool) -> bool {
    with_magic_caster_mut(
        actor,
        source,
        "sdk::gameplay::magic::set_dual_casting_for_source()",
        |caster| caster.set_dual_casting(set),
    )
    .is_some()
}

/// Overrides the current spell assigned to one casting source.
pub fn set_current_spell_for_source(
    actor: &mut Actor,
    source: CastingSource,
    spell: GamePtr<MagicItem>,
) -> bool {
    with_magic_caster_mut(
        actor,
        source,
        "sdk::gameplay::magic::set_current_spell_for_source()",
        |caster| caster.set_current_spell(spell.as_ptr()),
    )
    .is_some()
}

/// Clears the current spell assigned to one casting source.
#[inline(always)]
pub fn clear_current_spell_for_source(actor: &mut Actor, source: CastingSource) -> bool {
    set_current_spell_for_source(actor, source, GamePtr::null())
}

/// Interrupts the current cast on one source.
pub fn interrupt_cast_for_source(actor: &mut Actor, source: CastingSource, refund: bool) -> bool {
    with_magic_caster_mut(
        actor,
        source,
        "sdk::gameplay::magic::interrupt_cast_for_source()",
        |caster| caster.interrupt_cast(refund),
    )
    .is_some()
}

/// Forces the current cast on one source to finish.
pub fn finish_cast_for_source(actor: &mut Actor, source: CastingSource) -> bool {
    with_magic_caster_mut(
        actor,
        source,
        "sdk::gameplay::magic::finish_cast_for_source()",
        |caster| caster.finish_cast(),
    )
    .is_some()
}

/// Returns `true` when the given spell is the current spell on the source.
#[inline(always)]
pub fn is_current_spell_for_source(
    actor: &mut Actor,
    source: CastingSource,
    spell: &MagicItem,
) -> bool {
    current_spell_for_source(actor, source).as_ptr() == spell as *const MagicItem as *mut MagicItem
}

/// Casts a spell immediately from an arbitrary reference.
///
/// This is the core immediate-cast helper; actor-specific wrappers below are
/// mostly ergonomic adapters around it.
pub fn cast_spell_immediate(
    caster: &mut TESObjectREFR,
    source: CastingSource,
    spell: &MagicItem,
    target: GamePtr<TESObjectREFR>,
    options: ImmediateCastOptions,
) -> bool {
    if invalid_casting_source("sdk::gameplay::magic::cast_spell_immediate()", source) {
        return false;
    }

    if !options.effectiveness.is_finite() || !options.magnitude_override.is_finite() {
        crate::defensive_sdk_warn!(
            "sdk::gameplay::magic::cast_spell_immediate() ignored non-finite effectiveness or magnitude_override"
        );
        return false;
    }

    let magic_caster = caster.get_magic_caster(source);
    let Some(magic_caster) = (unsafe { magic_caster.as_mut() }) else {
        crate::defensive_sdk_warn!(
            "sdk::gameplay::magic::cast_spell_immediate() skipped because get_magic_caster({:?}) returned null",
            source
        );
        return false;
    };

    magic_caster.cast_spell_immediate(
        spell as *const MagicItem as *mut MagicItem,
        options.no_hit_effect_art,
        target.as_ptr(),
        options.effectiveness,
        options.hostile_effectiveness_only,
        options.magnitude_override,
        options.blame_actor.as_ptr(),
    );
    true
}

/// Casts a spell immediately from a reference onto itself.
#[inline(always)]
pub fn cast_spell_immediate_on_self(
    caster: &mut TESObjectREFR,
    source: CastingSource,
    spell: &MagicItem,
    options: ImmediateCastOptions,
) -> bool {
    let caster_ptr = unsafe { GamePtr::from_raw(caster as *mut TESObjectREFR) };
    cast_spell_immediate(caster, source, spell, caster_ptr, options)
}

/// Casts a spell immediately from a reference onto another reference.
#[inline(always)]
pub fn cast_spell_immediate_on_reference(
    caster: &mut TESObjectREFR,
    source: CastingSource,
    spell: &MagicItem,
    target: &TESObjectREFR,
    options: ImmediateCastOptions,
) -> bool {
    let target = unsafe { GamePtr::from_raw(target as *const TESObjectREFR as *mut TESObjectREFR) };
    cast_spell_immediate(caster, source, spell, target, options)
}

/// Checks castability and then casts immediately from an actor when possible.
pub fn try_cast_spell_immediate_from_actor(
    actor: &mut Actor,
    source: CastingSource,
    spell: &MagicItem,
    target: GamePtr<TESObjectREFR>,
    options: ImmediateCastOptions,
) -> CastCheck {
    let dual_cast = is_dual_casting_for_source(actor, source);
    let check = check_spell_cast(actor, source, spell, dual_cast, false);
    if check.can_cast {
        let _ = cast_spell_immediate_from_actor(actor, source, spell, target, options);
    }
    check
}

/// Casts a spell immediately from an actor onto an arbitrary target reference.
#[inline(always)]
pub fn cast_spell_immediate_from_actor(
    actor: &mut Actor,
    source: CastingSource,
    spell: &MagicItem,
    target: GamePtr<TESObjectREFR>,
    options: ImmediateCastOptions,
) -> bool {
    cast_spell_immediate(&mut actor.base, source, spell, target, options)
}

/// Casts a spell immediately from an actor onto itself.
#[inline(always)]
pub fn cast_spell_immediate_from_actor_on_self(
    actor: &mut Actor,
    source: CastingSource,
    spell: &MagicItem,
    options: ImmediateCastOptions,
) -> bool {
    cast_spell_immediate_on_self(&mut actor.base, source, spell, options)
}

/// Checks castability and then casts immediately from an actor onto itself.
#[inline(always)]
pub fn try_cast_spell_immediate_from_actor_on_self(
    actor: &mut Actor,
    source: CastingSource,
    spell: &MagicItem,
    options: ImmediateCastOptions,
) -> CastCheck {
    let target = unsafe { GamePtr::from_raw(&mut actor.base as *mut TESObjectREFR) };
    try_cast_spell_immediate_from_actor(actor, source, spell, target, options)
}

/// Casts a spell immediately from one actor onto another actor.
#[inline(always)]
pub fn cast_spell_immediate_from_actor_on_actor(
    actor: &mut Actor,
    source: CastingSource,
    spell: &MagicItem,
    target: &Actor,
    options: ImmediateCastOptions,
) -> bool {
    cast_spell_immediate_on_reference(&mut actor.base, source, spell, &target.base, options)
}

/// Checks castability and then casts immediately from one actor onto another actor.
#[inline(always)]
pub fn try_cast_spell_immediate_from_actor_on_actor(
    actor: &mut Actor,
    source: CastingSource,
    spell: &MagicItem,
    target: &Actor,
    options: ImmediateCastOptions,
) -> CastCheck {
    try_cast_spell_immediate_from_actor(
        actor,
        source,
        spell,
        unsafe { GamePtr::from_raw(&target.base as *const TESObjectREFR as *mut TESObjectREFR) },
        options,
    )
}
