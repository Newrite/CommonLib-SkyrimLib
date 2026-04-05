//! High-level serialization registration layered over SKSE's low-level cosave
//! interface.
//!
//! This module owns the callback-registration side of serialization. The raw
//! binary format helpers live in `sdk::persistence::cosave`; this layer builds
//! a model/schema API on top so plugin authors can register one persistent
//! model and let the framework drive save/load/revert callbacks.
//!
//! The usual split is:
//!
//! - implement [`Model`] for your plugin state
//! - describe the binary layout with [`Schema`] and optional
//!   [`MigratingRecordBuilder`]
//! - install it once through [`register_model`]
//! - access runtime state through [`with_registered_model`] or
//!   [`with_registered_model_mut`]
//!
//! Decision guide:
//!
//! - implement [`Model`] when the plugin owns one persistent runtime state
//!   object that should be saved, loaded, reverted, and updated on form-delete
//! - use [`Schema`] plus `schema_fields!` when the state is largely value-like
//!   and maps cleanly onto named records
//! - use [`MigratingRecordBuilder`] when old record versions still need custom
//!   migration logic instead of a simple same-version load
//! - use [`with_registered_model`] / [`with_registered_model_mut`] when
//!   runtime code needs to borrow the installed model, not create a new one
//! - use [`last_runtime_error`] / [`take_last_runtime_error`] when the plugin
//!   wants to surface save/load failures in logs or debug UIs
//! - stay in [`crate::sdk::persistence::cosave`] instead when the plugin wants
//!   to iterate records manually, keep several unrelated state roots, or own
//!   the full save/load loop itself
//!
//! Typical install flow:
//!
//! 1. implement [`Model`] for one plugin-owned state type
//! 2. register it once through [`register_model`]
//! 3. let SKSE drive save/load/revert/form-delete callbacks
//! 4. borrow the live model later through [`with_registered_model`] or
//!    [`with_registered_model_mut`]
//!
//! Example:
//!
//! ```rust,ignore
//! use libskyrim::sdk::plugin::serialization;
//!
//! #[derive(Default, serialization::Cosave)]
//! struct SaveState {
//!     counter: u32,
//! }
//!
//! impl serialization::Model for SaveState {
//!     const UNIQUE_ID: serialization::UniqueId = serialization::unique_id!("EXMP");
//!
//!     fn schema(schema: &mut serialization::Schema<Self>) {
//!         serialization::schema_fields!(schema, {
//!             serialization::record_id!("CNT1") => 1 => counter,
//!         });
//!     }
//! }
//!
//! fn install_serialization() -> Result<(), serialization::RegistrationError> {
//!     serialization::register_model::<SaveState>()
//! }
//! ```
//!
//! Runtime borrow/diagnostics sketch:
//!
//! ```rust,ignore
//! use libskyrim::sdk::plugin::serialization;
//!
//! fn bump_counter() {
//!     serialization::with_registered_model_mut::<SaveState, _>(|state| {
//!         state.counter += 1;
//!     })
//!     .unwrap();
//! }
//!
//! fn log_last_failure() {
//!     if let Some(error) = serialization::take_last_runtime_error() {
//!         libskyrim::sdk::plugin::log::error(format!("serialization failed: {error}"));
//!     }
//! }
//! ```

/// Expand a list of top-level `Schema::value(...)` declarations using field
/// names instead of repetitive getter/setter closures.
///
/// This is the shortest way to describe plain value-like records in one
/// [`Schema`](crate::sdk::plugin::serialization::Schema).
///
/// ```rust,ignore
/// use libskyrim::sdk::plugin::serialization::{self, Schema};
///
/// #[derive(Default)]
/// struct SaveState {
///     counter: u32,
///     enabled: bool,
/// }
///
/// fn build_schema(schema: &mut Schema<SaveState>) {
///     serialization::schema_fields!(schema, {
///         serialization::record_id!("CNT1") => 1 => counter,
///         serialization::record_id!("ENBL") => 1 => enabled,
///     });
/// }
/// ```
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

/// Schema-definition helper macro for field-backed records.
pub use crate::schema_fields;

/// Raw typed cosave helpers re-exported for convenience from the persistence
/// layer.
pub use crate::sdk::persistence::cosave;

/// Low-level record identifiers, codecs, readers, writers, and errors shared by
/// the higher-level model/schema API.
pub use crate::sdk::persistence::cosave::{
    BoundedVec, BoundedVecError, CosaveDecode, CosaveEncode, LoadContext, LoadError, LoadedRecord,
    OwnedRecord, RecordHeader, RecordId, RecordReader, RecordWriter, ResolvedFormId,
    ResolvedVmHandle, SaveError, UniqueId, fourcc_from_str, read_next_record, record_id, unique_id,
};

/// Derive macro for straightforward `CosaveEncode` / `CosaveDecode` data types.
pub use libskyrim_macros::Cosave;

/// Runtime registration and installed-model access helpers.
pub use runtime::{
    has_registered_model, last_runtime_error, register_model, registered_model_unique_id,
    take_last_runtime_error, unregister_model, with_registered_model, with_registered_model_mut,
};

/// Declarative schema builders and standalone record helpers.
pub use schema::{MigratingRecordBuilder, Schema, write_value_record};

/// Core traits and error types for the model/schema layer.
pub use types::{
    LoadStatus, Model, ModelAccessError, RegistrationError, RuntimeError, SchemaBuildError,
};
