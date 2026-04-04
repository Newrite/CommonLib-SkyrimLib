use crate::re::{ITEM_REMOVE_REASON, ObjectRefHandle, TESBoundObject, TESObjectREFR};

use super::entries::collect_inventory_entries_matching_with_options;
use super::shared::remove_item_raw;
use super::types::{InventoryEntrySnapshot, InventoryQueryOptions};

pub fn remove_item_object<T>(
    container: &mut T,
    item: &TESBoundObject,
    count: i32,
) -> ObjectRefHandle
where
    T: AsMut<TESObjectREFR> + ?Sized,
{
    remove_item_object_with_reason(container, item, count, ITEM_REMOVE_REASON::Remove)
}

pub fn remove_item_object_with_reason<T>(
    container: &mut T,
    item: &TESBoundObject,
    count: i32,
    reason: ITEM_REMOVE_REASON,
) -> ObjectRefHandle
where
    T: AsMut<TESObjectREFR> + ?Sized,
{
    remove_item_raw(
        container.as_mut(),
        item as *const TESBoundObject as *mut TESBoundObject,
        count,
        reason,
        core::ptr::null_mut(),
    )
}

pub fn transfer_item_object<From, To>(
    from: &mut From,
    to: &mut To,
    item: &TESBoundObject,
    count: i32,
) -> ObjectRefHandle
where
    From: AsMut<TESObjectREFR> + ?Sized,
    To: AsMut<TESObjectREFR> + ?Sized,
{
    transfer_item_object_with_reason(from, to, item, count, ITEM_REMOVE_REASON::StoreInContainer)
}

pub fn transfer_item_object_with_reason<From, To>(
    from: &mut From,
    to: &mut To,
    item: &TESBoundObject,
    count: i32,
    reason: ITEM_REMOVE_REASON,
) -> ObjectRefHandle
where
    From: AsMut<TESObjectREFR> + ?Sized,
    To: AsMut<TESObjectREFR> + ?Sized,
{
    let to_ptr = to.as_mut() as *mut TESObjectREFR;
    let from_ref = from.as_mut();
    let from_ptr = from_ref as *mut TESObjectREFR;
    if core::ptr::eq(from_ptr, to_ptr) {
        return ObjectRefHandle::new();
    }

    remove_item_raw(
        from_ref,
        item as *const TESBoundObject as *mut TESBoundObject,
        count,
        reason,
        to_ptr,
    )
}

pub fn remove_inventory_entries_matching_with_options_and_reason<T>(
    container: &mut T,
    options: InventoryQueryOptions,
    reason: ITEM_REMOVE_REASON,
    predicate: impl FnMut(&mut InventoryEntrySnapshot) -> bool,
) -> i32
where
    T: AsMut<TESObjectREFR> + ?Sized,
{
    let entries = collect_inventory_entries_matching_with_options(container, options, predicate);
    let container = container.as_mut();

    let mut removed: i32 = 0;
    for snapshot in entries {
        let count = snapshot.count().max(0);
        if count <= 0 {
            continue;
        }

        let _ = remove_item_raw(
            container,
            snapshot.object().as_ptr(),
            count,
            reason,
            core::ptr::null_mut(),
        );
        removed = removed.saturating_add(count);
    }
    removed
}

#[inline(always)]
pub fn remove_inventory_entries_matching_with_options<T>(
    container: &mut T,
    options: InventoryQueryOptions,
    predicate: impl FnMut(&mut InventoryEntrySnapshot) -> bool,
) -> i32
where
    T: AsMut<TESObjectREFR> + ?Sized,
{
    remove_inventory_entries_matching_with_options_and_reason(
        container,
        options,
        ITEM_REMOVE_REASON::Remove,
        predicate,
    )
}

#[inline(always)]
pub fn remove_inventory_entries_matching<T>(
    container: &mut T,
    predicate: impl FnMut(&mut InventoryEntrySnapshot) -> bool,
) -> i32
where
    T: AsMut<TESObjectREFR> + ?Sized,
{
    remove_inventory_entries_matching_with_options(
        container,
        InventoryQueryOptions::default(),
        predicate,
    )
}

pub fn transfer_inventory_entries_matching_with_options_and_reason<From, To>(
    from: &mut From,
    to: &mut To,
    options: InventoryQueryOptions,
    reason: ITEM_REMOVE_REASON,
    predicate: impl FnMut(&mut InventoryEntrySnapshot) -> bool,
) -> i32
where
    From: AsMut<TESObjectREFR> + ?Sized,
    To: AsMut<TESObjectREFR> + ?Sized,
{
    let entries = collect_inventory_entries_matching_with_options(from, options, predicate);
    let to_ptr = to.as_mut() as *mut TESObjectREFR;
    let from_ref = from.as_mut();
    let from_ptr = from_ref as *mut TESObjectREFR;
    if core::ptr::eq(from_ptr, to_ptr) {
        return 0;
    }

    let mut transferred: i32 = 0;
    for snapshot in entries {
        let count = snapshot.count().max(0);
        if count <= 0 {
            continue;
        }

        let _ = remove_item_raw(from_ref, snapshot.object().as_ptr(), count, reason, to_ptr);
        transferred = transferred.saturating_add(count);
    }
    transferred
}

#[inline(always)]
pub fn transfer_inventory_entries_matching_with_options<From, To>(
    from: &mut From,
    to: &mut To,
    options: InventoryQueryOptions,
    predicate: impl FnMut(&mut InventoryEntrySnapshot) -> bool,
) -> i32
where
    From: AsMut<TESObjectREFR> + ?Sized,
    To: AsMut<TESObjectREFR> + ?Sized,
{
    transfer_inventory_entries_matching_with_options_and_reason(
        from,
        to,
        options,
        ITEM_REMOVE_REASON::StoreInContainer,
        predicate,
    )
}

#[inline(always)]
pub fn transfer_inventory_entries_matching<From, To>(
    from: &mut From,
    to: &mut To,
    predicate: impl FnMut(&mut InventoryEntrySnapshot) -> bool,
) -> i32
where
    From: AsMut<TESObjectREFR> + ?Sized,
    To: AsMut<TESObjectREFR> + ?Sized,
{
    transfer_inventory_entries_matching_with_options(
        from,
        to,
        InventoryQueryOptions::default(),
        predicate,
    )
}
