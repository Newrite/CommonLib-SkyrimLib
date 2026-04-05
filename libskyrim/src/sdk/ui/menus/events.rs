use alloc::borrow::ToOwned;

use crate::re::MenuOpenCloseEvent;
use crate::sdk::events::source::{EventFlow, EventInstallError, EventSubscription, IntoEventFlow};
use crate::sdk::events::ui as ui_events;

use super::NamedMenu;

/// Subscribes to every `MenuOpenCloseEvent`.
///
/// This is the generic entrypoint for menu lifecycle observers that want to
/// inspect `event.menu_name` and `event.opening` themselves.
pub fn subscribe_open_close<F, R>(
    mut callback: F,
) -> Result<EventSubscription<'static, MenuOpenCloseEvent>, EventInstallError>
where
    F: FnMut(&MenuOpenCloseEvent) -> R + 'static,
    R: IntoEventFlow,
{
    ui_events::subscribe(move |event| match event {
        Some(event) => callback(event).into_event_flow(),
        None => EventFlow::Continue,
    })
}

/// Subscribes to open/close events for one named menu.
pub fn subscribe_menu_open_close<F, R>(
    menu_name: &str,
    mut callback: F,
) -> Result<EventSubscription<'static, MenuOpenCloseEvent>, EventInstallError>
where
    F: FnMut(&MenuOpenCloseEvent) -> R + 'static,
    R: IntoEventFlow,
{
    let menu_name = menu_name.to_owned();
    subscribe_open_close(move |event| {
        if event.menu_name.as_str() == menu_name.as_str() {
            callback(event).into_event_flow()
        } else {
            EventFlow::Continue
        }
    })
}

/// Subscribes only to menu-open events for one named menu.
pub fn subscribe_menu_open<F, R>(
    menu_name: &str,
    mut callback: F,
) -> Result<EventSubscription<'static, MenuOpenCloseEvent>, EventInstallError>
where
    F: FnMut(&MenuOpenCloseEvent) -> R + 'static,
    R: IntoEventFlow,
{
    let menu_name = menu_name.to_owned();
    subscribe_open_close(move |event| {
        if event.opening && event.menu_name.as_str() == menu_name.as_str() {
            callback(event).into_event_flow()
        } else {
            EventFlow::Continue
        }
    })
}

/// Subscribes only to menu-close events for one named menu.
pub fn subscribe_menu_close<F, R>(
    menu_name: &str,
    mut callback: F,
) -> Result<EventSubscription<'static, MenuOpenCloseEvent>, EventInstallError>
where
    F: FnMut(&MenuOpenCloseEvent) -> R + 'static,
    R: IntoEventFlow,
{
    let menu_name = menu_name.to_owned();
    subscribe_open_close(move |event| {
        if !event.opening && event.menu_name.as_str() == menu_name.as_str() {
            callback(event).into_event_flow()
        } else {
            EventFlow::Continue
        }
    })
}

/// Typed variant of [`subscribe_menu_open_close`].
#[inline(always)]
pub fn subscribe_named_open_close<M, F, R>(
    callback: F,
) -> Result<EventSubscription<'static, MenuOpenCloseEvent>, EventInstallError>
where
    M: NamedMenu,
    F: FnMut(&MenuOpenCloseEvent) -> R + 'static,
    R: IntoEventFlow,
{
    subscribe_menu_open_close(M::MENU_NAME, callback)
}

/// Typed variant of [`subscribe_menu_open`].
#[inline(always)]
pub fn subscribe_named_open<M, F, R>(
    callback: F,
) -> Result<EventSubscription<'static, MenuOpenCloseEvent>, EventInstallError>
where
    M: NamedMenu,
    F: FnMut(&MenuOpenCloseEvent) -> R + 'static,
    R: IntoEventFlow,
{
    subscribe_menu_open(M::MENU_NAME, callback)
}

/// Typed variant of [`subscribe_menu_close`].
#[inline(always)]
pub fn subscribe_named_close<M, F, R>(
    callback: F,
) -> Result<EventSubscription<'static, MenuOpenCloseEvent>, EventInstallError>
where
    M: NamedMenu,
    F: FnMut(&MenuOpenCloseEvent) -> R + 'static,
    R: IntoEventFlow,
{
    subscribe_menu_close(M::MENU_NAME, callback)
}
