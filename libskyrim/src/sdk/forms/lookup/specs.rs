use crate::re::bs_core_types::FormID;
use crate::re::{FormCastable, TESForm};
use crate::sdk::core::GamePtr;
use crate::sdk::forms::PersistentFormPtr;

use super::editor_ids::{lookup_editor_id, lookup_editor_id_typed};
use super::handler::{lookup_form, lookup_form_typed, lookup_raw_form, lookup_raw_form_typed};
use super::shared::{parse_form_id_hex, trimmed_lookup_spec};

/// Borrowed `"Plugin.esp|0x123"` style form spec.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PluginFormSpec<'a> {
    pub plugin_name: &'a str,
    pub form_id: FormID,
}

/// Parse failure for a plugin-qualified form spec.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParsePluginFormSpecError {
    Empty,
    MissingSeparator,
    MissingPluginName,
    MissingFormId,
    InvalidFormId,
}

#[inline]
pub fn parse_plugin_form_spec(spec: &str) -> Result<PluginFormSpec<'_>, ParsePluginFormSpecError> {
    let spec = spec.trim();
    if spec.is_empty() {
        return Err(ParsePluginFormSpecError::Empty);
    }

    let (plugin_name, form_id) = spec
        .split_once('|')
        .ok_or(ParsePluginFormSpecError::MissingSeparator)?;

    let plugin_name = plugin_name.trim();
    if plugin_name.is_empty() {
        return Err(ParsePluginFormSpecError::MissingPluginName);
    }

    let form_id = form_id.trim();
    if form_id.is_empty() {
        return Err(ParsePluginFormSpecError::MissingFormId);
    }

    let form_id = parse_form_id_hex(form_id).ok_or(ParsePluginFormSpecError::InvalidFormId)?;
    Ok(PluginFormSpec {
        plugin_name,
        form_id,
    })
}

#[inline]
fn lookup_spec_impl<T>(
    spec: &str,
    plugin_lookup: impl FnOnce(FormID, &str) -> GamePtr<T>,
    editor_id_lookup: impl FnOnce(&str) -> GamePtr<T>,
) -> GamePtr<T> {
    let Some(spec) = trimmed_lookup_spec(spec) else {
        return GamePtr::null();
    };

    if spec.contains('|') {
        match parse_plugin_form_spec(spec) {
            Ok(parsed) => plugin_lookup(parsed.form_id, parsed.plugin_name),
            Err(_) => {
                crate::defensive_sdk_warn!(
                    "libskyrim sdk::forms::lookup failed to parse plugin form spec"
                );
                GamePtr::null()
            }
        }
    } else {
        editor_id_lookup(spec)
    }
}

#[inline]
pub fn lookup_form_spec(spec: &str) -> GamePtr<TESForm> {
    lookup_spec_impl(spec, lookup_form, lookup_editor_id)
}

#[inline]
pub fn lookup_form_spec_typed<T: FormCastable>(spec: &str) -> GamePtr<T> {
    lookup_spec_impl(spec, lookup_form_typed::<T>, lookup_editor_id_typed::<T>)
}

#[inline]
pub fn lookup_persistent_form_spec<T: FormCastable>(spec: &str) -> PersistentFormPtr<T> {
    PersistentFormPtr::from_game_ptr(lookup_form_spec_typed::<T>(spec))
}

#[inline(always)]
pub fn require_persistent_form_spec<T: FormCastable>(
    spec: &str,
    context: &str,
) -> crate::sdk::forms::PersistentForm<T> {
    lookup_persistent_form_spec::<T>(spec).require(context)
}

#[inline]
pub fn lookup_raw_form_spec(spec: &str) -> GamePtr<TESForm> {
    lookup_spec_impl(spec, lookup_raw_form, lookup_editor_id)
}

#[inline]
pub fn lookup_raw_form_spec_typed<T: FormCastable>(spec: &str) -> GamePtr<T> {
    lookup_spec_impl(
        spec,
        lookup_raw_form_typed::<T>,
        lookup_editor_id_typed::<T>,
    )
}

#[cfg(test)]
mod tests {
    use super::{ParsePluginFormSpecError, parse_plugin_form_spec};

    #[test]
    fn parses_plugin_form_spec_with_hex_prefix() {
        let parsed = parse_plugin_form_spec("Skyrim.esm|0x800").unwrap();
        assert_eq!(parsed.plugin_name, "Skyrim.esm");
        assert_eq!(parsed.form_id, 0x800);
    }

    #[test]
    fn parses_plugin_form_spec_without_hex_prefix() {
        let parsed = parse_plugin_form_spec("MyMod.esp|ABC").unwrap();
        assert_eq!(parsed.plugin_name, "MyMod.esp");
        assert_eq!(parsed.form_id, 0xABC);
    }

    #[test]
    fn rejects_missing_separator() {
        let error = parse_plugin_form_spec("ActorTypeDragon").unwrap_err();
        assert_eq!(error, ParsePluginFormSpecError::MissingSeparator);
    }

    #[test]
    fn rejects_empty_form_id() {
        let error = parse_plugin_form_spec("Skyrim.esm|").unwrap_err();
        assert_eq!(error, ParsePluginFormSpecError::MissingFormId);
    }
}
