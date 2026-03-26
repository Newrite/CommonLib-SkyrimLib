//!
//! @file lib.rs
//! @author Andrew Spaulding (Kasplat)
//! @brief Keywords/blocks/types which should be in Rust, but aren't.
//!
//! This crate collects low-level helpers that are shared across the project:
//! utility macros, pointer helpers, FFI string helpers, and lightweight
//! synchronization primitives for global state.
//!

#![no_std]

pub use core;

mod cells;
mod macros;
mod ptr;
mod strings;

pub mod enum_set;
pub mod enum_value;

pub use cells::*;
pub use enum_set::*;
pub use enum_value::*;
pub use ptr::*;
pub use strings::*;
