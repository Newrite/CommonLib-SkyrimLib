//! Native smart-pointer traits shared across SDK domains.

mod builtins;
mod traits;

pub use traits::{NativeOwner, NativeOwnerCastExt, NativeOwnerFamily};
