//! Rust-local plugin event bus primitives.
//!
//! This domain is intentionally plugin-local and does not attempt to replace
//! engine-backed `BSTEventSource<T>` or SKSE messaging. It is intended for
//! intra-plugin orchestration, especially when one system such as a hook or
//! gameplay callback wants to publish a mutable event payload to multiple Rust
//! subscribers.

mod runtime;
mod types;

pub use runtime::{Bus, BusSubscription};
pub use types::{PublishResult, SubscriberPriority};

#[cfg(test)]
mod tests;
