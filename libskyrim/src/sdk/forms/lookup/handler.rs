use crate::re::bs_core_types::FormID;
use crate::re::{FormCastable, TESDataHandler, TESFile, TESForm};
use crate::sdk::core::{GamePtr, GameRef};

use super::shared::{non_zero_form_id, trimmed_plugin_name};

/// Returns the live `TESDataHandler` singleton.
///
/// This is the low-level starting point behind the rest of the lookup helpers.
#[inline(always)]
pub fn data_handler() -> GameRef<TESDataHandler> {
    unsafe { GameRef::from_raw(TESDataHandler::get_singleton(true)) }
}

/// Returns the named plugin file regardless of load state.
#[inline(always)]
pub fn plugin_file(plugin_name: &str) -> GamePtr<TESFile> {
    let Some(plugin_name) = trimmed_plugin_name(plugin_name) else {
        return GamePtr::null();
    };
    unsafe { GamePtr::from_raw(data_handler().lookup_mod_by_name(plugin_name) as *mut TESFile) }
}

/// Returns the named plugin file only if it is loaded.
#[inline(always)]
pub fn loaded_plugin_file(plugin_name: &str) -> GamePtr<TESFile> {
    let Some(plugin_name) = trimmed_plugin_name(plugin_name) else {
        return GamePtr::null();
    };
    unsafe {
        GamePtr::from_raw(data_handler().lookup_loaded_mod_by_name(plugin_name) as *mut TESFile)
    }
}

/// Returns `true` when the named plugin file is currently loaded.
#[inline(always)]
pub fn plugin_loaded(plugin_name: &str) -> bool {
    loaded_plugin_file(plugin_name).is_some()
}

/// Returns the plugin index for the named file, if available.
#[inline(always)]
pub fn plugin_index(plugin_name: &str) -> Option<u8> {
    let plugin_name = trimmed_plugin_name(plugin_name)?;
    data_handler().get_mod_index(plugin_name)
}

/// Returns the loaded-plugin index for the named file, if available.
#[inline(always)]
pub fn loaded_plugin_index(plugin_name: &str) -> Option<u8> {
    let plugin_name = trimmed_plugin_name(plugin_name)?;
    data_handler().get_loaded_mod_index(plugin_name)
}

/// Expands a plugin-local form ID into a runtime form ID.
///
/// This is the ID-only variant of [`lookup_form`]. Use it when the plugin wants
/// to cache or compare form IDs rather than retrieve the form immediately.
#[inline(always)]
pub fn resolve_local_form_id(local_form_id: FormID, plugin_name: &str) -> Option<FormID> {
    let local_form_id = non_zero_form_id(local_form_id)?;
    let plugin_name = trimmed_plugin_name(plugin_name)?;
    let form_id = data_handler().lookup_form_id(local_form_id, plugin_name);
    if form_id == 0 { None } else { Some(form_id) }
}

/// Resolves a raw form ID against a plugin file without local-ID expansion.
#[inline(always)]
pub fn resolve_raw_form_id(raw_form_id: FormID, plugin_name: &str) -> Option<FormID> {
    let raw_form_id = non_zero_form_id(raw_form_id)?;
    let plugin_name = trimmed_plugin_name(plugin_name)?;
    let form_id = data_handler().lookup_form_id_raw(raw_form_id, plugin_name);
    if form_id == 0 { None } else { Some(form_id) }
}

/// Looks up a form from a plugin-local form ID plus plugin file name.
///
/// This is the normal `plugin + local form ID` path and therefore performs the
/// same local-ID expansion that `TESDataHandler` uses for ordinary mod forms.
#[inline(always)]
pub fn lookup_form(local_form_id: FormID, plugin_name: &str) -> GamePtr<TESForm> {
    let Some(local_form_id) = non_zero_form_id(local_form_id) else {
        return GamePtr::null();
    };
    let Some(plugin_name) = trimmed_plugin_name(plugin_name) else {
        return GamePtr::null();
    };
    unsafe { GamePtr::from_raw(data_handler().lookup_form(local_form_id, plugin_name)) }
}

/// Typed variant of [`lookup_form`].
#[inline(always)]
pub fn lookup_form_typed<T: FormCastable>(local_form_id: FormID, plugin_name: &str) -> GamePtr<T> {
    let Some(local_form_id) = non_zero_form_id(local_form_id) else {
        return GamePtr::null();
    };
    let Some(plugin_name) = trimmed_plugin_name(plugin_name) else {
        return GamePtr::null();
    };
    unsafe { GamePtr::from_raw(data_handler().lookup_form_typed::<T>(local_form_id, plugin_name)) }
}

/// Looks up a form from a raw form ID without local-ID expansion.
///
/// Use this when the plugin intentionally stores or receives raw runtime IDs
/// and should not reinterpret them as local IDs within the target plugin file.
#[inline(always)]
pub fn lookup_raw_form(raw_form_id: FormID, plugin_name: &str) -> GamePtr<TESForm> {
    let Some(raw_form_id) = non_zero_form_id(raw_form_id) else {
        return GamePtr::null();
    };
    let Some(plugin_name) = trimmed_plugin_name(plugin_name) else {
        return GamePtr::null();
    };
    unsafe { GamePtr::from_raw(data_handler().lookup_form_raw(raw_form_id, plugin_name)) }
}

/// Typed variant of [`lookup_raw_form`].
#[inline(always)]
pub fn lookup_raw_form_typed<T: FormCastable>(
    raw_form_id: FormID,
    plugin_name: &str,
) -> GamePtr<T> {
    let Some(raw_form_id) = non_zero_form_id(raw_form_id) else {
        return GamePtr::null();
    };
    let Some(plugin_name) = trimmed_plugin_name(plugin_name) else {
        return GamePtr::null();
    };
    unsafe {
        GamePtr::from_raw(data_handler().lookup_form_raw_typed::<T>(raw_form_id, plugin_name))
    }
}
