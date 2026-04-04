//! Shared engine-object wrappers for SDK-facing APIs.

mod game_ptr;
mod game_ref;

#[cfg(test)]
mod tests;

pub use game_ptr::GamePtr;
pub use game_ref::GameRef;
