use crate::re::FormCastable;
use crate::sdk::core::GamePtr;

use super::lookup::lookup_editor_id_typed;
use super::persistent::{PersistentForm, PersistentFormPtr};

#[inline(always)]
pub(super) fn trimmed_non_empty(value: &str) -> Option<&str> {
    let value = value.trim();
    (!value.is_empty()).then_some(value)
}

#[inline(always)]
pub(super) fn game_ptr_from_ref<T>(value: &T) -> GamePtr<T> {
    unsafe { GamePtr::from_raw(core::ptr::from_ref(value).cast_mut()) }
}

#[inline(always)]
pub(super) fn lookup_persistent_editor_id_typed<T: FormCastable>(
    editor_id: &str,
) -> PersistentFormPtr<T> {
    PersistentFormPtr::from(lookup_editor_id_typed::<T>(editor_id))
}

#[inline(always)]
pub(super) fn require_persistent_editor_id_typed<T: FormCastable>(
    editor_id: &str,
    context: &str,
) -> PersistentForm<T> {
    lookup_persistent_editor_id_typed::<T>(editor_id).require(context)
}
