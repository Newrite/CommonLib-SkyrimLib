use core::fmt;

use crate::sdk::events::source::EventFlow;

/// Subscriber execution priority for a plugin-local event bus.
///
/// Higher values run earlier. Equal priorities keep subscription order.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct SubscriberPriority(i32);

impl SubscriberPriority {
    /// Highest common priority bucket.
    pub const FIRST: Self = Self(100);
    /// Early priority bucket.
    pub const EARLY: Self = Self(50);
    /// Default subscription priority.
    pub const NORMAL: Self = Self(0);
    /// Late priority bucket.
    pub const LATE: Self = Self(-50);
    /// Lowest common priority bucket.
    pub const LAST: Self = Self(-100);

    /// Build an explicit priority value.
    ///
    /// Prefer the named buckets when they are sufficient; use this when the
    /// plugin needs finer control over ordering between several local systems.
    #[inline(always)]
    pub const fn new(value: i32) -> Self {
        Self(value)
    }

    /// Returns the underlying priority integer.
    #[inline(always)]
    pub const fn value(self) -> i32 {
        self.0
    }
}

/// Result of publishing an owned payload through a bus.
///
/// This is the natural result type for plugin-local bus workflows where
/// subscribers are allowed to mutate the event payload before the publisher
/// inspects the final state.
pub struct PublishResult<T> {
    event: T,
    flow: EventFlow,
}

impl<T> PublishResult<T> {
    #[inline(always)]
    pub(crate) const fn new(event: T, flow: EventFlow) -> Self {
        Self { event, flow }
    }

    /// Return the final propagation flow.
    ///
    /// Use this when the publisher cares whether one subscriber stopped
    /// propagation.
    #[inline(always)]
    pub const fn flow(&self) -> EventFlow {
        self.flow
    }

    /// Borrow the final event payload.
    ///
    /// This is the common query path after `publish_owned`, `publish_default`,
    /// or `publish_with`.
    #[inline(always)]
    pub fn event(&self) -> &T {
        &self.event
    }

    /// Mutably borrow the final event payload.
    ///
    /// This is mainly useful for follow-up adjustments before the publisher
    /// consumes the result.
    #[inline(always)]
    pub fn event_mut(&mut self) -> &mut T {
        &mut self.event
    }

    /// Consume the result and return the final event payload.
    ///
    /// Use this when the publisher wants ownership of the fully processed
    /// payload again.
    #[inline(always)]
    pub fn into_event(self) -> T {
        self.event
    }

    /// Consume the result and return both payload and propagation flow.
    ///
    /// This is the most explicit way to inspect the outcome of one publish
    /// call when both the final payload and the stop/continue state matter.
    #[inline(always)]
    pub fn into_parts(self) -> (T, EventFlow) {
        (self.event, self.flow)
    }
}

impl<T: fmt::Debug> fmt::Debug for PublishResult<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("PublishResult")
            .field("event", &self.event)
            .field("flow", &self.flow)
            .finish()
    }
}
