//! Pathing-oriented gameplay helpers over translated `BSPathing*` surfaces.
//!
//! This module intentionally stays thin and source-backed: it wraps the RE
//! entrypoints that are already translated without pretending that a complete
//! global `Pathing` singleton surface is available yet.
//!
//! Prefer this layer when you need pathing-cell state, recent/loaded cell
//! snapshots, navmesh-info graph hints, or portal descriptors while staying
//! close to the translated RE pathing surface.
//!
//! Typical workflow:
//!
//! - start with [`singleton`] or [`nav_mesh_info_map`] when the plugin already
//!   needs native pathing state
//! - inspect cell readiness through [`inspect_pathing_cell`] or
//!   [`inspect_concrete_pathing_cell`]
//! - collect runtime-retained cells with [`collect_loaded_pathing_cells`] or
//!   [`collect_recent_pathing_cells`]
//! - use navmesh-info graph helpers from [`infos`] when pathing decisions need
//!   approximate cross-mesh connectivity
//!
//! Example:
//!
//! ```rust,ignore
//! use libskyrim::sdk::gameplay::pathing;
//!
//! fn loaded_ready_cell_count() -> usize {
//!     pathing::collect_loaded_pathing_cells()
//!         .into_iter()
//!         .filter(|entry| {
//!             pathing::inspect_concrete_pathing_cell_ptr(&entry.cell)
//!                 .is_some_and(|state| state.base.ready())
//!         })
//!         .count()
//! }
//! ```

mod cells;
mod infos;
mod location_search;
mod portals;
mod runtime;
mod shared;
mod types;

pub use cells::*;
pub use infos::*;
pub use location_search::*;
pub use portals::*;
pub use runtime::*;
pub use types::*;
