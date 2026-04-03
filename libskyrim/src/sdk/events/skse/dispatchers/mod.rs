//! High-level helpers for SKSE dispatcher-backed `BSTEventSource<T>`.
//!
//! This domain is intended for events retrieved through the SKSE API storage,
//! for example `ModCallbackEvent`, `CameraEvent`, `CrosshairRefEvent`,
//! `ActionEvent`, and `NiNodeUpdateEvent`.

mod api;
mod registry;

pub use api::{prepend, subscribe};
pub use registry::DispatcherEvent;
