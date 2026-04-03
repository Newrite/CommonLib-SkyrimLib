use core::ffi::c_char;

use crate::re::bs_core_types::FormID;
use crate::re::{FormCastable, FormType, TESForm};
use crate::rex::W32::{GetModuleHandleW, GetProcAddress};
use crate::sdk::core::GamePtr;

use super::shared::trimmed_editor_id;

type Po3GetFormEditorId = unsafe extern "C" fn(FormID) -> *const c_char;

#[inline(always)]
pub fn lookup_editor_id(editor_id: &str) -> GamePtr<TESForm> {
    let Some(editor_id) = trimmed_editor_id(editor_id) else {
        return GamePtr::null();
    };
    let form = TESForm::lookup_by_editor_id(editor_id).unwrap_or(core::ptr::null_mut());
    unsafe { GamePtr::from_raw(form) }
}

#[inline(always)]
pub fn lookup_editor_id_typed<T: FormCastable>(editor_id: &str) -> GamePtr<T> {
    let form = lookup_editor_id(editor_id).as_ptr();
    if form.is_null() {
        return GamePtr::null();
    }

    if unsafe { (*form).is(T::TARGET_FORM_TYPE) } {
        unsafe { GamePtr::from_raw(form.cast::<T>()) }
    } else {
        GamePtr::null()
    }
}

#[inline(always)]
pub fn editor_id(form: &TESForm) -> &str {
    try_editor_id(form).unwrap_or("")
}

#[inline]
pub fn try_editor_id(form: &TESForm) -> Option<&str> {
    if native_editor_id_supported(form.get_form_type()) {
        let editor_id = form.get_form_editor_id_as_str();
        if editor_id.is_empty() {
            None
        } else {
            Some(editor_id)
        }
    } else {
        po3_editor_id(form.form_id)
    }
}

#[inline]
fn native_editor_id_supported(form_type: FormType) -> bool {
    matches!(
        form_type,
        FormType::Keyword
            | FormType::LocationRefType
            | FormType::Action
            | FormType::MenuIcon
            | FormType::Global
            | FormType::HeadPart
            | FormType::Race
            | FormType::Sound
            | FormType::Script
            | FormType::Navigation
            | FormType::Cell
            | FormType::WorldSpace
            | FormType::Land
            | FormType::NavMesh
            | FormType::Dialogue
            | FormType::Quest
            | FormType::Idle
            | FormType::AnimatedObject
            | FormType::ImageAdapter
            | FormType::VoiceType
            | FormType::Ragdoll
            | FormType::DefaultObject
            | FormType::MusicType
            | FormType::StoryManagerBranchNode
            | FormType::StoryManagerQuestNode
            | FormType::StoryManagerEventNode
    )
}

#[inline]
fn po3_editor_id(form_id: FormID) -> Option<&'static str> {
    if form_id == 0 {
        return None;
    }

    let module_name = core_util::create_utf16_string::<15>("po3_Tweaks.dll");
    let module = unsafe { GetModuleHandleW(module_name.as_ptr()) };
    if module.is_null() {
        return None;
    }

    let proc = unsafe { GetProcAddress(module, b"GetFormEditorID\0".as_ptr()) }?;
    let func: Po3GetFormEditorId = unsafe { core::mem::transmute(proc) };
    let editor_id = unsafe { func(form_id) };
    if editor_id.is_null() {
        None
    } else {
        let editor_id = core_util::ptr_to_str(editor_id);
        if editor_id.is_empty() {
            None
        } else {
            Some(editor_id)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::native_editor_id_supported;
    use crate::re::FormType;

    #[test]
    fn native_editor_id_support_matches_expected_types() {
        assert!(native_editor_id_supported(FormType::Keyword));
        assert!(native_editor_id_supported(FormType::Quest));
        assert!(!native_editor_id_supported(FormType::Weapon));
        assert!(!native_editor_id_supported(FormType::Spell));
    }
}
