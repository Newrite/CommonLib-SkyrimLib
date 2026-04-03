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
    #[inline(always)]
    pub const fn new(event_name: &'static str, error: EventInstallError) -> Self {
        Self { event_name, error }
    }

    #[inline(always)]
    pub const fn event_name(self) -> &'static str {
        self.event_name
    }

    #[inline(always)]
    pub const fn error(self) -> EventInstallError {
        self.error
    }

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

pub type EventBatchInstallResult = Result<(), EventBatchError>;
pub type EventInstallFn = for<'a> fn(&mut EventBatch<'a>) -> EventBatchInstallResult;

#[derive(Clone, Copy)]
pub struct EventInstaller {
    name: &'static str,
    install: EventInstallFn,
}

impl EventInstaller {
    #[inline(always)]
    pub const fn new(name: &'static str, install: EventInstallFn) -> Self {
        Self { name, install }
    }

    #[inline(always)]
    pub const fn name(self) -> &'static str {
        self.name
    }

    #[inline(always)]
    pub const fn install_fn(self) -> EventInstallFn {
        self.install
    }

    #[inline(always)]
    pub fn try_install(self, batch: &mut EventBatch<'_>) -> EventBatchInstallResult {
        (self.install)(batch)
    }

    #[inline(always)]
    pub fn install_or_fatal(self, batch: &mut EventBatch<'_>) {
        if let Err(error) = self.try_install(batch) {
            error.install_or_fatal();
        }
    }
}
