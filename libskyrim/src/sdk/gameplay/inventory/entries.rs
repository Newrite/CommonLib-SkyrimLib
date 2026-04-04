use alloc::vec::Vec;

use crate::re::{BGSKeyword, TESBoundObject, TESObjectREFR};
use crate::sdk::core::GamePtr;

use super::shared::{for_each_snapshot_with, into_owned_snapshot};
use super::types::{InventoryEntrySnapshot, InventoryQueryOptions};

pub fn for_each_inventory_entry_with_options<T>(
    container: &mut T,
    options: InventoryQueryOptions,
    visit: impl FnMut(&mut InventoryEntrySnapshot) -> core::ops::ControlFlow<()>,
) -> core::ops::ControlFlow<()>
where
    T: AsMut<TESObjectREFR> + ?Sized,
{
    for_each_snapshot_with(container, options, visit)
}

#[inline(always)]
pub fn for_each_inventory_entry<T>(
    container: &mut T,
    visit: impl FnMut(&mut InventoryEntrySnapshot) -> core::ops::ControlFlow<()>,
) -> core::ops::ControlFlow<()>
where
    T: AsMut<TESObjectREFR> + ?Sized,
{
    for_each_inventory_entry_with_options(container, InventoryQueryOptions::default(), visit)
}

pub fn collect_inventory_entries_with_options<T>(
    container: &mut T,
    options: InventoryQueryOptions,
) -> Vec<InventoryEntrySnapshot>
where
    T: AsMut<TESObjectREFR> + ?Sized,
{
    collect_inventory_entries_matching_with_options(container, options, |_| true)
}

#[inline(always)]
pub fn collect_inventory_entries<T>(container: &mut T) -> Vec<InventoryEntrySnapshot>
where
    T: AsMut<TESObjectREFR> + ?Sized,
{
    collect_inventory_entries_with_options(container, InventoryQueryOptions::default())
}

pub fn collect_inventory_entries_matching_with_options<T>(
    container: &mut T,
    options: InventoryQueryOptions,
    mut predicate: impl FnMut(&mut InventoryEntrySnapshot) -> bool,
) -> Vec<InventoryEntrySnapshot>
where
    T: AsMut<TESObjectREFR> + ?Sized,
{
    let mut entries = Vec::new();
    let _ = for_each_snapshot_with(container, options, |snapshot| {
        if predicate(snapshot) {
            entries.push(into_owned_snapshot(snapshot));
        }
        core::ops::ControlFlow::Continue(())
    });
    entries
}

#[inline(always)]
pub fn collect_inventory_entries_matching<T>(
    container: &mut T,
    predicate: impl FnMut(&mut InventoryEntrySnapshot) -> bool,
) -> Vec<InventoryEntrySnapshot>
where
    T: AsMut<TESObjectREFR> + ?Sized,
{
    collect_inventory_entries_matching_with_options(
        container,
        InventoryQueryOptions::default(),
        predicate,
    )
}

pub fn find_inventory_entry_matching_with_options<T>(
    container: &mut T,
    options: InventoryQueryOptions,
    mut predicate: impl FnMut(&mut InventoryEntrySnapshot) -> bool,
) -> Option<InventoryEntrySnapshot>
where
    T: AsMut<TESObjectREFR> + ?Sized,
{
    let mut found = None;
    let _ = for_each_snapshot_with(container, options, |snapshot| {
        if predicate(snapshot) {
            found = Some(into_owned_snapshot(snapshot));
            return core::ops::ControlFlow::Break(());
        }
        core::ops::ControlFlow::Continue(())
    });
    found
}

#[inline(always)]
pub fn find_inventory_entry_matching<T>(
    container: &mut T,
    predicate: impl FnMut(&mut InventoryEntrySnapshot) -> bool,
) -> Option<InventoryEntrySnapshot>
where
    T: AsMut<TESObjectREFR> + ?Sized,
{
    find_inventory_entry_matching_with_options(
        container,
        InventoryQueryOptions::default(),
        predicate,
    )
}

pub fn count_inventory_items_matching_with_options<T>(
    container: &mut T,
    options: InventoryQueryOptions,
    mut predicate: impl FnMut(&mut InventoryEntrySnapshot) -> bool,
) -> i32
where
    T: AsMut<TESObjectREFR> + ?Sized,
{
    let mut total: i32 = 0;
    let _ = for_each_snapshot_with(container, options, |snapshot| {
        if predicate(snapshot) {
            total = total.saturating_add(snapshot.count());
        }
        core::ops::ControlFlow::Continue(())
    });
    total
}

#[inline(always)]
pub fn count_inventory_items_matching<T>(
    container: &mut T,
    predicate: impl FnMut(&mut InventoryEntrySnapshot) -> bool,
) -> i32
where
    T: AsMut<TESObjectREFR> + ?Sized,
{
    count_inventory_items_matching_with_options(
        container,
        InventoryQueryOptions::default(),
        predicate,
    )
}

#[inline(always)]
pub fn count_inventory_items_with_options<T>(
    container: &mut T,
    options: InventoryQueryOptions,
) -> i32
where
    T: AsMut<TESObjectREFR> + ?Sized,
{
    count_inventory_items_matching_with_options(container, options, |_| true)
}

#[inline(always)]
pub fn count_inventory_items<T>(container: &mut T) -> i32
where
    T: AsMut<TESObjectREFR> + ?Sized,
{
    count_inventory_items_with_options(container, InventoryQueryOptions::default())
}

pub fn collect_inventory_objects_with_options<T>(
    container: &mut T,
    options: InventoryQueryOptions,
) -> Vec<GamePtr<TESBoundObject>>
where
    T: AsMut<TESObjectREFR> + ?Sized,
{
    collect_inventory_entries_with_options(container, options)
        .into_iter()
        .map(|snapshot| snapshot.object())
        .collect()
}

#[inline(always)]
pub fn collect_inventory_objects<T>(container: &mut T) -> Vec<GamePtr<TESBoundObject>>
where
    T: AsMut<TESObjectREFR> + ?Sized,
{
    collect_inventory_objects_with_options(container, InventoryQueryOptions::default())
}

pub fn collect_inventory_objects_matching_with_options<T>(
    container: &mut T,
    options: InventoryQueryOptions,
    predicate: impl FnMut(&mut InventoryEntrySnapshot) -> bool,
) -> Vec<GamePtr<TESBoundObject>>
where
    T: AsMut<TESObjectREFR> + ?Sized,
{
    collect_inventory_entries_matching_with_options(container, options, predicate)
        .into_iter()
        .map(|snapshot| snapshot.object())
        .collect()
}

#[inline(always)]
pub fn collect_inventory_objects_matching<T>(
    container: &mut T,
    predicate: impl FnMut(&mut InventoryEntrySnapshot) -> bool,
) -> Vec<GamePtr<TESBoundObject>>
where
    T: AsMut<TESObjectREFR> + ?Sized,
{
    collect_inventory_objects_matching_with_options(
        container,
        InventoryQueryOptions::default(),
        predicate,
    )
}

pub fn collect_typed_inventory_objects_with_options<TObject, TContainer>(
    container: &mut TContainer,
    options: InventoryQueryOptions,
) -> Vec<GamePtr<TObject>>
where
    TObject: crate::relocation::RttiType,
    TContainer: AsMut<TESObjectREFR> + ?Sized,
{
    collect_inventory_objects_with_options(container, options)
        .into_iter()
        .filter_map(|object| {
            let typed = object.try_cast::<TObject>();
            typed.is_some().then_some(typed)
        })
        .collect()
}

#[inline(always)]
pub fn collect_typed_inventory_objects<TObject, TContainer>(
    container: &mut TContainer,
) -> Vec<GamePtr<TObject>>
where
    TObject: crate::relocation::RttiType,
    TContainer: AsMut<TESObjectREFR> + ?Sized,
{
    collect_typed_inventory_objects_with_options(container, InventoryQueryOptions::default())
}

pub fn collect_typed_inventory_objects_matching_with_options<TObject, TContainer>(
    container: &mut TContainer,
    options: InventoryQueryOptions,
    predicate: impl FnMut(&mut InventoryEntrySnapshot) -> bool,
) -> Vec<GamePtr<TObject>>
where
    TObject: crate::relocation::RttiType,
    TContainer: AsMut<TESObjectREFR> + ?Sized,
{
    collect_inventory_objects_matching_with_options(container, options, predicate)
        .into_iter()
        .filter_map(|object| {
            let typed = object.try_cast::<TObject>();
            typed.is_some().then_some(typed)
        })
        .collect()
}

#[inline(always)]
pub fn collect_typed_inventory_objects_matching<TObject, TContainer>(
    container: &mut TContainer,
    predicate: impl FnMut(&mut InventoryEntrySnapshot) -> bool,
) -> Vec<GamePtr<TObject>>
where
    TObject: crate::relocation::RttiType,
    TContainer: AsMut<TESObjectREFR> + ?Sized,
{
    collect_typed_inventory_objects_matching_with_options(
        container,
        InventoryQueryOptions::default(),
        predicate,
    )
}

pub fn find_typed_inventory_object_matching_with_options<TObject, TContainer>(
    container: &mut TContainer,
    options: InventoryQueryOptions,
    predicate: impl FnMut(&mut InventoryEntrySnapshot) -> bool,
) -> GamePtr<TObject>
where
    TObject: crate::relocation::RttiType,
    TContainer: AsMut<TESObjectREFR> + ?Sized,
{
    find_inventory_entry_matching_with_options(container, options, predicate)
        .map_or(GamePtr::null(), |snapshot| {
            snapshot.object().try_cast::<TObject>()
        })
}

#[inline(always)]
pub fn find_typed_inventory_object_matching<TObject, TContainer>(
    container: &mut TContainer,
    predicate: impl FnMut(&mut InventoryEntrySnapshot) -> bool,
) -> GamePtr<TObject>
where
    TObject: crate::relocation::RttiType,
    TContainer: AsMut<TESObjectREFR> + ?Sized,
{
    find_typed_inventory_object_matching_with_options(
        container,
        InventoryQueryOptions::default(),
        predicate,
    )
}

#[inline(always)]
pub fn count_item_with_options<T>(
    container: &mut T,
    options: InventoryQueryOptions,
    item: &TESBoundObject,
) -> i32
where
    T: AsMut<TESObjectREFR> + ?Sized,
{
    let item_ptr = item as *const TESBoundObject as *mut TESBoundObject;
    count_inventory_items_matching_with_options(container, options, |snapshot| {
        snapshot.object().as_ptr() == item_ptr
    })
}

#[inline(always)]
pub fn count_item<T>(container: &mut T, item: &TESBoundObject) -> i32
where
    T: AsMut<TESObjectREFR> + ?Sized,
{
    count_item_with_options(container, InventoryQueryOptions::default(), item)
}

#[inline(always)]
pub fn has_item_with_options<T>(
    container: &mut T,
    options: InventoryQueryOptions,
    item: &TESBoundObject,
) -> bool
where
    T: AsMut<TESObjectREFR> + ?Sized,
{
    count_item_with_options(container, options, item) > 0
}

#[inline(always)]
pub fn has_item<T>(container: &mut T, item: &TESBoundObject) -> bool
where
    T: AsMut<TESObjectREFR> + ?Sized,
{
    has_item_with_options(container, InventoryQueryOptions::default(), item)
}

#[inline(always)]
pub fn count_items_by_editor_id_with_options<T>(
    container: &mut T,
    options: InventoryQueryOptions,
    editor_id: &str,
) -> i32
where
    T: AsMut<TESObjectREFR> + ?Sized,
{
    count_inventory_items_matching_with_options(container, options, |snapshot| {
        snapshot.form_editor_id() == editor_id
    })
}

#[inline(always)]
pub fn count_items_by_editor_id<T>(container: &mut T, editor_id: &str) -> i32
where
    T: AsMut<TESObjectREFR> + ?Sized,
{
    count_items_by_editor_id_with_options(container, InventoryQueryOptions::default(), editor_id)
}

#[inline(always)]
pub fn has_items_by_editor_id_with_options<T>(
    container: &mut T,
    options: InventoryQueryOptions,
    editor_id: &str,
) -> bool
where
    T: AsMut<TESObjectREFR> + ?Sized,
{
    count_items_by_editor_id_with_options(container, options, editor_id) > 0
}

#[inline(always)]
pub fn has_items_by_editor_id<T>(container: &mut T, editor_id: &str) -> bool
where
    T: AsMut<TESObjectREFR> + ?Sized,
{
    has_items_by_editor_id_with_options(container, InventoryQueryOptions::default(), editor_id)
}

#[inline(always)]
pub fn find_inventory_entry_by_editor_id_with_options<T>(
    container: &mut T,
    options: InventoryQueryOptions,
    editor_id: &str,
) -> Option<InventoryEntrySnapshot>
where
    T: AsMut<TESObjectREFR> + ?Sized,
{
    find_inventory_entry_matching_with_options(container, options, |snapshot| {
        snapshot.form_editor_id() == editor_id
    })
}

#[inline(always)]
pub fn find_inventory_entry_by_editor_id<T>(
    container: &mut T,
    editor_id: &str,
) -> Option<InventoryEntrySnapshot>
where
    T: AsMut<TESObjectREFR> + ?Sized,
{
    find_inventory_entry_by_editor_id_with_options(
        container,
        InventoryQueryOptions::default(),
        editor_id,
    )
}

#[inline(always)]
pub fn count_items_with_keyword_with_options<T>(
    container: &mut T,
    options: InventoryQueryOptions,
    keyword: &BGSKeyword,
) -> i32
where
    T: AsMut<TESObjectREFR> + ?Sized,
{
    count_inventory_items_matching_with_options(container, options, |snapshot| {
        snapshot.has_keyword(keyword)
    })
}

#[inline(always)]
pub fn count_items_with_keyword<T>(container: &mut T, keyword: &BGSKeyword) -> i32
where
    T: AsMut<TESObjectREFR> + ?Sized,
{
    count_items_with_keyword_with_options(container, InventoryQueryOptions::default(), keyword)
}

#[inline(always)]
pub fn has_items_with_keyword_with_options<T>(
    container: &mut T,
    options: InventoryQueryOptions,
    keyword: &BGSKeyword,
) -> bool
where
    T: AsMut<TESObjectREFR> + ?Sized,
{
    count_items_with_keyword_with_options(container, options, keyword) > 0
}

#[inline(always)]
pub fn has_items_with_keyword<T>(container: &mut T, keyword: &BGSKeyword) -> bool
where
    T: AsMut<TESObjectREFR> + ?Sized,
{
    has_items_with_keyword_with_options(container, InventoryQueryOptions::default(), keyword)
}

#[inline(always)]
pub fn count_items_with_keyword_editor_id_with_options<T>(
    container: &mut T,
    options: InventoryQueryOptions,
    editor_id: &str,
) -> i32
where
    T: AsMut<TESObjectREFR> + ?Sized,
{
    count_inventory_items_matching_with_options(container, options, |snapshot| {
        snapshot.has_keyword_with_editor_id(editor_id)
    })
}

#[inline(always)]
pub fn count_items_with_keyword_editor_id<T>(container: &mut T, editor_id: &str) -> i32
where
    T: AsMut<TESObjectREFR> + ?Sized,
{
    count_items_with_keyword_editor_id_with_options(
        container,
        InventoryQueryOptions::default(),
        editor_id,
    )
}

#[inline(always)]
pub fn has_items_with_keyword_editor_id_with_options<T>(
    container: &mut T,
    options: InventoryQueryOptions,
    editor_id: &str,
) -> bool
where
    T: AsMut<TESObjectREFR> + ?Sized,
{
    count_items_with_keyword_editor_id_with_options(container, options, editor_id) > 0
}

#[inline(always)]
pub fn has_items_with_keyword_editor_id<T>(container: &mut T, editor_id: &str) -> bool
where
    T: AsMut<TESObjectREFR> + ?Sized,
{
    has_items_with_keyword_editor_id_with_options(
        container,
        InventoryQueryOptions::default(),
        editor_id,
    )
}
