//! Helpers for list-like form domains.
//!
//! The current SDK surface is centered on `BGSListForm`: lookup, iteration,
//! snapshots, membership checks, typed collection, and append/ensure helpers.

use alloc::vec::Vec;
use core::ops::ControlFlow;

use crate::re::bs_core_types::FormID;
use crate::re::{BGSListForm, FormCastable, FormType, TESForm};
use crate::relocation::RttiType;
use crate::sdk::core::GamePtr;

use super::lookup::{lookup_editor_id_typed, try_editor_id};
use super::shared::{
    game_ptr_from_ref, lookup_persistent_editor_id_typed, require_persistent_editor_id_typed,
    trimmed_non_empty,
};
use super::{PersistentForm, PersistentFormPtr};

#[inline]
pub fn matches_form_editor_id(form: &TESForm, editor_id: &str) -> bool {
    trimmed_non_empty(editor_id).is_some_and(|editor_id| try_editor_id(form) == Some(editor_id))
}

#[inline(always)]
pub fn lookup_list_editor_id(editor_id: &str) -> GamePtr<BGSListForm> {
    lookup_editor_id_typed::<BGSListForm>(editor_id)
}

#[inline(always)]
pub fn lookup_persistent_list_editor_id(editor_id: &str) -> PersistentFormPtr<BGSListForm> {
    lookup_persistent_editor_id_typed::<BGSListForm>(editor_id)
}

#[inline(always)]
pub fn require_persistent_list_editor_id(
    editor_id: &str,
    context: &str,
) -> PersistentForm<BGSListForm> {
    require_persistent_editor_id_typed::<BGSListForm>(editor_id, context)
}

pub fn count_forms<T>(list: &T) -> usize
where
    T: AsRef<BGSListForm> + ?Sized,
{
    let mut count = 0;
    let _ = for_each_form(list, |_| {
        count += 1;
        ControlFlow::Continue(())
    });
    count
}

pub fn for_each_form<T>(
    list: &T,
    mut visit: impl FnMut(&TESForm) -> ControlFlow<()>,
) -> ControlFlow<()>
where
    T: AsRef<BGSListForm> + ?Sized,
{
    let mut flow = ControlFlow::Continue(());
    list.as_ref().for_each_form(|form| {
        let Some(form) = (unsafe { form.as_ref() }) else {
            return crate::re::BSContainerForEachResult::Continue;
        };

        flow = visit(form);
        if flow.is_break() {
            crate::re::BSContainerForEachResult::Stop
        } else {
            crate::re::BSContainerForEachResult::Continue
        }
    });
    flow
}

pub fn collect_forms<T>(list: &T) -> Vec<GamePtr<TESForm>>
where
    T: AsRef<BGSListForm> + ?Sized,
{
    collect_forms_matching(list, |_| true)
}

pub fn collect_forms_matching<T>(
    list: &T,
    mut predicate: impl FnMut(&TESForm) -> bool,
) -> Vec<GamePtr<TESForm>>
where
    T: AsRef<BGSListForm> + ?Sized,
{
    let mut forms = Vec::new();
    let _ = for_each_form(list, |form| {
        if predicate(form) {
            forms.push(game_ptr_from_ref(form));
        }
        ControlFlow::Continue(())
    });
    forms
}

pub fn collect_typed_forms<T, U>(list: &T) -> Vec<GamePtr<U>>
where
    T: AsRef<BGSListForm> + ?Sized,
    U: FormCastable + RttiType,
{
    collect_typed_forms_matching::<T, U>(list, |_| true)
}

pub fn collect_typed_forms_matching<T, U>(
    list: &T,
    mut predicate: impl FnMut(&U) -> bool,
) -> Vec<GamePtr<U>>
where
    T: AsRef<BGSListForm> + ?Sized,
    U: FormCastable + RttiType,
{
    let mut forms = Vec::new();
    let _ = for_each_form(list, |form| {
        let typed = game_ptr_from_ref(form).try_cast::<U>();
        let Some(typed_ref) = typed.as_ref() else {
            return ControlFlow::Continue(());
        };

        if predicate(typed_ref) {
            forms.push(typed);
        }
        ControlFlow::Continue(())
    });
    forms
}

pub fn find_form_matching<T>(
    list: &T,
    mut predicate: impl FnMut(&TESForm) -> bool,
) -> GamePtr<TESForm>
where
    T: AsRef<BGSListForm> + ?Sized,
{
    let mut found = GamePtr::null();
    let _ = for_each_form(list, |form| {
        if predicate(form) {
            found = game_ptr_from_ref(form);
            ControlFlow::Break(())
        } else {
            ControlFlow::Continue(())
        }
    });
    found
}

pub fn find_typed_form_matching<T, U>(list: &T, mut predicate: impl FnMut(&U) -> bool) -> GamePtr<U>
where
    T: AsRef<BGSListForm> + ?Sized,
    U: FormCastable + RttiType,
{
    let mut found = GamePtr::null();
    let _ = for_each_form(list, |form| {
        let typed = game_ptr_from_ref(form).try_cast::<U>();
        let Some(typed_ref) = typed.as_ref() else {
            return ControlFlow::Continue(());
        };

        if predicate(typed_ref) {
            found = typed;
            ControlFlow::Break(())
        } else {
            ControlFlow::Continue(())
        }
    });
    found
}

#[inline(always)]
pub fn find_form<T>(list: &T, form: &TESForm) -> GamePtr<TESForm>
where
    T: AsRef<BGSListForm> + ?Sized,
{
    find_form_matching(list, |candidate| core::ptr::eq(candidate, form))
}

#[inline(always)]
pub fn find_form_id<T>(list: &T, form_id: FormID) -> GamePtr<TESForm>
where
    T: AsRef<BGSListForm> + ?Sized,
{
    find_form_matching(list, |candidate| candidate.form_id == form_id)
}

#[inline(always)]
pub fn find_form_editor_id<T>(list: &T, editor_id: &str) -> GamePtr<TESForm>
where
    T: AsRef<BGSListForm> + ?Sized,
{
    find_form_matching(list, |candidate| {
        matches_form_editor_id(candidate, editor_id)
    })
}

#[inline(always)]
pub fn has_form<T>(list: &T, form: &TESForm) -> bool
where
    T: AsRef<BGSListForm> + ?Sized,
{
    list.as_ref().has_form(core::ptr::from_ref(form))
}

#[inline(always)]
pub fn has_form_id<T>(list: &T, form_id: FormID) -> bool
where
    T: AsRef<BGSListForm> + ?Sized,
{
    find_form_id(list, form_id).is_some()
}

#[inline(always)]
pub fn has_form_editor_id<T>(list: &T, editor_id: &str) -> bool
where
    T: AsRef<BGSListForm> + ?Sized,
{
    find_form_editor_id(list, editor_id).is_some()
}

pub fn has_any_forms<T>(list: &T, forms: &[&TESForm]) -> bool
where
    T: AsRef<BGSListForm> + ?Sized,
{
    forms.iter().copied().any(|form| has_form(list, form))
}

pub fn has_all_forms<T>(list: &T, forms: &[&TESForm]) -> bool
where
    T: AsRef<BGSListForm> + ?Sized,
{
    forms.iter().copied().all(|form| has_form(list, form))
}

pub fn has_any_form_ids<T>(list: &T, form_ids: &[FormID]) -> bool
where
    T: AsRef<BGSListForm> + ?Sized,
{
    form_ids
        .iter()
        .copied()
        .any(|form_id| has_form_id(list, form_id))
}

pub fn has_all_form_ids<T>(list: &T, form_ids: &[FormID]) -> bool
where
    T: AsRef<BGSListForm> + ?Sized,
{
    form_ids
        .iter()
        .copied()
        .all(|form_id| has_form_id(list, form_id))
}

#[inline(always)]
pub fn contains_only_form_type<T>(list: &T, form_type: FormType) -> bool
where
    T: AsRef<BGSListForm> + ?Sized,
{
    list.as_ref().contains_only_type(form_type)
}

#[inline(always)]
pub fn contains_only_type<T, U>(list: &T) -> bool
where
    T: AsRef<BGSListForm> + ?Sized,
    U: FormCastable,
{
    contains_only_form_type(list, U::TARGET_FORM_TYPE)
}

#[inline(always)]
pub fn append_form<T>(list: &mut T, form: &TESForm)
where
    T: AsRef<BGSListForm> + AsMut<BGSListForm> + ?Sized,
{
    list.as_mut().add_form(core::ptr::from_ref(form).cast_mut())
}

pub fn append_forms<T>(list: &mut T, forms: &[&TESForm])
where
    T: AsRef<BGSListForm> + AsMut<BGSListForm> + ?Sized,
{
    for &form in forms {
        append_form(list, form);
    }
}

pub fn ensure_form<T>(list: &mut T, form: &TESForm) -> bool
where
    T: AsRef<BGSListForm> + AsMut<BGSListForm> + ?Sized,
{
    if has_form(list, form) {
        false
    } else {
        append_form(list, form);
        true
    }
}

pub fn ensure_forms<T>(list: &mut T, forms: &[&TESForm]) -> usize
where
    T: AsRef<BGSListForm> + AsMut<BGSListForm> + ?Sized,
{
    let mut added = 0;
    for &form in forms {
        if ensure_form(list, form) {
            added += 1;
        }
    }
    added
}

#[cfg(test)]
mod tests {
    use super::{
        collect_forms, contains_only_form_type, count_forms, find_form_id, for_each_form,
        has_all_form_ids, has_any_forms, has_form, has_form_id,
    };
    use crate::re::base_form_component::BaseFormComponent;
    use crate::re::bst_array::BSTArray;
    use crate::re::{BGSListForm, FormType, InGameFormFlag, RecordFlag, TESFileContainer, TESForm};
    use alloc::vec::Vec;
    use core::ops::ControlFlow;
    use core_util::EnumSet;

    fn test_form(form_id: u32, form_type: FormType) -> TESForm {
        TESForm {
            base: BaseFormComponent {
                vtable: core::ptr::null(),
            },
            source_files: TESFileContainer {
                array: core::ptr::null_mut(),
            },
            form_flags: RecordFlag::empty(),
            form_id,
            in_game_form_flags: EnumSet::<InGameFormFlag, u16>::from_underlying(0),
            form_type: EnumSet::from(form_type),
            pad1b: 0,
            pad1c: 0,
        }
    }

    fn list_from_forms(forms: impl IntoIterator<Item = *mut TESForm>) -> BGSListForm {
        BGSListForm {
            base: unsafe { core::mem::zeroed::<TESForm>() },
            forms: forms.into_iter().collect::<BSTArray<*mut TESForm>>(),
            script_added_temp_forms: core::ptr::null_mut(),
            script_added_form_count: 0,
            pad44: 0,
        }
    }

    #[test]
    fn list_queries_work_on_static_entries() {
        let mut spell = test_form(0x0100, FormType::Spell);
        let mut keyword = test_form(0x0101, FormType::Keyword);
        let list = list_from_forms([
            &mut spell as *mut TESForm,
            core::ptr::null_mut(),
            &mut keyword as *mut TESForm,
        ]);

        assert_eq!(count_forms(&list), 2);
        assert!(has_form(&list, &spell));
        assert!(has_form_id(&list, 0x0101));
        assert!(has_any_forms(&list, &[&spell]));
        assert!(has_all_form_ids(&list, &[0x0100, 0x0101]));

        let snapshot = collect_forms(&list);
        assert_eq!(snapshot.len(), 2);
        assert_eq!(snapshot[0].as_ptr(), &mut spell as *mut TESForm);
        assert_eq!(snapshot[1].as_ptr(), &mut keyword as *mut TESForm);
        assert_eq!(
            find_form_id(&list, 0x0101).as_ptr(),
            &mut keyword as *mut TESForm
        );
    }

    #[test]
    fn type_queries_respect_form_type_storage() {
        let mut spell = test_form(0x0200, FormType::Spell);
        let mut keyword = test_form(0x0201, FormType::Keyword);
        let mut list = list_from_forms([&mut spell as *mut TESForm]);

        assert!(contains_only_form_type(&list, FormType::Spell));

        list.forms = [&mut spell as *mut TESForm, &mut keyword as *mut TESForm]
            .into_iter()
            .collect::<BSTArray<*mut TESForm>>();

        assert!(!contains_only_form_type(&list, FormType::Spell));
    }

    #[test]
    fn iteration_skips_null_entries() {
        let mut spell = test_form(0x0300, FormType::Spell);
        let list = list_from_forms([core::ptr::null_mut(), &mut spell as *mut TESForm]);

        let mut visited = Vec::new();
        let flow = for_each_form(&list, |form| {
            visited.push(form.form_id);
            ControlFlow::Continue(())
        });

        assert!(flow.is_continue());
        assert_eq!(visited, [0x0300]);
    }
}
