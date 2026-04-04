//! Defensive helpers for iterating contiguous engine-owned containers.
//!
//! These helpers live in the SDK layer on purpose: they provide reusable
//! defensive traversal over raw RE container storage without pretending the
//! underlying RE types expose a universally safe slice API.

mod builtins;
mod snapshot;
mod traversal;
mod types;

#[cfg(test)]
mod tests;

pub use snapshot::{
    snapshot_contiguous_cloned, snapshot_contiguous_cloned_named, snapshot_contiguous_copied,
    snapshot_contiguous_copied_named,
};
pub use traversal::{
    contiguous_sequence_bounds, contiguous_sequence_bounds_named, for_each_contiguous_sequence,
    for_each_contiguous_sequence_named,
};
pub use types::{ContiguousSequence, ContiguousSequenceIterationOptions};
