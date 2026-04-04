use crate::sdk::core::{GamePtr, HandleFamilyTarget, ResolvableHandle};

use super::types::{TaskHandoffOrigin, TaskQueueKind};

#[inline(always)]
pub(crate) fn queue_with_kind(queue: TaskQueueKind, f: impl FnOnce() + Send + 'static) {
    match queue {
        TaskQueueKind::Background => crate::skse::task::add_task(f),
        TaskQueueKind::Ui => crate::skse::task::add_ui_task(f),
    }
}

#[inline(always)]
pub(crate) fn capture_target_handle<T>(
    _caller: &str,
    _origin: TaskHandoffOrigin,
    target: GamePtr<T>,
) -> Option<T::Handle>
where
    T: HandleFamilyTarget,
{
    if target.is_null() {
        crate::defensive_sdk_warn!(
            "sdk::plugin::task::{}() rejected null {} handoff target",
            _caller,
            _origin.as_str()
        );
        return None;
    }

    let handle = T::canonical_handle(target.as_ptr());
    if handle.is_null() {
        crate::defensive_sdk_warn!(
            "sdk::plugin::task::{}() could not capture {} handoff target handle",
            _caller,
            _origin.as_str()
        );
        return None;
    }

    Some(handle)
}
