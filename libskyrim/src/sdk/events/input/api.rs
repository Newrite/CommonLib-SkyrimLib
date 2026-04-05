use crate::re::{BSInputDeviceManager, InputEvent};

use crate::sdk::events::source::{self, EventInstallError, EventSubscription, IntoEventFlow};

use super::InputEvents;

#[inline(always)]
fn manager_source() -> *mut crate::re::BSTEventSource<*mut InputEvent> {
    let manager = BSInputDeviceManager::get_singleton();
    if manager.is_null() {
        core::ptr::null_mut()
    } else {
        unsafe { &mut (*manager).event_source }
    }
}

/// Subscribe to `BSInputDeviceManager` input chains.
///
/// This is the ordinary entry point for plugins that want a borrowed
/// [`InputEvents`] view every time `BSInputDeviceManager` dispatches a fresh
/// `InputEvent*` chain.
pub fn subscribe<F, R>(
    callback: F,
) -> Result<EventSubscription<'static, *mut InputEvent>, EventInstallError>
where
    F: for<'a> FnMut(InputEvents<'a>) -> R + 'static,
    R: IntoEventFlow,
{
    let mut callback = callback;
    unsafe {
        source::subscribe_static(manager_source(), move |event| {
            let head = event.copied().unwrap_or(core::ptr::null_mut());
            callback(InputEvents::from_raw(head))
        })
    }
}

/// Subscribe to `BSInputDeviceManager` input chains at the front of the sink
/// list.
///
/// Use this when ordering matters and the plugin wants to see or mutate input
/// before later sinks in the dispatcher chain.
pub fn prepend<F, R>(
    callback: F,
) -> Result<EventSubscription<'static, *mut InputEvent>, EventInstallError>
where
    F: for<'a> FnMut(InputEvents<'a>) -> R + 'static,
    R: IntoEventFlow,
{
    let mut callback = callback;
    unsafe {
        source::prepend_static(manager_source(), move |event| {
            let head = event.copied().unwrap_or(core::ptr::null_mut());
            callback(InputEvents::from_raw(head))
        })
    }
}
