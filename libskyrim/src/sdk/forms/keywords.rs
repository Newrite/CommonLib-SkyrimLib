//! Keyword-centric form helpers.

use alloc::vec::Vec;
use core::ops::ControlFlow;

use crate::re::bs_core_types::FormID;
use crate::re::{BGSKeyword, BGSKeywordForm};
use crate::sdk::core::GamePtr;

use super::lookup::lookup_editor_id_typed;
use super::shared::{
    game_ptr_from_ref, lookup_persistent_editor_id_typed, require_persistent_editor_id_typed,
    trimmed_non_empty,
};
use super::{PersistentForm, PersistentFormPtr};

#[inline(always)]
pub fn keyword_editor_id(keyword: &BGSKeyword) -> &str {
    keyword.get_form_editor_id_as_str()
}

#[inline]
pub fn matches_keyword_editor_id(keyword: &BGSKeyword, editor_id: &str) -> bool {
    trimmed_non_empty(editor_id).is_some_and(|editor_id| keyword_editor_id(keyword) == editor_id)
}

#[inline(always)]
pub fn lookup_keyword_by_editor_id(editor_id: &str) -> GamePtr<BGSKeyword> {
    lookup_editor_id_typed::<BGSKeyword>(editor_id)
}

#[inline(always)]
pub fn lookup_persistent_keyword_by_editor_id(editor_id: &str) -> PersistentFormPtr<BGSKeyword> {
    lookup_persistent_editor_id_typed::<BGSKeyword>(editor_id)
}

#[inline(always)]
pub fn require_persistent_keyword_by_editor_id(
    editor_id: &str,
    context: &str,
) -> PersistentForm<BGSKeyword> {
    require_persistent_editor_id_typed::<BGSKeyword>(editor_id, context)
}

#[inline(always)]
pub fn count_keywords<T>(form: &T) -> usize
where
    T: AsRef<BGSKeywordForm> + ?Sized,
{
    form.as_ref().get_num_keywords() as usize
}

#[inline(always)]
pub fn default_keyword<T>(form: &T) -> GamePtr<BGSKeyword>
where
    T: AsRef<BGSKeywordForm> + ?Sized,
{
    unsafe { GamePtr::from_raw(form.as_ref().get_default_keyword()) }
}

pub fn for_each_keyword<T>(
    form: &T,
    mut visit: impl FnMut(&BGSKeyword) -> ControlFlow<()>,
) -> ControlFlow<()>
where
    T: AsRef<BGSKeywordForm> + ?Sized,
{
    for &keyword in form.as_ref().get_keywords() {
        let Some(keyword) = (unsafe { keyword.as_ref() }) else {
            continue;
        };

        let flow = visit(keyword);
        if flow.is_break() {
            return flow;
        }
    }

    ControlFlow::Continue(())
}

pub fn collect_keywords<T>(form: &T) -> Vec<GamePtr<BGSKeyword>>
where
    T: AsRef<BGSKeywordForm> + ?Sized,
{
    collect_keywords_matching(form, |_| true)
}

pub fn collect_keywords_matching<T>(
    form: &T,
    mut predicate: impl FnMut(&BGSKeyword) -> bool,
) -> Vec<GamePtr<BGSKeyword>>
where
    T: AsRef<BGSKeywordForm> + ?Sized,
{
    let mut keywords = Vec::new();
    for &keyword in form.as_ref().get_keywords() {
        let Some(keyword_ref) = (unsafe { keyword.as_ref() }) else {
            continue;
        };

        if predicate(keyword_ref) {
            keywords.push(game_ptr_from_ref(keyword_ref));
        }
    }
    keywords
}

pub fn find_keyword_matching<T>(
    form: &T,
    mut predicate: impl FnMut(&BGSKeyword) -> bool,
) -> GamePtr<BGSKeyword>
where
    T: AsRef<BGSKeywordForm> + ?Sized,
{
    for &keyword in form.as_ref().get_keywords() {
        let Some(keyword_ref) = (unsafe { keyword.as_ref() }) else {
            continue;
        };

        if predicate(keyword_ref) {
            return game_ptr_from_ref(keyword_ref);
        }
    }

    GamePtr::null()
}

#[inline(always)]
pub fn find_keyword<T>(form: &T, keyword: &BGSKeyword) -> GamePtr<BGSKeyword>
where
    T: AsRef<BGSKeywordForm> + ?Sized,
{
    find_keyword_matching(form, |candidate| core::ptr::eq(candidate, keyword))
}

#[inline(always)]
pub fn find_keyword_by_form_id<T>(form: &T, form_id: FormID) -> GamePtr<BGSKeyword>
where
    T: AsRef<BGSKeywordForm> + ?Sized,
{
    find_keyword_matching(form, |candidate| candidate.form_id == form_id)
}

#[inline(always)]
pub fn find_keyword_by_editor_id<T>(form: &T, editor_id: &str) -> GamePtr<BGSKeyword>
where
    T: AsRef<BGSKeywordForm> + ?Sized,
{
    find_keyword_matching(form, |candidate| {
        matches_keyword_editor_id(candidate, editor_id)
    })
}

#[inline(always)]
pub fn has_keyword<T>(form: &T, keyword: &BGSKeyword) -> bool
where
    T: AsRef<BGSKeywordForm> + ?Sized,
{
    find_keyword(form, keyword).is_some()
}

#[inline(always)]
pub fn has_keyword_with_form_id<T>(form: &T, form_id: FormID) -> bool
where
    T: AsRef<BGSKeywordForm> + ?Sized,
{
    find_keyword_by_form_id(form, form_id).is_some()
}

#[inline(always)]
pub fn has_keyword_with_editor_id<T>(form: &T, editor_id: &str) -> bool
where
    T: AsRef<BGSKeywordForm> + ?Sized,
{
    find_keyword_by_editor_id(form, editor_id).is_some()
}

pub fn has_any_keywords<T>(form: &T, keywords: &[&BGSKeyword]) -> bool
where
    T: AsRef<BGSKeywordForm> + ?Sized,
{
    keywords
        .iter()
        .copied()
        .any(|keyword| has_keyword(form, keyword))
}

pub fn has_all_keywords<T>(form: &T, keywords: &[&BGSKeyword]) -> bool
where
    T: AsRef<BGSKeywordForm> + ?Sized,
{
    keywords
        .iter()
        .copied()
        .all(|keyword| has_keyword(form, keyword))
}

pub fn has_any_keywords_with_editor_id<T>(form: &T, editor_ids: &[&str]) -> bool
where
    T: AsRef<BGSKeywordForm> + ?Sized,
{
    editor_ids
        .iter()
        .copied()
        .any(|editor_id| has_keyword_with_editor_id(form, editor_id))
}

pub fn has_all_keywords_with_editor_id<T>(form: &T, editor_ids: &[&str]) -> bool
where
    T: AsRef<BGSKeywordForm> + ?Sized,
{
    editor_ids
        .iter()
        .copied()
        .all(|editor_id| has_keyword_with_editor_id(form, editor_id))
}

#[inline(always)]
pub fn add_keyword<T>(form: &mut T, keyword: &BGSKeyword) -> bool
where
    T: AsRef<BGSKeywordForm> + AsMut<BGSKeywordForm> + ?Sized,
{
    form.as_mut()
        .add_keyword(core::ptr::from_ref(keyword).cast_mut())
}

pub fn add_keywords<T>(form: &mut T, keywords: &[&BGSKeyword]) -> bool
where
    T: AsRef<BGSKeywordForm> + AsMut<BGSKeywordForm> + ?Sized,
{
    let keywords = keyword_refs_to_raw(keywords);
    form.as_mut().add_keywords(&keywords)
}

#[inline(always)]
pub fn remove_keyword<T>(form: &mut T, keyword: &BGSKeyword) -> bool
where
    T: AsRef<BGSKeywordForm> + AsMut<BGSKeywordForm> + ?Sized,
{
    form.as_mut()
        .remove_keyword(core::ptr::from_ref(keyword).cast_mut())
}

pub fn remove_keywords<T>(form: &mut T, keywords: &[&BGSKeyword]) -> bool
where
    T: AsRef<BGSKeywordForm> + AsMut<BGSKeywordForm> + ?Sized,
{
    let keywords = keyword_refs_to_raw(keywords);
    form.as_mut().remove_keywords(&keywords)
}

pub fn remove_keywords_matching<T>(
    form: &mut T,
    predicate: impl FnMut(&BGSKeyword) -> bool,
) -> usize
where
    T: AsRef<BGSKeywordForm> + AsMut<BGSKeywordForm> + ?Sized,
{
    let keywords = collect_keywords_matching(form, predicate);
    if keywords.is_empty() {
        return 0;
    }

    let raw_keywords = keyword_ptrs_to_raw(&keywords);
    if form.as_mut().remove_keywords(&raw_keywords) {
        raw_keywords.len()
    } else {
        0
    }
}

#[inline(always)]
pub fn clear_keywords<T>(form: &mut T) -> usize
where
    T: AsRef<BGSKeywordForm> + AsMut<BGSKeywordForm> + ?Sized,
{
    remove_keywords_matching(form, |_| true)
}

#[inline(always)]
pub fn set_keyword_enabled<T>(form: &mut T, keyword: &BGSKeyword, enabled: bool) -> bool
where
    T: AsRef<BGSKeywordForm> + AsMut<BGSKeywordForm> + ?Sized,
{
    if enabled {
        add_keyword(form, keyword)
    } else {
        remove_keyword(form, keyword)
    }
}

fn keyword_refs_to_raw(keywords: &[&BGSKeyword]) -> Vec<*mut BGSKeyword> {
    keywords
        .iter()
        .copied()
        .map(|keyword| core::ptr::from_ref(keyword).cast_mut())
        .collect()
}

fn keyword_ptrs_to_raw(keywords: &[GamePtr<BGSKeyword>]) -> Vec<*mut BGSKeyword> {
    keywords.iter().copied().map(GamePtr::as_ptr).collect()
}

#[cfg(test)]
mod tests {
    use super::{
        collect_keywords, count_keywords, find_keyword_by_editor_id, for_each_keyword,
        has_all_keywords_with_editor_id, has_any_keywords, has_keyword, has_keyword_with_editor_id,
        matches_keyword_editor_id,
    };
    use crate::re::base_form_component::BaseFormComponent;
    use crate::re::bs_fixed_string::BSFixedString;
    use crate::re::{
        BGSKeyword, BGSKeywordForm, FormType, InGameFormFlag, RecordFlag, TESFileContainer, TESForm,
    };
    use alloc::boxed::Box;
    use alloc::vec::Vec;
    use core::ops::ControlFlow;
    use core_util::EnumSet;

    fn test_form(form_id: u32) -> TESForm {
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
            form_type: EnumSet::from(FormType::Keyword),
            pad1b: 0,
            pad1c: 0,
        }
    }

    fn test_keyword(editor_id: &str, form_id: u32) -> *mut BGSKeyword {
        Box::into_raw(Box::new(BGSKeyword {
            base: test_form(form_id),
            form_editor_id: BSFixedString::from_str(editor_id),
        }))
    }

    fn keyword_form_from_slice(keywords: &mut [*mut BGSKeyword]) -> BGSKeywordForm {
        BGSKeywordForm {
            base: BaseFormComponent {
                vtable: core::ptr::null(),
            },
            keywords: if keywords.is_empty() {
                core::ptr::null_mut()
            } else {
                keywords.as_mut_ptr()
            },
            num_keywords: keywords.len() as u32,
            pad14: 0,
        }
    }

    #[test]
    fn keyword_queries_work_on_keyword_form_slice() {
        let fire = test_keyword("MagicDamageFire", 0x0100);
        let frost = test_keyword("MagicDamageFrost", 0x0101);
        let fire_ref = unsafe { &*fire };
        let mut raw_keywords = [fire, frost];
        let keyword_form = keyword_form_from_slice(&mut raw_keywords);

        assert_eq!(count_keywords(&keyword_form), 2);
        assert!(matches_keyword_editor_id(fire_ref, "MagicDamageFire"));
        assert!(!matches_keyword_editor_id(fire_ref, " "));
        assert!(has_keyword(&keyword_form, fire_ref));
        assert!(has_keyword_with_editor_id(
            &keyword_form,
            "MagicDamageFrost"
        ));
        assert!(has_any_keywords(&keyword_form, &[fire_ref]));
        assert!(has_all_keywords_with_editor_id(
            &keyword_form,
            &["MagicDamageFire", "MagicDamageFrost"]
        ));

        let snapshot = collect_keywords(&keyword_form);
        assert_eq!(snapshot.len(), 2);
        assert_eq!(snapshot[0].as_ptr(), fire);
        assert_eq!(
            find_keyword_by_editor_id(&keyword_form, "MagicDamageFire").as_ptr(),
            fire
        );
    }

    #[test]
    fn keyword_iteration_skips_null_entries() {
        let keyword = test_keyword("ArmorLight", 0x0200);
        let mut raw_keywords = [core::ptr::null_mut(), keyword];
        let keyword_form = keyword_form_from_slice(&mut raw_keywords);

        let mut visited = Vec::new();
        let flow = for_each_keyword(&keyword_form, |entry| {
            visited.push(entry.form_id);
            ControlFlow::Continue(())
        });

        assert!(flow.is_continue());
        assert_eq!(visited, [0x0200]);
    }
}
