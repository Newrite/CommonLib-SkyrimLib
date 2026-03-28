//! Generic helpers for already-known raw `BSTEventSource<T>` pointers.
//!
//! This is the low-friction escape hatch for event owners that do not fit the
//! fixed singleton domains such as `game`, `ui`, or `skse::dispatchers`.

use alloc::boxed::Box;
use core::fmt;
use core::marker::PhantomData;
use core::ptr::NonNull;

use crate::re::{BSEventNotifyControl, BSTEventHandler, BSTEventSource, OwnedBSTEventSink};

/// SDK-facing event propagation result.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EventFlow {
    Continue,
    Stop,
}

impl EventFlow {
    #[inline(always)]
    pub const fn is_continue(self) -> bool {
        matches!(self, Self::Continue)
    }

    #[inline(always)]
    pub const fn is_stop(self) -> bool {
        matches!(self, Self::Stop)
    }
}

impl From<EventFlow> for BSEventNotifyControl {
    #[inline(always)]
    fn from(value: EventFlow) -> Self {
        match value {
            EventFlow::Continue => Self::Continue,
            EventFlow::Stop => Self::Stop,
        }
    }
}

/// Small conversion trait for event callbacks.
///
/// Returning `()` means "continue propagation", which keeps common handlers
/// concise without forcing explicit `EventFlow::Continue` at every call site.
pub trait IntoEventFlow {
    fn into_event_flow(self) -> EventFlow;
}

impl IntoEventFlow for EventFlow {
    #[inline(always)]
    fn into_event_flow(self) -> EventFlow {
        self
    }
}

impl IntoEventFlow for () {
    #[inline(always)]
    fn into_event_flow(self) -> EventFlow {
        EventFlow::Continue
    }
}

/// Installation failures for high-level event subscriptions.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum EventInstallError {
    SourceUnavailable,
    SinkAllocationFailed,
    MessagingListenerUnavailable,
}

impl EventInstallError {
    #[inline(always)]
    pub fn install_or_fatal(self, event_name: &str) -> ! {
        crate::skse::log::fatal_runtime(format_args!(
            "failed to install event `{event_name}`: {self}"
        ))
    }
}

impl fmt::Display for EventInstallError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SourceUnavailable => f.write_str("event source is unavailable"),
            Self::SinkAllocationFailed => f.write_str("failed to allocate owned event sink"),
            Self::MessagingListenerUnavailable => f.write_str("messaging listener is unavailable"),
        }
    }
}

impl core::error::Error for EventInstallError {}

type DynEventCallback<T> = dyn FnMut(Option<&T>) -> EventFlow + 'static;

struct ClosureEventHandler<T> {
    callback: Box<DynEventCallback<T>>,
}

impl<T> ClosureEventHandler<T> {
    #[inline(always)]
    fn new<F, R>(mut callback: F) -> Self
    where
        F: FnMut(Option<&T>) -> R + 'static,
        R: IntoEventFlow,
    {
        Self {
            callback: Box::new(move |event| callback(event).into_event_flow()),
        }
    }
}

impl<T> BSTEventHandler<T> for ClosureEventHandler<T> {
    #[inline(always)]
    fn process_event(
        &mut self,
        event: *const T,
        _event_source: *mut BSTEventSource<T>,
    ) -> BSEventNotifyControl {
        (self.callback)(unsafe { event.as_ref() }).into()
    }
}

/// Borrowed high-level wrapper around one `BSTEventSource<T>`.
///
/// This is the ergonomic bridge for dynamic or owner-bound event sources when
/// the caller already has a borrow proving the source will outlive the
/// subscription.
#[derive(Clone, Copy)]
pub struct EventSourceRef<'a, T> {
    source: NonNull<BSTEventSource<T>>,
    marker: PhantomData<&'a mut BSTEventSource<T>>,
}

impl<'a, T> EventSourceRef<'a, T> {
    #[inline(always)]
    pub fn new(source: &'a mut BSTEventSource<T>) -> Self {
        Self {
            source: NonNull::from(source),
            marker: PhantomData,
        }
    }

    /// # Safety
    /// `source` must remain valid for the full `'a` lifetime.
    #[inline(always)]
    pub unsafe fn from_raw(source: *mut BSTEventSource<T>) -> Result<Self, EventInstallError> {
        let Some(source) = NonNull::new(source) else {
            return Err(EventInstallError::SourceUnavailable);
        };

        Ok(Self {
            source,
            marker: PhantomData,
        })
    }

    #[inline(always)]
    pub fn as_ptr(self) -> *mut BSTEventSource<T> {
        self.source.as_ptr()
    }

    #[inline(always)]
    pub fn subscribe<F, R>(self, callback: F) -> Result<EventSubscription<'a, T>, EventInstallError>
    where
        F: FnMut(Option<&T>) -> R + 'static,
        R: IntoEventFlow,
    {
        unsafe { subscribe_with_order(self.source.as_ptr(), callback, SubscriptionOrder::Append) }
    }

    #[inline(always)]
    pub fn prepend<F, R>(self, callback: F) -> Result<EventSubscription<'a, T>, EventInstallError>
    where
        F: FnMut(Option<&T>) -> R + 'static,
        R: IntoEventFlow,
    {
        unsafe { subscribe_with_order(self.source.as_ptr(), callback, SubscriptionOrder::Prepend) }
    }
}

impl<'a, T> From<&'a mut BSTEventSource<T>> for EventSourceRef<'a, T> {
    #[inline(always)]
    fn from(source: &'a mut BSTEventSource<T>) -> Self {
        Self::new(source)
    }
}

/// Ergonomic extension trait for owner-bound `BSTEventSource<T>` values.
pub trait EventSourceExt<T> {
    fn event_source_ref(&mut self) -> EventSourceRef<'_, T>;

    fn subscribe_sdk<F, R>(
        &mut self,
        callback: F,
    ) -> Result<EventSubscription<'_, T>, EventInstallError>
    where
        F: FnMut(Option<&T>) -> R + 'static,
        R: IntoEventFlow;

    fn prepend_sdk<F, R>(
        &mut self,
        callback: F,
    ) -> Result<EventSubscription<'_, T>, EventInstallError>
    where
        F: FnMut(Option<&T>) -> R + 'static,
        R: IntoEventFlow;
}

impl<T> EventSourceExt<T> for BSTEventSource<T> {
    #[inline(always)]
    fn event_source_ref(&mut self) -> EventSourceRef<'_, T> {
        EventSourceRef::new(self)
    }

    #[inline(always)]
    fn subscribe_sdk<F, R>(
        &mut self,
        callback: F,
    ) -> Result<EventSubscription<'_, T>, EventInstallError>
    where
        F: FnMut(Option<&T>) -> R + 'static,
        R: IntoEventFlow,
    {
        self.event_source_ref().subscribe(callback)
    }

    #[inline(always)]
    fn prepend_sdk<F, R>(
        &mut self,
        callback: F,
    ) -> Result<EventSubscription<'_, T>, EventInstallError>
    where
        F: FnMut(Option<&T>) -> R + 'static,
        R: IntoEventFlow,
    {
        self.event_source_ref().prepend(callback)
    }
}

/// RAII handle for a subscription installed on an engine `BSTEventSource<T>`.
pub struct EventSubscription<'a, T> {
    source: NonNull<BSTEventSource<T>>,
    sink: OwnedBSTEventSink<T, ClosureEventHandler<T>>,
    marker: PhantomData<&'a mut BSTEventSource<T>>,
}

impl<T> EventSubscription<'_, T> {
    #[inline(always)]
    pub fn source_ptr(&self) -> *mut BSTEventSource<T> {
        self.source.as_ptr()
    }

    #[inline(always)]
    pub fn sink_ptr(&self) -> *mut crate::re::BSTEventSink<T> {
        self.sink.as_ptr()
    }
}

impl<T> Drop for EventSubscription<'_, T> {
    fn drop(&mut self) {
        unsafe {
            self.sink.remove_from_source(self.source.as_ptr());
        }
    }
}

#[derive(Clone, Copy)]
enum SubscriptionOrder {
    Append,
    Prepend,
}

unsafe fn subscribe_with_order<'a, T, F, R>(
    source: *mut BSTEventSource<T>,
    callback: F,
    order: SubscriptionOrder,
) -> Result<EventSubscription<'a, T>, EventInstallError>
where
    F: FnMut(Option<&T>) -> R + 'static,
    R: IntoEventFlow,
{
    let Some(source) = NonNull::new(source) else {
        return Err(EventInstallError::SourceUnavailable);
    };

    let Some(sink) = OwnedBSTEventSink::new(ClosureEventHandler::new(callback)) else {
        return Err(EventInstallError::SinkAllocationFailed);
    };

    match order {
        SubscriptionOrder::Append => unsafe { sink.add_to_source(source.as_ptr()) },
        SubscriptionOrder::Prepend => unsafe { sink.prepend_to_source(source.as_ptr()) },
    }

    Ok(EventSubscription {
        source,
        sink,
        marker: PhantomData,
    })
}

/// Register a Rust closure on a raw `BSTEventSource<T>`.
///
/// # Safety
/// `source` must remain a valid `BSTEventSource<T>` pointer until the returned
/// subscription is dropped.
#[inline(always)]
pub unsafe fn subscribe<'a, T, F, R>(
    source: *mut BSTEventSource<T>,
    callback: F,
) -> Result<EventSubscription<'a, T>, EventInstallError>
where
    F: FnMut(Option<&T>) -> R + 'static,
    R: IntoEventFlow,
{
    unsafe { subscribe_with_order(source, callback, SubscriptionOrder::Append) }
}

/// Register a Rust closure at the front of a raw `BSTEventSource<T>`.
///
/// # Safety
/// `source` must remain a valid `BSTEventSource<T>` pointer until the returned
/// subscription is dropped.
#[inline(always)]
pub unsafe fn prepend<'a, T, F, R>(
    source: *mut BSTEventSource<T>,
    callback: F,
) -> Result<EventSubscription<'a, T>, EventInstallError>
where
    F: FnMut(Option<&T>) -> R + 'static,
    R: IntoEventFlow,
{
    unsafe { subscribe_with_order(source, callback, SubscriptionOrder::Prepend) }
}

pub(crate) unsafe fn subscribe_static<T, F, R>(
    source: *mut BSTEventSource<T>,
    callback: F,
) -> Result<EventSubscription<'static, T>, EventInstallError>
where
    F: FnMut(Option<&T>) -> R + 'static,
    R: IntoEventFlow,
{
    unsafe { subscribe(source, callback) }
}

pub(crate) unsafe fn prepend_static<T, F, R>(
    source: *mut BSTEventSource<T>,
    callback: F,
) -> Result<EventSubscription<'static, T>, EventInstallError>
where
    F: FnMut(Option<&T>) -> R + 'static,
    R: IntoEventFlow,
{
    unsafe { prepend(source, callback) }
}

#[cfg(test)]
mod tests {
    use alloc::rc::Rc;
    use core::cell::Cell;

    use super::*;

    #[test]
    fn null_source_is_rejected() {
        let result =
            unsafe { subscribe::<u32, _, _>(core::ptr::null_mut(), |_| EventFlow::Continue) };
        assert!(matches!(result, Err(EventInstallError::SourceUnavailable)));
    }

    #[test]
    fn subscription_receives_events_and_unsubscribes_on_drop() {
        let mut source = BSTEventSource::<u32>::new();
        let seen = Rc::new(Cell::new(0u32));
        let last = Rc::new(Cell::new(0u32));

        {
            let seen_ref = Rc::clone(&seen);
            let last_ref = Rc::clone(&last);
            let _subscription = unsafe {
                subscribe(&mut source, move |event| {
                    let Some(event) = event else {
                        return EventFlow::Continue;
                    };

                    seen_ref.set(seen_ref.get() + 1);
                    last_ref.set(*event);
                    EventFlow::Continue
                })
            }
            .expect("subscription should install");

            let value = 42u32;
            unsafe {
                source.send_event(&value);
            }
        }

        assert_eq!(seen.get(), 1);
        assert_eq!(last.get(), 42);
        assert!(unsafe { source.sinks.as_slice() }.is_empty());
    }

    #[test]
    fn source_ref_installs_without_raw_pointer_plumbing() {
        let mut source = BSTEventSource::<u32>::new();
        let seen = Rc::new(Cell::new(0u32));

        {
            let source_ref = EventSourceRef::new(&mut source);
            let source_ptr = source_ref.as_ptr();
            let seen_ref = Rc::clone(&seen);
            let _subscription = source_ref
                .subscribe(move |event| {
                    if let Some(event) = event {
                        seen_ref.set(*event);
                    }
                })
                .expect("source_ref should install");

            let value = 7u32;
            unsafe {
                (*source_ptr).send_event(&value);
            }
        }

        assert_eq!(seen.get(), 7);
        assert!(unsafe { source.sinks.as_slice() }.is_empty());
    }

    #[test]
    fn event_source_ext_subscribes_through_owner_borrow() {
        let mut source = BSTEventSource::<u32>::new();
        let seen = Rc::new(Cell::new(0u32));

        {
            let source_ptr = core::ptr::from_mut(&mut source);
            let seen_ref = Rc::clone(&seen);
            let _subscription = source
                .subscribe_sdk(move |event| {
                    if let Some(event) = event {
                        seen_ref.set(*event);
                    }
                })
                .expect("extension trait should install");

            let value = 11u32;
            unsafe {
                (*source_ptr).send_event(&value);
            }
        }

        assert_eq!(seen.get(), 11);
        assert!(unsafe { source.sinks.as_slice() }.is_empty());
    }
}
