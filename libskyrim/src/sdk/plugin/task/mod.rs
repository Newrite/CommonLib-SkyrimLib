//! High-level deferred-task helpers built on top of `libskyrim::skse::task`.
//!
//! This layer is intentionally small but more expressive than raw
//! `add_task(...)`:
//!
//! - name the handoff origin (`event`, `papyrus`, or generic)
//! - optionally gate execution behind the current gameplay-safe phase
//! - capture handles now and resolve them later inside the queued task

mod api;
mod shared;
mod types;

pub use crate::skse::task::{add_task, add_ui_task};
pub use api::*;
pub use types::*;

#[cfg(test)]
mod tests;
