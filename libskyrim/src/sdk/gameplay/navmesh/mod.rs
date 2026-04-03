//! Navmesh-oriented gameplay helpers over translated `NavMesh` / `BSNavmesh`.
//!
//! This layer intentionally stays query-oriented. It exposes cell-local
//! snapshots, nearest-point helpers, and conservative same-mesh path-cost
//! heuristics without pretending that we already have a full Bethesda pathing
//! stack.

mod query;
mod reachability;
mod shared;
mod types;

pub use query::*;
pub use reachability::*;
pub use types::*;
