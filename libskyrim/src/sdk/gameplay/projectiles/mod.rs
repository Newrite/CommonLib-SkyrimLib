//! Projectile runtime and launch helpers.
//!
//! This SDK layer intentionally stays narrower than a full projectile
//! framework. It focuses on the building blocks repeatedly rebuilt by SKSE
//! plugins such as `NewProjectilesTMP`: manager traversal, runtime snapshots,
//! target acquisition, behavior steering, and ergonomic launch wrappers over
//! source-backed `ProjectileLaunchData`.
//!
//! The module is organized as:
//!
//! - [`runtime`] for manager traversal and live projectile snapshots
//! - [`targeting`] for target search and retarget selection
//! - [`behavior`] for intercept, aim, and steering helpers
//! - [`launch`] for ergonomic launch wrappers
//!
//! Typical workflow:
//!
//! - inspect live projectiles with [`collect_managed_projectile_snapshots`]
//! - acquire a desired target with [`collect_projectile_targets`] or one of the
//!   `find_*` helpers
//! - steer or retarget through [`refresh_projectile_desired_target`] and
//!   [`steer_projectile_towards_desired_target`]
//! - launch new projectiles with [`launch_spell`] or [`launch_arrow_auto`]
//!
//! Example:
//!
//! ```rust,ignore
//! use libskyrim::sdk::gameplay::projectiles;
//!
//! fn active_projectile_count() -> usize {
//!     projectiles::collect_managed_projectile_snapshots().len()
//! }
//! ```

mod behavior;
mod launch;
mod runtime;
mod shared;
mod targeting;
mod types;

pub use behavior::*;
pub use launch::*;
pub use runtime::*;
pub use targeting::*;
pub use types::*;

#[cfg(test)]
mod tests;
