//! Rust-local plugin event bus primitives.
//!
//! This domain is intentionally plugin-local and does not attempt to replace
//! engine-backed `BSTEventSource<T>` or SKSE messaging. It is intended for
//! intra-plugin orchestration, especially when one system such as a hook or
//! gameplay callback wants to publish a mutable event payload to multiple Rust
//! subscribers.
//!
//! Decision guide:
//!
//! - use `bus` when the event should stay entirely inside one plugin
//! - use [`crate::sdk::events::game`], [`crate::sdk::events::ui`], or
//!   [`crate::sdk::events::skse`] when the source of truth is engine/SKSE-owned
//! - use [`crate::sdk::events::install`] when several bus and engine
//!   subscriptions should be retained under one bootstrap owner
//!
//! Typical pattern:
//!
//! 1. subscribe gameplay/UI/hook callbacks to engine-owned sources
//! 2. republish distilled plugin-local state through one [`Bus`]
//! 3. let multiple Rust systems observe and mutate that payload in priority order

mod runtime;
mod types;

pub use runtime::{Bus, BusSubscription};
pub use types::{PublishResult, SubscriberPriority};

#[cfg(test)]
mod tests;
