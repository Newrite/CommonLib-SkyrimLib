//! Dynamic cast helpers shared across SDK domains.

mod builtins;
mod resolved;
mod traits;

#[cfg(test)]
mod tests;

pub use traits::{ConstRttiCastSource, DynamicCastExt, DynamicCastMutExt, MutRttiCastSource};
