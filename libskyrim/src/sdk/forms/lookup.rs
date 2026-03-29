//! High-level form lookup helpers.
//!
//! This module gathers the frequent `TESDataHandler`, plugin-file, editor-ID,
//! and `"Plugin.esp|0x123"` workflows that repeatedly show up in gameplay and
//! config-driven plugins.

use core::ffi::c_char;

use crate::re::tes_form::FormID;
use crate::re::{FormCastable, FormType, TESDataHandler, TESFile, TESForm};
use crate::rex::W32::{GetModuleHandleW, GetProcAddress};
use crate::sdk::core::{GamePtr, GameRef};

type Po3GetFormEditorId = unsafe extern "C" fn(FormID) -> *const c_char;

/// Borrowed `"Plugin.esp|0x123"` style form spec.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PluginFormId<'a> {
    pub plugin_name: &'a str,
    pub form_id: FormID,
}

/// Parse failure for a plugin-qualified form spec.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParsePluginFormIdError {
    Empty,
    MissingSeparator,
    MissingPluginName,
    MissingFormId,
    InvalidFormId,
}

#[inline(always)]
pub fn data_handler() -> GameRef<TESDataHandler> {
    unsafe { GameRef::from_raw(TESDataHandler::get_singleton(true)) }
}

#[inline(always)]
pub fn plugin_file(plugin_name: &str) -> GamePtr<TESFile> {
    unsafe { GamePtr::from_raw(data_handler().lookup_mod_by_name(plugin_name) as *mut TESFile) }
}

#[inline(always)]
pub fn loaded_plugin_file(plugin_name: &str) -> GamePtr<TESFile> {
    unsafe {
        GamePtr::from_raw(data_handler().lookup_loaded_mod_by_name(plugin_name) as *mut TESFile)
    }
}

#[inline(always)]
pub fn plugin_loaded(plugin_name: &str) -> bool {
    loaded_plugin_file(plugin_name).is_some()
}

#[inline(always)]
pub fn plugin_index(plugin_name: &str) -> Option<u8> {
    data_handler().get_mod_index(plugin_name)
}

#[inline(always)]
pub fn loaded_plugin_index(plugin_name: &str) -> Option<u8> {
    data_handler().get_loaded_mod_index(plugin_name)
}

#[inline(always)]
pub fn resolve_form_id(local_form_id: FormID, plugin_name: &str) -> Option<FormID> {
    let form_id = data_handler().lookup_form_id(local_form_id, plugin_name);
    if form_id == 0 { None } else { Some(form_id) }
}

#[inline(always)]
pub fn resolve_raw_form_id(raw_form_id: FormID, plugin_name: &str) -> Option<FormID> {
    let form_id = data_handler().lookup_form_id_raw(raw_form_id, plugin_name);
    if form_id == 0 { None } else { Some(form_id) }
}

#[inline(always)]
pub fn lookup_form(local_form_id: FormID, plugin_name: &str) -> GamePtr<TESForm> {
    unsafe { GamePtr::from_raw(data_handler().lookup_form(local_form_id, plugin_name)) }
}

#[inline(always)]
pub fn lookup_form_typed<T: FormCastable>(local_form_id: FormID, plugin_name: &str) -> GamePtr<T> {
    unsafe { GamePtr::from_raw(data_handler().lookup_form_typed::<T>(local_form_id, plugin_name)) }
}

#[inline(always)]
pub fn lookup_form_raw(raw_form_id: FormID, plugin_name: &str) -> GamePtr<TESForm> {
    unsafe { GamePtr::from_raw(data_handler().lookup_form_raw(raw_form_id, plugin_name)) }
}

#[inline(always)]
pub fn lookup_form_raw_typed<T: FormCastable>(
    raw_form_id: FormID,
    plugin_name: &str,
) -> GamePtr<T> {
    unsafe {
        GamePtr::from_raw(data_handler().lookup_form_raw_typed::<T>(raw_form_id, plugin_name))
    }
}

#[inline(always)]
pub fn lookup_by_editor_id(editor_id: &str) -> GamePtr<TESForm> {
    let editor_id = editor_id.trim();
    let form = TESForm::lookup_by_editor_id(editor_id).unwrap_or(core::ptr::null_mut());
    unsafe { GamePtr::from_raw(form) }
}

#[inline(always)]
pub fn lookup_by_editor_id_typed<T: FormCastable>(editor_id: &str) -> GamePtr<T> {
    let form = lookup_by_editor_id(editor_id).as_ptr();
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
pub fn parse_plugin_form_id(spec: &str) -> Result<PluginFormId<'_>, ParsePluginFormIdError> {
    let spec = spec.trim();
    if spec.is_empty() {
        return Err(ParsePluginFormIdError::Empty);
    }

    let (plugin_name, form_id) = spec
        .split_once('|')
        .ok_or(ParsePluginFormIdError::MissingSeparator)?;

    let plugin_name = plugin_name.trim();
    if plugin_name.is_empty() {
        return Err(ParsePluginFormIdError::MissingPluginName);
    }

    let form_id = form_id.trim();
    if form_id.is_empty() {
        return Err(ParsePluginFormIdError::MissingFormId);
    }

    let form_id = parse_form_id_hex(form_id).ok_or(ParsePluginFormIdError::InvalidFormId)?;
    Ok(PluginFormId {
        plugin_name,
        form_id,
    })
}

#[inline]
pub fn form_from_string(spec: &str) -> GamePtr<TESForm> {
    if spec.contains('|') {
        match parse_plugin_form_id(spec) {
            Ok(parsed) => lookup_form(parsed.form_id, parsed.plugin_name),
            Err(_) => GamePtr::null(),
        }
    } else {
        lookup_by_editor_id(spec)
    }
}

#[inline]
pub fn form_from_string_typed<T: FormCastable>(spec: &str) -> GamePtr<T> {
    if spec.contains('|') {
        match parse_plugin_form_id(spec) {
            Ok(parsed) => lookup_form_typed::<T>(parsed.form_id, parsed.plugin_name),
            Err(_) => GamePtr::null(),
        }
    } else {
        lookup_by_editor_id_typed::<T>(spec)
    }
}

#[inline]
pub fn form_from_string_raw(spec: &str) -> GamePtr<TESForm> {
    if spec.contains('|') {
        match parse_plugin_form_id(spec) {
            Ok(parsed) => lookup_form_raw(parsed.form_id, parsed.plugin_name),
            Err(_) => GamePtr::null(),
        }
    } else {
        lookup_by_editor_id(spec)
    }
}

#[inline]
pub fn form_from_string_raw_typed<T: FormCastable>(spec: &str) -> GamePtr<T> {
    if spec.contains('|') {
        match parse_plugin_form_id(spec) {
            Ok(parsed) => lookup_form_raw_typed::<T>(parsed.form_id, parsed.plugin_name),
            Err(_) => GamePtr::null(),
        }
    } else {
        lookup_by_editor_id_typed::<T>(spec)
    }
}

#[inline]
fn parse_form_id_hex(value: &str) -> Option<FormID> {
    let value = value
        .strip_prefix("0x")
        .or_else(|| value.strip_prefix("0X"))
        .unwrap_or(value);
    u32::from_str_radix(value, 16).ok()
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
    use super::{ParsePluginFormIdError, native_editor_id_supported, parse_plugin_form_id};
    use crate::re::FormType;

    #[test]
    fn parses_plugin_form_id_with_hex_prefix() {
        let parsed = parse_plugin_form_id("Skyrim.esm|0x800").unwrap();
        assert_eq!(parsed.plugin_name, "Skyrim.esm");
        assert_eq!(parsed.form_id, 0x800);
    }

    #[test]
    fn parses_plugin_form_id_without_hex_prefix() {
        let parsed = parse_plugin_form_id("MyMod.esp|ABC").unwrap();
        assert_eq!(parsed.plugin_name, "MyMod.esp");
        assert_eq!(parsed.form_id, 0xABC);
    }

    #[test]
    fn rejects_missing_separator() {
        let error = parse_plugin_form_id("ActorTypeDragon").unwrap_err();
        assert_eq!(error, ParsePluginFormIdError::MissingSeparator);
    }

    #[test]
    fn rejects_empty_form_id() {
        let error = parse_plugin_form_id("Skyrim.esm|").unwrap_err();
        assert_eq!(error, ParsePluginFormIdError::MissingFormId);
    }

    #[test]
    fn native_editor_id_support_matches_expected_types() {
        assert!(native_editor_id_supported(FormType::Keyword));
        assert!(native_editor_id_supported(FormType::Quest));
        assert!(!native_editor_id_supported(FormType::Weapon));
        assert!(!native_editor_id_supported(FormType::Spell));
    }
}
