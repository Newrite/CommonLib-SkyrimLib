//! Generic helpers for already-known raw `BSTEventSource<T>` pointers.
//!
//! This is the low-friction escape hatch for event owners that do not fit the
//! fixed singleton domains such as `game`, `ui`, or `skse::dispatchers`.

mod flow;
mod subscription;

pub use flow::{EventFlow, EventInstallError, IntoEventFlow};
pub use subscription::{EventSourceExt, EventSourceRef, EventSubscription, prepend, subscribe};
pub(crate) use subscription::{prepend_static, subscribe_static};

#[cfg(test)]
mod tests;
