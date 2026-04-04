use crate::sdk::core::{GamePtr, HandleFamilyTarget, ResolvableHandle, Resolved, ResolvedHandle};

use super::types::TaskHandoff;

#[inline(always)]
pub const fn background_handoff() -> TaskHandoff {
    TaskHandoff::background()
}

#[inline(always)]
pub const fn ui_handoff() -> TaskHandoff {
    TaskHandoff::ui()
}

#[inline(always)]
pub const fn event_handoff() -> TaskHandoff {
    TaskHandoff::event()
}

#[inline(always)]
pub const fn ui_event_handoff() -> TaskHandoff {
    TaskHandoff::ui_event()
}

#[inline(always)]
pub const fn papyrus_handoff() -> TaskHandoff {
    TaskHandoff::papyrus()
}

#[inline(always)]
pub const fn ui_papyrus_handoff() -> TaskHandoff {
    TaskHandoff::ui_papyrus()
}

#[inline(always)]
pub const fn gameplay_handoff() -> TaskHandoff {
    background_handoff().requiring_safe_gameplay_phase()
}

#[inline(always)]
pub const fn gameplay_event_handoff() -> TaskHandoff {
    event_handoff().requiring_safe_gameplay_phase()
}

#[inline(always)]
pub const fn gameplay_papyrus_handoff() -> TaskHandoff {
    papyrus_handoff().requiring_safe_gameplay_phase()
}

#[inline(always)]
pub fn queue_event_task(f: impl FnOnce() + Send + 'static) {
    event_handoff().dispatch(f)
}

#[inline(always)]
pub fn queue_ui_event_task(f: impl FnOnce() + Send + 'static) {
    ui_event_handoff().dispatch(f)
}

#[inline(always)]
pub fn queue_papyrus_task(f: impl FnOnce() + Send + 'static) {
    papyrus_handoff().dispatch(f)
}

#[inline(always)]
pub fn queue_ui_papyrus_task(f: impl FnOnce() + Send + 'static) {
    ui_papyrus_handoff().dispatch(f)
}

#[inline(always)]
pub fn queue_gameplay_task(f: impl FnOnce() + Send + 'static) {
    gameplay_handoff().dispatch(f)
}

#[inline(always)]
pub fn queue_gameplay_event_task(f: impl FnOnce() + Send + 'static) {
    gameplay_event_handoff().dispatch(f)
}

#[inline(always)]
pub fn queue_gameplay_papyrus_task(f: impl FnOnce() + Send + 'static) {
    gameplay_papyrus_handoff().dispatch(f)
}

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
