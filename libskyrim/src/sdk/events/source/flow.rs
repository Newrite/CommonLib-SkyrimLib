use core::fmt;

use crate::re::BSEventNotifyControl;

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
