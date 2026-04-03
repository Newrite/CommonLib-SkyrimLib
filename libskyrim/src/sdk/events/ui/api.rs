use crate::re::{UI, UIEventSourceEvent};

use crate::sdk::events::source::{
    EventInstallError, EventSubscription, IntoEventFlow, prepend_static, subscribe_static,
};

#[inline(always)]
fn event_source<E: UIEventSourceEvent>() -> *mut crate::re::BSTEventSource<E> {
    let ui = UI::get_singleton();
    if ui.is_null() {
        core::ptr::null_mut()
    } else {
        unsafe { (*ui).get_event_source::<E>() }
    }
}

#[inline(always)]
pub fn subscribe<E, F, R>(callback: F) -> Result<EventSubscription<'static, E>, EventInstallError>
where
    E: UIEventSourceEvent,
    F: FnMut(Option<&E>) -> R + 'static,
    R: IntoEventFlow,
{
    unsafe { subscribe_static(event_source::<E>(), callback) }
}

#[inline(always)]
pub fn prepend<E, F, R>(callback: F) -> Result<EventSubscription<'static, E>, EventInstallError>
where
    E: UIEventSourceEvent,
    F: FnMut(Option<&E>) -> R + 'static,
    R: IntoEventFlow,
{
    unsafe { prepend_static(event_source::<E>(), callback) }
}
