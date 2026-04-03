use alloc::boxed::Box;
use core::marker::PhantomData;
use core::ptr::NonNull;

use crate::re::{BSTEventHandler, BSTEventSource, OwnedBSTEventSink};

use super::flow::{EventFlow, EventInstallError, IntoEventFlow};

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
    ) -> crate::re::BSEventNotifyControl {
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
