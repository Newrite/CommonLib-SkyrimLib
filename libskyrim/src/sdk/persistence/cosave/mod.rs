//! Typed cosave helpers layered over the low-level SKSE serialization ABI.
//!
//! This module provides a length-bounded reader/writer, strongly typed record
//! identifiers, and reusable codecs for common plugin data containers.
//!
//! Decision guide:
//!
//! - use [`RecordWriter`] plus [`CosaveEncode`] when save code is assembling one
//!   payload incrementally or wants to append several typed fragments manually;
//! - use [`OwnedRecord::from_value`] when one value already implements
//!   [`CosaveEncode`] and should become a whole record directly;
//! - use [`read_next_record`] plus [`LoadedRecord::decode`] when load code wants
//!   a simple record-by-record loop with one typed payload per record;
//! - use [`LoadedRecord::reader`] when the payload shape is partially dynamic or
//!   should be decoded incrementally;
//! - use [`ResolvedFormId`] / [`ResolvedVmHandle`] together with [`LoadContext`]
//!   when persisted identifiers must be resolved for the current runtime;
//! - use [`crate::sdk::plugin::serialization`] instead when the plugin prefers
//!   one registered model/schema instead of manual record loops.
//!
//! Typical flow:
//!
//! - during save, encode one typed value into a [`RecordWriter`] or directly
//!   into an [`OwnedRecord`];
//! - during load, pull each raw record with [`read_next_record`], then decode
//!   its payload through [`LoadedRecord::decode`];
//! - use [`LoadContext`] whenever record payloads contain saved `FormID`s or
//!   `VMHandle`s that must be resolved through `SKSE::SerializationInterface`.
//!
//! Manual record-loop sketch:
//!
//! ```rust,ignore
//! use libskyrim::sdk::persistence::cosave::{
//!     LoadContext, LoadedRecord, OwnedRecord, RecordId, read_next_record,
//! };
//!
//! fn save_counter(counter: u32) -> OwnedRecord {
//!     OwnedRecord::from_value(RecordId::from_bytes(*b"CNT1"), 1, &counter).unwrap()
//! }
//!
//! fn load_all(
//!     serialization: &libskyrim::skse::SerializationInterface,
//! ) -> Result<Vec<LoadedRecord>, libskyrim::sdk::persistence::cosave::LoadError> {
//!     let mut records = Vec::new();
//!     while let Some(record) = read_next_record(serialization)? {
//!         records.push(record);
//!     }
//!     Ok(records)
//! }
//!
//! fn decode_counter(record: &LoadedRecord) -> u32 {
//!     record.decode::<u32>(LoadContext::empty()).unwrap()
//! }
//! ```

/// Compile-time checked four-character record ID literal.
///
/// Prefer this over raw `u32` literals when declaring stable record IDs in
/// source so the four-character width and ASCII encoding stay validated at
/// compile time.
///
/// ```rust,ignore
/// let id = libskyrim::sdk::persistence::cosave::record_id!("CNT1");
/// assert_eq!(id.to_bytes(), *b"CNT1");
/// ```
#[macro_export]
macro_rules! record_id {
    ($value:literal) => {
        $crate::sdk::persistence::cosave::RecordId::from_raw(
            $crate::sdk::persistence::cosave::fourcc_from_str($value),
        )
    };
}

/// Compile-time checked four-character plugin unique ID literal.
///
/// This is the usual source-level declaration form for
/// [`crate::sdk::plugin::serialization::Model::UNIQUE_ID`].
///
/// ```rust,ignore
/// let uid = libskyrim::sdk::persistence::cosave::unique_id!("EXMP");
/// assert_eq!(uid.to_bytes(), *b"EXMP");
/// ```
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

/// Codec traits and byte-oriented record reader/writer helpers.
pub use io::{CosaveDecode, CosaveEncode, RecordReader, RecordWriter};

/// High-level record wrappers and the low-level record-pull helper.
pub use records::{LoadedRecord, OwnedRecord, read_next_record};

/// Shared cosave identifiers, bounded containers, contexts, and error types.
pub use types::{
    BoundedVec, BoundedVecError, LoadContext, LoadError, RecordHeader, RecordId, ResolvedFormId,
    ResolvedVmHandle, SaveError, UniqueId, fourcc_from_str,
};
