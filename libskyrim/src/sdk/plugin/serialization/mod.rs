//! High-level serialization registration layered over SKSE's low-level cosave
//! interface.
//!
//! This module owns the callback-registration side of serialization. The raw
//! binary format helpers live in `sdk::persistence::cosave`; this layer builds
//! a model/schema API on top so plugin authors can register one persistent
//! model and let the framework drive save/load/revert callbacks.

/// Expand a list of top-level `Schema::value(...)` declarations using field
/// names instead of repetitive getter/setter closures.
#[macro_export]
macro_rules! schema_fields {
    ($schema:ident, { $($record_id:expr => $version:expr => $field:ident),+ $(,)? }) => {{
        let __sdk_schema = &mut $schema;
        $(
            __sdk_schema.value(
                $record_id,
                $version,
                |state| &state.$field,
                |state, value| state.$field = value,
            );
        )+
    }};
    ($schema:expr, { $($record_id:expr => $version:expr => $field:ident),+ $(,)? }) => {{
        let __sdk_schema: &mut _ = $schema;
        $(
            __sdk_schema.value(
                $record_id,
                $version,
                |state| &state.$field,
                |state, value| state.$field = value,
            );
        )+
    }};
}

mod runtime;
mod schema;
#[cfg(test)]
mod tests;
mod types;

pub use crate::schema_fields;
pub use crate::sdk::persistence::cosave;
pub use crate::sdk::persistence::cosave::{
    BoundedVec, BoundedVecError, CosaveDecode, CosaveEncode, LoadContext, LoadError, LoadedRecord,
    OwnedRecord, RecordHeader, RecordId, RecordReader, RecordWriter, ResolvedFormId,
    ResolvedVmHandle, SaveError, UniqueId, fourcc_from_str, read_next_record, record_id, unique_id,
};
pub use libskyrim_macros::Cosave;

pub use runtime::{
    is_registered, last_error, register_model, registered_unique_id, take_last_error,
    unregister_model, with_model, with_model_mut,
};
pub use schema::{MigratingRecordBuilder, Schema, write_value_record};
pub use types::{
    LoadStatus, Model, ModelAccessError, RegistrationError, RuntimeError, SchemaBuildError,
};
