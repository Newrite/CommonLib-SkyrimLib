use alloc::boxed::Box;
use alloc::vec::Vec;

use core::ops::ControlFlow;

use crate::re::{
    BGSKeyword, ITEM_REMOVE_REASON, InventoryEntryData, ObjectRefHandle, TESBoundObject, TESForm,
    TESObjectREFR,
};
use crate::sdk::core::GamePtr;

use super::types::{InventoryEntrySnapshot, InventoryQueryOptions};

#[inline(always)]
pub(crate) fn bound_object_form(object: GamePtr<TESBoundObject>) -> GamePtr<TESForm> {
    unsafe { GamePtr::from_raw(object.as_ptr().cast::<TESForm>()) }
}

#[inline(always)]
pub(crate) fn form_has_keyword(form: GamePtr<TESForm>, keyword: &BGSKeyword) -> bool {
    form.map_or(false, |form| {
        form.has_keyword_in_array(&[keyword as *const BGSKeyword as *mut BGSKeyword], false)
    })
}

#[inline(always)]
pub(crate) fn form_has_keyword_editor_id(form: GamePtr<TESForm>, editor_id: &str) -> bool {
    form.map_or(false, |form| form.has_keyword_by_editor_id(editor_id))
}

#[inline(always)]
pub(crate) fn snapshot_from_inventory_entry(
    object: *mut TESBoundObject,
    count: i32,
    entry: Box<InventoryEntryData>,
) -> Option<InventoryEntrySnapshot> {
    if object.is_null() || count <= 0 {
        return None;
    }

    Some(InventoryEntrySnapshot {
        object: unsafe { GamePtr::from_raw(object) },
        count,
        entry,
    })
}

pub(crate) fn for_each_snapshot_with<T>(
    container: &mut T,
    options: InventoryQueryOptions,
    mut visit: impl FnMut(&mut InventoryEntrySnapshot) -> ControlFlow<()>,
) -> ControlFlow<()>
where
    T: AsMut<TESObjectREFR> + ?Sized,
{
    let inventory = container
        .as_mut()
        .get_inventory_with(|_| true, options.no_init);
    for (object, (count, entry)) in inventory {
        let Some(mut snapshot) = snapshot_from_inventory_entry(object, count, entry) else {
            continue;
        };

        let flow = visit(&mut snapshot);
        if flow.is_break() {
            return flow;
        }
    }

    ControlFlow::Continue(())
}

#[inline(always)]
pub(crate) fn into_owned_snapshot(snapshot: &mut InventoryEntrySnapshot) -> InventoryEntrySnapshot {
    InventoryEntrySnapshot {
        object: snapshot.object,
        count: snapshot.count,
        entry: core::mem::take(&mut snapshot.entry),
    }
}

#[inline(always)]
pub(crate) fn push_unique_form(forms: &mut Vec<GamePtr<TESForm>>, form: GamePtr<TESForm>) {
    if form.is_some() && !forms.contains(&form) {
        forms.push(form);
    }
}

#[inline(always)]
pub(crate) fn remove_item_raw(
    container: &mut TESObjectREFR,
    item: *mut TESBoundObject,
    count: i32,
    reason: ITEM_REMOVE_REASON,
    move_to: *mut TESObjectREFR,
) -> ObjectRefHandle {
    if item.is_null() || count <= 0 {
        return ObjectRefHandle::new();
    }

    container.remove_item(
        item,
        count,
        reason,
        core::ptr::null_mut(),
        move_to,
        core::ptr::null(),
        core::ptr::null(),
    )
}
