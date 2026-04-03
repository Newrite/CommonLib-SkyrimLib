//! Projectile runtime and launch helpers.
//!
//! This SDK layer intentionally stays narrower than a full projectile
//! framework. It focuses on the building blocks repeatedly rebuilt by SKSE
//! plugins such as `NewProjectilesTMP`: manager traversal, runtime snapshots,
//! target acquisition, behavior steering, and ergonomic launch wrappers over
//! source-backed `ProjectileLaunchData`.

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
