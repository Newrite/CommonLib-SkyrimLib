//! Magic and active-effect helpers for common gameplay workflows.
//!
//! This SDK layer focuses first on read-mostly inspection helpers that C++
//! SKSE plugins repeatedly rebuild: active-effect queries, actor spell scans,
//! and `MagicItem` effect-base inspection.

mod active_effects;
mod caster;
mod items;
mod shared;
mod spells;
mod types;

pub use active_effects::*;
pub use caster::*;
pub use items::*;
pub use spells::*;
pub use types::*;
