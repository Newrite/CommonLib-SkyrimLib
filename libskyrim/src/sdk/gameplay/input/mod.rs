//! Gameplay-facing input helpers.
//!
//! This layer intentionally focuses on the parts of Skyrim input state that
//! plugin code repeatedly needs:
//!
//! - `ControlMap` singleton access
//! - context-stack inspection and scoped context pushes
//! - enabled-control snapshots and restore-on-drop guards
//! - player-input hard blocking through `PlayerControls`
//! - simple user-event mapping lookup helpers
//! - directional snapshots and input-gesture helpers
//! - scoped movement/look/attack handler toggles

mod access;
mod contexts;
mod directional;
mod gestures;
mod handlers;
mod mapping;
mod shared;
mod state;
mod types;

pub use access::*;
pub use contexts::*;
pub use directional::*;
pub use gestures::*;
pub use handlers::*;
pub use mapping::*;
pub use state::*;
pub use types::*;
