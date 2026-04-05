use crate::sdk::core::{GamePtr, HandleFamilyTarget, ResolvableHandle, Resolved, ResolvedHandle};

use super::types::TaskHandoff;

/// Construct the default background handoff descriptor.
#[inline(always)]
pub const fn background_handoff() -> TaskHandoff {
    TaskHandoff::background()
}

/// Construct the default UI handoff descriptor.
#[inline(always)]
pub const fn ui_handoff() -> TaskHandoff {
    TaskHandoff::ui()
}

/// Construct the default background event-origin handoff descriptor.
#[inline(always)]
pub const fn event_handoff() -> TaskHandoff {
    TaskHandoff::event()
}

/// Construct the default UI event-origin handoff descriptor.
#[inline(always)]
pub const fn ui_event_handoff() -> TaskHandoff {
    TaskHandoff::ui_event()
}

/// Construct the default background Papyrus-origin handoff descriptor.
#[inline(always)]
pub const fn papyrus_handoff() -> TaskHandoff {
    TaskHandoff::papyrus()
}

/// Construct the default UI Papyrus-origin handoff descriptor.
#[inline(always)]
pub const fn ui_papyrus_handoff() -> TaskHandoff {
    TaskHandoff::ui_papyrus()
}

/// Construct a background handoff that runs only during gameplay-safe phases.
#[inline(always)]
pub const fn gameplay_handoff() -> TaskHandoff {
    background_handoff().requiring_safe_gameplay_phase()
}

/// Event-origin variant of [`gameplay_handoff`].
#[inline(always)]
pub const fn gameplay_event_handoff() -> TaskHandoff {
    event_handoff().requiring_safe_gameplay_phase()
}

/// Papyrus-origin variant of [`gameplay_handoff`].
#[inline(always)]
pub const fn gameplay_papyrus_handoff() -> TaskHandoff {
    papyrus_handoff().requiring_safe_gameplay_phase()
}

/// Queue background work that originated in an event callback.
#[inline(always)]
pub fn queue_event_task(f: impl FnOnce() + Send + 'static) {
    event_handoff().dispatch(f)
}

/// Queue UI work that originated in an event callback.
#[inline(always)]
pub fn queue_ui_event_task(f: impl FnOnce() + Send + 'static) {
    ui_event_handoff().dispatch(f)
}

/// Queue background work that originated in Papyrus code.
#[inline(always)]
pub fn queue_papyrus_task(f: impl FnOnce() + Send + 'static) {
    papyrus_handoff().dispatch(f)
}

/// Queue UI work that originated in Papyrus code.
#[inline(always)]
pub fn queue_ui_papyrus_task(f: impl FnOnce() + Send + 'static) {
    ui_papyrus_handoff().dispatch(f)
}

/// Queue background work that should run only during gameplay-safe phases.
#[inline(always)]
pub fn queue_gameplay_task(f: impl FnOnce() + Send + 'static) {
    gameplay_handoff().dispatch(f)
}

/// Event-origin variant of [`queue_gameplay_task`].
#[inline(always)]
pub fn queue_gameplay_event_task(f: impl FnOnce() + Send + 'static) {
    gameplay_event_handoff().dispatch(f)
}

/// Papyrus-origin variant of [`queue_gameplay_task`].
#[inline(always)]
pub fn queue_gameplay_papyrus_task(f: impl FnOnce() + Send + 'static) {
    gameplay_papyrus_handoff().dispatch(f)
}

/// Queue background work after resolving a handle at execution time.
#[inline(always)]
pub fn queue_task_resolving_handle<H>(
    handle: H,
    f: impl FnOnce(ResolvedHandle<H>) + Send + 'static,
) -> bool
where
    H: ResolvableHandle + Send + 'static,
{
    background_handoff().dispatch_resolving_handle(handle, f)
}

/// UI variant of [`queue_task_resolving_handle`].
#[inline(always)]
pub fn queue_ui_task_resolving_handle<H>(
    handle: H,
    f: impl FnOnce(ResolvedHandle<H>) + Send + 'static,
) -> bool
where
    H: ResolvableHandle + Send + 'static,
{
    ui_handoff().dispatch_resolving_handle(handle, f)
}

/// Gameplay-safe variant of [`queue_task_resolving_handle`].
#[inline(always)]
pub fn queue_gameplay_task_resolving_handle<H>(
    handle: H,
    f: impl FnOnce(ResolvedHandle<H>) + Send + 'static,
) -> bool
where
    H: ResolvableHandle + Send + 'static,
{
    gameplay_handoff().dispatch_resolving_handle(handle, f)
}

/// Queue background work after capturing and later resolving a target handle.
#[inline(always)]
pub fn queue_task_resolving_target<T>(
    target: impl Into<GamePtr<T>>,
    f: impl FnOnce(Resolved<T>) + Send + 'static,
) -> bool
where
    T: HandleFamilyTarget,
    T::Handle: Send + 'static,
{
    background_handoff().dispatch_resolving_target(target, f)
}

/// UI variant of [`queue_task_resolving_target`].
#[inline(always)]
pub fn queue_ui_task_resolving_target<T>(
    target: impl Into<GamePtr<T>>,
    f: impl FnOnce(Resolved<T>) + Send + 'static,
) -> bool
where
    T: HandleFamilyTarget,
    T::Handle: Send + 'static,
{
    ui_handoff().dispatch_resolving_target(target, f)
}

/// Gameplay-safe variant of [`queue_task_resolving_target`].
#[inline(always)]
pub fn queue_gameplay_task_resolving_target<T>(
    target: impl Into<GamePtr<T>>,
    f: impl FnOnce(Resolved<T>) + Send + 'static,
) -> bool
where
    T: HandleFamilyTarget,
    T::Handle: Send + 'static,
{
    gameplay_handoff().dispatch_resolving_target(target, f)
}
