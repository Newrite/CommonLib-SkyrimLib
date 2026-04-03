use crate::re::{BSFixedString, ControlMap, INPUT_CONTEXT_ID, INPUT_DEVICE};

use super::access::control_map;
use super::shared::{validate_context_query, validate_device_query};

#[inline(always)]
pub fn mapped_key(event_id: &str, device: INPUT_DEVICE, context: INPUT_CONTEXT_ID) -> Option<u32> {
    if !validate_device_query("mapped_key", device) {
        return None;
    }
    if !validate_context_query("mapped_key", context) {
        return None;
    }

    let key = control_map().get_mapped_key(event_id, device, context);
    (key != ControlMap::kInvalid).then_some(key)
}

#[inline(always)]
pub fn button_name_from_user_event(event_id: &str, device: INPUT_DEVICE) -> Option<BSFixedString> {
    if !validate_device_query("button_name_from_user_event", device) {
        return None;
    }

    let event_id = BSFixedString::from_str(event_id);
    let mut button_name = BSFixedString::default();
    control_map()
        .get_button_name_from_user_event(&event_id, device, &mut button_name)
        .then_some(button_name)
}

#[inline(always)]
pub fn user_event_name(
    button_id: u32,
    device: INPUT_DEVICE,
    context: INPUT_CONTEXT_ID,
) -> Option<BSFixedString> {
    if !validate_device_query("user_event_name", device) {
        return None;
    }
    if !validate_context_query("user_event_name", context) {
        return None;
    }

    control_map()
        .get_user_event_name(button_id, device, context)
        .cloned()
}
