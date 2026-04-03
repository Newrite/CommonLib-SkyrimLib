use crate::re::{INPUT_CONTEXT_ID, INPUT_DEVICE, USER_EVENT_FLAG};

#[inline(always)]
pub(super) fn is_valid_context(context: INPUT_CONTEXT_ID) -> bool {
    context != INPUT_CONTEXT_ID::kNone() && context.get() < INPUT_CONTEXT_ID::kTotal()
}

#[inline(always)]
pub(super) fn is_valid_device(device: INPUT_DEVICE) -> bool {
    let index = device as i32;
    index >= 0 && (index as u32) < INPUT_DEVICE::total()
}

#[inline(always)]
pub(super) fn has_invalid_control_flag(flags: USER_EVENT_FLAG) -> bool {
    (flags as u32 & USER_EVENT_FLAG::kInvalid as u32) != 0
}

#[inline(always)]
pub(super) fn validate_context(_caller: &str, context: INPUT_CONTEXT_ID) -> bool {
    if is_valid_context(context) {
        return true;
    }

    crate::defensive_sdk_warn!(
        "sdk::gameplay::input::{}() ignored invalid context id={}",
        _caller,
        context.get()
    );
    false
}

#[inline(always)]
pub(super) fn validate_context_query(_caller: &str, context: INPUT_CONTEXT_ID) -> bool {
    if is_valid_context(context) {
        return true;
    }

    crate::defensive_sdk_warn!(
        "sdk::gameplay::input::{}() queried invalid context id={}",
        _caller,
        context.get()
    );
    false
}

#[inline(always)]
pub(super) fn validate_device_query(_caller: &str, device: INPUT_DEVICE) -> bool {
    if is_valid_device(device) {
        return true;
    }

    crate::defensive_sdk_warn!(
        "sdk::gameplay::input::{}() queried invalid device={}",
        _caller,
        device as i32
    );
    false
}

#[inline(always)]
pub(super) fn warn_invalid_control_flags(_caller: &str, _flags: USER_EVENT_FLAG) {
    crate::defensive_sdk_warn!(
        "sdk::gameplay::input::{}() ignored invalid control flags=0x{:08X}",
        _caller,
        _flags as u32
    );
}

#[inline(always)]
pub(super) fn warn_skipped_invalid_control_flags(_caller: &str, _flags: USER_EVENT_FLAG) {
    crate::defensive_sdk_warn!(
        "sdk::gameplay::input::{}() skipped invalid control flags=0x{:08X}",
        _caller,
        _flags as u32
    );
}
