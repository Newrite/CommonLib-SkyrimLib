//! Havok and physics-oriented helpers.
//!
//! This module intentionally separates two layers that many C++ SKSE projects
//! blur together:
//!
//! - [`CFilter`] is the Havok query filter info sent into the raycast.
//! - [`LayerMask`] is a post-query collision-layer mask used to decide which
//!   hits should count as blockers.
//!
//! Common entry points:
//!
//! - [`raycast_segment`] / [`raycast_all_segment`] for ordinary line traces
//! - [`has_line_of_sight`] for boolean visibility checks
//! - [`ground_snap_point`] when a world-space point needs terrain anchoring
//! - [`validate_spawn_point`] when the plugin needs a higher-level placement
//!   sanity check
//!
//! Example:
//!
//! ```rust,ignore
//! use libskyrim::sdk::advanced::physics::{
//!     LayerMask, filter_for_layer, ground_snap_point, has_line_of_sight, validate_spawn_point,
//! };
//! use libskyrim::re::ColLayer;
//!
//! fn try_place_marker(origin: libskyrim::re::NiPoint3) -> Option<libskyrim::re::NiPoint3> {
//!     let filter = filter_for_layer(ColLayer::kLOS);
//!     let snapped = ground_snap_point(origin, filter, LayerMask::all())?;
//!     let validation = validate_spawn_point(snapped, filter, LayerMask::all());
//!     validation.is_valid().then_some(snapped)
//! }
//! ```

mod filters;
mod geometry;
mod probes;
mod raycast;
mod shared;
mod types;

pub use filters::*;
pub use geometry::*;
pub use probes::*;
pub use raycast::*;
pub use types::*;
