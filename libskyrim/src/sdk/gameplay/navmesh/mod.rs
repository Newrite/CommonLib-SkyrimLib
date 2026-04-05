//! Navmesh-oriented gameplay helpers over translated `NavMesh` / `BSNavmesh`.
//!
//! This layer intentionally stays query-oriented. It exposes cell-local
//! snapshots, nearest-point helpers, and conservative same-mesh path-cost
//! heuristics without pretending that we already have a full Bethesda pathing
//! stack.
//!
//! Most plugin code will start with [`NavMeshCellSnapshot`],
//! [`NavMeshPointQuery`], or [`NavMeshReachabilityHeuristics`] from [`types`].
//!
//! Typical workflow:
//!
//! - capture one cell-local snapshot with [`snapshot_cell_navmeshes`] or
//!   [`snapshot_reference_navmeshes`]
//! - inspect nearest support through [`query_point_in_cell`] or
//!   [`query_point_from_reference`]
//! - ask coarse pathability questions with
//!   [`evaluate_reachability_in_cell`] or
//!   [`evaluate_reachability_from_reference`]
//!
//! Example:
//!
//! ```rust,ignore
//! use libskyrim::sdk::gameplay::navmesh;
//!
//! fn reference_has_support(reference: &libskyrim::re::TESObjectREFR) -> bool {
//!     navmesh::has_navmesh_support_for_reference(reference, 128.0)
//! }
//! ```

mod query;
mod reachability;
mod shared;
mod types;

pub use query::*;
pub use reachability::*;
pub use types::*;
