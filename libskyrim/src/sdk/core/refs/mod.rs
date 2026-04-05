//! Shared engine-object wrappers for SDK-facing APIs.
//!
//! [`GameRef`] represents a trusted non-null engine pointer. [`GamePtr`] is the
//! nullable companion for APIs that may or may not resolve to a live object.
//! Together they let higher-level SDK modules express pointer expectations
//! without exposing raw `*mut T` everywhere.
//!
//! Use these wrappers when an SDK-facing API:
//!
//! - returns a borrowed singleton or engine-owned object reference
//! - needs nullable object identity without upgrading to a handle
//! - wants pointer equality / hashing semantics without inventing a new wrapper
//!
//! Example:
//!
//! ```rust,ignore
//! use libskyrim::sdk::core::{GamePtr, GameRef};
//! use libskyrim::re::{PlayerCharacter, TESObjectREFR};
//!
//! fn player_and_mount(
//!     player: GameRef<PlayerCharacter>,
//!     mount: GamePtr<TESObjectREFR>,
//! ) -> bool {
//!     !mount.is_null() && player.base().as_ref().is_on_mount()
//! }
//! ```

mod game_ptr;
mod game_ref;

#[cfg(test)]
mod tests;

pub use game_ptr::GamePtr;
pub use game_ref::GameRef;
