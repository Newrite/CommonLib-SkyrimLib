use alloc::vec::Vec;

use crate::re::{ActorExt, BGSKeyword, BipedObjectSlot, TESForm, TESObjectARMO, TESObjectREFR};
use crate::sdk::core::GamePtr;

use super::entries::{
    collect_inventory_entries_matching_with_options, find_inventory_entry_matching_with_options,
};
use super::shared::{form_has_keyword, form_has_keyword_editor_id, push_unique_form};
use super::types::{InventoryEntrySnapshot, InventoryQueryOptions};

#[inline(always)]
pub fn collect_worn_entries_with_options<T>(
    actor: &mut T,
    options: InventoryQueryOptions,
) -> Vec<InventoryEntrySnapshot>
where
    T: AsMut<TESObjectREFR> + ?Sized,
{
    collect_inventory_entries_matching_with_options(actor, options, |snapshot| snapshot.is_worn())
}

#[inline(always)]
pub fn collect_worn_entries<T>(actor: &mut T) -> Vec<InventoryEntrySnapshot>
where
    T: AsMut<TESObjectREFR> + ?Sized,
{
    collect_worn_entries_with_options(actor, InventoryQueryOptions::default())
}

pub fn collect_worn_armor_with_options<T>(
    actor: &mut T,
    options: InventoryQueryOptions,
) -> Vec<GamePtr<TESObjectARMO>>
where
    T: AsMut<TESObjectREFR> + ?Sized,
{
    collect_inventory_entries_matching_with_options(actor, options, |snapshot| {
        snapshot.is_worn() && snapshot.is_armor()
    })
    .into_iter()
    .filter_map(|snapshot| {
        let armor = snapshot.try_cast_form::<TESObjectARMO>();
        armor.is_some().then_some(armor)
    })
    .collect()
}

#[inline(always)]
pub fn collect_worn_armor<T>(actor: &mut T) -> Vec<GamePtr<TESObjectARMO>>
where
    T: AsMut<TESObjectREFR> + ?Sized,
{
    collect_worn_armor_with_options(actor, InventoryQueryOptions::default())
}

#[inline(always)]
pub fn find_worn_entry_matching_with_options<T>(
    actor: &mut T,
    options: InventoryQueryOptions,
    mut predicate: impl FnMut(&mut InventoryEntrySnapshot) -> bool,
) -> Option<InventoryEntrySnapshot>
where
    T: AsMut<TESObjectREFR> + ?Sized,
{
    find_inventory_entry_matching_with_options(actor, options, |snapshot| {
        snapshot.is_worn() && predicate(snapshot)
    })
}

#[inline(always)]
pub fn find_worn_entry_matching<T>(
    actor: &mut T,
    predicate: impl FnMut(&mut InventoryEntrySnapshot) -> bool,
) -> Option<InventoryEntrySnapshot>
where
    T: AsMut<TESObjectREFR> + ?Sized,
{
    find_worn_entry_matching_with_options(actor, InventoryQueryOptions::default(), predicate)
}

#[inline(always)]
pub fn worn_armor_in_slot_with_options<T>(
    actor: &mut T,
    slot: BipedObjectSlot,
    options: InventoryQueryOptions,
) -> GamePtr<TESObjectARMO>
where
    T: ActorExt + ?Sized,
{
    unsafe { GamePtr::from_raw(actor.get_worn_armor(slot, options.no_init)) }
}

#[inline(always)]
pub fn worn_armor_in_slot<T>(actor: &mut T, slot: BipedObjectSlot) -> GamePtr<TESObjectARMO>
where
    T: ActorExt + ?Sized,
{
    worn_armor_in_slot_with_options(actor, slot, InventoryQueryOptions::default())
}

#[inline(always)]
pub fn has_worn_armor_in_slot_with_options<T>(
    actor: &mut T,
    slot: BipedObjectSlot,
    options: InventoryQueryOptions,
) -> bool
where
    T: ActorExt + ?Sized,
{
    worn_armor_in_slot_with_options(actor, slot, options).is_some()
}

#[inline(always)]
pub fn has_worn_armor_in_slot<T>(actor: &mut T, slot: BipedObjectSlot) -> bool
where
    T: ActorExt + ?Sized,
{
    has_worn_armor_in_slot_with_options(actor, slot, InventoryQueryOptions::default())
}

pub fn collect_equipped_forms_with_options<T>(
    actor: &mut T,
    options: InventoryQueryOptions,
) -> Vec<GamePtr<TESForm>>
where
    T: ActorExt + AsMut<TESObjectREFR> + ?Sized,
{
    let mut forms = Vec::new();
    push_unique_form(&mut forms, unsafe {
        GamePtr::from_raw(actor.get_equipped_object(false))
    });
    push_unique_form(&mut forms, unsafe {
        GamePtr::from_raw(actor.get_equipped_object(true))
    });

    for armor in collect_worn_armor_with_options(actor, options) {
        push_unique_form(&mut forms, unsafe {
            GamePtr::from_raw(armor.as_ptr().cast::<TESForm>())
        });
    }

    forms
}

#[inline(always)]
pub fn collect_equipped_forms<T>(actor: &mut T) -> Vec<GamePtr<TESForm>>
where
    T: ActorExt + AsMut<TESObjectREFR> + ?Sized,
{
    collect_equipped_forms_with_options(actor, InventoryQueryOptions::default())
}

#[inline(always)]
pub fn has_equipped_form_with_options<T>(
    actor: &mut T,
    form: &TESForm,
    options: InventoryQueryOptions,
) -> bool
where
    T: ActorExt + AsMut<TESObjectREFR> + ?Sized,
{
    let form_ptr = form as *const TESForm as *mut TESForm;
    collect_equipped_forms_with_options(actor, options)
        .into_iter()
        .any(|equipped| equipped.as_ptr() == form_ptr)
}

#[inline(always)]
pub fn has_equipped_form<T>(actor: &mut T, form: &TESForm) -> bool
where
    T: ActorExt + AsMut<TESObjectREFR> + ?Sized,
{
    has_equipped_form_with_options(actor, form, InventoryQueryOptions::default())
}

pub fn has_equipped_keyword_with_options<T>(
    actor: &mut T,
    keyword: &BGSKeyword,
    options: InventoryQueryOptions,
) -> bool
where
    T: ActorExt + AsMut<TESObjectREFR> + ?Sized,
{
    collect_equipped_forms_with_options(actor, options)
        .into_iter()
        .any(|form| form_has_keyword(form, keyword))
}

#[inline(always)]
pub fn has_equipped_keyword<T>(actor: &mut T, keyword: &BGSKeyword) -> bool
where
    T: ActorExt + AsMut<TESObjectREFR> + ?Sized,
{
    has_equipped_keyword_with_options(actor, keyword, InventoryQueryOptions::default())
}

pub fn has_equipped_keyword_with_editor_id_with_options<T>(
    actor: &mut T,
    editor_id: &str,
    options: InventoryQueryOptions,
) -> bool
where
    T: ActorExt + AsMut<TESObjectREFR> + ?Sized,
{
    collect_equipped_forms_with_options(actor, options)
        .into_iter()
        .any(|form| form_has_keyword_editor_id(form, editor_id))
}

#[inline(always)]
pub fn has_equipped_keyword_with_editor_id<T>(actor: &mut T, editor_id: &str) -> bool
where
    T: ActorExt + AsMut<TESObjectREFR> + ?Sized,
{
    has_equipped_keyword_with_editor_id_with_options(
        actor,
        editor_id,
        InventoryQueryOptions::default(),
    )
}
