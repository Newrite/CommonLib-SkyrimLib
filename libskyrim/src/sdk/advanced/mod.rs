//! Advanced engine-facing SDK domains.
//!
//! These modules are intentionally separate from the early default SDK surface.
//! They are expected to wrap more volatile or engine-heavy families.

pub mod physics;
pub mod render;
pub mod scene;
pub mod vm;
