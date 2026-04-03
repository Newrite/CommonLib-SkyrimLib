use crate::re::bs_core_types::FormID;

#[inline]
pub(super) fn trimmed_plugin_name(plugin_name: &str) -> Option<&str> {
    let plugin_name = plugin_name.trim();
    if plugin_name.is_empty() {
        crate::defensive_sdk_warn!("libskyrim sdk::forms::lookup received an empty plugin name");
        None
    } else {
        Some(plugin_name)
    }
}

#[inline]
pub(super) fn trimmed_editor_id(editor_id: &str) -> Option<&str> {
    let editor_id = editor_id.trim();
    if editor_id.is_empty() {
        crate::defensive_sdk_warn!("libskyrim sdk::forms::lookup received an empty editor id");
        None
    } else {
        Some(editor_id)
    }
}

#[inline]
pub(super) fn trimmed_lookup_spec(spec: &str) -> Option<&str> {
    let spec = spec.trim();
    if spec.is_empty() {
        crate::defensive_sdk_warn!("libskyrim sdk::forms::lookup received an empty form spec");
        None
    } else {
        Some(spec)
    }
}

#[inline]
pub(super) fn non_zero_form_id(form_id: FormID) -> Option<FormID> {
    if form_id == 0 {
        crate::defensive_sdk_warn!("libskyrim sdk::forms::lookup received FormID 0");
        None
    } else {
        Some(form_id)
    }
}

#[inline]
pub(super) fn parse_form_id_hex(value: &str) -> Option<FormID> {
    let value = value
        .strip_prefix("0x")
        .or_else(|| value.strip_prefix("0X"))
        .unwrap_or(value);
    u32::from_str_radix(value, 16).ok()
}

#[cfg(test)]
mod tests {
    use super::{non_zero_form_id, trimmed_editor_id, trimmed_plugin_name};

    #[test]
    fn rejects_empty_plugin_name_in_normalizer() {
        assert_eq!(trimmed_plugin_name("   "), None);
        assert_eq!(trimmed_plugin_name(" Skyrim.esm "), Some("Skyrim.esm"));
    }

    #[test]
    fn rejects_empty_editor_id_in_normalizer() {
        assert_eq!(trimmed_editor_id("   "), None);
        assert_eq!(
            trimmed_editor_id(" ActorTypeDragon "),
            Some("ActorTypeDragon")
        );
    }

    #[test]
    fn rejects_zero_form_id() {
        assert_eq!(non_zero_form_id(0), None);
        assert_eq!(non_zero_form_id(0x123), Some(0x123));
    }
}
