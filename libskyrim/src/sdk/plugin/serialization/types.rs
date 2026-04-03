use core::fmt;

use crate::re::bs_core_types::VMHandle;
use crate::sdk::plugin::serialization::{LoadError, Schema};
use crate::sdk::plugin::serialization::{RecordHeader, RecordId, SaveError, UniqueId};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LoadStatus {
    Handled,
    Unhandled,
}

pub trait Model: Default + Send + 'static {
    const UNIQUE_ID: UniqueId;

    fn schema(schema: &mut Schema<Self>);

    fn on_revert(&mut self) {}

    fn on_form_delete(&mut self, _handle: VMHandle) {}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SchemaBuildError {
    DuplicateRecordId(RecordId),
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RegistrationError {
    InterfaceUnavailable,
    AlreadyRegistered,
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ModelAccessError {
    NotRegistered,
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RuntimeError {
    SaveRecord {
        id: RecordId,
        version: u32,
        source: SaveError,
    },
    ReadNextRecord {
        source: LoadError,
    },
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
