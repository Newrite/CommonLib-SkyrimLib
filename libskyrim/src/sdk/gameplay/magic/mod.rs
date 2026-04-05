//! Magic and active-effect helpers for common gameplay workflows.
//!
//! This SDK layer focuses first on read-mostly inspection helpers that C++
//! SKSE plugins repeatedly rebuild: active-effect queries, actor spell scans,
//! and `MagicItem` effect-base inspection.
//!
//! The surface is organized into:
//!
//! - [`active_effects`] for live active-effect traversal and filtering
//! - [`spells`] for actor spellbook inspection
//! - [`items`] for `MagicItem` / effect-base inspection
//! - [`caster`] for immediate casting and castability checks
//! - [`types`] for small reusable option/result snapshots
//!
//! Decision guide:
//!
//! - use [`active_effects`] when runtime code cares about live effects
//!   currently applied to an actor or other [`crate::re::MagicTarget`];
//! - use [`spells`] when the plugin is inspecting or mutating an actor's
//!   learned spell set;
//! - use [`items`] when the plugin starts from a `MagicItem` and wants to
//!   inspect its effect definitions without touching live runtime state;
//! - use [`caster`] when the plugin wants to check or trigger an immediate cast;
//! - use [`types`] when a workflow wants small reusable snapshots or option
//!   structs shared across those layers.

mod active_effects;
mod caster;
mod items;
mod shared;
mod spells;
mod types;

/// Live active-effect traversal, filtering, and dispel helpers.
pub use active_effects::*;

/// Immediate castability checks and cast helpers.
pub use caster::*;

/// `MagicItem` / effect-base inspection helpers.
pub use items::*;

/// Actor spellbook traversal and mutation helpers.
pub use spells::*;

/// Shared option/result snapshot types for the magic domain.
pub use types::*;
