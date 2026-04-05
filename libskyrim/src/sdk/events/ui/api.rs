use crate::re::{UI, UIEventSourceEvent};

use crate::sdk::events::source::{
    EventInstallError, EventSubscription, IntoEventFlow, prepend_static, subscribe_static,
};

/// Resolves the UI-owned `BSTEventSource<E>` from the `UI` singleton.
#[inline(always)]
fn event_source<E: UIEventSourceEvent>() -> *mut crate::re::BSTEventSource<E> {
    let ui = UI::get_singleton();
    if ui.is_null() {
        core::ptr::null_mut()
    } else {
        unsafe { (*ui).get_event_source::<E>() }
    }
}

/// Subscribes one closure to the UI event source for `E`.
///
/// This is the default entrypoint for menu and HUD oriented engine event
/// families.
#[inline(always)]
pub fn subscribe<E, F, R>(callback: F) -> Result<EventSubscription<'static, E>, EventInstallError>
where
    E: UIEventSourceEvent,
    F: FnMut(Option<&E>) -> R + 'static,
    R: IntoEventFlow,
{
    unsafe { subscribe_static(event_source::<E>(), callback) }
}

/// Subscribes one closure at the front of the UI event sink list for `E`.
///
/// Prefer this only when the callback must run before later listeners.
#[inline(always)]
pub fn prepend<E, F, R>(callback: F) -> Result<EventSubscription<'static, E>, EventInstallError>
where
    E: UIEventSourceEvent,
    F: FnMut(Option<&E>) -> R + 'static,
    R: IntoEventFlow,
{
    unsafe { prepend_static(event_source::<E>(), callback) }
}
