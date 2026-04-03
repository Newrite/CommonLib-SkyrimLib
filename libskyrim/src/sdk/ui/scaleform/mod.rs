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

mod api;
mod shared;
mod surface;

pub use api::*;
pub use surface::*;
