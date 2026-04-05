use alloc::boxed::Box;
use core::fmt;

use super::batch::EventBatch;
use crate::sdk::events::source::EventInstallError;

pub(super) trait EventKeepAlive {}

impl<T> EventKeepAlive for T {}

pub(super) struct InstalledEvent<'a> {
    pub(super) name: &'static str,
    pub(super) _keepalive: Option<Box<dyn EventKeepAlive + 'a>>,
}

impl InstalledEvent<'_> {
    #[inline(always)]
    pub(super) fn name(&self) -> &'static str {
        self.name
    }
}

/// Error reported when one event installation inside a batch fails.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EventBatchError {
    event_name: &'static str,
    error: EventInstallError,
}

impl EventBatchError {
    /// Build a batch error from one event installer name and underlying cause.
    #[inline(always)]
    pub const fn new(event_name: &'static str, error: EventInstallError) -> Self {
        Self { event_name, error }
    }

    /// Event installer name that failed.
    #[inline(always)]
    pub const fn event_name(self) -> &'static str {
        self.event_name
    }

    /// Underlying event-install error.
    #[inline(always)]
    pub const fn error(self) -> EventInstallError {
        self.error
    }

    /// Abort plugin startup with a fatal runtime error.
    ///
    /// Use this only when event installation is mandatory for the plugin to
    /// function at all.
    #[inline(always)]
    pub fn install_or_fatal(self) -> ! {
        self.error.install_or_fatal(self.event_name)
    }
}

impl fmt::Display for EventBatchError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "failed to install event `{}`: {:?}",
            self.event_name, self.error
        )
    }
}

impl core::error::Error for EventBatchError {}

/// Result type used by one batch-install recipe.
pub type EventBatchInstallResult = Result<(), EventBatchError>;

/// Function ABI used by one event installer entry.
///
/// Install recipes receive the shared [`EventBatch`] owner so they can retain
/// every subscription they create under one named bootstrap unit.
pub type EventInstallFn = for<'a> fn(&mut EventBatch<'a>) -> EventBatchInstallResult;

/// Typed description of one installable event recipe.
///
/// These values are what the `install_all!` macros and batch helpers operate
/// on. They let plugin code keep a stable list of named install recipes rather
/// than open-coding bootstrap subscription logic everywhere.
#[derive(Clone, Copy)]
pub struct EventInstaller {
    name: &'static str,
    install: EventInstallFn,
}

impl EventInstaller {
    /// Construct one event installer from a name and install function.
    #[inline(always)]
    pub const fn new(name: &'static str, install: EventInstallFn) -> Self {
        Self { name, install }
    }

    /// Stable human-readable event installer name.
    #[inline(always)]
    pub const fn name(self) -> &'static str {
        self.name
    }

    /// Raw install function pointer.
    #[inline(always)]
    pub const fn install_fn(self) -> EventInstallFn {
        self.install
    }

    /// Attempt installation inside an [`EventBatch`].
    ///
    /// Prefer this when plugin bootstrap wants to surface install failures
    /// normally instead of aborting the process immediately.
    #[inline(always)]
    pub fn try_install(self, batch: &mut EventBatch<'_>) -> EventBatchInstallResult {
        (self.install)(batch)
    }

    /// Install or fail fatally.
    ///
    /// Use this only when the event is required for correct plugin behavior
    /// and there is no sensible degraded mode.
    #[inline(always)]
    pub fn install_or_fatal(self, batch: &mut EventBatch<'_>) {
        if let Err(error) = self.try_install(batch) {
            error.install_or_fatal();
        }
    }
}
