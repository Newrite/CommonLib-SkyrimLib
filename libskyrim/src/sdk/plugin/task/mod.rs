//! High-level deferred-task helpers built on top of `libskyrim::skse::task`.
//!
//! This layer is intentionally small but more expressive than raw
//! `add_task(...)`:
//!
//! - name the handoff origin (`event`, `papyrus`, or generic)
//! - optionally gate execution behind the current gameplay-safe phase
//! - capture handles now and resolve them later inside the queued task
//!
//! Reach for this layer when plugin code wants to say *why* work is being
//! deferred and *where* it should run, instead of dropping immediately to raw
//! `add_task(...)` / `add_ui_task(...)`.
//!
//! Example:
//!
//! ```rust,ignore
//! use libskyrim::re::Actor;
//! use libskyrim::sdk::{core::Resolved, plugin};
//!
//! fn queue_actor_work(actor: libskyrim::sdk::core::GamePtr<Actor>) {
//!     let _queued = plugin::task::queue_gameplay_task_resolving_target(
//!         actor,
//!         |actor: Resolved<Actor>| {
//!             let _ = actor.get_position();
//!         },
//!     );
//! }
//! ```

mod api;
mod shared;
mod types;

pub use crate::skse::task::{add_task, add_ui_task};
pub use api::*;
pub use types::*;

#[cfg(test)]
mod tests;
