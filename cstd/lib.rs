//!
//! Thin `no_std` wrappers for the Skyrim allocator and a small subset of C
//! runtime file and time APIs.
//!
//! The crate is intentionally minimal: it provides enough functionality for
//! allocator setup and basic file I/O without pulling in `std`.
//!

#![no_std]

extern crate alloc;

pub mod io;
pub mod mem;
pub mod time;
