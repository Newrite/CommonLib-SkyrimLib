use crate::sdk::events::source::{
    EventInstallError, EventSubscription, IntoEventFlow, prepend_static, subscribe_static,
};

use super::DispatcherEvent;

#[inline(always)]
pub fn subscribe<E, F, R>(callback: F) -> Result<EventSubscription<'static, E>, EventInstallError>
where
    E: DispatcherEvent,
    F: FnMut(Option<&E>) -> R + 'static,
    R: IntoEventFlow,
{
    unsafe { subscribe_static(E::source(), callback) }
}

#[inline(always)]
pub fn prepend<E, F, R>(callback: F) -> Result<EventSubscription<'static, E>, EventInstallError>
where
    E: DispatcherEvent,
    F: FnMut(Option<&E>) -> R + 'static,
    R: IntoEventFlow,
{
    unsafe { prepend_static(E::source(), callback) }
}
