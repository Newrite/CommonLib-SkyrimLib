use crate::sdk::events::source::{
    EventInstallError, EventSubscription, IntoEventFlow, prepend_static, subscribe_static,
};

use super::DispatcherEvent;

/// Subscribes one closure to the SKSE dispatcher-backed event source for `E`.
///
/// This is the normal entrypoint for dispatcher families such as
/// `ModCallbackEvent`, `ActionEvent`, or `CrosshairRefEvent`.
#[inline(always)]
pub fn subscribe<E, F, R>(callback: F) -> Result<EventSubscription<'static, E>, EventInstallError>
where
    E: DispatcherEvent,
    F: FnMut(Option<&E>) -> R + 'static,
    R: IntoEventFlow,
{
    unsafe { subscribe_static(E::source(), callback) }
}

/// Subscribes one closure at the front of the dispatcher sink list for `E`.
///
/// Prefer this only when the listener must observe or stop propagation before
/// later listeners.
#[inline(always)]
pub fn prepend<E, F, R>(callback: F) -> Result<EventSubscription<'static, E>, EventInstallError>
where
    E: DispatcherEvent,
    F: FnMut(Option<&E>) -> R + 'static,
    R: IntoEventFlow,
{
    unsafe { prepend_static(E::source(), callback) }
}
