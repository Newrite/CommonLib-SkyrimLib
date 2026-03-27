//! High-level Rust-first SDK layer built on top of `re` and `skse`.
//!
//! This namespace is intentionally domain-oriented. It is the home for
//! ergonomic, consumer-facing helpers that compose the low-level ABI layers
//! instead of mirroring them one-to-one.

pub mod advanced;
pub mod core;
pub mod events;
pub mod forms;
pub mod gameplay;
pub mod hooks;
pub mod interop;
pub mod papyrus;
pub mod persistence;
pub mod plugin;
pub mod prelude;
pub mod ui;
