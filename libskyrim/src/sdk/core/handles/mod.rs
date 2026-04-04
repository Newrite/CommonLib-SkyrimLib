//! Native handle helpers shared across SDK domains.

mod builtins;
mod resolved;
mod traits;

pub use resolved::{Resolved, ResolvedHandle};
pub use traits::{CanonicalHandle, HandleFamilyTarget, HandleTarget, ResolvableHandle};
