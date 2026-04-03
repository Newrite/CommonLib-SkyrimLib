use alloc::vec::Vec;
use core::ffi::c_void;
use core::ops::ControlFlow;

use crate::re::{
    Actor, ActorForEachSpellVisitor, BGSKeyword, BSContainerForEachResult, EffectArchetypeId,
    MagicItem, SpellItem,
};
use crate::sdk::core::GamePtr;

use super::items::{
    has_item_effect_with_archetype, has_item_effect_with_keyword,
    has_item_effect_with_keyword_editor_id,
};
use super::shared::{
    actor_spell_has_keyword, actor_spell_has_keyword_editor_id, casting_source_slot,
};

type SpellVisitorCallback =
    unsafe extern "C" fn(*mut c_void, *mut SpellItem) -> BSContainerForEachResult;

#[repr(C)]
struct ActorVisitSpellsVisitor {
    base: ActorForEachSpellVisitor,
    ctx: *mut c_void,
    callback: SpellVisitorCallback,
}

unsafe extern "C" fn actor_visit_spells_visitor_dtor(_this: *mut ActorVisitSpellsVisitor) {}

unsafe extern "C" fn actor_visit_spells_visitor_visit(
    this: *mut ActorVisitSpellsVisitor,
    spell: *mut SpellItem,
) -> BSContainerForEachResult {
    unsafe { ((*this).callback)((*this).ctx, spell) }
}

struct ActorVisitSpellsVisitorVTable([*const (); 2]);

unsafe impl Sync for ActorVisitSpellsVisitorVTable {}

static ACTOR_VISIT_SPELLS_VISITOR_VTABLE: ActorVisitSpellsVisitorVTable =
    ActorVisitSpellsVisitorVTable([
        actor_visit_spells_visitor_dtor as *const (),
        actor_visit_spells_visitor_visit as *const (),
    ]);

unsafe extern "C" fn actor_visit_spells_closure<F>(
    ctx: *mut c_void,
    spell: *mut SpellItem,
) -> BSContainerForEachResult
where
    F: FnMut(*mut SpellItem) -> BSContainerForEachResult,
{
    unsafe { (&mut *(ctx as *mut F))(spell) }
}

fn visit_actor_spells_raw<F>(actor: &mut Actor, callback: &mut F)
where
    F: FnMut(*mut SpellItem) -> BSContainerForEachResult,
{
    let mut visitor = ActorVisitSpellsVisitor {
        base: ActorForEachSpellVisitor {
            vtable: ACTOR_VISIT_SPELLS_VISITOR_VTABLE.0.as_ptr().cast(),
        },
        ctx: (callback as *mut F).cast(),
        callback: actor_visit_spells_closure::<F>,
    };

    actor.visit_spells(&mut visitor.base);
}

#[inline(always)]
fn flow_to_engine_result(flow: ControlFlow<()>) -> BSContainerForEachResult {
    match flow {
        ControlFlow::Continue(()) => BSContainerForEachResult::Continue,
        ControlFlow::Break(()) => BSContainerForEachResult::Stop,
    }
}

#[inline(always)]
fn actor_has_spell_matching(
    actor: &mut Actor,
    mut predicate: impl FnMut(&SpellItem) -> bool,
) -> bool {
    for_each_actor_spell(actor, |spell| {
        if predicate(spell) {
            ControlFlow::Break(())
        } else {
            ControlFlow::Continue(())
        }
    })
    .is_break()
}

pub fn for_each_actor_spell(
    actor: &mut Actor,
    mut visit: impl FnMut(&SpellItem) -> ControlFlow<()>,
) -> ControlFlow<()> {
    let mut flow = ControlFlow::Continue(());
    let mut callback = |spell: *mut SpellItem| {
        let Some(spell) = (unsafe { spell.as_ref() }) else {
            return BSContainerForEachResult::Continue;
        };

        flow = visit(spell);
        flow_to_engine_result(flow)
    };

    visit_actor_spells_raw(actor, &mut callback);
    flow
}

pub fn collect_actor_spells(actor: &mut Actor) -> Vec<GamePtr<SpellItem>> {
    collect_actor_spells_matching(actor, |_| true)
}

pub fn collect_actor_spells_matching(
    actor: &mut Actor,
    mut predicate: impl FnMut(&SpellItem) -> bool,
) -> Vec<GamePtr<SpellItem>> {
    let mut spells = Vec::new();
    let mut callback = |spell: *mut SpellItem| {
        let Some(spell_ref) = (unsafe { spell.as_ref() }) else {
            return BSContainerForEachResult::Continue;
        };

        if predicate(spell_ref) {
            spells.push(unsafe { GamePtr::from_raw(spell) });
        }

        BSContainerForEachResult::Continue
    };

    visit_actor_spells_raw(actor, &mut callback);
    spells
}

pub fn remove_actor_spells_matching(
    actor: &mut Actor,
    mut predicate: impl FnMut(&SpellItem) -> bool,
    deselect_matching: bool,
) -> usize {
    let queued = collect_actor_spells_matching(actor, |spell| predicate(spell));
    let mut removed = 0usize;
    for spell in queued {
        if deselect_matching {
            actor.deselect_spell(spell.as_ptr());
        }

        if actor.remove_spell(spell.as_ptr()) {
            removed += 1;
        }
    }
    removed
}

pub fn has_actor_spell_with_keyword_editor_id(actor: &mut Actor, editor_id: &str) -> bool {
    actor_has_spell_matching(actor, |spell| {
        actor_spell_has_keyword_editor_id(spell, editor_id)
    })
}

pub fn has_actor_spell_with_keyword(actor: &mut Actor, keyword: &BGSKeyword) -> bool {
    actor_has_spell_matching(actor, |spell| actor_spell_has_keyword(spell, keyword))
}

pub fn has_actor_spell_with_effect_keyword(actor: &mut Actor, keyword: &BGSKeyword) -> bool {
    actor_has_spell_matching(actor, |spell| has_item_effect_with_keyword(spell, keyword))
}

pub fn has_actor_spell_with_effect_keyword_editor_id(actor: &mut Actor, editor_id: &str) -> bool {
    actor_has_spell_matching(actor, |spell| {
        has_item_effect_with_keyword_editor_id(spell, editor_id)
    })
}

pub fn has_actor_spell_with_effect_archetype(
    actor: &mut Actor,
    archetype: EffectArchetypeId,
) -> bool {
    actor_has_spell_matching(actor, |spell| {
        has_item_effect_with_archetype(spell, archetype)
    })
}

pub fn collect_actor_spells_with_keyword(
    actor: &mut Actor,
    keyword: &BGSKeyword,
) -> Vec<GamePtr<SpellItem>> {
    collect_actor_spells_matching(actor, |spell| actor_spell_has_keyword(spell, keyword))
}

pub fn collect_actor_spells_with_keyword_editor_id(
    actor: &mut Actor,
    editor_id: &str,
) -> Vec<GamePtr<SpellItem>> {
    collect_actor_spells_matching(actor, |spell| {
        actor_spell_has_keyword_editor_id(spell, editor_id)
    })
}

pub fn collect_actor_spells_with_effect_keyword(
    actor: &mut Actor,
    keyword: &BGSKeyword,
) -> Vec<GamePtr<SpellItem>> {
    collect_actor_spells_matching(actor, |spell| has_item_effect_with_keyword(spell, keyword))
}

pub fn collect_actor_spells_with_effect_keyword_editor_id(
    actor: &mut Actor,
    editor_id: &str,
) -> Vec<GamePtr<SpellItem>> {
    collect_actor_spells_matching(actor, |spell| {
        has_item_effect_with_keyword_editor_id(spell, editor_id)
    })
}

pub fn collect_actor_spells_with_effect_archetype(
    actor: &mut Actor,
    archetype: EffectArchetypeId,
) -> Vec<GamePtr<SpellItem>> {
    collect_actor_spells_matching(actor, |spell| {
        has_item_effect_with_archetype(spell, archetype)
    })
}

#[inline(always)]
pub fn add_actor_spell(actor: &mut Actor, spell: &SpellItem) -> bool {
    actor.add_spell(spell as *const SpellItem as *mut SpellItem)
}

#[inline(always)]
pub fn has_actor_spell(actor: &Actor, spell: &SpellItem) -> bool {
    actor.has_spell(spell as *const SpellItem as *mut SpellItem)
}

#[inline(always)]
pub fn ensure_actor_spell(actor: &mut Actor, spell: &SpellItem) -> bool {
    has_actor_spell(actor, spell) || add_actor_spell(actor, spell)
}

#[inline(always)]
pub fn set_actor_spell_enabled(
    actor: &mut Actor,
    spell: &SpellItem,
    enabled: bool,
    deselect_first: bool,
) -> bool {
    if enabled {
        ensure_actor_spell(actor, spell)
    } else {
        remove_actor_spell(actor, spell, deselect_first)
    }
}

#[inline(always)]
pub fn remove_actor_spell(actor: &mut Actor, spell: &SpellItem, deselect_first: bool) -> bool {
    let spell = spell as *const SpellItem as *mut SpellItem;
    if deselect_first {
        actor.deselect_spell(spell);
    }
    actor.remove_spell(spell)
}

#[inline(always)]
pub fn selected_actor_spell(
    actor: &Actor,
    source: crate::re::magic_system::CastingSource,
) -> GamePtr<MagicItem> {
    let Some(slot) = casting_source_slot(source) else {
        return GamePtr::null();
    };

    unsafe { GamePtr::from_raw(actor.get_actor_runtime_data().selected_spells[slot]) }
}

#[inline(always)]
pub fn is_selected_actor_spell(
    actor: &Actor,
    source: crate::re::magic_system::CastingSource,
    spell: &MagicItem,
) -> bool {
    selected_actor_spell(actor, source).as_ptr() == spell as *const MagicItem as *mut MagicItem
}

pub fn remove_spells_with_keyword(
    actor: &mut Actor,
    keyword: &BGSKeyword,
    deselect_matching: bool,
) -> usize {
    remove_actor_spells_matching(
        actor,
        |spell| actor_spell_has_keyword(spell, keyword),
        deselect_matching,
    )
}

pub fn remove_spells_with_keyword_editor_id(
    actor: &mut Actor,
    editor_id: &str,
    deselect_matching: bool,
) -> usize {
    remove_actor_spells_matching(
        actor,
        |spell| actor_spell_has_keyword_editor_id(spell, editor_id),
        deselect_matching,
    )
}

pub fn remove_spells_with_effect_keyword(
    actor: &mut Actor,
    keyword: &BGSKeyword,
    deselect_matching: bool,
) -> usize {
    remove_actor_spells_matching(
        actor,
        |spell| has_item_effect_with_keyword(spell, keyword),
        deselect_matching,
    )
}

pub fn remove_spells_with_effect_keyword_editor_id(
    actor: &mut Actor,
    editor_id: &str,
    deselect_matching: bool,
) -> usize {
    remove_actor_spells_matching(
        actor,
        |spell| has_item_effect_with_keyword_editor_id(spell, editor_id),
        deselect_matching,
    )
}

pub fn remove_spells_with_effect_archetype(
    actor: &mut Actor,
    archetype: EffectArchetypeId,
    deselect_matching: bool,
) -> usize {
    remove_actor_spells_matching(
        actor,
        |spell| has_item_effect_with_archetype(spell, archetype),
        deselect_matching,
    )
}
