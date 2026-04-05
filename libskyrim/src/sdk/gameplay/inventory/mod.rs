//! Inventory and equipment helpers for common gameplay plugins.
//!
//! The public surface is split into:
//!
//! - [`types`] for entry snapshots and query options
//! - [`entries`] for inventory traversal, counting, lookup, and typed
//!   collection
//! - [`equipment`] for worn/equipped inspection
//! - [`mutation`] for remove/transfer operations built on the honest RE item
//!   APIs

mod entries;
mod equipment;
mod mutation;
mod shared;
mod types;

pub use entries::*;
pub use equipment::*;
pub use mutation::*;
pub use types::*;

#[cfg(test)]
mod tests;
