//! Scaleform-oriented SDK helpers.
//!
//! Current support focuses on the menu-side Scaleform surfaces we can already
//! model honestly:
//!
//! - access to open menu `GFxMovieView` instances
//! - access to menu `FxDelegate` pointers
//! - top-most menu Scaleform lookup
//! - menu movie `invoke` / `get_variable` / `set_variable` helpers
//!
//! ActionScript object traversal sugar and C++ callback registration remain
//! intentionally narrow; the raw `GFxValue` surface is now present, but we keep
//! SDK helpers centered on the repeated menu-side workflows plugins actually
//! need first.
//!
//! Decision guide:
//!
//! - use [`surface`] or [`api::surface`] when code wants a reusable
//!   `MenuSurface` owner object;
//! - use the top-level [`api`] helpers when one-off menu-name-based queries are
//!   enough;
//! - prefer `sdk::ui::widgets` / `hud_runtime` / `controller` for deferred
//!   runtime orchestration, and use this module for the actual Scaleform
//!   reads/writes/invokes they eventually perform.

mod api;
mod shared;
mod surface;

/// One-off menu-name-based Scaleform helpers.
pub use api::*;

/// Owner-backed Scaleform surface for an open menu.
pub use surface::*;
