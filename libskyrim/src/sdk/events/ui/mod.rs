//! High-level helpers over `UI`-owned `BSTEventSource<T>` registrations.
//!
//! This domain is intentionally separate from gameplay and SKSE event
//! dispatchers because the owner, lifecycle, and registration timing differ.

mod api;

pub use api::{prepend, subscribe};
