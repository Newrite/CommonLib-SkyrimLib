//! Placeholder for `RE/T/TypeTraits.h`.
//!
//! C++ traits here are compile-time templates; no runtime layout is required.

pub trait IsReturnConvertible {}

impl<T> IsReturnConvertible for T {}
