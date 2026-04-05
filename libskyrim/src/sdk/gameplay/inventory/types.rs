use alloc::boxed::Box;

use core::fmt;

use crate::re::{BGSKeyword, InventoryEntryData, TESBoundObject, TESForm};
use crate::sdk::core::GamePtr;

use super::shared::{bound_object_form, form_has_keyword, form_has_keyword_editor_id};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InventoryQueryOptions {
    /// Forwarded into source-backed inventory traversal to avoid initializing
    /// inventory state on demand.
    pub no_init: bool,
}

impl InventoryQueryOptions {
    /// Construct the default inventory-query behavior.
    #[inline(always)]
    pub const fn new() -> Self {
        Self { no_init: false }
    }

    /// Control whether traversal should skip `InventoryChanges` initialization.
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

/// Snapshot of one inventory entry plus a cloned `InventoryEntryData`.
///
/// This is the main value type for the inventory SDK layer. It keeps enough
/// information to support filtering, display, and lightweight mutation-style
/// decisions without forcing callers back into raw `InventoryEntryData`
/// traversal every time.
pub struct InventoryEntrySnapshot {
    pub(crate) object: GamePtr<TESBoundObject>,
    pub(crate) count: i32,
    pub(crate) entry: Box<InventoryEntryData>,
}

impl InventoryEntrySnapshot {
    /// Return the concrete inventory object for this entry.
    #[inline(always)]
    pub fn object(&self) -> GamePtr<TESBoundObject> {
        self.object
    }

    /// Return the entry as a generic form.
    #[inline(always)]
    pub fn form(&self) -> GamePtr<TESForm> {
        bound_object_form(self.object)
    }

    /// Attempt an RTTI-aware cast of the entry form.
    #[inline(always)]
    pub fn try_cast_form<T>(&self) -> GamePtr<T>
    where
        T: crate::relocation::RttiType,
    {
        self.form().try_cast::<T>()
    }

    /// Return the stacked item count for this entry.
    #[inline(always)]
    pub const fn count(&self) -> i32 {
        self.count
    }

    /// Borrow the cloned source-backed `InventoryEntryData`.
    #[inline(always)]
    pub fn entry(&self) -> &InventoryEntryData {
        &self.entry
    }

    /// Mutably borrow the cloned source-backed `InventoryEntryData`.
    #[inline(always)]
    pub fn entry_mut(&mut self) -> &mut InventoryEntryData {
        &mut self.entry
    }

    /// Return the form ID of the entry object, or `0` when missing.
    #[inline(always)]
    pub fn form_id(&self) -> u32 {
        self.form().map_or(0, TESForm::get_form_id)
    }

    /// Return the editor ID of the entry object, or an empty string.
    #[inline(always)]
    pub fn form_editor_id(&self) -> &str {
        core_util::ptr_to_str(
            self.form()
                .map_or(c"".as_ptr(), TESForm::get_form_editor_id),
        )
    }

    /// Return the display name exposed by the entry data.
    #[inline(always)]
    pub fn display_name(&mut self) -> &str {
        if self.object.is_null() {
            return "";
        }
        core_util::ptr_to_str(self.entry.get_display_name())
    }

    /// Return the owner reported by the entry data.
    #[inline(always)]
    pub fn owner(&mut self) -> GamePtr<TESForm> {
        if self.object.is_null() {
            return GamePtr::null();
        }
        unsafe { GamePtr::from_raw(self.entry.get_owner()) }
    }

    /// Return the per-item weight.
    #[inline(always)]
    pub fn weight(&self) -> f32 {
        if self.object.is_null() {
            return -1.0;
        }
        self.entry.get_weight()
    }

    /// Return the total stacked weight.
    #[inline(always)]
    pub fn stack_weight(&self) -> f32 {
        let weight = self.weight();
        if self.count <= 0 || weight <= 0.0 {
            0.0
        } else {
            weight * self.count as f32
        }
    }

    /// Return the per-item value.
    #[inline(always)]
    pub fn value(&self) -> i32 {
        if self.object.is_null() {
            return 0;
        }
        self.entry.get_value()
    }

    /// Return the total stacked value.
    #[inline(always)]
    pub fn stack_value(&self) -> i32 {
        self.value().saturating_mul(self.count.max(0))
    }

    /// Return the current enchantment charge, if the entry exposes one.
    #[inline(always)]
    pub fn enchantment_charge(&self) -> Option<f64> {
        if self.object.is_null() {
            return None;
        }
        self.entry.get_enchantment_charge()
    }

    /// Whether the entry object is armor.
    #[inline(always)]
    pub fn is_armor(&self) -> bool {
        self.form().map_or(false, TESForm::is_armor)
    }

    /// Whether the entry object is a weapon.
    #[inline(always)]
    pub fn is_weapon(&self) -> bool {
        self.form().map_or(false, TESForm::is_weapon)
    }

    /// Whether the entry object is ammo.
    #[inline(always)]
    pub fn is_ammo(&self) -> bool {
        self.form().map_or(false, TESForm::is_ammo)
    }

    /// Whether this entry is currently worn.
    #[inline(always)]
    pub fn is_worn(&self) -> bool {
        if self.object.is_null() {
            return false;
        }
        self.entry.is_worn()
    }

    /// Whether this entry is currently worn in the left hand.
    #[inline(always)]
    pub fn is_worn_left(&self) -> bool {
        if self.object.is_null() {
            return false;
        }
        self.entry.is_worn_on(true)
    }

    /// Whether this entry is currently worn in the right hand.
    #[inline(always)]
    pub fn is_worn_right(&self) -> bool {
        if self.object.is_null() {
            return false;
        }
        self.entry.is_worn_on(false)
    }

    /// Whether this entry is favorited.
    #[inline(always)]
    pub fn is_favorited(&self) -> bool {
        if self.object.is_null() {
            return false;
        }
        self.entry.is_favorited()
    }

    /// Whether this entry is enchanted.
    #[inline(always)]
    pub fn is_enchanted(&self) -> bool {
        if self.object.is_null() {
            return false;
        }
        self.entry.is_enchanted()
    }

    /// Whether this entry comes from leveled content.
    #[inline(always)]
    pub fn is_leveled(&self) -> bool {
        if self.object.is_null() {
            return false;
        }
        self.entry.is_leveled()
    }

    /// Whether this entry is poisoned.
    #[inline(always)]
    pub fn is_poisoned(&self) -> bool {
        if self.object.is_null() {
            return false;
        }
        self.entry.is_poisoned()
    }

    /// Whether this entry is marked as a quest object.
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
    pub fn has_keyword_with_editor_id(&self, editor_id: &str) -> bool {
        form_has_keyword_editor_id(self.form(), editor_id)
    }
}

impl fmt::Debug for InventoryEntrySnapshot {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("InventoryEntrySnapshot")
            .field("object", &format_args!("{:p}", self.object.as_ptr()))
            .field("form_id", &self.form_id())
            .field("editor_id", &self.form_editor_id())
            .field("count", &self.count)
            .field("worn", &self.is_worn())
            .field("favorited", &self.is_favorited())
            .field("quest_object", &self.is_quest_object())
            .finish()
    }
}
