//! Quest and quest-stage gameplay helpers.
//!
//! This module keeps quest objects themselves source-backed while adding a
//! higher-level Rust-first layer around the common plugin workflows:
//! quest state inspection, alias/objective/stage traversal, and small safe
//! wrappers around the most common lifecycle operations.

mod aliases;
mod lifecycle;
mod objectives;
mod shared;
mod stages;
mod types;

pub use aliases::*;
pub use lifecycle::*;
pub use objectives::*;
pub use stages::*;
pub use types::*;

#[cfg(test)]
mod tests;
