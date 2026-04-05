//! Higher-level spatial candidate evaluation built on top of world, actor, and
//! physics helpers.
//!
//! This is the "decision" layer above `world`, `physics`, and `navmesh`:
//! callers feed candidate points in and get scored/tiered evaluations back for
//! spawn, teleport, reposition, or respawn style workflows.
//!
//! Typical workflow:
//!
//! - evaluate one candidate with [`evaluate_spawn_candidate`]
//! - derive a score through [`score_spawn_candidate`] or
//!   [`evaluate_and_score_spawn_candidate`]
//! - rank a whole candidate set with [`evaluate_spawn_candidates`] or
//!   [`best_spawn_candidate`]
//! - apply fallback tiers through [`select_spawn_candidate_with_fallbacks`] when
//!   one "best overall" score is not enough
//!
//! Example:
//!
//! ```rust,ignore
//! use libskyrim::sdk::gameplay::spatial;
//!
//! fn has_valid_spawn(
//!     cell: &libskyrim::re::TESObjectCELL,
//!     candidate: libskyrim::re::NiPoint3,
//!     options: spatial::SpatialCandidateEvaluationOptions,
//! ) -> bool {
//!     spatial::evaluate_spawn_candidate(cell, candidate, options).is_valid
//! }
//! ```

mod evaluation;
mod shared;
mod tiers;
mod types;

pub use evaluation::*;
pub use tiers::*;
pub use types::*;
