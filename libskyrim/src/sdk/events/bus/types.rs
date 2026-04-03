use core::fmt;

use crate::sdk::events::source::EventFlow;

/// Subscriber execution priority for a plugin-local event bus.
///
/// Higher values run earlier. Equal priorities keep subscription order.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct SubscriberPriority(i32);

impl SubscriberPriority {
    pub const FIRST: Self = Self(100);
    pub const EARLY: Self = Self(50);
    pub const NORMAL: Self = Self(0);
    pub const LATE: Self = Self(-50);
    pub const LAST: Self = Self(-100);

    #[inline(always)]
    pub const fn new(value: i32) -> Self {
        Self(value)
    }

    #[inline(always)]
    pub const fn value(self) -> i32 {
        self.0
    }
}

/// Result of publishing an owned payload through a bus.
pub struct PublishResult<T> {
    event: T,
    flow: EventFlow,
}

impl<T> PublishResult<T> {
    #[inline(always)]
    pub(crate) const fn new(event: T, flow: EventFlow) -> Self {
        Self { event, flow }
    }

    #[inline(always)]
    pub const fn flow(&self) -> EventFlow {
        self.flow
    }

    #[inline(always)]
    pub fn event(&self) -> &T {
        &self.event
    }

    #[inline(always)]
    pub fn event_mut(&mut self) -> &mut T {
        &mut self.event
    }

    #[inline(always)]
    pub fn into_event(self) -> T {
        self.event
    }

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
