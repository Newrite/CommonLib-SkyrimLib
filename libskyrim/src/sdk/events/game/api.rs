use crate::re::{ScriptEventSourceHolder, ScriptEventSourceHolderEvent};

use crate::sdk::events::source::{
    EventInstallError, EventSubscription, IntoEventFlow, prepend_static, subscribe_static,
};

/// Resolves the gameplay-owned `BSTEventSource<E>` from `ScriptEventSourceHolder`.
#[inline(always)]
fn event_source<E: ScriptEventSourceHolderEvent>() -> *mut crate::re::BSTEventSource<E> {
    let holder = ScriptEventSourceHolder::get_singleton();
    if holder.is_null() {
        core::ptr::null_mut()
    } else {
        unsafe { (*holder).get_event_source::<E>() }
    }
}

/// Subscribes one closure to the gameplay event source for `E`.
///
/// This is the usual high-level entrypoint when the plugin wants to observe
/// engine-owned gameplay events without manually resolving
/// `ScriptEventSourceHolder` or raw `BSTEventSource<T>` pointers.
#[inline(always)]
pub fn subscribe<E, F, R>(callback: F) -> Result<EventSubscription<'static, E>, EventInstallError>
where
    E: ScriptEventSourceHolderEvent,
    F: FnMut(Option<&E>) -> R + 'static,
    R: IntoEventFlow,
{
    unsafe { subscribe_static(event_source::<E>(), callback) }
}

/// Subscribes one closure at the front of the gameplay event sink list for `E`.
///
/// Prefer this only when the callback must observe or stop propagation before
/// later listeners.
#[inline(always)]
pub fn prepend<E, F, R>(callback: F) -> Result<EventSubscription<'static, E>, EventInstallError>
where
    E: ScriptEventSourceHolderEvent,
    F: FnMut(Option<&E>) -> R + 'static,
    R: IntoEventFlow,
{
    unsafe { prepend_static(event_source::<E>(), callback) }
}
