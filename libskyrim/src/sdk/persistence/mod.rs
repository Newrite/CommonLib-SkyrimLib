//! Persistence helpers for plugin authors.
//!
//! This module is the low-level persistence entrypoint in the SDK.
//! [`cosave`] provides typed building blocks over SKSE's record-oriented save
//! API: strongly typed record IDs, bounded readers/writers, and reusable codecs
//! for common payload shapes.
//!
//! Decision guide:
//!
//! - use [`cosave`] directly when the plugin wants manual control over record
//!   IDs, payload layout, or per-record migration;
//! - use [`crate::sdk::plugin::serialization`] when the plugin prefers one
//!   registered model/schema that the SDK will drive through save/load/revert
//!   callbacks.
//!
//! In practice the split usually looks like this:
//!
//! - choose [`cosave`] for bespoke formats, tooling, migration experiments, or
//!   helper code that reads and writes individual records directly;
//! - choose [`crate::sdk::plugin::serialization`] for the common
//!   "one plugin-owned state object" workflow where bootstrap registers one
//!   model and the SDK handles callback wiring.
//!
//! Raw cosave sketch:
//!
//! ```rust,ignore
//! use libskyrim::sdk::persistence::cosave::{LoadContext, OwnedRecord, RecordId};
//!
//! fn build_record(counter: u32) -> OwnedRecord {
//!     OwnedRecord::from_value(RecordId::from_bytes(*b"CNT1"), 1, &counter).unwrap()
//! }
//!
//! fn decode_record(record: &libskyrim::sdk::persistence::cosave::LoadedRecord) -> u32 {
//!     record.decode::<u32>(LoadContext::empty()).unwrap()
//! }
//! ```
//!
//! Registered-model sketch:
//!
//! ```rust,ignore
//! use libskyrim::sdk::plugin::serialization::{self, Model, Schema};
//!
//! #[derive(Default)]
//! struct SaveState {
//!     counter: u32,
//! }
//!
//! impl Model for SaveState {
//!     const UNIQUE_ID: serialization::UniqueId = serialization::unique_id!("EXMP");
//!
//!     fn schema(schema: &mut Schema<Self>) {
//!         schema.value(serialization::record_id!("CNT1"), 1, |s| &s.counter, |s, v| s.counter = v);
//!     }
//! }
//! ```

/// Typed cosave helpers layered over the raw SKSE serialization ABI.
pub mod cosave;
