use core::fmt;

use crate::re::bs_core_types::VMHandle;
use crate::sdk::plugin::serialization::{LoadError, Schema};
use crate::sdk::plugin::serialization::{RecordHeader, RecordId, SaveError, UniqueId};

/// Result from one schema loader when deciding whether it consumed a record.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LoadStatus {
    /// The record/version pair was accepted and consumed by the schema entry.
    ///
    /// Returning this stops later schema candidates for the same record ID from
    /// running.
    Handled,
    /// The schema entry declined the record, allowing later candidates to try.
    ///
    /// This is primarily useful in custom or migrating loaders that only want
    /// to claim specific record versions.
    Unhandled,
}

/// Plugin-owned cosave model driven by [`super::Schema`].
///
/// Implement this when the plugin wants one registered persistent state object
/// that SKSE serialization callbacks can save, load, revert, and update on
/// form deletion.
///
/// The usual lifecycle is:
///
/// - provide [`Default`] as the empty/reverted state
/// - declare one stable [`UniqueId`]
/// - describe records in [`Self::schema`]
/// - optionally react to revert or form-delete callbacks
///
/// ```rust,ignore
/// use libskyrim::sdk::plugin::serialization::{self, Model, Schema};
///
/// #[derive(Default)]
/// struct SaveState {
///     counter: u32,
/// }
///
/// impl Model for SaveState {
///     const UNIQUE_ID: serialization::UniqueId = serialization::unique_id!("EXMP");
///
///     fn schema(schema: &mut Schema<Self>) {
///         schema.value(serialization::record_id!("CNT1"), 1, |s| &s.counter, |s, v| s.counter = v);
///     }
/// }
/// ```
pub trait Model: Default + Send + 'static {
    /// Stable per-plugin unique ID used when registering with SKSE.
    const UNIQUE_ID: UniqueId;

    /// Describe all records used by this model.
    ///
    /// In most plugins this is the only required method beyond [`Default`] and
    /// [`Self::UNIQUE_ID`].
    fn schema(schema: &mut Schema<Self>);

    /// Reset plugin state during revert/new-game style flows.
    ///
    /// The default implementation does nothing because many models can simply
    /// fall back to `Default::default()`. Override this when a reverted model
    /// also needs to clear caches, refresh derived data, or run extra cleanup.
    fn on_revert(&mut self) {}

    /// React to VM/form-handle deletion notifications when needed.
    ///
    /// Override this only when the model stores handles or other data that
    /// should be cleaned up after deletion notifications.
    fn on_form_delete(&mut self, _handle: VMHandle) {}
}

/// Error raised while building a [`Schema`].
///
/// These are structural schema-definition errors detected before registration
/// installs any SKSE callbacks.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SchemaBuildError {
    /// Two schema entries declared the same record ID incompatibly.
    DuplicateRecordId(RecordId),
    /// One record ID declared the same loader version more than once.
    DuplicateRecordVersion { id: RecordId, version: u32 },
}

impl fmt::Display for SchemaBuildError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DuplicateRecordId(id) => write!(f, "duplicate serialization record id {}", id),
            Self::DuplicateRecordVersion { id, version } => write!(
                f,
                "duplicate serialization loader version {} for record {}",
                version, id
            ),
        }
    }
}

impl core::error::Error for SchemaBuildError {}

/// Failure while installing the runtime serialization model.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RegistrationError {
    /// The SKSE serialization interface was unavailable during installation.
    InterfaceUnavailable,
    /// The SDK already has a registered model for this plugin.
    AlreadyRegistered,
    /// The declared schema was invalid.
    Schema(SchemaBuildError),
}

impl fmt::Display for RegistrationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InterfaceUnavailable => write!(f, "SKSE serialization interface is unavailable"),
            Self::AlreadyRegistered => write!(f, "a serialization model is already registered"),
            Self::Schema(error) => write!(f, "invalid serialization schema: {}", error),
        }
    }
}

impl core::error::Error for RegistrationError {}

/// Failure while borrowing the currently registered model.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ModelAccessError {
    /// No model is currently registered.
    NotRegistered,
    /// A model is registered, but it is not of the requested type.
    ///
    /// This usually means the plugin requested a different concrete model type
    /// than the one installed through `register_model::<T>()`.
    WrongModelType,
}

impl fmt::Display for ModelAccessError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotRegistered => write!(f, "no serialization model is registered"),
            Self::WrongModelType => {
                write!(
                    f,
                    "registered serialization model type does not match request"
                )
            }
        }
    }
}

impl core::error::Error for ModelAccessError {}

/// Runtime save/load failure captured by the registered serialization driver.
///
/// These errors are recorded by the installed callback driver and can be
/// inspected later through `last_runtime_error()` or `take_last_runtime_error()`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RuntimeError {
    /// Saving one record failed.
    ///
    /// The `id`/`version` pair identifies which schema entry produced the
    /// failing record.
    SaveRecord {
        id: RecordId,
        version: u32,
        source: SaveError,
    },
    /// Iterating the next record from the SKSE stream failed.
    ///
    /// This usually points at truncated or otherwise malformed on-disk data
    /// before any individual schema entry got a chance to decode it.
    ReadNextRecord { source: LoadError },
    /// Loading one record into the schema/model failed.
    ///
    /// The `header` identifies the on-disk record that caused the typed decode
    /// or schema dispatch failure.
    LoadRecord {
        header: RecordHeader,
        source: LoadError,
    },
}

impl fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SaveRecord {
                id,
                version,
                source,
            } => write!(
                f,
                "failed to save serialization record {} version {}: {}",
                id, version, source
            ),
            Self::ReadNextRecord { source } => {
                write!(
                    f,
                    "failed to read the next serialization record: {}",
                    source
                )
            }
            Self::LoadRecord { header, source } => write!(
                f,
                "failed to load serialization record {} version {}: {}",
                header.id(),
                header.version(),
                source
            ),
        }
    }
}

impl core::error::Error for RuntimeError {}
