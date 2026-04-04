use alloc::boxed::Box;

use core::fmt;

use crate::re::{BGSKeyword, InventoryEntryData, TESBoundObject, TESForm};
use crate::sdk::core::GamePtr;

use super::shared::{bound_object_form, form_has_keyword, form_has_keyword_editor_id};

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
    pub(crate) object: GamePtr<TESBoundObject>,
    pub(crate) count: i32,
    pub(crate) entry: Box<InventoryEntryData>,
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
    pub fn form_editor_id(&self) -> &str {
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
