//! Batch installation helpers for SDK event registrations.

mod batch;
mod helpers;
mod types;

pub use batch::EventBatch;
pub use helpers::{install_batch_or_fatal, try_install_all};
pub use types::{EventBatchError, EventBatchInstallResult, EventInstallFn, EventInstaller};

#[cfg(test)]
mod tests;
