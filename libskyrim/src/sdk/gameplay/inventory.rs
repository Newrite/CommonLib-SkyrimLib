//! Inventory and equipment helpers for common gameplay plugins.

use alloc::boxed::Box;
use alloc::vec::Vec;
use core::fmt;
use core::ops::ControlFlow;

use crate::re::{
    ActorExt, BGSKeyword, BipedObjectSlot, ITEM_REMOVE_REASON, InventoryEntryData, ObjectRefHandle,
    TESBoundObject, TESForm, TESObjectARMO, TESObjectREFR,
};
use crate::sdk::core::GamePtr;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InventoryQueryOptions {
    pub no_init: bool,
}

impl InventoryQueryOptions {
    #[inline(always)]
    pub const fn new() -> Self {
        Self { no_init: false }
    }

    #[inline(always)]
    pub const fn no_init(mut self, value: bool) -> Self {
        self.no_init = value;
        self
    }
}

impl Default for InventoryQueryOptions {
    #[inline(always)]
    fn default() -> Self {
        Self::new()
    }
}

pub struct InventoryEntrySnapshot {
    object: GamePtr<TESBoundObject>,
    count: i32,
    entry: Box<InventoryEntryData>,
}

impl InventoryEntrySnapshot {
    #[inline(always)]
    pub fn object(&self) -> GamePtr<TESBoundObject> {
        self.object
    }

    #[inline(always)]
    pub fn form(&self) -> GamePtr<TESForm> {
        bound_object_form(self.object)
    }

    #[inline(always)]
    pub fn try_cast_form<T>(&self) -> GamePtr<T>
    where
        T: crate::relocation::RttiType,
    {
        self.form().try_cast::<T>()
    }

    #[inline(always)]
    pub const fn count(&self) -> i32 {
        self.count
    }

    #[inline(always)]
    pub fn entry(&self) -> &InventoryEntryData {
        &self.entry
    }

    #[inline(always)]
    pub fn entry_mut(&mut self) -> &mut InventoryEntryData {
        &mut self.entry
    }

    #[inline(always)]
    pub fn form_id(&self) -> u32 {
        self.form().map_or(0, TESForm::get_form_id)
    }

    #[inline(always)]
    pub fn editor_id(&self) -> &str {
        core_util::ptr_to_str(
            self.form()
                .map_or(c"".as_ptr(), TESForm::get_form_editor_id),
        )
    }

    #[inline(always)]
    pub fn display_name(&mut self) -> &str {
        if self.object.is_null() {
            return "";
        }
        core_util::ptr_to_str(self.entry.get_display_name())
    }

    #[inline(always)]
    pub fn owner(&mut self) -> GamePtr<TESForm> {
        if self.object.is_null() {
            return GamePtr::null();
        }
        unsafe { GamePtr::from_raw(self.entry.get_owner()) }
    }

    #[inline(always)]
    pub fn weight(&self) -> f32 {
        if self.object.is_null() {
            return -1.0;
        }
        self.entry.get_weight()
    }

    #[inline(always)]
    pub fn stack_weight(&self) -> f32 {
        let weight = self.weight();
        if self.count <= 0 || weight <= 0.0 {
            0.0
        } else {
            weight * self.count as f32
        }
    }

    #[inline(always)]
    pub fn value(&self) -> i32 {
        if self.object.is_null() {
            return 0;
        }
        self.entry.get_value()
    }

    #[inline(always)]
    pub fn stack_value(&self) -> i32 {
        self.value().saturating_mul(self.count.max(0))
    }

    #[inline(always)]
    pub fn enchantment_charge(&self) -> Option<f64> {
        if self.object.is_null() {
            return None;
        }
        self.entry.get_enchantment_charge()
    }

    #[inline(always)]
    pub fn is_armor(&self) -> bool {
        self.form().map_or(false, TESForm::is_armor)
    }

    #[inline(always)]
    pub fn is_weapon(&self) -> bool {
        self.form().map_or(false, TESForm::is_weapon)
    }

    #[inline(always)]
    pub fn is_ammo(&self) -> bool {
        self.form().map_or(false, TESForm::is_ammo)
    }

    #[inline(always)]
    pub fn is_worn(&self) -> bool {
        if self.object.is_null() {
            return false;
        }
        self.entry.is_worn()
    }

    #[inline(always)]
    pub fn is_worn_left(&self) -> bool {
        if self.object.is_null() {
            return false;
        }
        self.entry.is_worn_on(true)
    }

    #[inline(always)]
    pub fn is_worn_right(&self) -> bool {
        if self.object.is_null() {
            return false;
        }
        self.entry.is_worn_on(false)
    }

    #[inline(always)]
    pub fn is_favorited(&self) -> bool {
        if self.object.is_null() {
            return false;
        }
        self.entry.is_favorited()
    }

    #[inline(always)]
    pub fn is_enchanted(&self) -> bool {
        if self.object.is_null() {
            return false;
        }
        self.entry.is_enchanted()
    }

    #[inline(always)]
    pub fn is_leveled(&self) -> bool {
        if self.object.is_null() {
            return false;
        }
        self.entry.is_leveled()
    }

    #[inline(always)]
    pub fn is_poisoned(&self) -> bool {
        if self.object.is_null() {
            return false;
        }
        self.entry.is_poisoned()
    }

    #[inline(always)]
    pub fn is_quest_object(&self) -> bool {
        if self.object.is_null() {
            return false;
        }
        self.entry.is_quest_object()
    }

    #[inline(always)]
    pub fn has_keyword(&self, keyword: &BGSKeyword) -> bool {
        form_has_keyword(self.form(), keyword)
    }

    #[inline(always)]
    pub fn has_keyword_editor_id(&self, editor_id: &str) -> bool {
        form_has_keyword_editor_id(self.form(), editor_id)
    }
}

impl fmt::Debug for InventoryEntrySnapshot {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("InventoryEntrySnapshot")
            .field("object", &format_args!("{:p}", self.object.as_ptr()))
            .field("form_id", &self.form_id())
            .field("editor_id", &self.editor_id())
            .field("count", &self.count)
            .field("worn", &self.is_worn())
            .field("favorited", &self.is_favorited())
            .field("quest_object", &self.is_quest_object())
            .finish()
    }
}

#[inline(always)]
fn bound_object_form(object: GamePtr<TESBoundObject>) -> GamePtr<TESForm> {
    unsafe { GamePtr::from_raw(object.as_ptr().cast::<TESForm>()) }
}

#[inline(always)]
fn form_has_keyword(form: GamePtr<TESForm>, keyword: &BGSKeyword) -> bool {
    form.map_or(false, |form| {
        form.has_keyword_in_array(&[keyword as *const BGSKeyword as *mut BGSKeyword], false)
    })
}

#[inline(always)]
fn form_has_keyword_editor_id(form: GamePtr<TESForm>, editor_id: &str) -> bool {
    form.map_or(false, |form| form.has_keyword_by_editor_id(editor_id))
}

#[inline(always)]
fn snapshot_from_inventory_entry(
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

fn for_each_snapshot_with<T>(
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
fn push_unique_form(forms: &mut Vec<GamePtr<TESForm>>, form: GamePtr<TESForm>) {
    if form.is_some() && !forms.contains(&form) {
        forms.push(form);
    }
}

#[inline(always)]
fn remove_item_raw(
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

pub fn for_each_inventory_entry_with<T>(
    container: &mut T,
    options: InventoryQueryOptions,
    visit: impl FnMut(&mut InventoryEntrySnapshot) -> ControlFlow<()>,
) -> ControlFlow<()>
where
    T: AsMut<TESObjectREFR> + ?Sized,
{
    for_each_snapshot_with(container, options, visit)
}

#[inline(always)]
pub fn for_each_inventory_entry<T>(
    container: &mut T,
    visit: impl FnMut(&mut InventoryEntrySnapshot) -> ControlFlow<()>,
) -> ControlFlow<()>
where
    T: AsMut<TESObjectREFR> + ?Sized,
{
    for_each_inventory_entry_with(container, InventoryQueryOptions::default(), visit)
}

pub fn collect_inventory_entries_with<T>(
    container: &mut T,
    options: InventoryQueryOptions,
) -> Vec<InventoryEntrySnapshot>
where
    T: AsMut<TESObjectREFR> + ?Sized,
{
    collect_inventory_entries_matching_with(container, options, |_| true)
}

#[inline(always)]
pub fn collect_inventory_entries<T>(container: &mut T) -> Vec<InventoryEntrySnapshot>
where
    T: AsMut<TESObjectREFR> + ?Sized,
{
    collect_inventory_entries_with(container, InventoryQueryOptions::default())
}

pub fn collect_inventory_entries_matching_with<T>(
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
            entries.push(InventoryEntrySnapshot {
                object: snapshot.object,
                count: snapshot.count,
                entry: core::mem::take(&mut snapshot.entry),
            });
        }
        ControlFlow::Continue(())
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
    collect_inventory_entries_matching_with(container, InventoryQueryOptions::default(), predicate)
}

pub fn find_inventory_entry_matching_with<T>(
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
            found = Some(InventoryEntrySnapshot {
                object: snapshot.object,
                count: snapshot.count,
                entry: core::mem::take(&mut snapshot.entry),
            });
            return ControlFlow::Break(());
        }
        ControlFlow::Continue(())
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
    find_inventory_entry_matching_with(container, InventoryQueryOptions::default(), predicate)
}

pub fn count_inventory_items_matching_with<T>(
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
        ControlFlow::Continue(())
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
    count_inventory_items_matching_with(container, InventoryQueryOptions::default(), predicate)
}

#[inline(always)]
pub fn count_inventory_items_with<T>(container: &mut T, options: InventoryQueryOptions) -> i32
where
    T: AsMut<TESObjectREFR> + ?Sized,
{
    count_inventory_items_matching_with(container, options, |_| true)
}

#[inline(always)]
pub fn count_inventory_items<T>(container: &mut T) -> i32
where
    T: AsMut<TESObjectREFR> + ?Sized,
{
    count_inventory_items_with(container, InventoryQueryOptions::default())
}

pub fn collect_inventory_objects_with<T>(
    container: &mut T,
    options: InventoryQueryOptions,
) -> Vec<GamePtr<TESBoundObject>>
where
    T: AsMut<TESObjectREFR> + ?Sized,
{
    collect_inventory_entries_with(container, options)
        .into_iter()
        .map(|snapshot| snapshot.object())
        .collect()
}

#[inline(always)]
pub fn collect_inventory_objects<T>(container: &mut T) -> Vec<GamePtr<TESBoundObject>>
where
    T: AsMut<TESObjectREFR> + ?Sized,
{
    collect_inventory_objects_with(container, InventoryQueryOptions::default())
}

pub fn collect_inventory_objects_matching_with<T>(
    container: &mut T,
    options: InventoryQueryOptions,
    predicate: impl FnMut(&mut InventoryEntrySnapshot) -> bool,
) -> Vec<GamePtr<TESBoundObject>>
where
    T: AsMut<TESObjectREFR> + ?Sized,
{
    collect_inventory_entries_matching_with(container, options, predicate)
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
    collect_inventory_objects_matching_with(container, InventoryQueryOptions::default(), predicate)
}

pub fn collect_typed_inventory_objects_with<TObject, TContainer>(
    container: &mut TContainer,
    options: InventoryQueryOptions,
) -> Vec<GamePtr<TObject>>
where
    TObject: crate::relocation::RttiType,
    TContainer: AsMut<TESObjectREFR> + ?Sized,
{
    collect_inventory_objects_with(container, options)
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
    collect_typed_inventory_objects_with(container, InventoryQueryOptions::default())
}

pub fn collect_typed_inventory_objects_matching_with<TObject, TContainer>(
    container: &mut TContainer,
    options: InventoryQueryOptions,
    predicate: impl FnMut(&mut InventoryEntrySnapshot) -> bool,
) -> Vec<GamePtr<TObject>>
where
    TObject: crate::relocation::RttiType,
    TContainer: AsMut<TESObjectREFR> + ?Sized,
{
    collect_inventory_objects_matching_with(container, options, predicate)
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
    collect_typed_inventory_objects_matching_with(
        container,
        InventoryQueryOptions::default(),
        predicate,
    )
}

pub fn find_typed_inventory_object_matching_with<TObject, TContainer>(
    container: &mut TContainer,
    options: InventoryQueryOptions,
    predicate: impl FnMut(&mut InventoryEntrySnapshot) -> bool,
) -> GamePtr<TObject>
where
    TObject: crate::relocation::RttiType,
    TContainer: AsMut<TESObjectREFR> + ?Sized,
{
    find_inventory_entry_matching_with(container, options, predicate)
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
    find_typed_inventory_object_matching_with(
        container,
        InventoryQueryOptions::default(),
        predicate,
    )
}

#[inline(always)]
pub fn count_item_with<T>(
    container: &mut T,
    options: InventoryQueryOptions,
    item: &TESBoundObject,
) -> i32
where
    T: AsMut<TESObjectREFR> + ?Sized,
{
    let item_ptr = item as *const TESBoundObject as *mut TESBoundObject;
    count_inventory_items_matching_with(container, options, |snapshot| {
        snapshot.object().as_ptr() == item_ptr
    })
}

#[inline(always)]
pub fn count_item<T>(container: &mut T, item: &TESBoundObject) -> i32
where
    T: AsMut<TESObjectREFR> + ?Sized,
{
    count_item_with(container, InventoryQueryOptions::default(), item)
}

#[inline(always)]
pub fn has_item_with<T>(
    container: &mut T,
    options: InventoryQueryOptions,
    item: &TESBoundObject,
) -> bool
where
    T: AsMut<TESObjectREFR> + ?Sized,
{
    count_item_with(container, options, item) > 0
}

#[inline(always)]
pub fn has_item<T>(container: &mut T, item: &TESBoundObject) -> bool
where
    T: AsMut<TESObjectREFR> + ?Sized,
{
    has_item_with(container, InventoryQueryOptions::default(), item)
}

#[inline(always)]
pub fn count_items_with_editor_id_with<T>(
    container: &mut T,
    options: InventoryQueryOptions,
    editor_id: &str,
) -> i32
where
    T: AsMut<TESObjectREFR> + ?Sized,
{
    count_inventory_items_matching_with(container, options, |snapshot| {
        snapshot.editor_id() == editor_id
    })
}

#[inline(always)]
pub fn count_items_with_editor_id<T>(container: &mut T, editor_id: &str) -> i32
where
    T: AsMut<TESObjectREFR> + ?Sized,
{
    count_items_with_editor_id_with(container, InventoryQueryOptions::default(), editor_id)
}

#[inline(always)]
pub fn has_item_with_editor_id_with<T>(
    container: &mut T,
    options: InventoryQueryOptions,
    editor_id: &str,
) -> bool
where
    T: AsMut<TESObjectREFR> + ?Sized,
{
    count_items_with_editor_id_with(container, options, editor_id) > 0
}

#[inline(always)]
pub fn has_item_with_editor_id<T>(container: &mut T, editor_id: &str) -> bool
where
    T: AsMut<TESObjectREFR> + ?Sized,
{
    has_item_with_editor_id_with(container, InventoryQueryOptions::default(), editor_id)
}

#[inline(always)]
pub fn find_inventory_entry_with_editor_id_with<T>(
    container: &mut T,
    options: InventoryQueryOptions,
    editor_id: &str,
) -> Option<InventoryEntrySnapshot>
where
    T: AsMut<TESObjectREFR> + ?Sized,
{
    find_inventory_entry_matching_with(container, options, |snapshot| {
        snapshot.editor_id() == editor_id
    })
}

#[inline(always)]
pub fn find_inventory_entry_with_editor_id<T>(
    container: &mut T,
    editor_id: &str,
) -> Option<InventoryEntrySnapshot>
where
    T: AsMut<TESObjectREFR> + ?Sized,
{
    find_inventory_entry_with_editor_id_with(container, InventoryQueryOptions::default(), editor_id)
}

#[inline(always)]
pub fn count_items_with_keyword_with<T>(
    container: &mut T,
    options: InventoryQueryOptions,
    keyword: &BGSKeyword,
) -> i32
where
    T: AsMut<TESObjectREFR> + ?Sized,
{
    count_inventory_items_matching_with(container, options, |snapshot| {
        snapshot.has_keyword(keyword)
    })
}

#[inline(always)]
pub fn count_items_with_keyword<T>(container: &mut T, keyword: &BGSKeyword) -> i32
where
    T: AsMut<TESObjectREFR> + ?Sized,
{
    count_items_with_keyword_with(container, InventoryQueryOptions::default(), keyword)
}

#[inline(always)]
pub fn has_item_with_keyword_with<T>(
    container: &mut T,
    options: InventoryQueryOptions,
    keyword: &BGSKeyword,
) -> bool
where
    T: AsMut<TESObjectREFR> + ?Sized,
{
    count_items_with_keyword_with(container, options, keyword) > 0
}

#[inline(always)]
pub fn has_item_with_keyword<T>(container: &mut T, keyword: &BGSKeyword) -> bool
where
    T: AsMut<TESObjectREFR> + ?Sized,
{
    has_item_with_keyword_with(container, InventoryQueryOptions::default(), keyword)
}

#[inline(always)]
pub fn count_items_with_keyword_editor_id_with<T>(
    container: &mut T,
    options: InventoryQueryOptions,
    editor_id: &str,
) -> i32
where
    T: AsMut<TESObjectREFR> + ?Sized,
{
    count_inventory_items_matching_with(container, options, |snapshot| {
        snapshot.has_keyword_editor_id(editor_id)
    })
}

#[inline(always)]
pub fn count_items_with_keyword_editor_id<T>(container: &mut T, editor_id: &str) -> i32
where
    T: AsMut<TESObjectREFR> + ?Sized,
{
    count_items_with_keyword_editor_id_with(container, InventoryQueryOptions::default(), editor_id)
}

#[inline(always)]
pub fn has_item_with_keyword_editor_id_with<T>(
    container: &mut T,
    options: InventoryQueryOptions,
    editor_id: &str,
) -> bool
where
    T: AsMut<TESObjectREFR> + ?Sized,
{
    count_items_with_keyword_editor_id_with(container, options, editor_id) > 0
}

#[inline(always)]
pub fn has_item_with_keyword_editor_id<T>(container: &mut T, editor_id: &str) -> bool
where
    T: AsMut<TESObjectREFR> + ?Sized,
{
    has_item_with_keyword_editor_id_with(container, InventoryQueryOptions::default(), editor_id)
}

#[inline(always)]
pub fn collect_worn_entries_with<T>(
    actor: &mut T,
    options: InventoryQueryOptions,
) -> Vec<InventoryEntrySnapshot>
where
    T: AsMut<TESObjectREFR> + ?Sized,
{
    collect_inventory_entries_matching_with(actor, options, |snapshot| snapshot.is_worn())
}

#[inline(always)]
pub fn collect_worn_entries<T>(actor: &mut T) -> Vec<InventoryEntrySnapshot>
where
    T: AsMut<TESObjectREFR> + ?Sized,
{
    collect_worn_entries_with(actor, InventoryQueryOptions::default())
}

pub fn collect_worn_armor_with<T>(
    actor: &mut T,
    options: InventoryQueryOptions,
) -> Vec<GamePtr<TESObjectARMO>>
where
    T: AsMut<TESObjectREFR> + ?Sized,
{
    collect_inventory_entries_matching_with(actor, options, |snapshot| {
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
    collect_worn_armor_with(actor, InventoryQueryOptions::default())
}

#[inline(always)]
pub fn worn_armor_in_slot_with<T>(
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
    worn_armor_in_slot_with(actor, slot, InventoryQueryOptions::default())
}

#[inline(always)]
pub fn has_worn_armor_in_slot_with<T>(
    actor: &mut T,
    slot: BipedObjectSlot,
    options: InventoryQueryOptions,
) -> bool
where
    T: ActorExt + ?Sized,
{
    worn_armor_in_slot_with(actor, slot, options).is_some()
}

#[inline(always)]
pub fn has_worn_armor_in_slot<T>(actor: &mut T, slot: BipedObjectSlot) -> bool
where
    T: ActorExt + ?Sized,
{
    has_worn_armor_in_slot_with(actor, slot, InventoryQueryOptions::default())
}

pub fn collect_equipped_forms_with<T>(
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

    for armor in collect_worn_armor_with(actor, options) {
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
    collect_equipped_forms_with(actor, InventoryQueryOptions::default())
}

#[inline(always)]
pub fn has_equipped_form_with<T>(
    actor: &mut T,
    form: &TESForm,
    options: InventoryQueryOptions,
) -> bool
where
    T: ActorExt + AsMut<TESObjectREFR> + ?Sized,
{
    let form_ptr = form as *const TESForm as *mut TESForm;
    collect_equipped_forms_with(actor, options)
        .into_iter()
        .any(|equipped| equipped.as_ptr() == form_ptr)
}

#[inline(always)]
pub fn has_equipped_form<T>(actor: &mut T, form: &TESForm) -> bool
where
    T: ActorExt + AsMut<TESObjectREFR> + ?Sized,
{
    has_equipped_form_with(actor, form, InventoryQueryOptions::default())
}

pub fn has_equipped_keyword_with<T>(
    actor: &mut T,
    keyword: &BGSKeyword,
    options: InventoryQueryOptions,
) -> bool
where
    T: ActorExt + AsMut<TESObjectREFR> + ?Sized,
{
    collect_equipped_forms_with(actor, options)
        .into_iter()
        .any(|form| form_has_keyword(form, keyword))
}

#[inline(always)]
pub fn has_equipped_keyword<T>(actor: &mut T, keyword: &BGSKeyword) -> bool
where
    T: ActorExt + AsMut<TESObjectREFR> + ?Sized,
{
    has_equipped_keyword_with(actor, keyword, InventoryQueryOptions::default())
}

pub fn has_equipped_keyword_editor_id_with<T>(
    actor: &mut T,
    editor_id: &str,
    options: InventoryQueryOptions,
) -> bool
where
    T: ActorExt + AsMut<TESObjectREFR> + ?Sized,
{
    collect_equipped_forms_with(actor, options)
        .into_iter()
        .any(|form| form_has_keyword_editor_id(form, editor_id))
}

#[inline(always)]
pub fn has_equipped_keyword_editor_id<T>(actor: &mut T, editor_id: &str) -> bool
where
    T: ActorExt + AsMut<TESObjectREFR> + ?Sized,
{
    has_equipped_keyword_editor_id_with(actor, editor_id, InventoryQueryOptions::default())
}

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

pub fn remove_inventory_entries_matching_with_reason<T>(
    container: &mut T,
    options: InventoryQueryOptions,
    reason: ITEM_REMOVE_REASON,
    predicate: impl FnMut(&mut InventoryEntrySnapshot) -> bool,
) -> i32
where
    T: AsMut<TESObjectREFR> + ?Sized,
{
    let entries = collect_inventory_entries_matching_with(container, options, predicate);
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
pub fn remove_inventory_entries_matching_with<T>(
    container: &mut T,
    options: InventoryQueryOptions,
    predicate: impl FnMut(&mut InventoryEntrySnapshot) -> bool,
) -> i32
where
    T: AsMut<TESObjectREFR> + ?Sized,
{
    remove_inventory_entries_matching_with_reason(
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
    remove_inventory_entries_matching_with(container, InventoryQueryOptions::default(), predicate)
}

pub fn transfer_inventory_entries_matching_with_reason<From, To>(
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
    let entries = collect_inventory_entries_matching_with(from, options, predicate);
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
pub fn transfer_inventory_entries_matching_with<From, To>(
    from: &mut From,
    to: &mut To,
    options: InventoryQueryOptions,
    predicate: impl FnMut(&mut InventoryEntrySnapshot) -> bool,
) -> i32
where
    From: AsMut<TESObjectREFR> + ?Sized,
    To: AsMut<TESObjectREFR> + ?Sized,
{
    transfer_inventory_entries_matching_with_reason(
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
    transfer_inventory_entries_matching_with(from, to, InventoryQueryOptions::default(), predicate)
}

#[cfg(test)]
mod tests {
    use alloc::boxed::Box;

    use super::{InventoryEntrySnapshot, InventoryQueryOptions};
    use crate::re::InventoryEntryData;
    use crate::sdk::core::GamePtr;

    #[test]
    fn query_options_default_to_initializing_inventory() {
        assert_eq!(InventoryQueryOptions::default().no_init, false);
        assert!(InventoryQueryOptions::new().no_init(true).no_init);
    }

    #[test]
    fn null_snapshot_reports_safe_defaults() {
        let mut snapshot = InventoryEntrySnapshot {
            object: GamePtr::null(),
            count: 3,
            entry: Box::new(InventoryEntryData::default()),
        };

        assert_eq!(snapshot.form_id(), 0);
        assert_eq!(snapshot.editor_id(), "");
        assert_eq!(snapshot.display_name(), "");
        assert_eq!(snapshot.weight(), -1.0);
        assert_eq!(snapshot.stack_weight(), 0.0);
        assert!(!snapshot.is_armor());
        assert!(!snapshot.is_worn());
        assert!(!snapshot.is_favorited());
    }
}
