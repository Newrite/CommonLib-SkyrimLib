//! Menu-oriented high-level helpers.
//!
//! This is the lower-level UI orchestration layer under `widgets`,
//! `hud_runtime`, and `driver`. It exposes the repeated menu-side primitives
//! that plugins and SDK glue code need:
//!
//! - subscribe to open/close events;
//! - queue `UIMessageQueue` requests and typed message payloads;
//! - inspect live menu/UI singletons and top-most menu state;
//! - drive fade/loading transitions;
//! - work with typed menu names and message-data markers.
//!
//! Decision guide:
//!
//! - use [`events`] when a plugin wants to react to open/close transitions;
//! - use [`messages`] when code needs to open/close/reshow menus or send typed
//!   `UIMessageQueue` payloads;
//! - use [`surface`] when runtime code wants live `UI`/`UIMessageQueue` state;
//! - use [`transitions`] for loading/fader menu workflows;
//! - use [`types`] when generic code needs `NamedMenu`, typed message-data
//!   markers, or reusable request payload structs.

mod events;
mod messages;
mod surface;
mod transitions;
mod types;

/// Menu open/close subscription helpers.
pub use events::*;

/// `UIMessageQueue` send/open/close helpers.
pub use messages::*;

/// Live menu/UI singleton and top-most menu inspection helpers.
pub use surface::*;

/// Fader and loading-menu transition helpers.
pub use transitions::*;

/// Shared menu marker traits and request structs.
pub use types::*;
