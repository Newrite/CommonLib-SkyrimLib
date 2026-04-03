//! Typed cosave helpers layered over the low-level SKSE serialization ABI.
//!
//! This module provides a length-bounded reader/writer, strongly typed record
//! identifiers, and reusable codecs for common plugin data containers.

/// Compile-time checked four-character record ID literal.
#[macro_export]
macro_rules! record_id {
    ($value:literal) => {
        $crate::sdk::persistence::cosave::RecordId::from_raw(
            $crate::sdk::persistence::cosave::fourcc_from_str($value),
        )
    };
}

/// Compile-time checked four-character plugin unique ID literal.
#[macro_export]
macro_rules! unique_id {
    ($value:literal) => {
        $crate::sdk::persistence::cosave::UniqueId::from_raw(
            $crate::sdk::persistence::cosave::fourcc_from_str($value),
        )
    };
}

mod codecs;
mod io;
mod records;
#[cfg(test)]
mod tests;
mod types;

pub use crate::{record_id, unique_id};

pub use io::{CosaveDecode, CosaveEncode, RecordReader, RecordWriter};
pub use records::{LoadedRecord, OwnedRecord, read_next_record};
pub use types::{
    BoundedVec, BoundedVecError, LoadContext, LoadError, RecordHeader, RecordId, ResolvedFormId,
    ResolvedVmHandle, SaveError, UniqueId, fourcc_from_str,
};
